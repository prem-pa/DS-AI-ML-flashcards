//! Ollama HTTP client. Streams NDJSON from `/api/generate` (single-prompt) or
//! `/api/chat` (message list) on a background thread; the UI loop drains
//! `std::sync::mpsc` events alongside crossterm input.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::io::{BufRead, BufReader};
use std::sync::mpsc::{self, Receiver, Sender, TryRecvError};
use std::thread::{self, JoinHandle};
use std::time::Duration;

#[derive(Debug)]
pub enum Event {
    /// Newly streamed token text. Append to the in-progress buffer.
    Token(String),
    /// Stream completed cleanly. Carries the final assembled body.
    Done(String),
    /// Underlying request failed (Ollama unreachable, model missing, etc.).
    Error(String),
}

/// Handle to a streaming generation. Drop to abandon (the worker still runs
/// to completion in the background but its events are silently discarded).
pub struct StreamHandle {
    rx: Receiver<Event>,
    /// Detached on drop; we don't join because tokio-style cancellation isn't
    /// needed for our use case (model finishes in seconds, then exits).
    _worker: Option<JoinHandle<()>>,
}

impl StreamHandle {
    /// Drain any pending events without blocking. Returns `(tokens, terminal)`
    /// where `terminal` is `Some(Done|Error)` once the stream has ended.
    pub fn poll(&mut self) -> (Vec<String>, Option<Event>) {
        let mut tokens = Vec::new();
        let mut terminal = None;
        loop {
            match self.rx.try_recv() {
                Ok(Event::Token(t)) => tokens.push(t),
                Ok(ev @ Event::Done(_)) | Ok(ev @ Event::Error(_)) => {
                    terminal = Some(ev);
                    break;
                }
                Err(TryRecvError::Empty) | Err(TryRecvError::Disconnected) => break,
            }
        }
        (tokens, terminal)
    }
}

#[derive(Serialize)]
struct GenerateRequest<'a> {
    model: &'a str,
    prompt: &'a str,
    stream: bool,
    options: GenOptions,
}

#[derive(Serialize)]
struct GenOptions {
    num_predict: i32,
    temperature: f32,
}

#[derive(Deserialize)]
struct GenerateChunk {
    #[serde(default)]
    response: String,
    #[serde(default)]
    done: bool,
    #[serde(default)]
    error: Option<String>,
}

/// Fire-and-forget streaming generate. The worker thread sends `Event::Token`
/// for each NDJSON chunk and `Event::Done` (or `Event::Error`) when it ends.
pub fn generate_stream(endpoint: &str, model: &str, prompt: String) -> StreamHandle {
    let (tx, rx) = mpsc::channel();
    let endpoint = endpoint.trim_end_matches('/').to_string();
    let model = model.to_string();
    let worker = thread::spawn(move || {
        if let Err(e) = run_generate(&endpoint, &model, &prompt, &tx) {
            let _ = tx.send(Event::Error(format!("{e:#}")));
        }
    });
    StreamHandle {
        rx,
        _worker: Some(worker),
    }
}

fn run_generate(endpoint: &str, model: &str, prompt: &str, tx: &Sender<Event>) -> Result<()> {
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(120))
        .connect_timeout(Duration::from_secs(3))
        .build()
        .context("building http client")?;
    let body = GenerateRequest {
        model,
        prompt,
        stream: true,
        options: GenOptions {
            num_predict: 220,
            temperature: 0.2,
        },
    };
    let resp = client
        .post(format!("{endpoint}/api/generate"))
        .json(&body)
        .send()
        .with_context(|| format!("POST {endpoint}/api/generate"))?;

    if !resp.status().is_success() {
        let status = resp.status();
        let txt = resp.text().unwrap_or_default();
        anyhow::bail!("ollama returned {status}: {}", truncate(&txt, 240));
    }

    let mut full = String::new();
    let reader = BufReader::new(resp);
    for line in reader.lines() {
        let line = line.context("reading streamed line")?;
        if line.trim().is_empty() {
            continue;
        }
        let chunk: GenerateChunk = match serde_json::from_str(&line) {
            Ok(c) => c,
            Err(e) => anyhow::bail!("bad NDJSON line ({e}): {}", truncate(&line, 240)),
        };
        if let Some(err) = chunk.error {
            anyhow::bail!("{}", err);
        }
        if !chunk.response.is_empty() {
            full.push_str(&chunk.response);
            // best-effort send; if the receiver has been dropped, abandon.
            if tx.send(Event::Token(chunk.response)).is_err() {
                return Ok(());
            }
        }
        if chunk.done {
            let _ = tx.send(Event::Done(full));
            return Ok(());
        }
    }
    // Server closed the stream without a `done` chunk; treat what we have as final.
    let _ = tx.send(Event::Done(full));
    Ok(())
}

/// Cheap reachability check via `GET /api/tags`. Returns `Ok(true)` if Ollama
/// is up at the endpoint, `Ok(false)` if reachable but errored, and `Err` if
/// the network call itself failed.
pub fn ping(endpoint: &str) -> Result<bool> {
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(2))
        .build()
        .context("building http client")?;
    let url = format!("{}/api/tags", endpoint.trim_end_matches('/'));
    let resp = client
        .get(&url)
        .send()
        .with_context(|| format!("GET {url}"))?;
    Ok(resp.status().is_success())
}

#[derive(Deserialize)]
struct TagsResponse {
    #[serde(default)]
    models: Vec<TagModel>,
}

#[derive(Deserialize)]
struct TagModel {
    #[serde(default)]
    name: String,
}

/// List installed model names. Returns an empty Vec on connect-but-error;
/// `Err` only on network failure.
pub fn list_models(endpoint: &str) -> Result<Vec<String>> {
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(3))
        .build()?;
    let url = format!("{}/api/tags", endpoint.trim_end_matches('/'));
    let resp = client.get(&url).send().with_context(|| format!("GET {url}"))?;
    if !resp.status().is_success() {
        return Ok(Vec::new());
    }
    let parsed: TagsResponse = resp.json().context("parsing /api/tags response")?;
    Ok(parsed.models.into_iter().map(|m| m.name).collect())
}

fn truncate(s: &str, n: usize) -> String {
    if s.chars().count() <= n {
        s.to_string()
    } else {
        s.chars().take(n).collect::<String>() + "…"
    }
}

// ---------- chat (multi-turn) ----------

#[derive(Serialize)]
struct ChatRequest<'a> {
    model: &'a str,
    messages: &'a [ChatMessage<'a>],
    stream: bool,
    options: GenOptions,
}

#[derive(Serialize, Clone)]
pub struct ChatMessage<'a> {
    pub role: &'a str, // "system" | "user" | "assistant"
    pub content: &'a str,
}

#[derive(Deserialize)]
struct ChatChunk {
    #[serde(default)]
    message: Option<ChatChunkMessage>,
    #[serde(default)]
    done: bool,
    #[serde(default)]
    error: Option<String>,
}

#[derive(Deserialize)]
struct ChatChunkMessage {
    #[serde(default)]
    content: String,
}

pub fn chat_stream(endpoint: &str, model: &str, messages: Vec<OwnedChatMessage>) -> StreamHandle {
    let (tx, rx) = mpsc::channel();
    let endpoint = endpoint.trim_end_matches('/').to_string();
    let model = model.to_string();
    let worker = thread::spawn(move || {
        if let Err(e) = run_chat(&endpoint, &model, &messages, &tx) {
            let _ = tx.send(Event::Error(format!("{e:#}")));
        }
    });
    StreamHandle {
        rx,
        _worker: Some(worker),
    }
}

#[derive(Clone)]
pub struct OwnedChatMessage {
    pub role: String,
    pub content: String,
}

fn run_chat(
    endpoint: &str,
    model: &str,
    messages: &[OwnedChatMessage],
    tx: &Sender<Event>,
) -> Result<()> {
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(180))
        .connect_timeout(Duration::from_secs(3))
        .build()?;
    let borrowed: Vec<ChatMessage> = messages
        .iter()
        .map(|m| ChatMessage {
            role: m.role.as_str(),
            content: m.content.as_str(),
        })
        .collect();
    let body = ChatRequest {
        model,
        messages: &borrowed,
        stream: true,
        options: GenOptions {
            num_predict: 400,
            temperature: 0.3,
        },
    };
    let resp = client
        .post(format!("{endpoint}/api/chat"))
        .json(&body)
        .send()
        .with_context(|| format!("POST {endpoint}/api/chat"))?;
    if !resp.status().is_success() {
        let status = resp.status();
        let txt = resp.text().unwrap_or_default();
        anyhow::bail!("ollama returned {status}: {}", truncate(&txt, 240));
    }
    let mut full = String::new();
    let reader = BufReader::new(resp);
    for line in reader.lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }
        let chunk: ChatChunk = match serde_json::from_str(&line) {
            Ok(c) => c,
            Err(e) => anyhow::bail!("bad NDJSON line ({e}): {}", truncate(&line, 240)),
        };
        if let Some(err) = chunk.error {
            anyhow::bail!("{}", err);
        }
        if let Some(m) = chunk.message {
            if !m.content.is_empty() {
                full.push_str(&m.content);
                if tx.send(Event::Token(m.content)).is_err() {
                    return Ok(());
                }
            }
        }
        if chunk.done {
            let _ = tx.send(Event::Done(full));
            return Ok(());
        }
    }
    let _ = tx.send(Event::Done(full));
    Ok(())
}

//! Ollama integration: blocking HTTP on a worker thread, NDJSON streaming
//! into the UI loop via `std::sync::mpsc`. No tokio runtime — keeps the rest
//! of the app sync.

pub mod client;
pub mod context;
pub mod prompt;

pub use client::{ping, Event, StreamHandle};

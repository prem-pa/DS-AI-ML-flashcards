//! Prompt builders. Designed so:
//!  - the MCQ explanation is option-agnostic — same response regardless of the
//!    user's pick; the UI just highlights the picked option's section.
//!  - the response is compact (~80 words) so latency stays in the 2-3 s range
//!    on phi4-mini.

use crate::llm::context::Bundle;

const MCQ_BASE_RULES: &str = "\
You are tutoring an interview candidate on data science / ML / AI engineering.

Hard rules:
- Output STRICTLY the markdown structure given in OUTPUT FORMAT, nothing else.
- Total length: <= 90 words across all bullets combined.
- Never contradict the REFERENCE ANSWER. If a 'wrong' option is partially right,
  acknowledge briefly without weakening the correct answer.
- Be concrete. No filler like 'great question' or 'in summary'.
";

/// Build an MCQ explanation prompt that's aware of the user's pick:
///   - `Some(key)` matches the correct option → confirm + add a tight rationale.
///   - `Some(key)` is wrong → contrast the user's pick against the correct one.
///   - `None`     → option-agnostic breakdown (used by the on-demand `?` path).
pub fn build_mcq_prompt(bundle: &Bundle, picked_key: Option<&str>) -> String {
    let card = &bundle.card;
    let mut s = String::with_capacity(2048);
    s.push_str(MCQ_BASE_RULES);
    s.push_str("\nCARD\n");
    s.push_str(&format!("concept: {}\n", card.concept_title));
    s.push_str(&format!("difficulty: {}/5\n", card.difficulty));
    if !bundle.tags.is_empty() {
        s.push_str(&format!("tags: {}\n", bundle.tags.join(", ")));
    }
    s.push_str(&format!("\nQ: {}\n\nOPTIONS:\n", card.front.trim()));
    let mut correct_key = String::from("?");
    for ch in &card.choices {
        s.push_str(&format!("{}) {}\n", ch.key.to_uppercase(), ch.text));
        if ch.correct {
            correct_key = ch.key.to_uppercase();
        }
    }
    s.push_str("\nREFERENCE ANSWER (ground truth — never contradict):\n");
    s.push_str(card.back.trim());
    s.push_str("\n\n");

    if !bundle.neighbors.is_empty() {
        s.push_str("NEARBY CONCEPTS (context only — do not lecture from these):\n");
        for n in &bundle.neighbors {
            let one_line = first_meaningful_line(&n.intuition);
            if one_line.is_empty() {
                s.push_str(&format!("- {}\n", n.title));
            } else {
                s.push_str(&format!("- {}: {}\n", n.title, one_line));
            }
        }
        s.push('\n');
    }

    let picked_upper = picked_key.map(|k| k.to_ascii_uppercase());
    match picked_upper.as_deref() {
        Some(p) if p == correct_key => {
            s.push_str(&format!(
                "USER PICKED: {}  (CORRECT)\n\nOUTPUT FORMAT (markdown, no preamble):\n",
                p
            ));
            s.push_str(&format!(
                "**Correct: {}** — yes, that's the right pick.\n",
                p
            ));
            s.push_str("**Why it's right:** <≤25-word rationale grounded in the reference answer>\n");
            s.push_str("**Watch out for:** <≤15-word note on a common confusion or edge case>\n");
        }
        Some(p) => {
            s.push_str(&format!(
                "USER PICKED: {}  (INCORRECT — correct is {})\n\nOUTPUT FORMAT (markdown, no preamble):\n",
                p, correct_key
            ));
            s.push_str(&format!(
                "**You picked {} — not quite.** <≤15-word reason it misses>\n",
                p
            ));
            s.push_str(&format!(
                "**Correct: {}** — <≤25-word rationale grounded in the reference answer>\n",
                correct_key
            ));
            s.push_str("**To remember:** <≤15-word distinguishing rule>\n");
        }
        None => {
            s.push_str("USER HAS NOT PICKED YET (give a pick-agnostic breakdown).\n\nOUTPUT FORMAT (markdown, no preamble):\n");
            s.push_str(&format!(
                "**Correct: {}** — <one tight sentence>\n",
                correct_key
            ));
            s.push_str("**Why others miss:**\n");
            for ch in &card.choices {
                let k = ch.key.to_uppercase();
                if k != correct_key {
                    s.push_str(&format!("- {}: <≤14-word reason>\n", k));
                }
            }
        }
    }
    s
}

/// Build a HINT prompt. Strict: the model is told to never reveal the answer
/// or which option is right/wrong. Reference answer is intentionally NOT
/// included so the model can't accidentally leak it. We rely on the nearby
/// concepts + the question itself for grounding.
pub fn build_hint_prompt(bundle: &Bundle) -> String {
    let card = &bundle.card;
    let mut s = String::with_capacity(1024);
    s.push_str("\
You are giving a HINT, not an answer.

HARD RULES (violating any is a failure):
- DO NOT reveal the answer.
- DO NOT say which option is correct or which is wrong.
- DO NOT use phrases like 'the answer is', 'the correct option is', or 'X is right/wrong'.
- DO NOT eliminate options by name.
- Output ONE short paragraph, <= 50 words, no preamble, no bullets.
- Be concrete: name a relevant concept or property to consider, or a definition the user should recall.
");
    s.push_str(&format!("\nCONCEPT: {}\n", card.concept_title));
    s.push_str(&format!("Q: {}\n", card.front.trim()));
    if card.kind == "mcq" {
        s.push_str("\nOPTIONS (visible to the user — do NOT pick or eliminate any by letter):\n");
        for ch in &card.choices {
            s.push_str(&format!("{}) {}\n", ch.key.to_uppercase(), ch.text));
        }
    }
    if !bundle.tags.is_empty() {
        s.push_str(&format!("\nTAGS: {}\n", bundle.tags.join(", ")));
    }
    if !bundle.neighbors.is_empty() {
        s.push_str("\nNEARBY CONCEPTS (use as inspiration to nudge the user):\n");
        for n in &bundle.neighbors {
            let one_line = first_meaningful_line(&n.intuition);
            if one_line.is_empty() {
                s.push_str(&format!("- {}\n", n.title));
            } else {
                s.push_str(&format!("- {}: {}\n", n.title, one_line));
            }
        }
    }
    s.push_str("\nNow output the hint (one short paragraph, no spoilers):\n");
    s
}

/// Snapshot of the user's MCQ pick at the moment chat is opened. Used to
/// seed both the (hidden) system message and the (visible) primer line.
#[derive(Debug, Clone)]
pub struct PickContext {
    pub picked_key: String,
    pub picked_text: String,
    pub correct_key: String,
    pub correct_text: String,
    pub was_correct: bool,
}

/// User-visible primer that opens a fresh chat thread. Shows the question,
/// the pick (when present), and the verdict; ends with an open invite.
pub fn build_chat_primer(bundle: &Bundle, pick: Option<&PickContext>) -> String {
    let card = &bundle.card;
    let mut s = String::with_capacity(512);
    s.push_str(&format!("**{}**\n\n", card.concept_title));
    s.push_str(&format!("Q: {}\n", card.front.trim()));
    if let Some(p) = pick {
        let verdict = if p.was_correct { "✓ correct" } else { "✗ not quite" };
        s.push_str(&format!(
            "\nYou picked **{}**) {}  —  {}\n",
            p.picked_key.to_uppercase(),
            p.picked_text,
            verdict
        ));
        if !p.was_correct {
            s.push_str(&format!(
                "Correct: **{}**) {}\n",
                p.correct_key.to_uppercase(),
                p.correct_text
            ));
        }
    } else if !card.back.trim().is_empty() {
        s.push_str(&format!("A: {}\n", card.back.trim()));
    }
    s.push_str("\nWhat would you like to talk more about?");
    s.push_str(" _(e.g. \"why is option B wrong?\", \"when does this fail?\", \"how does this relate to {topic}?\")_");
    s
}

/// System prompt for chat. Keeps the model anchored to the card + the KB
/// context; subsequent user turns are appended by the caller. When `pick`
/// is supplied, the model also gets the user's pick + verdict so follow-ups
/// can reference it without the user having to restate.
pub fn build_chat_system_with_pick(bundle: &Bundle, pick: Option<&PickContext>) -> String {
    let mut s = build_chat_system(bundle);
    if let Some(p) = pick {
        s.push_str("\n--- USER'S PICK (for this session) ---\n");
        s.push_str(&format!(
            "picked: {}) {}\n",
            p.picked_key.to_uppercase(),
            p.picked_text
        ));
        s.push_str(&format!(
            "correct: {}) {}\n",
            p.correct_key.to_uppercase(),
            p.correct_text
        ));
        s.push_str(&format!(
            "verdict: {}\n",
            if p.was_correct { "correct" } else { "incorrect" }
        ));
        s.push_str(
            "Reference this when the user says 'why is mine wrong?' or 'why is the right one right?' \
             — don't make them restate.\n",
        );
    }
    s
}

pub fn build_chat_system(bundle: &Bundle) -> String {
    let card = &bundle.card;
    let mut s = String::with_capacity(1024);
    s.push_str("\
You are a helpful, concise tutor for ML / DS / AI engineering interview prep.
Keep replies short (3-6 sentences) unless the user explicitly asks for depth.
Use the card's REFERENCE ANSWER as ground truth. If a question is outside the
card and nearby concepts, say so briefly instead of speculating.
");
    s.push_str("\n--- CARD CONTEXT ---\n");
    s.push_str(&format!("concept: {}\n", card.concept_title));
    s.push_str(&format!("Q: {}\n", card.front.trim()));
    s.push_str(&format!("A: {}\n", card.back.trim()));
    if !bundle.tags.is_empty() {
        s.push_str(&format!("tags: {}\n", bundle.tags.join(", ")));
    }
    if !bundle.neighbors.is_empty() {
        s.push_str("nearby concepts:\n");
        for n in &bundle.neighbors {
            let one_line = first_meaningful_line(&n.intuition);
            if one_line.is_empty() {
                s.push_str(&format!("- {}\n", n.title));
            } else {
                s.push_str(&format!("- {}: {}\n", n.title, one_line));
            }
        }
    }
    s
}

fn first_meaningful_line(s: &str) -> String {
    for line in s.lines() {
        let l = line.trim();
        if !l.is_empty() && !l.starts_with('#') {
            return truncate_chars(l, 140);
        }
    }
    String::new()
}

fn truncate_chars(s: &str, n: usize) -> String {
    if s.chars().count() <= n {
        s.to_string()
    } else {
        let mut out: String = s.chars().take(n.saturating_sub(1)).collect();
        out.push('…');
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::{CardChoice, CardView};
    use crate::llm::context::{Bundle, Neighbor};

    fn sample_bundle() -> Bundle {
        Bundle {
            card: CardView {
                id: "c1".into(),
                concept_id: "k1".into(),
                concept_title: "Bias-Variance Tradeoff".into(),
                concept_slug: "bias-variance-tradeoff".into(),
                track: "data-scientist".into(),
                topic: "statistics".into(),
                difficulty: 2,
                kind: "mcq".into(),
                front: "Which best describes high bias?".into(),
                back: "Underfitting; both train and test errors are high.".into(),
                choices: vec![
                    CardChoice { key: "a".into(), text: "Underfitting".into(), correct: true },
                    CardChoice { key: "b".into(), text: "Overfitting".into(), correct: false },
                    CardChoice { key: "c".into(), text: "Perfect fit".into(), correct: false },
                    CardChoice { key: "d".into(), text: "No model".into(), correct: false },
                ],
                due: 0, reps: 0, lapses: 0, state: 0, last_review: None,
            },
            neighbors: vec![Neighbor {
                title: "Variance".into(),
                intuition: "Sensitivity to training noise — model wobbles per dataset draw.".into(),
            }],
            tags: vec!["stats".into(), "generalization".into()],
        }
    }

    #[test]
    fn mcq_prompt_pre_pick_lists_all_options() {
        let p = build_mcq_prompt(&sample_bundle(), None);
        assert!(p.contains("Q: Which best describes high bias?"));
        assert!(p.contains("A) Underfitting"));
        assert!(p.contains("B) Overfitting"));
        assert!(p.contains("REFERENCE ANSWER"));
        assert!(p.contains("HAS NOT PICKED"));
        assert!(p.contains("**Correct: A**"));
        for k in ["B:", "C:", "D:"] {
            assert!(p.contains(k), "{p}");
        }
    }

    #[test]
    fn mcq_prompt_correct_pick_says_so() {
        let p = build_mcq_prompt(&sample_bundle(), Some("a"));
        assert!(p.contains("USER PICKED: A  (CORRECT)"));
        assert!(p.contains("yes, that's the right pick"));
    }

    #[test]
    fn mcq_prompt_wrong_pick_contrasts() {
        let p = build_mcq_prompt(&sample_bundle(), Some("b"));
        assert!(p.contains("USER PICKED: B  (INCORRECT — correct is A)"));
        assert!(p.contains("**You picked B"));
        assert!(p.contains("**Correct: A**"));
    }

    #[test]
    fn chat_system_grounds_to_card() {
        let s = build_chat_system(&sample_bundle());
        assert!(s.contains("Bias-Variance Tradeoff"));
        assert!(s.contains("Q: Which best describes high bias?"));
        assert!(s.contains("Variance"));
    }

    #[test]
    fn hint_prompt_excludes_reference_answer() {
        let p = build_hint_prompt(&sample_bundle());
        // The reference answer text must not be passed to the model.
        assert!(!p.contains("Underfitting; both train and test errors are high."));
        assert!(p.contains("HARD RULES"));
        assert!(p.contains("DO NOT reveal the answer"));
        assert!(p.contains("Q: Which best describes high bias?"));
        // options included so the hint can reference the framing without naming
        assert!(p.contains("A) Underfitting"));
    }
}

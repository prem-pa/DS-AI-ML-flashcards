---
id: be2c2ae8-b829-49c9-8dbc-911d0d8979e8
title: Leaderboards (Open LLM Leaderboard, LMSYS)
track: ai-llm-engineer
topic: evaluation-benchmarking
difficulty: 1
tags:
- leaderboards
- rankings
- model-comparison
- arena
- elo-ratings
aliases:
- HuggingFace Leaderboard
- Chatbot Arena
- open LLM rankings
sources:
- url: https://www.meta-intelligence.tech/en/insight-llm-evaluation
  label: LLM Evaluation Framework 2026
- url: https://arxiv.org/abs/2306.05685
  label: Judging LLM-as-Judge with MT-Bench and Chatbot Arena
- url: https://www.vellum.ai/llm-leaderboard
  label: LLM Leaderboard 2026 — Vellum
- url: https://llm-stats.com/
  label: LLM Leaderboard 2026 — 300+ Top Models
cards:
- id: 61107dd8-4f8d-47d9-bc3f-884e61bd64d3
  type: flip
  front: What is the primary limitation of aggregating multiple benchmarks into a single leaderboard score?
  back: Equal weighting masks failure modes. A model weak in reasoning but strong in knowledge gets inflated
    scores. Different models excel at different tasks; single rankings hide this diversity.
- id: 4a749566-4de0-4bbc-803e-fd3db4d78a96
  type: mcq
  front: Chatbot Arena uses which ranking system to convert pairwise votes into model ratings?
  back: Chatbot Arena uses the same Elo system as competitive chess, adapted for pairwise model comparisons.
  choices:
  - key: a
    text: Logistic regression
    correct: false
  - key: b
    text: Bradley-Terry model (chess Elo)
    correct: true
  - key: c
    text: Simple win percentage
    correct: false
  - key: d
    text: Trueskill
    correct: false
- id: 8543fa9d-c99a-460f-bedc-b0825832f394
  type: flip
  front: Why do Chatbot Arena rankings sometimes diverge from Open LLM Leaderboard rankings?
  back: Leaderboards optimize for aggregate benchmark scores; Arena optimizes for human preference. A
    model strong in reasoning (MATH) may rank lower if users prefer conversational ability or factuality.
- id: 736215e9-ff55-4283-9e72-5948f9247aa6
  type: mcq
  front: As of May 2026, how many human votes has Chatbot Arena accumulated?
  back: By April 2026, Chatbot Arena had accumulated 2M+ human votes across 100+ models.
  choices:
  - key: a
    text: 500K
    correct: false
  - key: b
    text: 1M
    correct: false
  - key: c
    text: 2M+
    correct: true
  - key: d
    text: 10M
    correct: false
---

## Intuition
Leaderboards aggregate benchmark results and human judgments into rankings. They serve two purposes: (1) provide a snapshot of relative model capability, (2) expose limitations of aggregation (no single model dominates every benchmark). Understanding what leaderboards measure—and what they hide—is critical for interview credibility.

## Detail
**Open LLM Leaderboard (HuggingFace)**: Aggregates 4–8 standard benchmarks (MMLU, GSM8K, ARC, HellaSwag, etc.) into single leaderboard. Pros:
- Simple to interpret
- Covers breadth of capabilities
- Community-driven

Cons:
- Equal weighting masks failure modes (a model strong in knowledge but weak in reasoning gets inflated score)
- Doesn't capture long-context or reasoning depth
- Leaderboard games (models trained on test data)

**LMSYS Chatbot Arena**: Human preference voting via pairwise comparison. Users see responses from two anonymous models and vote for the better one. Elo ratings derived using Bradley-Terry model (chess ranking system). As of 2026:
- 2M+ cumulative votes across 100+ models
- Official positioning by OpenAI, Google, Anthropic
- Captures nuanced preferences beyond benchmark scores

Key feature: **multi-agent evaluation** uses role-assigned judges (Critic, Psychologist, Scientist) debating model responses, improving signal.

## Common gotchas / interview framings
- **Leaderboard gaming**: Models can be fine-tuned on benchmark test sets; always verify train/test splits.
- **Aggregation artifacts**: No single model dominates 2026 leaderboards. Claude Opus leads GPQA, DeepSeek leads SWE-bench, GPT-5.4 leads MMLU—different models excel at different tasks.
- **Human preference ≠ benchmark score**: Chatbot Arena voting captures user preference, which differs from objective benchmark performance. A model strong in reasoning (MATH) may rank lower in Arena if users value conversational ability.
- **Interview framing**: "Why might a model rank high in Open LLM Leaderboard but lower in Chatbot Arena?" Benchmark strength ≠ user preference. The leaderboard optimizes for aggregate benchmark score; Arena optimizes for user satisfaction.

## See also
- [[open-llm-leaderboard]]
- [[chatbot-arena]]
- [[elo-rating]]
- [[human-evaluation]]
- [[model-rankings]]
- [[leaderboard-bias]]

## Sources
See frontmatter `sources:`.

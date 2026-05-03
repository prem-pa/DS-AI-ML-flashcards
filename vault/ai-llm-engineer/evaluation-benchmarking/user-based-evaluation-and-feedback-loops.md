---
id: 0395faf6-202c-4f80-b4dd-b78af6e148ee
title: User-based evaluation and feedback loops
track: ai-llm-engineer
topic: evaluation-benchmarking
difficulty: 3
tags:
- user-feedback
- online-evaluation
- real-performance
- metric-alignment
- feedback-loops
aliases:
- online evaluation
- user signals
- feedback collection
- production metrics
sources:
- url: https://www.digitalapplied.com/blog/agent-observability-platforms-langsmith-langfuse-arize-2026
  label: Agent Observability 2026
- url: https://www.meta-intelligence.tech/en/insight-llm-evaluation
  label: LLM Evaluation Framework 2026
cards:
- id: a9da8c19-981c-4aa1-967d-818302820936
  type: flip
  front: Why does binary (thumbs up/down) feedback have lower friction than numeric ratings, and what
    does this mean for data quality?
  back: 'Binary is faster (yes/no decision). Higher completion rate → more feedback samples. But lower
    granularity; misses nuance (4-star user might have found error but rated high overall). Trade-off:
    volume vs. detail.'
- id: 70aa18a2-1ee7-489d-9eaa-8bcfc271c92c
  type: mcq
  front: You notice users are giving 5-star ratings to a summarization LLM that occasionally misses key
    points. What does this indicate?
  back: High ratings despite missing key points suggests users prioritize conversational quality over
    factual completeness. Structural feedback ("Did summary capture all key points?") would clarify.
  choices:
  - key: a
    text: The model is perfect
    correct: false
  - key: b
    text: Users value fluency/readability over completeness
    correct: true
  - key: c
    text: Your rating scale is broken
    correct: false
  - key: d
    text: No conclusion can be drawn
    correct: false
- id: d1476e7c-0859-46f6-9519-2b9899ec1316
  type: flip
  front: What is the difference between explicit feedback (user ratings) and implicit signals for evaluating
    LLM quality?
  back: 'Explicit: user-rated (thumbs up/down, stars). Implicit: user behavior (retry query, bookmark,
    share, support tickets). Implicit avoids bias but requires more data. Combine both for robust signal.'
- id: 36aa1de4-b73c-422d-85d7-f52265c0aa58
  type: mcq
  front: You want to validate that improving a metric (e.g., faithfulness) actually increases user satisfaction.
    Which approach would you use?
  back: Correlation between metric and user satisfaction doesn't imply causation. A/B testing isolates
    causal impact; randomize users to new/old model and compare feedback rates.
  choices:
  - key: a
    text: Compare feedback before/after metric improvement
    correct: false
  - key: b
    text: A/B test new version against baseline
    correct: true
  - key: c
    text: Trust the correlation between metrics
    correct: false
  - key: d
    text: Skip validation
    correct: false
---

## Intuition
Benchmarks and automated metrics optimize for statistical performance. Users optimize for outcomes: "Did this help me?" User-based evaluation measures real-world utility, not lab performance. The goal is to close the gap between benchmark metrics and user satisfaction, then iterate quickly.

## Detail
**Feedback mechanisms** (2026 best practices):
1. **Thumbs up/down (binary)**: Simple, low friction. Users quickly rate outputs. Risk: binary misses nuance (liked output but had factual error).
2. **Structured feedback (checkboxes)**: "Was this factually correct?" "Helpful for your task?" "Clear and concise?" Captures specific quality dimensions.
3. **Numeric ratings (1–5)**: Finer granularity but higher friction; users less likely to complete. Correlation with accuracy is weak (5 stars ≠ correct).
4. **Open comments**: Users explain why they liked/disliked. Rich signal but expensive to analyze (or auto-analyze with LLM summarization).

**Practical workflow**:
- **Collect**: Embed feedback widget in product. Ask at key moments (after user acts on LLM output).
- **Aggregate**: Track feedback rate (what % of users complete feedback?), average rating, common themes.
- **Correlate**: Link feedback to Langfuse traces. Find output patterns users disliked. Hypothesize root causes (e.g., retriever returned irrelevant docs).
- **Iterate**: Update system (better prompt, fix retriever), measure feedback shift.

**Metric alignment risk**: Optimizing for user feedback can diverge from ground truth. Example: users love confident, conversational outputs, even if slightly hallucinated. Tie feedback to business outcomes (retention, support tickets) to validate.

## Common gotchas / interview framings
- **Feedback bias**: Users rating highly may be satisfied despite errors; users rating low might be demanding. Structured feedback reduces bias vs. open-ended ratings.
- **Selection bias**: Only engaged users leave feedback. Feedback doesn't represent all users; use implicit signals (retry rate, bookmark, share) alongside explicit feedback.
- **Causal confusion**: Correlation between feedback and model change ≠ causation. A/B test new versions; don't rely on feedback trends alone.
- **Interview framing**: "Design a feedback loop for a production LLM product. What signals would you track, and how would you validate that metrics align with user outcomes?" Combine structured feedback, implicit signals (retry rate), and A/B testing. Validate metrics against business outcomes (retention, support load, revenue impact).

## See also
- [[user-evaluation]]
- [[feedback-loops]]
- [[online-metrics]]
- [[real-world-performance]]
- [[metric-alignment]]
- [[production-impact]]

## Sources
See frontmatter `sources:`.

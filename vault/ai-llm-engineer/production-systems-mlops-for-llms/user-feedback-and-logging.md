---
id: 134a7821-a7a7-44d7-9663-421267c01c45
title: User feedback and logging
track: ai-llm-engineer
topic: production-systems-mlops-for-llms
difficulty: 1
tags:
- monitoring
- feedback
- data-collection
- observability
- user-signals
aliases:
- thumbs-up/down
- user satisfaction
- logging infrastructure
- RLHF data collection
sources:
- url: https://www.braintrust.dev/articles/what-is-llm-monitoring
  label: 'Braintrust: LLM Monitoring'
- url: https://www.helicone.ai/blog/the-complete-guide-to-LLM-observability-platforms
  label: 'Helicone: LLM Observability Guide'
cards:
- id: 53dbd82a-6608-4c64-b19c-9bd745539aed
  type: flip
  front: You collect thumbs-up/down feedback on chat responses. 2% thumbs-down, 3% thumbs-up, 95% no feedback.
    How do you interpret this?
  back: '95% no feedback = selection bias. You''re only seeing feedback from engaged users (very happy
    or very angry). The true satisfaction rate is unknown. Better approach: (1) Monitor implicit signals
    (session length, re-engagement) for all users. (2) Run occasional A/B tests (show different versions
    to random cohorts) to measure true preference. (3) Ask downvoters for reasons to prioritize fixes.'
- id: e2347a72-59bf-48f0-b476-43fcd2983b1e
  type: flip
  front: You want to use downvoted responses to improve your system. What's the data pipeline?
  back: '1. Collect downvotes: store query + response + downvote timestamp. 2. Analyze: use LLM to classify
    downvote reasons (e.g., ''hallucination'', ''missed context''). 3. Prioritize: which reasons are most
    common? 4. Improve: if 30% of downvotes are ''missed context'', update system prompt to emphasize
    context. 5. A/B test: canary the new prompt to new users; measure if downvote rate drops. 6. Iterate.'
- id: a4f14d1e-18dc-428d-8e20-422f9a6d1167
  type: mcq
  front: You observe 10% upvote rate on new model version vs 8% on old version. Is the new model better?
  back: 10% vs 8% could be random noise if sample sizes are small (e.g., 100 total feedbacks on each).
    Use chi-squared test or binomial test to determine if the difference is statistically significant.
  choices:
  - key: a
    text: Yes, 10% > 8%
    correct: false
  - key: b
    text: Probably, but you need statistical significance testing (sample size matters)
    correct: true
  - key: c
    text: Only if downvote rate decreased
    correct: false
  - key: d
    text: No, feedback is too sparse to trust
    correct: false
- id: f9c8bac8-1a9b-4dce-b21f-efb2fb3d10f7
  type: mcq
  front: Why are implicit signals (session length, re-engagement) better than explicit thumbs-up/down
    for measuring user satisfaction?
  back: Implicit signals measure behavior from all users (not just those who click feedback buttons),
    reducing selection bias. A user who silently spends 10 minutes reading a response is satisfied, but
    won't click thumbs-up.
  choices:
  - key: a
    text: They are always more accurate
    correct: false
  - key: b
    text: 'They reduce bias: you collect data from all users, not just engaged ones who click buttons'
    correct: true
  - key: c
    text: They are cheaper to implement
    correct: false
  - key: d
    text: They require less storage
    correct: false
---

## Intuition
Automated metrics (BLEU, latency) are fast but incomplete. User feedback ('Is this response helpful?') is the ground truth for production quality. A simple thumbs-up/down button provides signal; analyzing feedback uncovers user preferences and failure modes. Logged interactions also fuel RLHF (Reinforcement Learning from Human Feedback) and continuous improvement.

## Detail
**Feedback collection**:
- **In-UI feedback**: Thumbs up/down, star rating (1–5), or text comment on each response. Aim for 1–5% feedback rate (most users don't provide it).
- **Implicit signals**: Click-through rate, time spent reading response, share/export actions (proxy for satisfaction).
- **Surveys**: Periodic NPS (Net Promoter Score) surveys ('Would you recommend this feature?').

**Logging**:
- Log every request: `{timestamp, user_id, session_id, query, model, response, latency, cost, version, ...}`.
- Store in data warehouse for analysis (BigQuery, S3 + Athena).
- Support queries like: 'Show me all queries where latency > 5s', 'What % of responses were downvoted?', 'Which prompts generate the most complaints?'

**Feedback as signal**:
- Aggregate feedback: % upvoted, % downvoted, average rating. Compare before/after model changes.
- Downvote reasons: Parse comments or use LLM to classify reasons ('too slow', 'inaccurate', 'irrelevant'). Prioritize fixes.
- Bias check: Are certain user groups downvoting more? (E.g., non-English speakers downvote more → model bias?)

**RLHF pipeline**:
- Collect positive (upvoted) and negative (downvoted) examples.
- Use as training signal for fine-tuning (if you own the model) or as eval set for prompt optimization.
- Example: 100 downvoted examples analyzed → common pattern: model ignores user context. Update system prompt to emphasize context awareness. Re-evaluate.

**Challenges**:
- Sparse feedback: most users don't provide it. Selection bias: engaged users upvote more than silent satisfied users.
- Feedback delay: user upvotes a response 1 week later; hard to correlate with production state.
- Privacy: storing user feedback requires consent and careful PII handling.

## Common gotchas / interview framings
- Implicit signals > explicit feedback. Click-through and session length are more reliable than optional thumbs-up.
- Feedback is biased. 1% feedback rate means angry/very happy users are overrepresented. Use A/B tests to validate.
- Don't act on feedback from a single user. Aggregate feedback over weeks and look for trends (if 5% downvote 'too slow', prioritize latency).
- RLHF data collection is continuous. Fine-tuning is not; if you want models to improve with user feedback, instrument feedback loops from day one.

## See also
- [[latency-throughput-error-rates-cost]]
- [[quality-metrics-bleu-rouge-exact-match]]
- [[model-output-drift-quality-change]]

## Sources
See frontmatter `sources:`.

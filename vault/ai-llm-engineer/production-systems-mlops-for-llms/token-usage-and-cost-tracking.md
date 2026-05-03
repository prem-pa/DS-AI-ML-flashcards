---
id: 6702aee4-2b2c-4d84-a5e0-2ea628730f43
title: Token usage and cost tracking
track: ai-llm-engineer
topic: production-systems-mlops-for-llms
difficulty: 3
tags:
- monitoring
- billing
- cost
- observability
- accounting
aliases:
- token accounting
- billing system
- cost attribution
- usage metering
sources:
- url: https://langfuse.com/docs/prompt-management/features/a-b-testing
  label: 'Langfuse: Token Tracking'
- url: https://zenvanriel.com/ai-engineer-blog/llm-api-cost-comparison-2026/
  label: LLM API Cost 2026
cards:
- id: 0a2ce641-625c-4cf1-8af2-14bf01b56acb
  type: flip
  front: 'You log a request: input_tokens=2000, output_tokens=100. You calculate cost using local tokenizer,
    but the API reports input_tokens=2050, output_tokens=95. How do you fix cost tracking?'
  back: 'Always use the API''s reported token counts (from the response `usage` field), not your local
    tokenizer. The API''s count is canonical; tokenizers can have off-by-one errors or version mismatches.
    Log: `cost = (api_reported_input_tokens * input_price + api_reported_output_tokens * output_price)
    / 1M`.'
- id: c8eac788-28e5-48c7-b604-4be0e4c26b19
  type: flip
  front: You're implementing cost-based pricing for a multi-tenant LLM API. How do you attribute cost
    to users fairly?
  back: 1. Log cost per request. 2. Aggregate daily per user (sum of all request costs). 3. Add markup
    (2–5x) for infrastructure. 4. Charge user at end of month or use prepaid tokens. For fairness, provide
    a cost dashboard so users see their usage and can optimize.
- id: 69d5088f-a703-48cb-b423-f8108dc6e51d
  type: mcq
  front: Your LLM feature suddenly shows 3x token usage spike (avg output tokens 100→300). What's the
    most likely cause?
  back: 'A 3x spike in output tokens (input likely unchanged) suggests the model is generating longer
    responses. Most likely: (1) prompt inflation (you''re accidentally appending context multiple times),
    (2) hallucination (model generating extra text), or (3) a debug flag left on. Check your prompt construction
    code first.'
  choices:
  - key: a
    text: Model updated and became more verbose
    correct: false
  - key: b
    text: Users changed their prompt or prompt injection attack
    correct: false
  - key: c
    text: Hallucination detection or a prompt inflation bug in your system
    correct: true
  - key: d
    text: Network latency increased
    correct: false
- id: eb7dbca6-a8ef-4a28-a459-0cef72f8a1e9
  type: mcq
  front: 'Anthropic Claude Prompt Caching: cached input tokens cost 0.1x, uncached cost 1x. You have 1000
    input tokens (800 cached, 200 uncached). Cost at $3/1M input tokens?'
  back: 'Cached tokens (800) are charged at 0.1x the full rate: 800 * 0.1 * ($3/1M) = 0.08 * ($3/1M) =
    $0.00024. Uncached tokens (200): 200 * ($3/1M) = $0.0006. Total = $0.00084.'
  choices:
  - key: a
    text: 1000 * 3 / 1M = $0.003
    correct: false
  - key: b
    text: (800 * 0.1 + 200 * 1) * 3 / 1M = $0.0012
    correct: false
  - key: c
    text: 800 * 0.1 * 3 / 1M + 200 * 3 / 1M = $0.000240 + $0.0006 = $0.000840
    correct: true
  - key: d
    text: Cannot compute without knowing the cache hit rate
    correct: false
---

## Intuition
Every LLM API call consumes tokens: input tokens (cheaper) and output tokens (more expensive). To track cost and optimize, you must log both, calculate cost per request, and aggregate by user/feature/model. This feeds cost monitoring, billing, and ROI analysis. Token tracking is also the foundation for detecting cost anomalies (sudden spike in output tokens → hallucination?).

## Detail
**Token counting**:
- Input tokens: user query + system prompt + context (RAG docs).
- Output tokens: generated response.
- Models vary: gpt-4 token counts differ from Claude tokens. Use provider's tokenizer (tiktoken for OpenAI, anthropic.Tokenizer for Claude).

**Cost calculation**:
- OpenAI gpt-4: $0.03/1M input, $0.06/1M output. A request with 1000 input + 500 output tokens = (1000*0.03 + 500*0.06) / 1M = $0.000045.
- Anthropic Claude: $3/1M input, $15/1M output (3-Opus pricing). Same request = (1000*3 + 500*15) / 1M = $0.0105.
- Prompt caching reduces effective input token cost: cached tokens charged at 10–25% of full rate.

**Tracking infrastructure**:
- Log every request: `{timestamp, user_id, model, input_tokens, output_tokens, cost, duration, feature}`.
- Store in data warehouse (BigQuery, Redshift, Timescale).
- Dashboard: cost over time, cost by user, cost by feature, top-10 expensive requests.
- Alerting: if daily cost exceeds threshold or cost per request spikes, investigate (hallucination, prompt inflation, user abuse).

**Cost attribution**:
- For multi-tenant SaaS: attribute cost to user. Use cost as an input to pricing (charge users based on usage).
- For internal tools: attribute cost to team/product. Use cost to ROI analysis ('this feature costs $5k/month; is it generating value?').

**Billing system**:
- Real-time metering: compute cost immediately, add to running balance.
- Batch billing: aggregate daily cost, invoice monthly.
- Cost controls: set per-user or per-feature budgets; alert or block if exceeded.

## Common gotchas / interview framings
- Off-by-one in token counting: tokenizer might round differently than API's actual count. Use API's returned `usage` field, not local tokenizer.
- Cached tokens: Some providers charge 0.1x or 0.25x for cached input tokens. Miss this in cost calc → budget projections fail.
- Cost attribution in agents: If an agent makes 10 API calls, attribute all 10 costs to the original user request or split by call type?
- Audit trail: If you're billing users, you need immutable logs (append-only) of every token use for audit/dispute resolution.

## See also
- [[latency-throughput-error-rates-cost]]
- [[cost-management-prompt-caching-batching-model-selection]]
- [[user-feedback-and-logging]]

## Sources
See frontmatter `sources:`.

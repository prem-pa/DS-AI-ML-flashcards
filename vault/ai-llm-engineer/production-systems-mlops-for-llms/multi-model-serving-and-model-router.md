---
id: bd0c308d-9076-4bdd-b22e-0707ea0bb4ff
title: Multi-model serving and model router
track: ai-llm-engineer
topic: production-systems-mlops-for-llms
difficulty: 5
tags:
- architecture
- routing
- multi-model
- latency
- cost
aliases:
- intelligent model routing
- request classification
- dynamic model selection
- ensemble routing
sources:
- url: https://zenvanriel.com/ai-engineer-blog/llm-api-cost-comparison-2026/
  label: LLM API Cost Comparison 2026
- url: https://render.com/articles/best-practices-for-running-ai-output-a-b-test-in-production
  label: 'Render: A/B Testing Best Practices'
- url: https://calmops.com/architecture/llmops-architecture-managing-llm-production-2026/
  label: 'Calmops: LLMOps Architecture 2026'
cards:
- id: a980dba3-e671-4bf1-810e-67cc6afbfca9
  type: flip
  front: Design a router for a customer support system. Requests range from 'password reset' (simple)
    to 'explain billing dispute' (complex). How do you classify?
  back: 'Option 1 (simple): Keyword matching (''reset password'' → FAQ model). Option 2 (robust): Train
    a 50M-param intent classifier on labeled support tickets (intent: account, billing, technical). Route
    account→cheap, billing+technical→gpt-4. Log failures to retrain classifier.'
- id: 250564e8-da2f-4f57-a8f3-d77b6e61645b
  type: flip
  front: A router routes 90% to gpt-3.5 and 10% to gpt-4. How do you detect if the router is underestimating
    gpt-3.5's capability?
  back: 'Monitor the 10% gpt-4 cohort: if gpt-4''s answers are nearly identical to gpt-3.5''s on those
    queries, increase the gpt-3.5 percentage (e.g., 95/5). Use semantic similarity (embed both responses,
    check cosine distance) or LLM-as-Judge to compare.'
- id: bcd0853f-7f52-4a8d-b3a7-d84e91ae79ae
  type: mcq
  front: In a multi-model serving setup, your router's latency is 200ms (classifier) and the cheap model
    adds 500ms. The expensive model adds 1500ms. What's the latency cost of a misroute (sending expensive
    query to cheap model, getting failure, escalating)?
  back: 'Misroute: classifier (200) → cheap attempt (500) → failure detection → re-classify (200) → escalate
    to expensive (1500) = 2400ms total. Direct routing would be 200 + 1500 = 1700ms. Misroute penalty:
    700ms.'
  choices:
  - key: a
    text: 200 + 500 + 1500 = 2200ms
    correct: false
  - key: b
    text: 200 + 500 + 200 + 1500 = 2400ms (reclassification + escalation)
    correct: true
  - key: c
    text: 1500ms (just use expensive directly)
    correct: false
  - key: d
    text: 500ms (cheap attempt, ignore failure)
    correct: false
- id: 5cbf48e7-cb74-4400-a76a-97f72b0d71b7
  type: mcq
  front: 'You''re designing a fallback chain: gpt-3.5 → gpt-4-turbo → gpt-4. What metric should trigger
    escalation?'
  back: Confidence, refusal patterns, and token mismatch are fine-grained failure signals. Blindly escalating
    all gpt-3.5 negates cost savings. Error rate > 1% is too coarse and slow to detect.
  choices:
  - key: a
    text: Any response from gpt-3.5 (always escalate to be safe)
    correct: false
  - key: b
    text: Model confidence score < 0.7 OR refusal detected OR expected tokens < actual
    correct: true
  - key: c
    text: Response length; if shorter than expected, escalate
    correct: false
  - key: d
    text: Error rate > 1%
    correct: false
---

## Intuition
Not all queries need gpt-4. Simple FAQs suit gpt-3.5-turbo; nuanced reasoning needs gpt-4; creative writing might use Llama-70B. A **model router** classifies the request and dispatches it to the best model based on capability, cost, and latency. This multiplexing strategy reduces cost by 30–60% and latency by routing lightweight tasks to fast, cheap models.

## Detail
Router architecture:
```
  user_request
    ↓
  [lightweight classifier] → query intent (faq, reasoning, creative, code)
    ↓
  [router logic] → select model (gpt-3.5 | gpt-4 | claude | llama)
    ↓
  [call model]
    ↓
  [confidence check] → if failure, escalate to gpt-4
```

Classifier options:
- **Heuristic**: Query length, keyword matching ('debug my code' → code model).
- **Lightweight ML model**: A small classifier trained on intent labels (50M parameters, <100ms inference).
- **Embedding similarity**: Embed query; find nearest neighbors in labeled intent clusters.

Router decision logic:
- **Greedy**: Always pick cheapest model that handles the intent. Risk: failure→escalation costs more.
- **Probabilistic**: Route 80% to cheap, 20% to expensive for validation/improvement.
- **Confidence-based**: Return model's confidence score; if < threshold, escalate.

Fallback and escalation:
- Tier 1 (cheap): gpt-3.5-turbo, Llama-3-8B.
- Tier 2 (medium): gpt-4-turbo, Claude-3-sonnet.
- Tier 3 (expensive): gpt-4, Claude-3-opus.
- Detect failure: low confidence, refusal rate, token count exceeds expected. Escalate with context ('first attempt failed').

## Common gotchas / interview framings
- How do you prevent cascading failures (cheap model fails, escalate to expensive, expensive fails)? Add circuit breakers and timeouts.
- Classifier latency: if the router adds 100ms, it might negate latency savings from cheap models. Keep classifier <50ms.
- Training the router: labeled intents require human curation; difficult for niche domains.
- Cost vs. quality tradeoff: aggressive cost minimization may hurt quality. Define a cost-quality Pareto frontier.

## See also
- [[load-balancing-and-autoscaling-for-llm-endpoints]]
- [[cost-management-prompt-caching-batching-model-selection]]
- [[latency-throughput-error-rates-cost]]
- [[model-versioning-and-canary-rollouts]]

## Sources
See frontmatter `sources:`.

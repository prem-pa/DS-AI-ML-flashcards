---
id: b2c48550-d5da-4077-8bc5-f3cac985887d
title: Cost management (prompt caching, batching, model selection)
track: ai-llm-engineer
topic: production-systems-mlops-for-llms
difficulty: 3
tags:
- cost-optimization
- inference
- economics
- efficiency
- architecture
aliases:
- token cost reduction
- batch processing
- model routing by cost
- cache hit rate optimization
sources:
- url: https://projectdiscovery.io/blog/how-we-cut-llm-cost-with-prompt-caching
  label: 'ProjectDiscovery: Cutting LLM Costs with Prompt Caching'
- url: https://jangwook.net/en/blog/en/claude-api-prompt-caching-cost-optimization-guide/
  label: Claude Prompt Caching Cost Guide
- url: https://zenvanriel.com/ai-engineer-blog/llm-api-cost-comparison-2026/
  label: LLM API Cost Comparison 2026
cards:
- id: 55da6f4a-16e0-4ed1-9183-94b349793746
  type: flip
  front: You're building a RAG system with 10k documents cached in the prompt. Anthropic's 5-minute TTL
    for prompt caching. How do you maximize cache hit rate?
  back: 'Group requests by document set or user. If you have 100 users but 10k docs, randomized doc selection
    means each request sees different cached content—poor hit rate. Instead: (1) Serve users from a small
    set of ''hot'' docs (80/20). (2) Batch intra-user requests (queue 10 queries, send together). (3)
    Use a separate cache per user if traffic allows.'
- id: a2b67da0-720c-4a99-b4ce-dbc7926c6098
  type: flip
  front: When should you use the Batch API vs. standard API for LLM requests?
  back: 'Batch API (~50% discount): non-urgent tasks (overnight email summaries, weekly reports). Standard
    API: interactive user-facing requests. Hybrid: use batch for background quality scoring, standard
    for chat.'
- id: 52b3f7d9-a769-4f5a-ac26-af0e7f051ba9
  type: flip
  front: 'You have a model router: cheap model for FAQs, expensive for complex questions. How do you handle
    misclassification (router sends a complex question to cheap model)?'
  back: 'Fallback chain: (1) Route to cheap model. (2) Detect failure (low confidence, refusal). (3) Escalate
    to expensive model with original query + ''first attempt failed'' context. Log misclassifications
    to retrain router.'
- id: c3b39d10-4289-4aef-b099-f6bdbadf38e8
  type: mcq
  front: Prompt caching reduces costs by 70–90% but requires repeated cache hits. Which scenario maximizes
    cache hits?
  back: Cache hit requires identical [static_prefix | cache_breakpoints]. A FAQ bot answering 1000 users
    from the same FAQ doc set = high hit rate. One-off diverse queries = low hit rate.
  choices:
  - key: a
    text: One-off user queries to a search engine
    correct: false
  - key: b
    text: High-volume FAQ bot with consistent doc set across users
    correct: true
  - key: c
    text: Batch processing of diverse datasets
    correct: false
  - key: d
    text: Real-time market data analysis with unique inputs
    correct: false
- id: da4a7487-6d4a-4a2e-99de-b5b88714d002
  type: mcq
  front: You want to route requests to the cheapest model that can handle them. What's the bottleneck
    in a model selection strategy?
  back: If your classifier misroutes complex queries to cheap models, they fail silently (hallucinate)
    or refuse. Classifier accuracy directly impacts cost savings and user satisfaction.
  choices:
  - key: a
    text: Latency of the expensive models
    correct: false
  - key: b
    text: Accuracy of the classifier deciding which model is needed
    correct: true
  - key: c
    text: Token usage per model
    correct: false
  - key: d
    text: Load balancing between model instances
    correct: false
---

## Intuition
LLM inference cost dominates production budgets: a 100-token response on gpt-4 costs $0.0015. At scale, cost balloons. Three levers reduce it: (1) **Prompt caching**: reuse expensive static context across requests (reduce input tokens by 60–90%). (2) **Batching**: collect requests and process in parallel (cheaper per-token pricing for batch APIs, ~50% discount). (3) **Model selection**: route simple queries to cheap models (gpt-3.5-turbo), complex ones to powerful models (gpt-4).

## Detail
**Prompt caching** (Anthropic, OpenAI 2026):
- Structure prompts as [static_prefix | cache_breakpoints] + [dynamic_query]. The static prefix is cached; only the dynamic part is charged at full rate.
- Example: A RAG system caches the retrieved documents and system prompt, only charges for the user's question. Typical 70–80% cost reduction.
- Anthropic: explicit cache breakpoints in the API; 5-minute TTL (as of early 2026). OpenAI: automatic cache on exact prefix match, 1-hour TTL.
- Gotcha: If traffic is bursty, cache hits are low. Consistent traffic patterns maximize hit rate.

**Batching**:
- OpenAI Batch API, Anthropic Batch API process requests offline (e.g., overnight). ~50% cost savings but 24h latency.
- Trade: low-latency interactive requests use standard API; non-urgent background tasks use batch API.

**Model selection** (Router):
- Classification: Is query complex? Route to gpt-4 ($0.03/K in). Is query simple FAQ? Route to gpt-3.5-turbo ($0.001/K in). Typical 40–60% cost savings.
- Heuristic: Token count, query intent classification, or a tiny model (classify-as-simple) predicts which model to use.

## Common gotchas / interview framings
- Why does prompt caching need consistent traffic? Low-traffic systems have sparse cache hits; you need at least 2 identical requests within TTL.
- What's the cache TTL tradeoff? Long TTL (1h) means more hits but stale cached context if your documents update frequently.
- Can you batch interactive requests? No—batching has 24h latency. Only use for async workflows (email summaries, nightly reports).
- Model selection: what if your classifier is wrong? Gpt-4 rejects the query; gpt-3.5 hallucinates. Fallback chain: try cheap first, escalate to expensive if failure detected.

## See also
- [[multi-model-serving-and-model-router]]
- [[load-balancing-and-autoscaling-for-llm-endpoints]]
- [[token-usage-and-cost-tracking]]

## Sources
See frontmatter `sources:`.

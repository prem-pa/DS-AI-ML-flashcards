---
id: 90894602-c1cd-4a8a-9326-69cc591d64a0
title: Load balancing and autoscaling for LLM endpoints
track: ai-llm-engineer
topic: production-systems-mlops-for-llms
difficulty: 3
tags:
- serving
- infrastructure
- scaling
- latency
- resilience
aliases:
- LLM endpoint autoscaling
- request balancing
- replica management
- QPS-based scaling
sources:
- url: https://www.braintrust.dev/articles/what-is-llm-monitoring
  label: Braintrust LLM Monitoring Guide
- url: https://langfuse.com/docs/prompt-management/features/a-b-testing
  label: Langfuse Prompt Management
- url: https://www.digitalocean.com/blog/prompt-caching-with-digital-ocean
  label: DigitalOcean Prompt Caching Guide
cards:
- id: 7b059a25-2963-4c5c-9f14-6afe9e5c503c
  type: flip
  front: You observe p99 latency at 5s but average is 800ms. Your autoscaler is set to scale on average
    load. What's the problem and how do you fix it?
  back: 'Average latency hides tail behavior. Long-running requests (e.g., 200-token generation) block
    short ones behind them. Fix: Scale on p99 latency threshold (e.g., scale up if p99 > 3s) or use token-weighted
    routing instead of round-robin.'
- id: a3f3d46c-c402-4fd5-9660-4a4cb6bf4d79
  type: flip
  front: You're rolling out a new model version. How do you prevent request loss during replica shutdown?
  back: 'Use graceful drain: mark old replicas as ''draining''—load balancer stops routing new requests
    to them but lets in-flight requests complete (with timeout). New traffic routes to fresh replicas.'
- id: 484ade2c-5e91-4ee5-ae54-c1a40f45ca12
  type: mcq
  front: Which autoscaling metric is most appropriate for LLM endpoints where requests have highly variable
    response times?
  back: LLM response times are highly variable. p99 latency directly targets user experience (SLOs are
    usually defined as 'p99 latency < Xs'), and queue depth predicts future load. Average and CPU % miss
    tail behavior.
  choices:
  - key: a
    text: Average response time across all requests
    correct: false
  - key: b
    text: p99 latency percentile or queue depth
    correct: true
  - key: c
    text: Total number of pending requests
    correct: false
  - key: d
    text: CPU utilization percentage
    correct: false
- id: 2ba7f15b-e56f-41ba-b077-55eea8013e81
  type: mcq
  front: In a load-balanced LLM cluster, why is 'least-loaded' routing preferable to round-robin?
  back: Round-robin distributes evenly by request count, not work. If replica A gets three 50-token requests
    and replica B gets one 500-token request, B is overloaded but round-robin treats them equally. Least-loaded
    routes to the replica with fewest in-flight tokens or queue depth.
  choices:
  - key: a
    text: Round-robin is simpler to implement
    correct: false
  - key: b
    text: Requests have variable token counts; some replicas handle fast short requests, others get stuck
      on long ones
    correct: true
  - key: c
    text: Least-loaded routing is cheaper
    correct: false
  - key: d
    text: Round-robin doesn't support batching
    correct: false
---

## Intuition
LLM API endpoints receive variable traffic: some requests complete in 500ms, others take 30s. Load balancers distribute requests across replicas; autoscalers add/remove replicas based on queue depth, latency percentiles, or queries-per-second (QPS). The goal is to keep p99 latency under SLO while minimizing idle compute.

## Detail
Load balancing strategies for LLMs differ from traditional web services because:
- **Request heterogeneity**: Token generation is variable-cost (a 10-token response costs 1/10th a 100-token response). Round-robin fails; use least-loaded or token-aware balancing.
- **Autoscaling triggers**: Scale on latency percentiles (p99 latency > 3s?), not just average load. A single long-running request should trigger scale-up, not wait for 10 slow requests.
- **Cost tradeoff**: Overprovisioning keeps latency low but burns tokens on idle GPUs. Underprovisioning saves money but creates queue wait. SLOs define the boundary.

Common patterns:
- **Least-loaded routing**: Route to replica with fewest in-flight tokens, not requests.
- **Token-aware scheduling**: Batch small requests to same replica; split large batch requests across replicas.
- **Predictive scaling**: Queue-depth prediction—if queue grows, scale preemptively rather than reactively.

## Common gotchas / interview framings
- Why not just add more replicas? GPU memory is expensive; scaling doesn't help if bottleneck is model weight memory, not throughput.
- How do you handle tail latency (p99)? Average load metrics hide slow outliers; use percentile-based alarms.
- What happens during model updates? Load balancers must drain in-flight requests before terminating old replicas.
- Cold start cost? Spinning up a new GPU instance takes 30–60s; anticipate demand spikes.

## See also
- [[model-versioning-and-canary-rollouts]]
- [[latency-throughput-error-rates-cost]]
- [[multi-model-serving-and-model-router]]
- [[cost-management-prompt-caching-batching-model-selection]]

## Sources
See frontmatter `sources:`.

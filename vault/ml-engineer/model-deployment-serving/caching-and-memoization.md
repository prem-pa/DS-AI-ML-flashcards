---
id: d48c260e-7fff-4e78-bf9f-127926feb7cb
title: Caching and memoization
track: ml-engineer
topic: model-deployment-serving
difficulty: 3
tags:
- performance
- optimization
- caching
- latency
- throughput
- memory
aliases:
- prediction-cache
- result-cache
- feature-cache
- memoization
sources:
- url: https://en.wikipedia.org/wiki/Cache_(computing)
  label: Computing Cache Overview
- url: https://redis.io/
  label: Redis In-Memory Cache
cards:
- id: 232d15b1-8218-4c54-ac7c-8294e325cb75
  type: flip
  front: When is prediction caching most valuable?
  back: High repeat-query rate (e.g., recommend same product to multiple users), long model latency (>100ms),
    and predictable query patterns. Low value if query distribution is random or model updates frequently.
- id: 85d477da-abdf-4394-a6a2-9f3bed691f2e
  type: mcq
  front: Your model takes 200ms to infer, and 40% of requests are exact repeats. Should you add prediction
    caching? Where?
  back: 40% hit rate saves 200ms × 0.4 = 80ms average latency. In-process cache is fast (1-5μs lookup).
    Distributed cache (Redis) adds network latency (~1-2ms) and may not be worth it here unless replicas
    share state.
  choices:
  - key: a
    text: No; 40% hit rate is too low to justify overhead
    correct: false
  - key: b
    text: Yes; in-process cache (lru_cache) for speed; 40% hit rate saves ~80ms avg latency
    correct: true
  - key: c
    text: Yes; Redis for distributed cache across replicas
    correct: false
  - key: d
    text: No; cache only works if 100% of requests are identical
    correct: false
- id: 0ba1db23-58f8-42f7-86ce-ed16e0bbc8d1
  type: flip
  front: Your model is updated to v2.0. How do you ensure cached predictions from v1.0 aren't used?
  back: 'Include model version in cache key: f''{model_version}:{input_hash}'' or use separate Redis namespaces.
    Or flush entire cache on deploy, or use short TTL (e.g., 5min) to auto-expire stale entries.'
- id: 87f87acd-3b1b-4d85-9b4a-73192882037e
  type: flip
  front: Compare in-process caching (lru_cache) vs. distributed caching (Redis) for predictions.
  back: 'In-process: fast (<5μs), limited by RAM per replica, not shared. Redis: slower (~1-2ms network),
    larger capacity, shared across replicas. Use in-process for single-replica or high-frequency hits;
    Redis for multiple replicas or large caches.'
---

## Intuition
If a user asks for a prediction on the same input, don't rerun the model—return the cached result. Caching speeds up repeat queries (1ms vs. 100ms), reduces GPU load, and saves money. Downside: stale predictions if model updates, memory overhead, and invalidation complexity.

## Detail
**Caching strategies**:
- **Prediction cache**: Cache (input_hash → prediction) in Redis or in-process hashmap. Hit rate depends on query distribution; high if users repeat queries (e.g., recommendations, search).
- **Feature cache**: Pre-compute and cache features (e.g., embeddings) to avoid repeated feature extraction. Often higher ROI than prediction cache.
- **Model cache**: Load model once, reuse across requests (standard); or cache intermediate activations (not common due to memory overhead).

**TTL (time-to-live)**: Cache entries expire after T seconds. Trade-off: short TTL (10s) keeps predictions fresh but misses hits; long TTL (1h) increases hits but serves stale predictions. Model updates may invalidate cache; flush on deploy.

**Eviction policies**:
- **LRU (Least Recently Used)**: Remove oldest unused entry when cache is full; good for hotspot workloads
- **LFU (Least Frequently Used)**: Remove least-accessed entries; better for long-tail patterns

**Implementation**:
- In-process: `functools.lru_cache` (Python), `@Cached` decorator. Fast (no network), memory-limited by RAM
- Distributed: Redis, Memcached. Shared across replicas, larger capacity, network latency (~1-2ms)

## Common gotchas / interview framings
- "Cache hit rate is 5%; we're wasting memory." → Analyze query distribution; if truly random, caching won't help; consider feature-level caching instead
- "Cached predictions are stale after model update." → Implement versioned cache keys (model_v2.0:input_hash); flush on deploy, or use short TTL
- "Cache memory is the bottleneck." → Use lossy compression (store quantized predictions), or focus on feature cache (smaller than predictions)

## See also
- [[batch-inference-vs-online-real-time-inference]]
- [[rest-apis-and-grpc-for-model-endpoints]]
- [[load-balancing-and-horizontal-scaling]]

## Sources
See frontmatter `sources:`.

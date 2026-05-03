---
id: 56f4d651-aa9b-42ce-a778-8c6613df10ff
title: Batch inference vs online (real-time) inference
track: ml-engineer
topic: model-deployment-serving
difficulty: 3
tags:
- serving-patterns
- latency
- throughput
- real-time
- batch-processing
aliases:
- batch-serving
- online-serving
- latency-throughput-tradeoff
- request-batching
sources:
- url: https://docs.ray.io/en/latest/serve/tutorials/triton-server-integration.html
  label: Ray Serve Triton Integration
- url: https://en.wikipedia.org/wiki/Batch_processing
  label: Batch Processing Overview
cards:
- id: d9171a99-fd96-4304-b10a-ad483daef40f
  type: flip
  front: Explain the latency vs. throughput tradeoff between batch and online inference.
  back: 'Batch: low latency per sample, high throughput, high latency to get result (wait for batch).
    Online: high latency per sample, low throughput per replica, low latency for user. Hybrid: request
    batching—wait up to T_ms for batch or timeout.'
- id: 89d2df36-cece-44b1-a53e-5ea161e66be8
  type: mcq
  front: Your fraud-detection service must respond to each transaction in <50ms. Should you use batch
    or online inference?
  back: Online (or light request batching) is required for <50ms SLA. Pure batch processing introduces
    unacceptable delays. Small batches trade throughput for lower latency, but require more replicas.
  choices:
  - key: a
    text: Batch inference with 1000 samples per batch
    correct: false
  - key: b
    text: Online inference (immediate per-request) with multiple replicas and request batching timeout
      ~10-20ms
    correct: true
  - key: c
    text: Batch inference with small batches (10 samples)
    correct: false
  - key: d
    text: Pure online, no batching; overProvision replicas until SLA is met
    correct: false
- id: cfa293a5-c07a-4475-8fb6-f58bcad676cd
  type: flip
  front: What is request batching, and how does it help in online inference systems?
  back: Server accumulates individual requests for up to T_max milliseconds (e.g., 50ms) or until batch_size
    is reached, then runs one forward pass. Reduces per-sample latency compared to single-inference but
    keeps client latency <T_max.
- id: 934ea591-caa1-4e75-bca7-5002bb128543
  type: flip
  front: You're processing 1000 image classification requests per second. Should you batch? How?
  back: Yes. With 1000 req/s, a single replica can batch ~100 images per ~100ms or use dynamic batching
    with 10-50ms timeout. This amortizes forward-pass cost across samples, raising GPU utilization from
    ~5% (online only) to 40-80%.
---

## Intuition
Batch inference processes many samples in one call (100 images at once). Online inference processes single or small requests immediately (1 image per request). Batch is fast per-sample and cheap but has high latency for individual requests. Online is responsive but underutilizes compute (low GPU occupancy).

## Detail
**Batch inference**: Load 100 samples into a batch tensor, forward pass once, output 100 predictions. Per-sample latency = (load time + forward pass + unload) / 100 samples. GPUs achieve high throughput by parallelizing matrix ops across samples. Use when: queries can wait (nightly report generation, recommendation ranking), user tolerance is high (seconds), or cost matters more than latency.

**Online inference**: Client sends single request, server runs forward pass immediately, client gets response. Latency = model forward time (+ overhead). Throughput limited by model latency; to serve N requests/sec, need N/model_latency replicas. Use when: users expect <100ms response (search, fraud detection), query rate is unpredictable, or SLA is strict.

**Hybrid approach**: Request batching—server accumulates requests for T_max milliseconds or until batch_size samples, then runs one forward pass. Balances latency (don't wait forever) and throughput (batch ops). TensorFlow Serving, TorchServe, Triton support automatic batching configurations.

## Common gotchas / interview framings
- "We switched to batching and latency tripled." → Batching window too long; reduce T_max or batch_size
- "GPU utilization is only 5% in online mode." → Expected; too few concurrent requests. Increase timeout, batch_size, or add request batching
- "Batch inference fails on variable-size inputs." → Pad to max size or use dynamic batching (reshape as needed); no simple fix for ragged tensors

## See also
- [[rest-apis-and-grpc-for-model-endpoints]]
- [[load-balancing-and-horizontal-scaling]]
- [[cold-start-and-model-loading-time]]

## Sources
See frontmatter `sources:`.

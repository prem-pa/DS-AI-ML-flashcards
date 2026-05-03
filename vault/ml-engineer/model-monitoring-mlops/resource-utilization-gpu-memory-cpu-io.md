---
id: beaaef85-b3fa-4274-975f-01626b1da38f
title: Resource utilization (GPU memory, CPU, I/O)
track: ml-engineer
topic: model-monitoring-mlops
difficulty: 3
tags:
- infrastructure-monitoring
- performance-profiling
- capacity-planning
- cost-optimization
- observability
- bottleneck-detection
aliases:
- compute utilization
- hardware efficiency
- system performance
sources:
- url: https://www.peerspot.com/categories/model-monitoring
  label: 'PeerSpot: Best Model Monitoring solutions 2026'
cards:
- id: 97cbc366-5e63-427e-b703-f23f6399db34
  type: flip
  front: Your model's GPU memory is at 92%, but GPU utilization (compute kernels) is only 45%. Is this
    efficient? What should you investigate?
  back: '**Not efficient**. High memory (92%) with low utilization (45%) means:


    - Model weights dominate memory, but compute is underutilized (kernels not busy)

    - Likely causes: (1) Small batch size (model loaded, few samples to compute), (2) I/O-bound (waiting
    for feature store), (3) CPU bottleneck, (4) Kernel launch overhead


    Optimizations: (1) Increase batch size to fill GPU compute pipeline. (2) Check feature store latency—if
    500ms/req, model kernels idle. (3) Profile: use NVIDIA Profiler to identify where GPU is stalled.
    (4) Consider model quantization (int8) to reduce memory, free up budget for batching.'
- id: a5269e25-9fae-442e-8218-7276b353ed32
  type: mcq
  front: 'Inference latency is 150ms (SLO: 100ms). You profile: feature store query 60ms, model forward
    40ms, serialization 20ms, postprocessing 30ms. Where should you optimize first?'
  back: '**Correct: b** Feature store is 60/150ms = 40% of latency, the largest bottleneck. Reducing by
    30ms gets you to 120ms (still over 100ms, but moves needle fastest). Model forward is only 40/150ms
    = 27%; quantization gives ~10-15% speedup, saving 4-6ms (not enough). Priority: feature store caching,
    local embedding replica, or batch feature requests.'
  choices:
  - key: a
    text: Optimize model forward pass (use TensorRT quantization)
    correct: false
  - key: b
    text: Optimize feature store query (caching, local replica)
    correct: true
  - key: c
    text: Optimize serialization (faster JSON encoder)
    correct: false
  - key: d
    text: Add GPU to parallelize all components
    correct: false
- id: 4e7a0203-f69d-469f-9e4c-155a04e824d1
  type: flip
  front: Describe how to implement capacity planning for an ML inference service. If current load is 100
    req/sec using 70% GPU, how many GPUs do you need for 500 req/sec?
  back: '**Capacity planning steps**:

    1. Profile: Measure resource per req (GPU memory, GPU compute, CPU)

    2. Current: 100 req/sec → 70% GPU utilization on 1 GPU

    3. Linear scaling assumption: 500 req/sec → 500 * (70% / 100) = 350% = 3.5 GPUs

    4. Add headroom (20-30%): 3.5 * 1.3 = 4.55 → **5 GPUs**


    Refinements:

    - Verify linearity via load test (batching may improve efficiency at higher req/sec)

    - Account for peak-to-average ratio (if peaks are 2x average, scale to peaks)

    - Monitor tail latency (p99); 5 GPUs at 70% avg may have p99 queuing if bursty

    - Cost trade-off: Can you add CPU preprocessing, cache features, or quantize model to reduce GPU need?'
- id: 0a1d9609-9f00-4826-8925-3d29142292a9
  type: flip
  front: You notice GPU memory climbs from 65% → 92% over 2 weeks with stable request volume. Feature
    store cache hit rate drops from 90% → 70%. What is happening?
  back: '**Feature freshness issue**: Cache hit rate ↓ means cache is being invalidated more (or new features
    entering). If feature store is keeping more features in hot storage (higher memory footprint or GPU-cached
    tensors), GPU memory climbs.


    Root cause: (1) Feature set expanded (new features added), (2) Cache eviction policy changed, (3)
    Feature store memory leak, (4) Batch size increased, (5) New feature engineering pipeline with intermediate
    tensors.


    Next steps: (1) Measure GPU memory breakdown (model weights vs feature cache vs activations). (2)
    Check feature store version/configuration changes. (3) Profile memory allocation over time. (4) Consider
    reducing batch size or adding GPU memory, or moving feature cache off GPU.'
---

## Intuition
ML inference consumes compute: GPU/TPU memory for model weights & activations, CPU for preprocessing, I/O for feature lookups. Monitor utilization to detect inefficiencies (wasted resources), predict capacity needs, and reduce costs. A model using 95% GPU memory leaves no headroom; a model at 10% GPU suggests over-provisioned hardware.

## Detail
Key metrics: GPU memory (%, GB), GPU utilization (%), CPU %, I/O wait %, disk reads/writes, network bandwidth. Set thresholds: alert if GPU memory > 90% (OOM imminent), CPU > 80% sustained (bottleneck). Use tools: NVIDIA SMI (GPU), htop (CPU), iostat (I/O), Datadog/Prometheus to aggregate. Profile model: measure inference time breakdown—feature store query (40%), model forward pass (30%), serialization (20%), postprocessing (10%). Optimize the bottleneck: cache features, quantize model, batch requests. Plan capacity: if 100 req/sec uses 60% GPU, 200 req/sec needs 2x GPU or else request queuing.

## Common gotchas / interview framings
- GPU utilization ≠ GPU memory; can have low utilization (kernel idle) with high memory (weight storage)
- Not accounting for batching overhead; batch size 128 may be faster per-sample than batch size 1 despite higher latency
- I/O bottleneck misattributed to model; feature store is slow, not model
- Ignoring warm-up time; GPU kernels compile on first call, skewing latency profiles

## See also
- [[latency-throughput-error-rates]]
- [[schema-drift-and-pipeline-monitoring]]
- [[experiment-tracking-and-model-registry]]

## Sources
See frontmatter `sources:`.

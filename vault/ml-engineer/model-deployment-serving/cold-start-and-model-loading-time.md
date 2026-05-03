---
id: 2e301884-ed1f-4652-8c17-fbc77f839929
title: Cold start and model loading time
track: ml-engineer
topic: model-deployment-serving
difficulty: 3
tags:
- latency
- startup
- initialization
- optimization
- performance
- warmup
aliases:
- model-warmup
- startup-latency
- initialization-time
- first-request-latency
sources:
- url: https://docs.nvidia.com/deeplearning/triton-inference-server/user-guide/docs/index.html
  label: Triton Inference Server Documentation
- url: https://en.wikipedia.org/wiki/Cold_start_(computing)
  label: Cold Start Computing Overview
cards:
- id: 0f1f01e3-9b17-4e0a-92e6-60b2bf80a163
  type: flip
  front: List the main contributors to cold-start latency when a new replica is created.
  back: 1) Container startup (~1-5s); 2) Model deserialization/loading (~5-30s); 3) GPU context & memory
    allocation (~2-5s); 4) JIT/kernel compilation (~5-10s); 5) Dependency initialization.
- id: 859d2114-1359-4734-9eed-42fc38cd3e38
  type: mcq
  front: Cold-start latency is 45s. Most time is spent on model deserialization. Which optimization has
    the biggest impact?
  back: Quantization directly reduces deserialization time (file size & decompression). Faster deserializer
    helps but is secondary. Pre-loading helps only if model is downloaded at runtime.
  choices:
  - key: a
    text: Use faster deserializer (ONNX Runtime C++ vs. PyTorch pickle); saves ~30% (5-10s)
    correct: false
  - key: b
    text: Quantize model to INT8; shrinks size 4x, deserializes 4x faster (~30-40s → 10-15s)
    correct: true
  - key: c
    text: Pre-load model in Docker image; eliminates download time (~0-5s savings if downloaded)
    correct: false
  - key: d
    text: Increase GPU batch size (doesn't affect cold-start)
    correct: false
- id: 315152a7-978d-4d3c-ace8-1a644a7b4543
  type: flip
  front: What is a Kubernetes readiness probe, and how does it prevent cold-start issues?
  back: 'Readiness probe: periodic health check that returns ready/not-ready. Pod reports not-ready until
    model is loaded. K8s doesn''t route traffic until ready; prevents timeouts on cold-start. Probe delay
    must be >= actual warmup time.'
- id: 94b21957-da47-4fdf-85a5-75db28664720
  type: flip
  front: You have a 10GB model; deserialization takes 2 minutes. Autoscaling creates new replicas during
    traffic spikes, causing cascading failures. How do you fix it?
  back: 1) Quantize to INT8 (2.5GB, 30s deserialize); 2) Pre-load in Docker image; 3) Increase min replicas
    (prevent scaling); 4) Pre-warm new replicas with synthetic requests; 5) Use streaming deserialization
    if possible.
---

## Intuition
When a new replica starts, it must load the model into memory/GPU before serving requests. A 1GB model might take 10-30 seconds to deserialize and allocate GPU memory. Until then, requests are blocked or rejected. Cold-start latency is the time from pod creation to first successful prediction.

## Detail
**Sources of cold-start latency**:
1. **Container startup**: 1-5s for OS, runtime, dependencies
2. **Model deserialization**: 5-30s depending on format (pickle slow, ONNX/SavedModel faster)
3. **GPU memory allocation**: 2-5s for GPU context creation, buffer allocation
4. **Warm-up iterations**: 5-10s for GPU kernel compilation (CUDA/TensorRT kernels compile on first use)
5. **Dependency loading**: TensorFlow may spend seconds on graph construction, optimizer setup

**Optimization strategies**:
- **Pre-load in image**: Bake model into Docker image (no download at startup); trade-off: larger image, slower build/push
- **Streaming deserialization**: Load weights incrementally into GPU; start inference while loading (if possible)
- **Memory mapping**: Use mmap for large checkpoint files; kernel pages in chunks on-demand
- **Quantization**: INT8 models are 4x smaller, deserialize faster
- **Warmup requests**: After pod starts, send dummy requests to trigger JIT compilation before routing real traffic
- **Fast deserializers**: Use ONNX Runtime (C++ backend) instead of pickle (Python)

**Readiness probe**: K8s tool—pod reports "ready" only after model is loaded. Don't route traffic until ready; avoids cascading request timeouts.

## Common gotchas / interview framings
- "Cold-start is 60s; unacceptable." → Measure each phase; likely model deserialization + GPU warmup. Pre-load in image, quantize, or switch to faster format
- "Autoscaling creates cascade of cold-starts; user experience degrades." → Increase min replicas, or pre-warm new replicas with synthetic requests
- "Readiness probe passes but first real request times out." → Warmup time extends beyond readiness; increase probe delay or add synthetic requests

## See also
- [[model-checkpoints-and-state-dicts]]
- [[quantization-for-storage-and-inference-speedup]]
- [[container-orchestration-docker-kubernetes]]

## Sources
See frontmatter `sources:`.

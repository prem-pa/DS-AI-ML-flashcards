---
id: 8da39c1e-a6a7-4aaf-8b4d-0c5dfc37a367
title: Pipeline parallelism and micro-batching
track: ml-engineer
topic: training-pipelines-infrastructure
difficulty: 5
tags:
- pipeline-parallelism
- pp
- micro-batching
- gpipe
- bubble
- temporal-pipeline
aliases:
- PP
- inter-layer parallelism
- pipeline bubble
sources:
- url: https://arxiv.org/abs/1811.06965
  label: 'GPipe: Efficient Training of Giant Models on Multiple GPUs'
- url: https://arxiv.org/abs/2006.00650
  label: 'PipeDream: A More Effective Data Parallel Method for Distributed Deep Learning'
- url: https://pytorch.org/docs/stable/pipeline.html
  label: PyTorch Pipeline Parallel
cards:
- id: 23123673-54d5-4402-93f1-f918f08f0aba
  type: flip
  front: In pipeline parallelism with 4 GPUs and 8 microbatches, what is the pipeline depth and bubble?
  back: Depth = 8. Bubble = (4-1) / 8 = 37.5% idle time. GPUs not all busy; 3 GPUs wait for GPU 0 in first
    phase.
- id: b6bec228-7cf8-46dd-ad98-55fd84cad43a
  type: mcq
  front: How does microbatching in pipeline parallelism improve GPU utilization?
  back: ''
  choices:
  - key: a
    text: Reduces gradient size
    correct: false
  - key: b
    text: 'Creates temporal pipelining: GPUs process different microbatches, overlapping computation'
    correct: true
  - key: c
    text: Requires less memory
    correct: false
  - key: d
    text: Reduces communication overhead
    correct: false
- id: 16d0e7e7-9a67-4c34-8050-7846e0180d93
  type: flip
  front: Pipeline parallelism requires storing activations for backward pass. How can you reduce this
    memory?
  back: 'Recomputation (GPipe): recompute activations during backward instead of storing. Trades compute
    (2× forward compute) for memory (~50% activation memory saved).'
- id: eb61d112-26fa-413c-9a44-dcfad8f0c10c
  type: mcq
  front: In pipeline parallelism, why is the backward pass more complex than forward?
  back: ''
  choices:
  - key: a
    text: Backward requires all-reduce
    correct: false
  - key: b
    text: Backward needs activations from forward; storing all activations uses O(depth) memory
    correct: true
  - key: c
    text: Backward has no communication
    correct: false
  - key: d
    text: Backward is not complex
    correct: false
---

## Intuition
Pipeline Parallelism splits model layers across GPUs (GPU 0: layers 0-24, GPU 1: layers 25-48, etc.). Different GPUs process different microbatches in parallel, creating a pipeline. While GPU 1 processes microbatch 1, GPU 0 processes microbatch 2. Temporal pipelining: reduces idle time, increases GPU utilization.

## Detail
**Microbatching:** Split batch into M microbatches (e.g., batch=128 → 4 microbatches of 32). Forward pass: GPU 0 processes mb1, GPU 1 processes nothing. GPU 0 finishes, passes output to GPU 1 (GPU 0 starts mb2). GPU 1 processes mb1 while GPU 0 processes mb2. Pipeline of depth M.

**Forward vs Backward:** Forward pass straightforward (sequential microbatches). Backward more complex: need to store activations for all microbatches → large memory, or recompute (trade compute for memory). GPipe recomputes; PipeDream uses double buffering.

**Pipeline Bubble:** With P partitions and M microbatches (M >> P), bubble = (P-1)*M / (M+P-1) ≈ (P-1)/M for large M. Small M → large bubble (GPUs idle); large M → small bubble but more memory per microbatch.

**Backward Pass Scheduling:** GPipe uses synchronous backward (wait for all forward passes, then backward). PipeDream is async (overlap forward and backward of different microbatches). Async is harder but reduces bubble.

**Communication:** Activations passed between GPUs at layer boundaries. Size = batch_size * hidden_dim * num_layers_per_partition (can be large). Recomputation saves this activation memory.

## Common gotchas / interview framings
- Too few microbatches → large pipeline bubble, low utilization
- Too many microbatches → small microbatch size, inefficient compute (batch norm unstable)
- Backward pass not aligned with forward → gradient staleness, convergence issues
- Activation memory explosion with large hidden_dim + many layers
- Communication at layer boundaries: all-reduce not needed (sequential), just send activations

## See also
- [[tensor-parallelism-and-model-sharding]]
- [[data-parallelism-ddp]]
- [[gradient-accumulation]]

## Sources
See frontmatter `sources:`.

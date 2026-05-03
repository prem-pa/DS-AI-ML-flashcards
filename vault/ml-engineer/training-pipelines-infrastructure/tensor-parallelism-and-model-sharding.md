---
id: 7682a7d4-bcd7-4107-b7ca-01289d7ea17d
title: Tensor parallelism and model sharding
track: ml-engineer
topic: training-pipelines-infrastructure
difficulty: 5
tags:
- tensor-parallelism
- model-sharding
- tp
- column-parallel
- row-parallel
aliases:
- TP
- intra-layer parallelism
- model parallelism
sources:
- url: https://arxiv.org/abs/1909.08053
  label: 'Megatron-LM: Training Multi-Billion Parameter Language Models Using Model Parallelism'
- url: https://pytorch.org/docs/stable/distributed.tensor.html
  label: PyTorch Distributed Tensor (DTensor)
- url: https://github.com/pytorch/torchtitan
  label: 'TorchTitan: PyTorch Native Multi-GPU Training'
cards:
- id: 8b5573bc-f03a-4dc1-9637-74a7970619d7
  type: flip
  front: In tensor parallelism, how is a dense layer Y = XW split across 4 GPUs?
  back: 'Column-parallel: split W into 4 column chunks (W_1, W_2, W_3, W_4). Each GPU computes X @ W_i.
    Then all-reduce (sum) to get Y. Or row-parallel: split W by rows, all-gather X.'
- id: 4f460644-3112-4b47-b4ff-06d46d40ff09
  type: mcq
  front: Tensor parallelism requires synchronization via all-reduce. How does this differ from DDP?
  back: ''
  choices:
  - key: a
    text: Same all-reduce operation
    correct: false
  - key: b
    text: TP all-reduce is smaller (per-tensor), more frequent; DDP all-reduce is once per iteration
    correct: true
  - key: c
    text: DDP has no all-reduce
    correct: false
  - key: d
    text: TP is faster due to smaller tensors
    correct: false
- id: ac823054-0ea1-4df7-86a2-f3a128e85f7c
  type: flip
  front: A 7B parameter model doesn't fit on a single H100 (80GB). Should you use TP or DP to shard?
  back: TP (tensor parallelism) shards the model (weights) across GPUs. DP (data parallelism) replicates
    model on each GPU, needs GPU memory ≥ 7B params. For 7B on 80GB, DP works (with quantization or LoRA).
    TP needed for 70B+.
- id: 9bc3fdf1-e62b-480e-9795-01d831ad7e10
  type: mcq
  front: What is a key disadvantage of tensor parallelism?
  back: ''
  choices:
  - key: a
    text: Less efficient than data parallelism
    correct: false
  - key: b
    text: Frequent all-reduce operations; higher communication overhead than DP
    correct: true
  - key: c
    text: Cannot be combined with DP
    correct: false
  - key: d
    text: Requires custom model architecture
    correct: false
---

## Intuition
Tensor Parallelism splits individual weight matrices (tensors) across GPUs. For a dense layer Y = XW, split W column-wise: each GPU computes partial Y, then all-reduce. Different from Data Parallelism (replicate model, split data). Used when model too large to fit on single GPU.

## Detail
**Column Parallel (Output Projection):** Weight matrix W split by output dimension. X same on all ranks, each rank computes X @ W_i (partial output). Then all-reduce (sum partial outputs) to get Y. Communication: all-reduce of output size.

**Row Parallel (Input Projection):** Weight matrix W split by input dimension. Each rank computes X_i @ W, then all-gather X to reconstruct full output. Communication: all-gather of input size.

**Attention QKV Projections:** In Transformers, split Q, K, V projections via column parallelism. Each rank computes partial attention heads, gather to get full attention. Degree of TP = number of attention heads / GPU count (common: TP=8 with 96 heads).

**Activation Recomputation:** TP adds all-reduce/all-gather ops, increasing memory. Recompute activations instead of storing them (trade compute for memory). Critical for memory-constrained large models.

**Degree & Granularity:** TP degree (# of GPUs) limited by model architecture (e.g., # of heads, vocab size). Too high degree → fine-grained parallelism, communication overhead. Sweet spot: TP=8-16 for typical LLMs.

## Common gotchas / interview framings
- TP + DP together (2D parallelism): must coordinate tensor sharding + data sharding. DTensor simplifies this
- Pipeline Parallelism + TP = 3D: complex but necessary for 100B+ models
- Allreduce in TP synchronous; one slow rank → entire cluster waits
- Context Parallelism (CP) emerging: split sequence dimension, reduces activation memory (4D parallelism in TorchTitan)

## See also
- [[data-parallelism-ddp]]
- [[pipeline-parallelism-and-micro-batching]]
- [[gradient-synchronization-and-all-reduce]]

## Sources
See frontmatter `sources:`.

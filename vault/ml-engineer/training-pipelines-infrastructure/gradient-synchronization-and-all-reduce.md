---
id: 2bdcb7f7-7b51-4ca8-b3c7-af035594b742
title: Gradient synchronization and all-reduce
track: ml-engineer
topic: training-pipelines-infrastructure
difficulty: 3
tags:
- all-reduce
- communication
- nccl
- ring-allreduce
- collective-communication
aliases:
- gradient synchronization
- ring all-reduce
- NCCL communication
sources:
- url: https://docs.nvidia.com/deeplearning/nccl/user-guide/docs/overview.html
  label: NVIDIA NCCL Documentation
- url: https://pytorch.org/docs/stable/distributed.html
  label: PyTorch Distributed Package
- url: https://arxiv.org/abs/1811.05957
  label: Reducing Communication Overhead (Ring All-Reduce)
cards:
- id: 6ea9cb84-ae78-4654-ae24-b878ca27b15e
  type: flip
  front: What does ring all-reduce do and why is it efficient?
  back: Arranges N ranks in ring. Each rank sends gradient to next neighbor, receives from previous. N-1
    forward + N-1 backward rounds. Communication O(model_size), near-optimal. Reduces latency vs tree
    topology.
- id: 5a23e815-f40c-4172-a112-94e5ce98aadd
  type: mcq
  front: Gradient buckets are used in all-reduce. What is their purpose?
  back: ''
  choices:
  - key: a
    text: Reduce memory usage
    correct: false
  - key: b
    text: Overlap all-reduce communication with backward pass computation
    correct: true
  - key: c
    text: Compress gradients
    correct: false
  - key: d
    text: Reduce numerical errors
    correct: false
- id: 5c98c10f-3a2f-4597-a6c6-61032abb7955
  type: flip
  front: In a ring all-reduce, what's the bottleneck if one link is 10× slower than others?
  back: Entire ring is limited by slowest link. Each round waits for slowest link to finish, giving bottleneck
    = 10× the slowest bandwidth instead of N links in parallel.
- id: ff03bde0-2ffa-4f70-ba5c-343ef4b17193
  type: mcq
  front: NCCL bandwidth on H100 NVLink vs Ethernet. Which is faster?
  back: ''
  choices:
  - key: a
    text: NVLink ~50 GB/s, Ethernet ~200 GB/s
    correct: false
  - key: b
    text: NVLink ~900 GB/s, Ethernet ~200 GB/s
    correct: true
  - key: c
    text: Same speed
    correct: false
  - key: d
    text: Depends on gradient size
    correct: false
---

## Intuition
In DDP, all GPUs compute gradients independently, then must synchronize (average) to agree on shared model update. All-reduce is the collective communication primitive: each rank sends its gradients to all others, computes global average, all get same result. Bottleneck for large models or slow networks.

## Detail
**All-Reduce Definition:** Each rank i has vector g_i (gradients). All-reduce produces: result = (g_1 + g_2 + ... + g_N) / N on every rank. Operations: sum + reduction, broadcast result. Can be implemented via tree, butterfly, or ring topology.

**Ring All-Reduce:** N ranks arranged in ring topology. Each rank sends gradient to next, receives from previous. N-1 rounds of forward pass + N-1 rounds of backward pass. Communication cost: O(model_size), near-optimal. Less latency-sensitive than tree (which has log(N) latency, but larger bandwidth per node).

**NCCL (Nvidia Collective Communication Library):** NVIDIA's all-reduce implementation, optimized for NVLink/InfiniBand. Typical bandwidth: ~900 GB/s (H100 NVLink), ~200 GB/s (Ethernet).

**Gradient Buckets:** Instead of all-reduce entire model at once, split into buckets and all-reduce incrementally. Allows overlapping communication with backward pass of next layer. Typical bucket size: 25-100 MB.

**InfiniBand vs Ethernet:** InfiniBand (low latency, high bandwidth) much better than Ethernet. Multi-node training heavily benefits from InfiniBand. Ethernet can be bottleneck, especially with large models.

## Common gotchas / interview framings
- Gradient all-reduce latency-bound for small models/small N; bandwidth-bound for large models/large N
- Not overlapping all-reduce with computation → full synchronous wait, no pipelining
- Gradient bucket size too small → too many all-reduce ops, overhead; too large → no overlap
- Unbalanced ring (some links slower) → entire ring limited by slowest link
- Heterogeneous hardware → all-reduce speed limited by slowest device

## See also
- [[data-parallelism-ddp]]
- [[gradient-accumulation]]
- [[distributed-sampling-and-epoch-synchronization]]

## Sources
See frontmatter `sources:`.

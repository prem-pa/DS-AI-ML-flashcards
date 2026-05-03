---
id: a2d235da-7ef3-4e3b-95b7-55a7768e2ba7
title: Data parallelism (DDP)
track: ml-engineer
topic: training-pipelines-infrastructure
difficulty: 3
tags:
- ddp
- data-parallelism
- distributed-training
- synchronization
- all-reduce
aliases:
- distributed data parallel
- multi-GPU training
- gradient synchronization
sources:
- url: https://pytorch.org/docs/stable/generated/torch.nn.parallel.DistributedDataParallel.html
  label: PyTorch DistributedDataParallel
- url: https://pytorch.org/tutorials/intermediate/ddp_tutorial.html
  label: PyTorch DDP Tutorial
- url: https://pytorch.org/docs/stable/notes/cuda.html#cuda-memory-allocation
  label: PyTorch CUDA Memory Management
cards:
- id: 4f448bd9-d65b-42de-9054-e0393cb0ae06
  type: flip
  front: In DDP with 4 GPUs and batch_size=64, what is the effective batch size per iteration?
  back: 256 (64 × 4). Each GPU processes 64 samples, gradients averaged over 256. May need higher learning
    rate (linear scaling rule).
- id: c1c0723d-3d5c-4c45-800c-41bb056033d4
  type: mcq
  front: What happens if you use DistributedDataLoader without DistributedSampler?
  back: ''
  choices:
  - key: a
    text: Training works fine
    correct: false
  - key: b
    text: Each GPU sees the same batch; no true parallelism, wastes compute
    correct: true
  - key: c
    text: Gradients are randomly shuffled
    correct: false
  - key: d
    text: All-reduce fails
    correct: false
- id: 8839dbd1-8563-40ae-9ec6-235761253e84
  type: flip
  front: DDP synchronizes gradients via all-reduce. What is the communication overhead?
  back: O(model_size). All-reduce requires sending gradient tensors across all ranks. Ring all-reduce
    reduces from O(N²) to O(2N-2) links but still O(model_size) data volume. Bottleneck for very large
    models.
- id: f32ca2d8-ff4c-4dd7-8db1-c05ade3c5fa3
  type: mcq
  front: What does DDP do to handle stragglers (slower GPUs)?
  back: ''
  choices:
  - key: a
    text: Automatically balances workload
    correct: false
  - key: b
    text: Synchronizes at each backward() call (synchronous); slow GPU delays all others
    correct: true
  - key: c
    text: Uses async gradients to avoid waiting
    correct: false
  - key: d
    text: Sends partial gradients
    correct: false
---

## Intuition
Data Parallelism: replicate model on each GPU. Each GPU trains on different batch of data, computes gradients, synchronizes (all-reduce) to average gradients, updates model with averaged gradient. Effective batch size = batch_size * num_gpus. Simplest form of distributed training.

## Detail
**DDP Setup:** Wrap model: `model = DistributedDataParallel(model)`. Use DistributedSampler to partition data. Each rank (GPU) calls forward/backward independently, DDP synchronizes gradients in backward().

**Synchronization Mechanism:** Each rank computes local gradients → DDP buckets gradients (e.g., 1MB chunks) → all-reduce (NCCL) sums and averages gradients across ranks → each rank updates model with averaged gradient. Synchronous: all ranks must finish before averaging (slowdown if one rank slower).

**Effective Batch Size:** batch_size * num_gpus. Learning rate may need adjustment (linear scaling rule). Warmup typically scales with effective batch size.

**Gradient Buckets & Overlap:** DDP uses gradient buckets (default ~25MB) to overlap all-reduce communication with backward pass. Large buckets → less overhead but less overlap; small buckets → more overhead but better overlap.

**Broadcast & Parameters:** After all-reduce, DDP broadcasts updated model weights to all ranks (single weight copy for correctness).

## Common gotchas / interview framings
- Forgot DistributedSampler → data duplication across ranks (same batch on every GPU)
- Different batch sizes across ranks → all-reduce mismatch, deadlock
- Synchronous DDP with stragglers → slow rank delays all others (consider async gradient descent or DDP with async_reduce flag)
- Large gradient buckets → less communication overlap, slower
- Load imbalance (uneven data split) → some ranks finish early, idle time before all-reduce

## See also
- [[gradient-synchronization-and-all-reduce]]
- [[distributed-sampling-and-epoch-synchronization]]
- [[tensor-parallelism-and-model-sharding]]

## Sources
See frontmatter `sources:`.

---
id: a551d908-0087-4589-8c9d-ae596f9be5a7
title: Distributed sampling and epoch synchronization
track: ml-engineer
topic: training-pipelines-infrastructure
difficulty: 3
tags:
- distributed-sampling
- epoch-sync
- sampler
- data-duplication
- worker-coordination
aliases:
- sampler coordination
- rank synchronization
- epoch boundary
sources:
- url: https://pytorch.org/docs/stable/data.html#torch.utils.data.distributed.DistributedSampler
  label: PyTorch DistributedSampler
- url: https://pytorch.org/docs/stable/distributed.html#torch.distributed.barrier
  label: PyTorch Distributed Barrier
- url: https://docs.ray.io/en/latest/train/examples/pytorch/pytorch-dpp/README.html
  label: Ray Train Distributed Sampling
cards:
- id: b402528a-6a1a-4758-bdbd-9184814ed0a8
  type: flip
  front: In distributed training with 4 GPUs and 1000 samples, how many samples does each GPU see per
    epoch?
  back: '250 samples. DistributedSampler partitions: rank 0 [0:250], rank 1 [250:500], rank 2 [500:750],
    rank 3 [750:1000]. Non-overlapping.'
- id: 441abd82-62e4-4d7e-8e0b-70a7004a1ebc
  type: mcq
  front: Why must you call `sampler.set_epoch(epoch)` in distributed training?
  back: ''
  choices:
  - key: a
    text: To save epoch number
    correct: false
  - key: b
    text: To ensure different shuffle permutation each epoch, maintaining dataset diversity
    correct: true
  - key: c
    text: To sync all ranks
    correct: false
  - key: d
    text: No reason; it's optional
    correct: false
- id: d313cf5e-72e9-42be-a14e-27f6338ddfa7
  type: flip
  front: What does `torch.distributed.barrier()` do?
  back: Synchronization primitive. Each rank blocks until all ranks reach barrier. Ensures all ranks finish
    current phase (e.g., end of epoch) before proceeding to next. Prevents race conditions.
- id: 316a4a95-c22d-4018-9c5f-593d822341b8
  type: mcq
  front: Dataset has 1003 samples, 4 GPUs. DistributedSampler with drop_last=False. How many samples per
    rank?
  back: ''
  choices:
  - key: a
    text: 250 + 3 pad
    correct: true
  - key: b
    text: '250'
    correct: false
  - key: c
    text: '251'
    correct: false
  - key: d
    text: 250, and last rank gets 253
    correct: false
---

## Intuition
In distributed training, multiple workers (GPUs/machines) train in parallel. Each must see different data (no duplication) and stay synchronized (same epoch boundaries). DistributedSampler handles this: partitions data indices, each rank gets non-overlapping subset, all sync at epoch start.

## Detail
**DistributedSampler:** Splits dataset of N samples into P partitions (P = num_ranks). Rank i gets indices [i*N/P, (i+1)*N/P). No overlap, no duplication. Each epoch, shuffle with different seed per rank via set_epoch().

**Set Epoch:** Must call `sampler.set_epoch(epoch)` before each epoch. This sets random seed internally: seed_per_rank = seed_base + epoch. Different epochs → different shuffle permutations, but same base seed across ranks → same disjoint partition.

**Barrier Synchronization:** Use `torch.distributed.barrier()` to ensure all ranks finish epoch N before starting N+1. Prevents some ranks ahead causing stale data access (e.g., rank 0 loads epoch 10 while rank 1 still on epoch 9).

**Data Duplication Issues:** Forgetting DistributedSampler or not using shuffle properly → some samples seen by multiple ranks, others never seen. Gradient estimates biased, convergence affected.

**Non-Divisible Datasets:** If N % P != 0, some ranks get fewer samples. DistributedSampler pads with drop_last=False (default) or drops excess with drop_last=True. For reproducibility, use drop_last=True.

## Common gotchas / interview framings
- Forgot `set_epoch()` → same permutation every epoch; model may memorize batch order
- `shuffle=True` in DataLoader + DistributedSampler → double shuffling or inconsistent random state
- Missing barrier() → some ranks ahead of others, loss tracking wrong
- Non-synchronized random seed across ranks → different partitions (duplication or gaps)
- Changing num_workers per rank without adjusting sampler → inconsistent data splits

## See also
- [[data-loading-and-preprocessing-at-scale]]
- [[shuffling-and-batching-strategies]]
- [[data-parallelism-ddp]]

## Sources
See frontmatter `sources:`.

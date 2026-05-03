---
id: 1184edbd-7734-416a-8113-9d970388d32f
title: Data loading and preprocessing at scale
track: ml-engineer
topic: training-pipelines-infrastructure
difficulty: 3
tags:
- dataloader
- prefetching
- i/o-bottleneck
- multiprocessing
- async-loading
aliases:
- dataloader optimization
- batching pipeline
- I/O prefetch
sources:
- url: https://pytorch.org/docs/stable/data.html
  label: PyTorch DataLoader Documentation
- url: https://docs.ray.io/en/latest/train/user-guide/data-loading.html
  label: Ray Train Data Loading Guide
- url: https://lightning.ai/docs/pytorch/stable/data/dataloader.html
  label: PyTorch Lightning DataLoader Tutorial
cards:
- id: 52b24cb4-f41f-41fa-9614-635960d00efa
  type: flip
  front: Why does DataLoader with `num_workers=0` hurt GPU utilization?
  back: Main process blocks on CPU preprocessing, leaving GPU idle. `num_workers > 0` spawns worker processes
    that prefetch batches asynchronously while GPU trains, overlapping I/O with compute.
- id: bc69b1be-6300-44f5-bb29-3ae523038923
  type: mcq
  front: In distributed training with 4 GPUs, what does DistributedSampler do?
  back: ''
  choices:
  - key: a
    text: Randomly shuffles data across all workers every iteration
    correct: false
  - key: b
    text: Partitions dataset into 4 unique non-overlapping shards, each worker gets 1/4
    correct: true
  - key: c
    text: Duplicates data across workers for redundancy
    correct: false
  - key: d
    text: Creates 4 copies of the dataset in memory
    correct: false
- id: b9d8a2fb-e124-4630-9f99-5c09a48a5208
  type: flip
  front: When should you call `sampler.set_epoch(epoch)`?
  back: Before each epoch when using DistributedSampler. This ensures different random shuffling per epoch
    across all workers, maintaining statistical independence.
- id: 138304c8-1add-4693-aa4f-ccb2ffe7ad73
  type: mcq
  front: Which technique reduces memory footprint for billion-row datasets?
  back: ''
  choices:
  - key: a
    text: Increase batch size
    correct: false
  - key: b
    text: Load entire dataset into CPU RAM
    correct: false
  - key: c
    text: Use memory-mapped files (HDF5/Parquet) with lazy loading
    correct: true
  - key: d
    text: Use float32 instead of float64
    correct: false
---

## Intuition
Data loading is often the first bottleneck in training pipelines. GPUs train faster than CPUs can fetch and preprocess data, causing idle compute. At scale (TB+ datasets, distributed training), this gap grows exponentially. Effective data loading overlaps I/O with computation via prefetching and multiprocessing.

## Detail
**Prefetching & Async I/O:** Use `num_workers > 0` in DataLoader to spawn CPU processes that fetch and preprocess batches while GPU trains on current batch. Typical range: 4-8 workers per GPU. `pin_memory=True` locks tensors in CPU RAM for faster host-to-device transfer (~5-10% speedup).

**Distributed Sampling:** With DistributedSampler, each worker sees unique subset of data—no duplication across GPUs. Set `shuffle=True` per epoch and synchronize `set_epoch(epoch)` before each DataLoader reset to ensure correct shuffling across workers.

**Memory Efficiency:** Large datasets fit in memory via memory-mapped files (HDF5, Parquet) instead of loading all at once. Lazy loading + on-the-fly augmentation (torchvision, albumentations) reduces RAM footprint.

## Common gotchas / interview framings
- Forgot to call `sampler.set_epoch(epoch)` in distributed training → determinism breaks, train/val confusion
- `num_workers=0` is safe but slow; find optimal `num_workers` via profiling (too many → overhead)
- DistributedSampler + DataLoader shuffle=True → shuffle is redundant; let sampler handle it
- Memory-pinned batches require GPU memory; balance batch size vs available GPU RAM

## See also
- [[distributed-sampling-and-epoch-synchronization]]
- [[shuffling-and-batching-strategies]]
- [[validation-strategy-and-metric-selection]]
- [[gradient-accumulation]]

## Sources
See frontmatter `sources:`.

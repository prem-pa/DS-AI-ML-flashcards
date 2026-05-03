---
id: 62298f69-332d-41e4-a2ca-fc70d32c8b26
title: Shuffling and batching strategies
track: ml-engineer
topic: training-pipelines-infrastructure
difficulty: 3
tags:
- shuffling
- batch-size
- sampling
- epoch-coordination
- random-seed
aliases:
- batch strategy
- sampling order
- epoch sync
sources:
- url: https://pytorch.org/docs/stable/data.html#torch.utils.data.DistributedSampler
  label: PyTorch DistributedSampler
- url: https://pytorch.org/docs/stable/data.html#torch.utils.data.RandomSampler
  label: PyTorch RandomSampler
- url: https://en.wikipedia.org/wiki/Fisher%E2%80%93Yates_shuffle
  label: Fisher-Yates Shuffle Algorithm
cards:
- id: ec9b4d05-6c8d-4de9-9426-60589f7de2a5
  type: flip
  front: Why shuffle training data?
  back: Without shuffling, gradients are biased (model sees same correlations each epoch). Shuffling decorrelates
    samples, leading to unbiased gradient estimates and better generalization.
- id: 61710336-1e40-425a-9f3b-236d259bde3f
  type: mcq
  front: What happens if you use the same random seed for shuffling across all 100 epochs?
  back: ''
  choices:
  - key: a
    text: Improves reproducibility
    correct: false
  - key: b
    text: Data order is identical each epoch; model may memorize order patterns, hurting generalization
    correct: true
  - key: c
    text: Increases learning efficiency
    correct: false
  - key: d
    text: No effect on training
    correct: false
- id: 31eb2ba6-1108-42cb-9ece-1f9b6f1051a4
  type: flip
  front: In distributed training with 8 GPUs and batch_size=128, what is the effective batch size?
  back: 1024 samples per iteration (128 per GPU × 8 GPUs). This affects learning rate scheduling—larger
    effective batch often requires higher learning rate.
- id: d6614119-59c0-4f9d-8dbe-1ac29625bf02
  type: mcq
  front: Which sampling strategy ensures no overlap across 4 distributed workers?
  back: ''
  choices:
  - key: a
    text: Random shuffling on each worker independently
    correct: false
  - key: b
    text: DistributedSampler with same seed on each worker
    correct: true
  - key: c
    text: Sequential chunking (worker 0 gets indices 0-N/4, worker 1 gets N/4-N/2, etc.)
    correct: false
  - key: d
    text: Let each worker sample randomly with replacement
    correct: false
---

## Intuition
Shuffling decorrelates sequential patterns in data, making gradient updates less biased. Without shuffling, model sees same order every epoch and may learn spurious temporal dependencies. Batching aggregates losses and gradients, reducing noise and improving convergence.

## Detail
**Shuffling vs Order:** Shuffled training → unbiased gradients, better generalization. Deterministic order → reproducible but potentially biased. In distributed training, need coordinated shuffling across all workers to maintain independence.

**Batch Size Trade-offs:** Larger batches (e.g., 256→1024) are noisier (lower SNR per sample) but reduce communication overhead per iteration. Can lead to sharper minima (worse generalization) or flatter minima (better generalization) depending on optimization landscape.

**Random Seed & Reproducibility:** Set `seed=42` in sampler for reproducible shuffling. Different seeds per epoch prevent leakage. In distributed: each worker uses same base seed, sampler handles offset per worker to ensure disjoint subsets.

**Batching Impact:** Effective batch size = batch_size * num_gpus (data parallel). Some studies show larger batches need longer warmup and higher learning rate (linear scaling rule).

## Common gotchas / interview framings
- Shuffle in DataLoader AND DistributedSampler → double shuffling or inconsistent seed behavior
- Same seed across epochs → model may overfit to batch order patterns
- Forget `shuffle=True` in train DataLoader → training is deterministic, hurts regularization
- Changing batch size mid-training without adjusting learning rate → divergence

## See also
- [[data-loading-and-preprocessing-at-scale]]
- [[batch-size-effects-on-generalization]]
- [[distributed-sampling-and-epoch-synchronization]]

## Sources
See frontmatter `sources:`.

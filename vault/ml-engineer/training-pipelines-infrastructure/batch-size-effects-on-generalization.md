---
id: 5148fdff-1f82-4409-87c3-0f7e6fd1ba57
title: Batch size effects on generalization
track: ml-engineer
topic: training-pipelines-infrastructure
difficulty: 3
tags:
- batch-size
- generalization
- sharpness
- flatness
- gradient-noise
aliases:
- batch size and overfitting
- large batch training
- batch size and learning rate
sources:
- url: https://arxiv.org/abs/1609.04836
  label: On Large-Batch Training for Deep Learning
- url: https://arxiv.org/abs/1711.00489
  label: The Generalization Gap in Machine Learning
- url: https://arxiv.org/abs/1411.1134
  label: Deep Residual Learning (ResNet, batch size effects)
cards:
- id: a6bf3688-8a45-49f8-ba46-33bd99aa61f4
  type: flip
  front: Why might small batch sizes lead to better generalization than large batches?
  back: 'Small batches → noisy gradients → can escape sharp minima. Large batches → precise gradients
    → settle in nearby minimum (often sharp). Sharp minima: high loss near minimum, poor generalization.
    Noisy gradients help escape.'
- id: 87483bc5-3bdd-4879-8a84-c940d46b5dfd
  type: mcq
  front: You increase batch size from 256 to 2048 (8×). What should you do to learning rate?
  back: ''
  choices:
  - key: a
    text: Keep it same
    correct: false
  - key: b
    text: Increase by √8 ≈ 2.8× (linear scaling rule)
    correct: true
  - key: c
    text: Decrease to 1/8
    correct: false
  - key: d
    text: No clear answer
    correct: false
- id: 9da4ca9d-eec9-4083-a3a9-78315374a721
  type: flip
  front: Large batch training is prone to divergence early. How do you mitigate?
  back: Use longer warmup (5-10%+ of epochs). Start with low LR, linearly increase to target LR. Stabilizes
    early training before settling into minima.
- id: 2ea29af1-a368-4352-94b9-ba66d39f80f5
  type: mcq
  front: Batch norm statistics (mean/var) computed per-batch. Large batch → more stable BN, but loss landscape
    becomes?
  back: ''
  choices:
  - key: a
    text: Flatter
    correct: false
  - key: b
    text: Sharper
    correct: true
  - key: c
    text: Unaffected
    correct: false
  - key: d
    text: Depends on model
    correct: false
---

## Intuition
Large batch sizes (e.g., 4096) compute gradient from many samples, reducing noise. Small batches (e.g., 32) noisier gradients. Conventional wisdom: small batches → high variance → regularization effect → better generalization. Large batches → low variance → sharpness in loss landscape → overfitting. But empirical results mixed: large batches with proper LR tuning can match small batches.

## Detail
**Batch Size & Gradient Noise:** Small batch → noisier gradient estimates (high variance). Noise can escape sharp minima (saddle point avoidance). Large batch → precise gradient, settles into nearest minimum (sharp or flat). Sharp minima have lower test accuracy (small perturbations in weights cause large loss changes).

**Flatness & Generalization (SAC):** Sharpness Aware Minimization (SAM) finds flat minima, improving generalization. Sharp minima: high loss at small weight perturbations. Flat minima: robust to perturbations. Larger batch → risk of sharp minima.

**Linear Scaling Rule:** When increasing batch size by factor k, increase LR by √k (or k depending on regime). Allows large-batch training to match small-batch final accuracy. E.g., 8× larger batch → 2-3× higher LR.

**Warmup with Large Batch:** Large batches need longer warmup (5-10% of epochs or more). Early training with large batch unstable; warmup stabilizes by starting low LR, linearly increasing.

**Generalization Gap:** Train loss can be low on both large and small batches, but test gap (test loss - train loss) different. Large batch + large LR → larger test gap (overfitting). Small batch or regularization → smaller test gap.

## Common gotchas / interview framings
- Blindly increase batch size without adjusting LR → training diverges or converges slower
- Large batch training needs longer warmup; forget warmup → divergence in first epoch
- Same LR for different batch sizes → large batch may diverge (too small effective LR for batch size)
- Batch norm statistics computed per batch; large batch → more stable BN stats, but loss landscape sharper
- Gradient accumulation increases effective batch size; adjust LR accordingly

## See also
- [[shuffling-and-batching-strategies]]
- [[learning-rate-schedules-constant-step-decay-cosine-annealing-warm-restarts]]
- [[momentum-and-weight-decay-interaction]]

## Sources
See frontmatter `sources:`.

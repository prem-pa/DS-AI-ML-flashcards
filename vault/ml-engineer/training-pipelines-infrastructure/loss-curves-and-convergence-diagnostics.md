---
id: f261e5d4-c6b7-49d3-8d88-02e453cd2f5f
title: Loss curves and convergence diagnostics
track: ml-engineer
topic: training-pipelines-infrastructure
difficulty: 3
tags:
- loss-analysis
- convergence
- debugging
- learning-rate
- overfitting
aliases:
- convergence debugging
- loss plateauing
- NaN/divergence detection
sources:
- url: https://pytorch.org/docs/stable/amp.html
  label: PyTorch Automatic Mixed Precision
- url: https://wandb.ai/guides/loss-curves
  label: 'Weights & Biases: Loss Curves Guide'
- url: https://karpathy.github.io/2019/04/25/recipe/
  label: 'Andrej Karpathy: Recipe for Training Neural Networks'
cards:
- id: d5e9cfe2-606d-490f-a8b4-f7cdad90361c
  type: flip
  front: Loss suddenly spikes to NaN during training. What are the top 3 causes?
  back: 1) Learning rate too high (exploding gradients). 2) Numerical instability (log of negative, division
    by zero). 3) Mixed precision underflow/overflow without proper scaling. Check gradient norm, rescale
    targets, add clipping.
- id: d67a1b09-035b-4b82-94c4-c5d9c70a84ec
  type: mcq
  front: If training loss decreases smoothly but validation loss increases after epoch 5, what's likely
    happening?
  back: ''
  choices:
  - key: a
    text: Model is learning correctly
    correct: false
  - key: b
    text: Overfitting; need more regularization or early stopping
    correct: true
  - key: c
    text: Learning rate too low
    correct: false
  - key: d
    text: Batch size too small
    correct: false
- id: 52ebb268-421c-4ef2-b9cd-d6757acde70e
  type: flip
  front: Training loss plateaus at 2.3 after epoch 10, no improvement. What's your first diagnostic step?
  back: 'Check learning rate: if too low, loss progresses slowly. Plot gradient norm—if tiny, LR needs
    increase. If gradient norm large but loss flat, loss landscape is flat (poor conditioning, increase
    LR).'
- id: 14039f56-67dd-495f-a9be-13fc7db1a79d
  type: mcq
  front: In mixed precision training, loss becomes NaN after 100 iterations. Which fix is most likely
    to help?
  back: ''
  choices:
  - key: a
    text: Decrease batch size
    correct: false
  - key: b
    text: Increase loss scale or add gradient clipping
    correct: true
  - key: c
    text: Switch to float32
    correct: false
  - key: d
    text: Reduce model capacity
    correct: false
---

## Intuition
Loss curve tells the whole story: is learning rate too high (NaN)? Too low (plateaus)? Overfitting (train ↓, val ↑)? Underfitting (both flat)? Reading loss curves is core diagnosis skill in training.

## Detail
**Healthy Curve:** Loss decreases smoothly, steeper initially, flattens as learning rate schedule decays. Train/val gap widens slightly over epochs (normal—train sees more epochs). No spikes or NaNs.

**Learning Rate Too High:** Loss oscillates wildly, often spikes to NaN/inf. Gradient norm explodes. Fix: lower LR, add gradient clipping.

**Learning Rate Too Low:** Loss decreases very slowly, plateaus without reaching good performance. Gradient updates tiny. Fix: increase LR, shorten warmup.

**Overfitting:** Training loss ↓ continuously, validation loss ↑ after initial decrease. Gap widens. Fix: increase regularization (dropout, weight decay, augmentation), reduce model capacity, use early stopping.

**Underfitting:** Both training and validation loss plateau early at high value. Model capacity insufficient or LR too high (preventing learning). Fix: increase capacity, lower LR, train longer.

**NaN/Inf Detected:** Usually from exploding gradients, numerical instability in loss (log of negative), or mixed precision issues. Check gradient norm, rescale targets, add gradient clipping.

## Common gotchas / interview framings
- Loss oscillates slightly → normal in SGD; if huge swings → LR too high
- Train loss stops decreasing but val loss still improves → overfitting; train may not need more epochs
- Gradient norm grows unbounded → gradient clipping or LR reduction needed
- Batch size changed mid-training → learning rate must adjust (linear scaling rule)

## See also
- [[checkpointing-and-recovery]]
- [[validation-strategy-and-metric-selection]]
- [[learning-rate-schedules-constant-step-decay-cosine-annealing-warm-restarts]]

## Sources
See frontmatter `sources:`.

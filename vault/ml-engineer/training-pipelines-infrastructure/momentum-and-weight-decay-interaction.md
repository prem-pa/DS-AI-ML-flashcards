---
id: 5be57e20-e512-4c4c-948a-69f3dcccefe0
title: Momentum and weight decay interaction
track: ml-engineer
topic: training-pipelines-infrastructure
difficulty: 5
tags:
- momentum
- weight-decay
- adamw
- l2-regularization
- optimizer
aliases:
- weight decay vs L2
- decoupled weight decay
- momentum buffer
sources:
- url: https://arxiv.org/abs/1711.05101
  label: Decoupled Weight Decay Regularization (AdamW Paper)
- url: https://pytorch.org/docs/stable/generated/torch.optim.Adam.html
  label: PyTorch Adam Optimizer
- url: https://pytorch.org/docs/stable/generated/torch.optim.SGD.html
  label: PyTorch SGD with Momentum
cards:
- id: 392a2d8d-35fd-4c10-b8a8-0a35145602fb
  type: flip
  front: Why is AdamW recommended over Adam with weight decay?
  back: 'Adam couples weight decay with adaptive LR (scaled by second moment estimate). Large weight gradients
    → small adaptive LR → weak weight decay. AdamW decouples: applies weight decay directly, independent
    of adaptive LR, more effective.'
- id: 55af72b9-d0f2-4e9d-a3c3-c3143d943b7f
  type: mcq
  front: In SGD with momentum, weight decay ≠ L2 regularization because?
  back: ''
  choices:
  - key: a
    text: They're actually equivalent
    correct: false
  - key: b
    text: Weight decay term not accumulated in momentum buffer; momentum doesn't affect regularization
      term
    correct: true
  - key: c
    text: Weight decay is faster
    correct: false
  - key: d
    text: Momentum buffer amplifies weight decay
    correct: false
- id: 8537be8f-55ec-4c7d-a1a1-48d5385ed62a
  type: flip
  front: You're using Adam optimizer with weight_decay=0.01. Why is this ineffective?
  back: Adam's adaptive learning rate scales weight decay down when weights are large (when you want regularization
    most). Use AdamW instead, which decouples weight decay from adaptive LR.
- id: 5c3cd28d-22e4-44c5-ad9a-8a5e18698390
  type: mcq
  front: In training, you switch from SGD to Adam. Should you adjust weight decay value?
  back: ''
  choices:
  - key: a
    text: No, use same value
    correct: false
  - key: b
    text: Yes, typically increase weight decay for AdamW (different effective strength)
    correct: true
  - key: c
    text: Yes, set to zero
    correct: false
  - key: d
    text: Use L2 regularization instead
    correct: false
---

## Intuition
Weight decay regularizes by penalizing large weights. But its interaction with momentum differs: in SGD + momentum, weight decay scales inversely with momentum (implicit). In Adam, weight decay + momentum don't interact as intended. AdamW (decoupled) applies weight decay independently, fixing this. Critical distinction for modern training.

## Detail
**L2 Regularization:** Add λ * ||w||^2 to loss. Gradient = original_grad + 2*λ*w. Update: w -= lr * (grad + 2*λ*w). Discourages large weights, reduces variance (generalization).

**Weight Decay in SGD + Momentum:** w -= lr * (grad + λ*w). With momentum: m = β*m + grad → w -= lr * (m + λ*w). The λ*w term is NOT accumulated in momentum buffer. This causes weight decay to decouple from momentum scaling, an implicit effect of SGD (unintended but works empirically).

**Weight Decay in Adam (Coupled):** Gradient = grad + λ*w. Then Adam's adaptive learning applies. Problem: if weights large, grad is large, adaptive LR scales down, but weight decay term λ*w scaled by same adaptive LR. Weight decay is weakened when you want it most (large weights). Leads to ineffective regularization in Adam.

**AdamW (Decoupled Weight Decay):** Separate weight decay from gradient: w -= lr * (m_adapted + λ*w). Weight decay λ*w applied directly, not through adaptive LR. More consistent regularization, better generalization. Now standard (better than coupling).

**Practical Implications:** SGD + momentum: weight decay works well, use l2_regularization or weight_decay (equivalent). Adam: MUST use AdamW, not Adam + weight decay (coupled). Weight decay magnitude differs between optimizers; empirical tuning needed.

## Common gotchas / interview framings
- Adam + weight_decay > 0 → weight decay weakened, ineffective. Use AdamW instead
- Changing weight decay mid-training → regularization changes, affects convergence
- Weight decay too high → underfitting (over-regularization). Too low → overfitting
- Momentum buffer initialization: starts at 0, takes ~1/β epochs to stabilize
- Different optimizers need different weight decay values for same regularization effect

## See also
- [[loss-curves-and-convergence-diagnostics]]
- [[learning-rate-schedules-constant-step-decay-cosine-annealing-warm-restarts]]
- [[batch-size-effects-on-generalization]]

## Sources
See frontmatter `sources:`.

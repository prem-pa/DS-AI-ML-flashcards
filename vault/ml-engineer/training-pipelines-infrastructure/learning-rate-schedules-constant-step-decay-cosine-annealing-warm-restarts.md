---
id: 738aa9b2-78c5-4d4f-89b2-a7564e3b9859
title: Learning rate schedules (constant, step decay, cosine annealing, warm restarts)
track: ml-engineer
topic: training-pipelines-infrastructure
difficulty: 3
tags:
- learning-rate-schedule
- lr-decay
- cosine-annealing
- warmup
- warm-restarts
aliases:
- LR schedule
- learning rate decay
- annealing schedule
sources:
- url: https://pytorch.org/docs/stable/generated/torch.optim.lr_scheduler.html
  label: PyTorch Learning Rate Schedulers
- url: https://arxiv.org/abs/1608.03983
  label: 'SGDR: Stochastic Gradient Descent with Warm Restarts'
- url: https://arxiv.org/abs/1706.02677
  label: Warm Restarts and Learning Rate Annealing
cards:
- id: 524cdeb2-2f17-4018-b041-a2ac75d13f82
  type: flip
  front: Why use cosine annealing over constant learning rate?
  back: Cosine annealing smoothly reduces LR as training progresses, balancing exploration (high LR early)
    and exploitation (low LR late). Constant LR can't do both; cosine empirically outperforms.
- id: 52f5f303-66f6-445c-bf7d-bca4ec3a774e
  type: mcq
  front: Cosine annealing with T=100 (total epochs). At epoch 50, LR is approximately?
  back: ''
  choices:
  - key: a
    text: 0.5 * LR_max
    correct: false
  - key: b
    text: 0.15 * LR_max (near LR_min)
    correct: true
  - key: c
    text: Same as LR_max
    correct: false
  - key: d
    text: Increases
    correct: false
- id: a2a22088-9094-44fb-9482-c62b8afd3803
  type: flip
  front: When should you use warmup in learning rate schedule?
  back: Always in deep learning, especially with large batch sizes or fine-tuning. Warmup (e.g., 5-10%
    of epochs) prevents early divergence by starting with low LR, linearly increasing to target.
- id: a4a7cbfa-e2fe-4b0d-b724-468a8b0908b7
  type: mcq
  front: Warm Restarts (SGDR) periodically reset LR high. What is the benefit?
  back: ''
  choices:
  - key: a
    text: Faster convergence
    correct: false
  - key: b
    text: Multiple descent phases, may escape local minima or find better solutions
    correct: true
  - key: c
    text: Reduces memory
    correct: false
  - key: d
    text: No benefit; just adds complexity
    correct: false
---

## Intuition
Training dynamics benefit from high learning rate early (fast progress), lower LR late (convergence to minima). Learning rate schedule adapts LR over training. Constant → simple but suboptimal. Step decay → reduce by factor every N epochs. Cosine annealing → smooth decay. Warm restarts → periodically reset LR high, create multiple descent phases.

## Detail
**Constant LR:** Same LR throughout. Simple, but requires careful initial tuning. Often too high (early divergence) or too low (slow convergence late). Baseline for comparison.

**Step Decay:** Multiply LR by gamma every N epochs (e.g., LR *= 0.1 every 30 epochs). Discrete drops; loss may spike at transition. Common in older papers; mostly replaced by cosine annealing.

**Cosine Annealing:** LR decays smoothly as cosine function: LR(t) = LR_min + (LR_max - LR_min) * 0.5 * (1 + cos(π * t / T)). Smooth, no abrupt transitions. Empirically better than step decay.

**Warmup:** Start with low LR, increase linearly to LR_max over first K epochs. Prevents early divergence with large batches. Typical warmup fraction: 5-10% of total training. Critical for fine-tuning pretrained models.

**Warm Restarts (SGDR):** After cosine annealing reaches LR_min, restart (reset LR to LR_max), anneal again with longer period. Multiple descent phases, may escape local minima. Can find better solutions than single phase.

**Cyclical LR (CLR):** Cycle LR between bounds periodically (triangular, exp_range). Reduces need for precise LR tuning. Research shows it helps with learning rate ranges (e.g., LR_min to LR_max over cycle).

## Common gotchas / interview framings
- Warmup too short → early training unstable, divergence risk
- Warmup too long → wastes training on suboptimal LR
- Cosine annealing with wrong T (total epochs) → LR doesn't match actual training length
- Changing batch size mid-training → should adjust LR (linear scaling rule)
- Warm restarts with same period → may not escape minima; increase period each restart (SGDR) helps
- No warmup with large batch size (gradient accumulation) → divergence

## See also
- [[loss-curves-and-convergence-diagnostics]]
- [[gradient-accumulation]]
- [[momentum-and-weight-decay-interaction]]

## Sources
See frontmatter `sources:`.

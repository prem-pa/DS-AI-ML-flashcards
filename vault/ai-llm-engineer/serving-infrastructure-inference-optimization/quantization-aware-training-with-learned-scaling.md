---
id: b2a6732b-d612-4d87-bd24-e6dd09475813
title: Quantization-aware training with learned scaling
track: ai-llm-engineer
topic: serving-infrastructure-inference-optimization
difficulty: 5
tags:
- qat
- learned-scaling
- per-channel-quantization
- scale-learning
- gradient-based-optimization
aliases:
- QAT-scaling
- Learnable-scales
- Per-channel-scaling
- KL-divergence-minimization
sources:
- url: https://vrlatech.com/llm-quantization-explained-int4-int8-fp8-awq-and-gptq-in-2026/
  label: 'VRLA Tech: QAT with Learned Scaling (2026)'
- url: https://jarvislabs.ai/blog/vllm-quantization-complete-guide-benchmarks
  label: 'Jarvis Labs: QAT Implementation Guide'
cards:
- id: ae44abed-85d3-4331-8c59-d35b399cf8ec
  type: flip
  front: Why do learned scales in QAT produce better accuracy than fixed scales?
  back: Fixed scales are computed once via calibration and frozen. Learned scales are parameters that
    update during fine-tuning via backprop, optimizing to minimize task loss + KL divergence. The model
    learns where precision matters most, allocating finer scales to sensitive channels.
- id: 98da62fd-6e65-42ea-a26f-79676b6a0015
  type: mcq
  front: INT8 QAT with fixed scales loses 0.8 MMLU. Learned scales cost 50 GPU-hours to achieve 0.2-point
    loss. Worth it?
  back: ''
  choices:
  - key: a
    text: A) Yes, always use learned scales
    correct: false
  - key: b
    text: B) Only if the model will run for years (amortize compute cost)
    correct: false
  - key: c
    text: C) No, 50 hours is too expensive
    correct: false
  - key: d
    text: D) Depends on accuracy SLA
    correct: true
- id: 890fba83-62fb-4be2-9e42-693e66dbb059
  type: flip
  front: "What does per-channel learned scaling do?\n```python\nscale = nn.Parameter(torch.ones(out_channels))\n\
    for c in range(out_channels):\n  w_q[:, c] = fake_quantize(w[:, c], scale[c])\n```"
  back: Each output channel has its own learnable scale. During fine-tuning, backprop updates each scale
    independently to minimize loss for that channel. Some channels may need finer scales (8-bit), others
    coarser (4-bit), optimizing per-channel precision.
---

## Intuition
Learned scaling makes quantization scales **learnable parameters** during QAT fine-tuning, optimizing where precision matters most. Scales update via backprop alongside weights, achieving 0.5-1 point better accuracy than fixed-scale QAT.

## Detail
### Per-Channel Learned Scaling
```python
# Standard QAT: scales frozen after calibration
scale = compute_scale(activations)  # one-time calibration
x_q = round(x / scale) * scale      # scale is constant during training

# Learned scaling: scales become parameters
scale = nn.Parameter(torch.tensor([...]))  # learnable per channel
x_q = round(x / scale) * scale  # gradient flows through scale!
scale.grad = backprop()  # scales update to minimize loss
```

Each channel learns its optimal scale to minimize task loss + KL divergence penalty.

### Optimization
Minimize: `L = L_task(q_model) + λ * KL(logits_original || logits_q)`

Backprop computes `∂L/∂scale`, updating scales to reduce quantization error.

### Results
| Method | INT8 Loss | INT4 Loss | Time |
|--------|-------|-------|-------|
| Fixed | 0.8 pt | 2.0 pt | 1h |
| Learned (5 epochs) | 0.2 pt | 0.5 pt | 50h |

## Common gotchas
- **"Is learned scaling always better?"** Yes, but requires fine-tuning data + compute. Not worth it for short-lived models.
- **"Can I use learned scales from one model for another?"** No; scales are task-specific. Must re-train per model.

## See also
- [[quantization-aware-training]]
- [[scale-parameters]]
- [[gradient-flow]]
- [[kl-divergence]]
- [[fine-tuning]]

## Sources
See frontmatter `sources:`.

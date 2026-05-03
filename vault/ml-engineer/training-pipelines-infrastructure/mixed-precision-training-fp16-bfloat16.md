---
id: fe68acd8-d56c-464f-97e5-27a2d95842bd
title: Mixed precision training (fp16, bfloat16)
track: ml-engineer
topic: training-pipelines-infrastructure
difficulty: 3
tags:
- mixed-precision
- fp16
- bfloat16
- amp
- loss-scaling
- numerical-stability
aliases:
- FP16 training
- BF16 training
- automatic mixed precision
sources:
- url: https://pytorch.org/docs/stable/amp.html
  label: PyTorch Automatic Mixed Precision
- url: https://arxiv.org/abs/1710.03740
  label: Mixed Precision Training Paper
- url: https://docs.nvidia.com/deeplearning/performance/mixed-precision-training/
  label: NVIDIA Mixed Precision Training Guide
cards:
- id: f825fbbc-1720-49bd-a148-ed391c8f962a
  type: flip
  front: Why does FP16 training require loss scaling?
  back: FP16 has narrow range (~6e-5 to 6e4). Gradients from backprop can be ~1e-5, risking underflow
    to zero. Loss scaling (multiply loss by 2^k) prevents underflow by keeping intermediate values in
    FP16 range.
- id: 87c76a0a-fdd9-4365-a144-3533efba23ad
  type: mcq
  front: What's the key advantage of BF16 over FP16?
  back: ''
  choices:
  - key: a
    text: BF16 is faster
    correct: false
  - key: b
    text: BF16 has same exponent range as FP32, avoiding underflow
    correct: true
  - key: c
    text: BF16 has higher precision
    correct: false
  - key: d
    text: BF16 requires no weights in FP32
    correct: false
- id: 5ade4587-d37a-4853-99d5-4cfdce0a2f7c
  type: flip
  front: In PyTorch's autocast(), which operations are kept in FP32 and why?
  back: Reductions (sum, mean), losses (cross-entropy), softmax. These ops accumulate or have large intermediate
    values; FP16 precision insufficient to avoid numerical errors. Matmul/conv safe in FP16.
- id: 6ee30f65-1ad7-47c6-ad3d-736e560095cf
  type: mcq
  front: When combining mixed precision with gradient accumulation, how should you adjust loss scaling?
  back: ''
  choices:
  - key: a
    text: Use same loss scale as no accumulation
    correct: false
  - key: b
    text: Divide loss scale by accumulation_steps
    correct: true
  - key: c
    text: Multiply loss scale by accumulation_steps
    correct: false
  - key: d
    text: Loss scale independent of accumulation
    correct: false
---

## Intuition
FP32 uses 32 bits (expensive); FP16 uses 16 bits (2× faster, 2× less memory). But FP16 has lower range → gradient underflow. Solution: keep weights in FP32, compute forward/backward in FP16, loss-scale gradients to prevent underflow. Modern BF16 has same range as FP32 but lower precision—fewer underflows.

## Detail
**FP16 (Half Precision):** Exponent: 5 bits, Mantissa: 10 bits. Range: ~6e-5 to 6e4. Gradients in backprop can be ~1e-5, risking underflow. Loss scaling (multiply loss by 2^k, backprop, unscale) prevents this.

**BF16 (Brain Float):** Exponent: 8 bits (same as FP32!), Mantissa: 7 bits. Range: ~1e-38 to 1e38 (like FP32). Precision lower but range safe. No explicit loss scaling needed. Growing standard on NVIDIA (H100, B200) and TPU.

**FP8 (OCP Standard):** Newest: E5M2 (exponent 5, mantissa 2) for weights; E4M3 for activations. Even smaller but requires careful quantization. Used in torchtitan for layer-selective precision.

**Automatic Mixed Precision (AMP):** PyTorch autocast() wraps forward pass, automatically casts ops to FP16/BF16 where safe (matmul, conv), keeps FP32 where needed (reductions, losses). Transparent to user code.

**Loss Scaling & Gradient Accumulation:** Loss scale in AMP must account for gradient accumulation: scale = base_scale / accumulation_steps to avoid overflow.

## Common gotchas / interview framings
- FP16 + no loss scaling → gradients underflow to zero, model stops learning
- Loss scale too high → gradients overflow, NaN
- Mix FP16 in wrong places (reductions, softmax) → numerical instability
- BF16 different from FP16; BF16 needs NO explicit loss scaling (but some frameworks implement anyway)
- H100/B200 have fast TF32 (tensor float 32, precision between FP32 and FP16) → very efficient

## See also
- [[gradient-accumulation]]
- [[loss-curves-and-convergence-diagnostics]]
- [[tensor-parallelism-and-model-sharding]]

## Sources
See frontmatter `sources:`.

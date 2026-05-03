---
id: 139381fb-3092-4376-814f-8143326bdc0e
title: Quantization for storage and inference speedup
track: ml-engineer
topic: model-deployment-serving
difficulty: 3
tags:
- optimization
- compression
- inference
- precision
- model-size
aliases:
- int8-quantization
- model-compression
- precision-reduction
- post-training-quantization
sources:
- url: https://docs.nvidia.com/deeplearning/tensorrt/latest/getting-started/quick-start-guide.html
  label: NVIDIA TensorRT Quick Start
- url: https://pytorch.org/docs/stable/quantization.html
  label: PyTorch Quantization Docs
cards:
- id: 4a06c7da-3529-42dd-927e-38529ef3885f
  type: flip
  front: 'Explain the quantization formula: how does float32 map to int8?'
  back: int8_val = round((float32_val - zero_point) / scale). Scale is derived from the float32 range
    [min, max] and int8 target range [0, 255]. Zero-point ensures 0.0f maps exactly to int8_zero.
- id: 53832e1c-7048-48ed-b7d0-42bf660b7c5e
  type: mcq
  front: You want to quantize a model to int8 with minimal effort and maintain >99.5% accuracy. Which
    approach?
  back: QAT lets weights adapt during training; combined with a diverse calibration set, it preserves
    accuracy. PTQ is faster but may lose accuracy. Dynamic is simpler but only quantizes weights, not
    activations.
  choices:
  - key: a
    text: Post-training quantization (PTQ) with histogram-based calibration
    correct: false
  - key: b
    text: Quantization-aware training (QAT) with a representative calibration dataset
    correct: true
  - key: c
    text: Dynamic quantization (weights only), no calibration
    correct: false
  - key: d
    text: Manual bit-shifting without proper scale/zero-point
    correct: false
- id: 60d26ed3-582b-4d12-b099-eab1fde4e73e
  type: flip
  front: What is the difference between symmetric and asymmetric quantization?
  back: 'Symmetric: [-128, 127] range; zero_point = 0; simpler, used for weights. Asymmetric: [0, 255]
    range; zero_point ≠ 0; captures activation ranges better; used for activations and fully symmetric
    models.'
- id: d93a33f8-ab01-4695-9a2f-3b44fca1269d
  type: flip
  front: A quantized int8 model runs slow on your edge device. What are possible causes and fixes?
  back: 'Causes: hardware lacks int8 support (e.g., older ARM, lack of NEON). Fixes: 1) Fall back to float16
    or bfloat16; 2) Use weight-only dynamic quantization; 3) Replace int8 ops with float32 fallbacks;
    4) Upgrade hardware if possible.'
---

## Intuition
Quantization shrinks model weights from float32 (4 bytes per value) to int8 (1 byte) or bfloat16 (2 bytes). For a 1GB model, int8 quantization cuts it to 250MB—4x smaller, faster to load, cheaper to store, and often faster to compute if hardware has int8 kernels.

## Detail
Quantization maps float32 range [min, max] to int8 range [0, 255] using a scale and zero-point: `int8_val = round((float32_val - zero_point) / scale)`. Strategies:

- **Post-training quantization (PTQ)**: Quantize a trained float32 model without retraining; simple but may lose accuracy
- **Quantization-aware training (QAT)**: Simulate quantization during training so weights learn to be quantization-friendly; better accuracy, more effort
- **Symmetric vs. asymmetric**: Symmetric uses [-128, 127]; asymmetric uses [0, 255] and allows zero-point offset
- **Per-channel vs. per-layer**: Per-channel scales per output channel (better accuracy); per-layer uses one scale for entire layer (simpler, faster)

Hardware support: x86 has AVX512, ARM has NEON, NVIDIA GPUs support INT8 Tensor Cores (Turing+). Inference engines (TensorRT, ONNX Runtime, TFLite) auto-fuse ops and dispatch to int8 kernels.

## Common gotchas / interview framings
- "Accuracy dropped 2% after quantization." → Use QAT or per-channel quantization; or increase batch size during calibration to capture activation ranges
- "Quantized model is slow on my phone." → Hardware lacks int8 support; fallback to int16 or bfloat16, or use dynamic quantization (weight-only)
- "Why not quantize everything to int4?" → int4 is harder to implement; scales accumulate error; most frameworks default to int8 or int16

## See also
- [[model-checkpoints-and-state-dicts]]
- [[onnx-format-and-cross-framework-compatibility]]
- [[cold-start-and-model-loading-time]]

## Sources
See frontmatter `sources:`.

---
id: f3aba840-743b-4e14-8821-3cd8321fe05d
title: KV cache quantization and compression
track: ai-llm-engineer
topic: transformer-foundations-deep-dive
difficulty: 5
tags:
- quantization
- kv-cache
- compression
- int4
- int8
- inference-optimization
aliases:
- KV cache quantization
- INT4 cache
- INT8 KV
sources:
- url: https://arxiv.org/abs/2406.02918
  label: Quantized Transformer Inference with Optimized Kernels
- url: https://www.deepfa.ir/en/blog/kv-cache-optimization/
  label: 'DeepFA: KV Cache Optimization'
cards:
- id: 7439ded9-e458-4a5d-8d6f-73e44771d477
  type: flip
  front: Explain how INT8 quantization reduces KV cache size and the typical accuracy loss.
  back: 'INT8 stores each K, V value in 1 byte (range [-128, 127]) instead of 4 bytes (FP32), reducing
    cache size by 4x. A linear scale maps the floating-point range [min, max] to [-128, 127]: quant =
    round((value - min)/(max - min) * 255). Dequantization recovers approximately the original value.
    Typical accuracy loss with per-token or per-channel scales: <1%, acceptable for most inference workloads.'
- id: f68e84c6-3f2f-4a06-8d74-95e5bd4dc879
  type: flip
  front: What is the tradeoff between INT8 and INT4 KV cache quantization?
  back: INT8 (1 byte) achieves 4x compression with <1% accuracy loss and simple, fast dequantization.
    INT4 (0.5 bytes) achieves 8x compression but requires per-channel quantization, complex scale recovery,
    and typically 2-5% accuracy loss. INT4 is more memory-efficient but harder to implement efficiently;
    INT8 is the practical sweet spot for production LLM serving.
- id: ddadc0e9-2c62-4661-b328-501e1c4a5a38
  type: mcq
  front: During long-sequence generation with INT8 KV cache, why does accuracy loss remain small despite
    repeated quantization/dequantization?
  back: 'Correct: (a). Attention weights are computed via softmax(QK^T), which is scale-invariant to small
    perturbations in QK^T. Small INT8 quantization noise on individual K, V values averages out and doesn''t
    significantly affect the final attention distribution. This robustness allows INT8 to work well even
    for long sequences. (b) is false—cached KV is used. (c) and (d) are incorrect.'
  choices:
  - key: a
    text: INT8 quantization errors cancel out due to softmax normalization
    correct: true
  - key: b
    text: K, V are recomputed from scratch at each attention step
    correct: false
  - key: c
    text: Modern GPUs have built-in INT8 error correction
    correct: false
  - key: d
    text: Quantization error accumulation doesn't affect the final output
    correct: false
- id: 97dfa1e6-8a3d-44fb-87d8-8cea68ce14d1
  type: flip
  front: In a production LLM serving system, why might you quantize K, V to INT8 but keep Q in FP32?
  back: Q (queries) are computed fresh for each token at inference, so there's no compression benefit
    from quantizing Q. K, V are cached and their size dominates memory. Quantizing K, V to INT8 cuts cache
    size by 4x with minimal loss. Keeping Q in FP32 ensures high-precision dot products QK^T. This selective
    quantization balances memory savings (via INT8 K, V) with computation quality (via FP32 Q).
---

## Intuition
KV cache dominates memory during inference for long sequences. Storing K and V in lower precision (INT8 or INT4 instead of FP32) reduces memory by 4-8x with small accuracy loss. Quantization maps floating-point tensors to discrete integer ranges using learned or fixed scales. INT8 is widely practical; INT4 requires careful implementation.

## Detail
Standard KV cache stores K, V in FP32 (4 bytes/value) or FP16 (2 bytes/value). Quantized KV uses:
- **INT8:** 1 byte/value, mapping [-128, 127] to the range [min, max] of K/V via linear scaling: quant_value = round((value - min) / (max - min) * 255). Dequantized during attention: value = quant_value * (max - min) / 255 + min. Typical accuracy loss: <1% for well-tuned scales.
- **INT4:** 0.5 bytes/value. More aggressive; requires per-channel quantization and careful scale tuning. Accuracy loss: 2-5%.

Quantization-aware training (QAT) or post-training quantization (PTQ) can improve results. Per-token or per-channel scales are learned. Symmetric (range [-b, b]) vs. asymmetric (range [a, b]) quantization trade off simplicity and accuracy.

## Common gotchas / interview framings
- **Asymmetry:** K, V have different distributions; per-head quantization may be needed vs. global.
- **Dequantization cost:** INT8 dequantization adds latency; often fused into attention kernels to hide cost.
- **Accumulation error:** As sequences grow long, dequantization and recomputation errors accumulate; empirically <1% quality loss for 32K tokens with INT8.
- **Training-time loss:** Some models are trained with quantization-aware techniques for better INT8 accuracy.
- **Full precision optional:** Can keep first/last layers in FP32, middle in INT8 for best quality/speed tradeoff.

## See also
- [[kv-cache-in-autoregressive-generation]]
- [[grouped-query-attention-gqa-and-multi-query-attention-mqa]]

## Sources
See frontmatter `sources:`.

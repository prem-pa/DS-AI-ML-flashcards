---
id: 5493ee93-3f03-4f35-b196-e8766a07a04c
title: Efficient attention (FlashAttention, FlashAttention-2, FlashAttention-3)
track: ai-llm-engineer
topic: transformer-foundations-deep-dive
difficulty: 5
tags:
- attention-optimization
- gpu-efficiency
- io-aware
- flashattention
- hardware-efficiency
aliases:
- FlashAttention
- Flash-3
- IO-optimal attention
sources:
- url: https://arxiv.org/abs/2407.08608
  label: 'FlashAttention-3: Fast and Accurate Attention with Asynchrony and Low-precision'
- url: https://tridao.me/blog/2024/flash3/
  label: 'Tri Dao Blog: FlashAttention-3'
- url: https://pytorch.org/blog/flashattention-3/
  label: 'PyTorch Blog: FlashAttention-3'
cards:
- id: a802c0f7-225b-4f07-bd28-ef46de86ccc6
  type: flip
  front: Why is standard scaled dot-product attention IO-bottlenecked, and how does FlashAttention address
    this?
  back: 'Standard attention materializes the full n×n attention matrix in slow main memory, requiring
    O(n^2) read/write operations. FlashAttention uses tiling: partition Q, K, V into blocks, load each
    block into fast SRAM, compute local attention with online softmax (tracking max/sum), and write outputs.
    This reduces HBM accesses from O(n^2) to O(n).'
- id: cb4725e8-dbe5-40ac-8864-e1710886266d
  type: flip
  front: What are the key improvements FlashAttention-3 introduces over FlashAttention-2?
  back: 'FlashAttention-3 (2024) optimizes for Hopper GPUs via (1) warp specialization: overlapping block-wise
    matmul and softmax computation via specialized warp groups, (2) TMA (Tensor Memory Accelerator): hardware-accelerated
    data movement, (3) FP8 support: reduced memory bandwidth with 2.6x lower numerical error than baseline
    FP8. Result: 1.5-2x faster than FlashAttention-2 on H100 (740 TFLOPs/s FP16, 1.2 PFLOPs/s FP8).'
- id: cfd6d915-80fc-4aad-a893-dbadb6abcd1f
  type: mcq
  front: Which is the primary bottleneck that FlashAttention addresses in transformer inference?
  back: 'Correct: (b). Standard attention computes softmax(QK^T/sqrt(d))V but must materialize and write
    the n×n matrix to memory, creating O(n^2) memory traffic. Modern GPUs are compute-bound on matmul
    but memory-bound on attention. FlashAttention solves this via tiling to minimize HBM traffic. (a)
    is not bottlenecked. (c) is handled but not the main gain. (d) is secondary.'
  choices:
  - key: a
    text: Matrix multiply (GEMM) computation throughput
    correct: false
  - key: b
    text: Main memory (HBM) bandwidth for reading/writing intermediate matrices
    correct: true
  - key: c
    text: Softmax numerical stability
    correct: false
  - key: d
    text: Gradient computation in backpropagation
    correct: false
- id: 4d0d9432-6cc2-41e6-81ae-1cb584ed3aaf
  type: flip
  front: How does FlashAttention compute softmax online without materializing the full attention matrix?
  back: 'FlashAttention maintains running statistics: as it processes blocks of attention scores, it tracks
    the running maximum and sum of exp(scores - max). After processing each block, it updates the maximum
    and rescales previous exp values. This allows stable, incremental softmax computation matching the
    full-matrix approach without storing the entire n×n matrix.'
---

## Intuition
Standard attention's bottleneck is not computation (FLOPS) but data movement between slow main memory and fast GPU cache. FlashAttention redesigns the algorithm to minimize IO by tiling, reducing memory access by 10x. FlashAttention-2 extends this to Ampere GPUs. FlashAttention-3 exploits Hopper's new asynchronous features and FP8 to achieve 2-3x speedup on H100 GPUs.

## Detail
Standard attention materializes the full n×n attention matrix, requiring O(n^2) memory and communication. FlashAttention uses a tiling strategy:
1. Load blocks of Q, K, V from HBM (high-bandwidth memory) into SRAM (fast cache)
2. Compute attention for that block locally, tracking running max/sum for softmax
3. Write outputs back to HBM
This reduces HBM accesses from O(n^2) to O(n), achieving ~15 TFLOPs on A100 vs. ~6 TFLOPs for standard attention. FlashAttention-3 adds:
- **Warp specialization:** Overlaps block-wise matmul and softmax via specialized warp groups
- **TMA (Tensor Memory Accelerator):** Hardware-accelerated data movement
- **FP8 support:** Reduces memory bandwidth with minimal numerical error (2.6x lower error than baseline FP8)
FlashAttention-3 achieves 740 TFLOPs/s (75% H100 utilization) with FP16 and near 1.2 PFLOPs/s with FP8.

## Common gotchas / interview framings
- **Backward pass:** FlashAttention's backward is also IO-optimal, using checkpointing to avoid storing large intermediate matrices.
- **Dropout and bias:** FlashAttention handles causal masking and dropout correctly; biases added per layer.
- **Sequence length:** Speedups are more dramatic for longer sequences; short sequences (<256 tokens) see smaller gains.
- **GPU-specific:** FlashAttention-3 requires H100/H800; FlashAttention-2 is optimized for A100. Standard attention will be faster on CPU.
- **Numerical stability:** FlashAttention computes softmax numerically stably via online statistics (max/sum tracking) despite not materializing full matrix.

## See also
- [[scaled-dot-product-attention-derivation]]
- [[paged-attention-vllms-pagedattention]]
- [[kv-cache-in-autoregressive-generation]]

## Sources
See frontmatter `sources:`.

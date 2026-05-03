---
id: e94b4533-eeb5-4e55-96ac-20e41cc8bb58
title: KV cache in autoregressive generation
track: ai-llm-engineer
topic: transformer-foundations-deep-dive
difficulty: 3
tags:
- kv-cache
- autoregressive
- inference
- memory-efficiency
- generation
aliases:
- KV cache
- key-value cache
- prompt cache
sources:
- url: https://arxiv.org/abs/1706.03762
  label: Attention is All You Need
- url: https://www.deepfa.ir/en/blog/kv-cache-optimization/
  label: 'DeepFA: KV Cache Optimization'
cards:
- id: 9484bf15-19bc-4afc-9c15-7f875233720c
  type: flip
  front: Explain why KV cache is used in autoregressive generation and what computation it avoids.
  back: In autoregressive generation, each token is computed with attention over all previous tokens plus
    itself. Recomputing K and V for all past tokens at every step is O(n^2) wasteful. KV cache stores
    K and V from all previous steps in GPU memory. At step i, only the current token's K and V are computed;
    past K, V are retrieved from cache. This reduces per-step computation from O(n) to O(1) (just the
    current token), amortizing full-sequence computation once at the start.
- id: c5c992d4-0ed0-414a-9f42-bef03ec57448
  type: flip
  front: What is the memory cost of KV cache and how does it depend on sequence length and batch size?
  back: 'KV cache size is proportional to: 2 * batch_size * seq_len * d_model (in bytes, multiplied by
    precision: 4 for FP32, 2 for FP16, 1 for INT8). For batch_size=32, seq_len=4K, d_model=4096 with FP16:
    2 * 32 * 4096 * 4096 * 2 bytes ≈ 4 GB. This dominates model weights for long sequences. With [[grouped-query-attention-gqa-and-multi-query-attention-mqa]],
    cache is reduced by the ratio of KV heads to query heads.'
- id: 270341ef-977e-4df7-a85f-7bb1be9ef930
  type: mcq
  front: In autoregressive generation with KV cache, what is the complexity of computing each new token?
  back: 'Correct: (b). With KV cache, computing a new token requires (1) computing Q for the new token
    (O(1) mini attention ops), (2) retrieving stored K, V for all past tokens from cache (O(seq_len) memory
    access), (3) attention over current Q and cached K, V (O(seq_len) dot products and softmax). (a) is
    without cache. (c) is incorrect. (d) is not how KV cache works.'
  choices:
  - key: a
    text: O(seq_len^2) due to re-computing all attention scores
    correct: false
  - key: b
    text: O(seq_len) because we retrieve and update the KV cache
    correct: true
  - key: c
    text: O(1) constant time, independent of sequence length
    correct: false
  - key: d
    text: O(log seq_len) due to efficient cache indexing
    correct: false
- id: e3e02660-c9f3-4dc1-b55c-8bfeedfe5244
  type: flip
  front: How does KV cache quantization (e.g., INT8 instead of FP32) reduce memory usage, and what is
    the tradeoff?
  back: 'KV cache quantization stores K and V in lower precision (INT8 = 1 byte/value vs FP32 = 4 bytes/value),
    reducing cache size by 4x. Tradeoff: quantization error accumulates as tokens are generated. Careful
    quantization (symmetric, per-channel scales) maintains accuracy to ~1% quality loss. For long sequences
    (10K+ tokens), quantization savings (4-8x) often outweigh precision loss.'
---

## Intuition
During autoregressive generation, at each step the model computes attention over the full sequence (previous tokens + current token). Naively recomputing K, V for all past tokens is wasteful. KV cache stores K and V from previous steps, allowing attention computation over cached KV values, reducing redundant computation from O(n^2) to O(n) per step.

## Detail
At step i (generating token i), attention requires:
$$\text{Attn}(q_i, k_{1:i}, v_{1:i})$$

Naively, we recompute k_1, ..., k_i and v_1, ..., v_i every step. Instead, maintain KV cache: after computing step i-1, store cached_k = k_{1:i-1}, cached_v = v_{1:i-1}. At step i, compute only k_i, v_i, then concatenate: k = [cached_k, k_i]. The attention computation is now O(n) instead of O(n^2) per step. Downside: cache consumes GPU memory. For long generations, cache can be the dominant memory consumer. Typical growth: ~100 MB per billion tokens of context.

## Common gotchas / interview framings
- **Batch size interaction:** KV cache size scales with batch size. Serving many sequences in parallel multiplies cache memory.
- **Precision matters:** With FP32, cache is 8 bytes per value. INT8 quantization reduces to 1 byte, 8x saving. INT4 is possible with loss.
- **First token latency:** The first token (prompt processing) doesn't use cache, so it's slower than subsequent tokens.
- **Memory-bandwidth limited:** Accessing KV cache is memory-bound, not compute-bound. Optimization focuses on bandwidth (sparse patterns, quantization, paging).
- **Cold vs. warm cache:** Empty cache on sequence start; fills up as generation proceeds, reducing compute efficiency slightly.

## See also
- [[grouped-query-attention-gqa-and-multi-query-attention-mqa]]
- [[paged-attention-vllms-pagedattention]]
- [[kv-cache-quantization-and-compression]]

## Sources
See frontmatter `sources:`.

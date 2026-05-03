---
id: 3b8c44e8-851d-483a-9127-19e4c6c7a658
title: Grouped-query attention (GQA) and multi-query attention (MQA)
track: ai-llm-engineer
topic: transformer-foundations-deep-dive
difficulty: 5
tags:
- kv-cache
- inference-optimization
- memory-efficiency
- gqa
- mqa
- llm-serving
aliases:
- GQA
- MQA
- grouped-query
- multi-query
sources:
- url: https://arxiv.org/pdf/2305.13245
  label: 'GQA: Training Generalized Multi-Query Transformer Models'
- url: https://www.ibm.com/think/topics/grouped-query-attention
  label: 'IBM: What is Grouped Query Attention'
- url: https://friendli.ai/blog/gqa-vs-mha
  label: 'Friendli AI: GQA vs MHA Comparison'
cards:
- id: 454b140a-8f3c-4022-9545-b3509a149784
  type: flip
  front: How do GQA and MQA reduce KV cache size during inference, and what is the accuracy tradeoff?
  back: GQA/MQA share K, V heads across multiple query heads, reducing the KV cache from 2*seq_len*d to
    2*seq_len*(d/k) where k is the number of KV head groups. MQA (single shared KV head) achieves ~90%
    cache reduction but may lose 2-5% quality. GQA with g groups achieves 30-90% reduction with <1% quality
    loss by balancing compression and expressiveness.
- id: 926faf8c-ef19-4f22-8e92-26de308c37be
  type: flip
  front: In GQA, how many key-value heads does each query head attend to?
  back: 'In standard GQA with g groups and h query heads, each query head in group i attends to the i-th
    KV head. The ratio h/g determines sharing: h/g query heads share each KV head. For example, with h=32
    query heads and g=8 KV head groups, 4 query heads share each KV head.'
- id: 4f392508-b414-4020-884b-b1ad60da2f3a
  type: mcq
  front: Which of the following accurately describes the memory savings of GQA during autoregressive inference?
  back: 'Correct: (b). GQA''s main benefit is reducing KV cache footprint by sharing K, V projections.
    Attention computation (QK^T, softmax) still scales similarly to MHA. The savings become critical during
    long-sequence inference where KV cache dominates memory. (a) is incorrect because computation isn''t
    proportionally reduced. (c) is false—cache still exists, just smaller. (d) is false—GQA helps at any
    length.'
  choices:
  - key: a
    text: GQA reduces both attention computation and KV cache size proportionally
    correct: false
  - key: b
    text: GQA primarily saves KV cache memory by sharing K,V across multiple query heads; attention computation
      is similar to MHA
    correct: true
  - key: c
    text: GQA eliminates the need for KV cache entirely
    correct: false
  - key: d
    text: GQA is only effective for very long sequences (>1M tokens)
    correct: false
- id: 8f0eb996-66ae-4dad-a173-8e98a64db46c
  type: flip
  front: How can you convert a trained multi-head attention model to use GQA without retraining from scratch?
  back: Average (or pool) the K and V projections from the original h heads down to g groups. For example,
    with h=32 heads and target g=8 groups, average projections {0-3}, {4-7}, etc. into 8 groups. Initialize
    the GQA model with these averaged projections and fine-tune briefly. This often retains 95%+ of the
    original model's quality.
---

## Intuition
Multi-head attention (MHA) stores separate K and V for each of h query heads, consuming h times more cache. Multi-query attention (MQA) shares a single K, V across all heads. Grouped-query attention (GQA) is a middle ground: k < h key-value head groups share their K, V across multiple query heads, reducing cache size while retaining more quality than MQA.

## Detail
In standard MHA with h heads and dimension d/h per head, the KV cache size is proportional to 2 * seq_len * h * (d/h) = 2 * seq_len * d per layer. In MQA, only 1 KV head exists, so cache is 2 * seq_len * (d/h). In GQA with g groups (g < h), there are g KV heads shared among h query heads, cache is 2 * seq_len * g * (d/h).

The attention computation:
$$\text{GQA}(Q, K, V) = \text{Concat}(\text{head}_1, \ldots, \text{head}_h)W^O$$
where each of the h query heads attends to the same (or nearby) KV head. By training from scratch with GQA, models maintain near-MHA quality with 30-40% faster inference and 90% smaller KV cache vs. MHA.

## Common gotchas / interview framings
- **Training vs. inference:** GQA saves memory during inference (KV cache). Training memory cost is similar to MHA. MQA sometimes hurts training—quality drops more than GQA.
- **GQA adoption:** Llama 2, Llama 3, Qwen2 all use GQA; it is now standard in modern LLMs.
- **Conversion:** You can initialize GQA from a trained MHA model by averaging K, V projections—works reasonably well.
- **Extreme case:** GQA with g=1 is MQA; with g=h is equivalent to MHA. Sweet spot is often g=h/8 to h/4.
- **Not free:** Extreme compression (MQA) loses quality; GQA finds the Pareto frontier.

## See also
- [[multi-head-attention-and-head-roles]]
- [[kv-cache-in-autoregressive-generation]]
- [[paged-attention-vllms-pagedattention]]

## Sources
See frontmatter `sources:`.

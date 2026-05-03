---
id: fd3c0461-ef0f-49c2-84f7-be508d291b01
title: KV cache eviction policies and memory limits
track: ai-llm-engineer
topic: transformer-foundations-deep-dive
difficulty: 5
tags:
- memory-management
- eviction
- kv-cache
- cache-policies
- serving
aliases:
- cache-eviction
- LRU cache
- token-dropping
sources:
- url: https://arxiv.org/abs/2212.09539
  label: 'Breakdancer: Break-in Attention Mechanism for LLM Serving'
- url: https://vllm.ai/
  label: 'vLLM: LLM Serving Engine'
cards:
- id: e2fe1810-e510-4b97-bb5f-7749864effc7
  type: flip
  front: What are three common KV cache eviction policies, and how does each decide which tokens to drop?
  back: '1. **FIFO:** Drop oldest tokens first (simple, fair). 2. **LRU:** Drop tokens least recently
    attended to (good for cache locality). 3. **Token Importance:** Compute attention weights; drop tokens
    with lowest cumulative attention across layers (preserves model-important tokens). Each has tradeoffs:
    FIFO is simplest; importance-based is most accurate but requires attention computation.'
- id: 8de64444-074e-430a-b32d-2dca989a430b
  type: flip
  front: Why might an importance-based KV cache eviction policy outperform FIFO in accuracy?
  back: Token importance reflects what the model actually attends to. Dropping tokens with low cumulative
    attention keeps information the model relies on. FIFO drops oldest tokens regardless of their importance—if
    an earlier context token is important to later predictions, dropping it hurts accuracy. Importance-based
    policies preserve crucial context while safely dropping less-attended padding or repetitive tokens.
- id: ba7e280a-5fb4-4790-b97f-fe21c819dfea
  type: mcq
  front: In a memory-constrained serving system, when should KV cache eviction be triggered?
  back: 'Correct: (b). Evict-on-demand balances memory and quality: keep all KV as long as possible (good
    for accuracy), evict only when necessary (needed for throughput). Aggressive eviction (a) wastes memory
    by not using available space. Virtual memory paging (c) is slower than eviction. (d) doesn''t make
    sense—cache hasn''t been populated yet.'
  choices:
  - key: a
    text: Immediately after each token is generated, to minimize memory usage
    correct: false
  - key: b
    text: Only when cache memory reaches capacity and new tokens cannot be allocated
    correct: true
  - key: c
    text: Never; use virtual memory (paging to disk) instead
    correct: false
  - key: d
    text: At the beginning of each sequence, before generation starts
    correct: false
- id: 4a2481bf-65f9-439a-91ff-314cbbfd2405
  type: flip
  front: How do models trained with sparse (local) attention patterns like [[sparse-attention-patterns-local-strided-longformer-pattern]]
    interact with tail-token KV eviction?
  back: Models trained with local (sliding-window) attention naturally don't attend to tail tokens beyond
    window size. Evicting tail tokens has minimal impact on output quality because the model has learned
    not to rely on them. Conversely, models trained on full attention are hurt more by tail eviction.
    This suggests co-designing eviction policies with model architecture—local-attention models can safely
    use aggressive tail eviction.
---

## Intuition
With limited GPU memory, KV cache for all active sequences may exceed available space. When this happens, some cached tokens must be evicted (dropped). Different eviction policies (FIFO, LRU, LFU, token-importance) have different accuracy/throughput tradeoffs. Some policies drop older tokens (less critical), others drop less-attended tokens (lightly-used). The goal: maximize throughput while minimizing quality loss.

## Detail
Common eviction policies:
1. **FIFO (First-in, First-out):** Drop oldest tokens. Simple, fair, but doesn't account for importance.
2. **LRU (Least-Recently Used):** Drop tokens with least recent attention access. Good for cache locality.
3. **LFU (Least-Frequently Used):** Drop tokens attended to least. Preserves frequently-attended context.
4. **Token importance:** Compute attention weights; drop tokens with lowest attention sum across all layers. Preserves tokens model attends to most.

When cache memory is full and a new token's KV arrives, the least important tokens are evicted. The impact on output quality depends on policy: dropping tail (older) tokens may hurt next-token prediction minimally (recent context matters); dropping head (important context-providing) tokens hurts more.

## Common gotchas / interview framings
- **Accuracy vs. throughput:** Keeping all KV is best for accuracy; aggressive eviction improves throughput but hurts quality.
- **Sliding window approximation:** Some models (e.g., [[sparse-attention-patterns-local-strided-longformer-pattern]]) are trained with local attention, naturally tolerating tail-token eviction.
- **Recomputation trade-off:** Evicting KV reduces memory but may require recomputation if the token is needed again. Pure eviction (no recomputation) is simplest.
- **Batch-aware scheduling:** Different sequences have different lengths; scheduling decisions (which to evict) depend on batch composition.
- **No perfect solution:** Token importance is model- and task-specific; a general policy must be somewhat suboptimal.

## See also
- [[kv-cache-in-autoregressive-generation]]
- [[kv-cache-quantization-and-compression]]
- [[paged-attention-vllms-pagedattention]]

## Sources
See frontmatter `sources:`.

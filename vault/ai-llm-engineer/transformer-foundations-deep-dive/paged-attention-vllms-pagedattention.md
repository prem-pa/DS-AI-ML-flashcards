---
id: 7864291d-a975-4f95-8ef8-4cd1e84482b2
title: Paged attention (vLLM's PagedAttention)
track: ai-llm-engineer
topic: transformer-foundations-deep-dive
difficulty: 5
tags:
- paged-attention
- vllm
- memory-management
- kv-cache
- gpu-memory
aliases:
- PagedAttention
- vLLM paging
sources:
- url: https://arxiv.org/abs/2309.06180
  label: 'vLLM: Easy, Fast, and Cheap LLM Serving with PagedAttention'
- url: https://blog.vllm.ai/
  label: vLLM Blog
cards:
- id: 7fd23ece-82ab-4fc8-9c35-e163744d7c2f
  type: flip
  front: How does PagedAttention reduce memory fragmentation compared to contiguous KV cache allocation?
  back: Contiguous KV cache pre-allocates a fixed memory block per sequence, even if only partially used
    (fragmentation). PagedAttention allocates KV cache in fixed-size pages (e.g., 16 tokens/page), allocated
    on-demand. Pages can be scattered in memory. A 100-token sequence uses 7 pages; a 101-token sequence
    uses 7 pages plus 1 token in page 8. Pages can be reused across sequences. This avoids large pre-allocation
    and reduces wasted memory.
- id: 2352b056-7ae1-40b3-8ba2-50c25dd86dcd
  type: flip
  front: Explain how PagedAttention enables prompt-cache sharing across multiple requests.
  back: If 100 requests all start with the same 1K-token prompt, vLLM can compute KV pages for the prompt
    once and share them via logical page pointers. Each request maintains its own token sequence starting
    from the shared prompt. Requests that diverge after the prompt allocate new pages only for divergent
    tokens. This sharing dramatically reduces memory usage for multi-request scenarios and enables high
    throughput.
- id: e5680578-d1ec-4edf-88fd-8b12a1c713cc
  type: mcq
  front: What is the primary benefit of PagedAttention over fixed contiguous KV cache allocation?
  back: 'Correct: (b). PagedAttention allocates pages on-demand, avoiding wasted memory from over-allocation.
    Pages can be shared via pointers (e.g., shared prompt prefix), enabling 10-100x throughput improvements
    in batching scenarios. (a) is secondary—FlashAttention handles speed. (c) is not PagedAttention''s
    purpose. (d) is impossible; cache still exists.'
  choices:
  - key: a
    text: Faster attention computation through block-wise operations
    correct: false
  - key: b
    text: On-demand allocation reduces fragmentation and enables efficient sharing of cached KV across
      requests
    correct: true
  - key: c
    text: Automatic compression of KV tensors during storage
    correct: false
  - key: d
    text: Elimination of the need for GPU memory entirely
    correct: false
- id: 5d8e9c65-7ebc-4ea5-8e43-e4191f534796
  type: flip
  front: In PagedAttention, if the block size is 16 tokens and a sequence is 100 tokens long, how many
    pages are allocated?
  back: Ceiling(100 / 16) = 7 pages are allocated. The first 6 pages hold 16 tokens each (96 total); page
    7 holds the remaining 4 tokens. This is more efficient than contiguous allocation, which would reserve
    space for a multiple of 16 (e.g., 112 tokens), wasting 12 tokens of memory.
---

## Intuition
Traditional KV cache allocates contiguous GPU memory per sequence. If a sequence is short, allocated memory is wasted; if long, allocation fails (fragmentation). PagedAttention treats KV cache like virtual memory: break cache into fixed-size pages, allocate on-demand, pages can be scattered in memory. This reduces fragmentation, allows longer sequences, and enables memory sharing across sequences with the same prompt prefix (prompt-cache sharing).

## Detail
PagedAttention partitions KV cache into logical blocks (pages) of size B (e.g., B=16 tokens). Each sequence maintains a mapping from logical block index to physical GPU memory pages. At attention computation, the KV pages are gathered on-the-fly, and attention is computed over the blocks. Pages can be non-contiguous. This enables:
1. **Dynamic memory allocation:** Allocate pages as needed, not upfront.
2. **Efficient sharing:** Multiple sequences with the same prefix can share KV pages (e.g., multiple generations from the same prompt).
3. **Reduced fragmentation:** Variable-length sequences don't waste reserved memory.
VLLM's implementation achieves 24x higher throughput than HuggingFace on A100 by combining PagedAttention, batching, and FlashAttention.

## Common gotchas / interview framings
- **Block size tradeoff:** Larger blocks reduce page lookup overhead but waste space if sequence length is not a multiple of block size. Often 16-256 tokens per block.
- **Prefix sharing:** If 100 requests share the same 1K-token prompt, only 1K tokens of KV cache are stored once, shared via page pointers.
- **Scheduling complexity:** Batching different-length sequences and managing page allocation requires careful scheduling (FCFS, SJF, best-fit).
- **Not just memory:** PagedAttention also interacts with FlashAttention; vLLM fuses both for maximum efficiency.
- **Hardware support:** PagedAttention is implemented in CUDA; different GPUs may have performance variations.

## See also
- [[kv-cache-in-autoregressive-generation]]
- [[kv-cache-eviction-policies-and-memory-limits]]
- [[efficient-attention-flashattention-flashattention-2-flashattention-3]]

## Sources
See frontmatter `sources:`.

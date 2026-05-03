---
id: c8387a65-7d9b-452a-b7fb-717e0747dbc4
title: Ring attention and paged attention for long context
track: ai-llm-engineer
topic: transformer-foundations-deep-dive
difficulty: 5
tags:
- ring-attention
- distributed-attention
- long-context
- memory-efficient
- paged-attention
aliases:
- ring-attention
- distributed-attention
- long-context-scaling
sources:
- url: https://arxiv.org/abs/2310.01889
  label: Ring Attention with Blockwise Transformers for Context Scaling
- url: https://arxiv.org/abs/2309.06180
  label: 'vLLM: PagedAttention for Efficient LLM Serving'
cards:
- id: 5edd1fc9-c198-4723-8c61-052cbe903402
  type: flip
  front: How does ring attention distribute long-sequence KV cache across multiple GPUs, and what is its
    memory complexity?
  back: 'Ring attention partitions KV into N blocks, assigning each block to one of N GPUs. During attention,
    each GPU computes Q@K_local, then rotates KV in a ring: GPU i sends K_i to GPU i+1, receives K_{i-1}.
    After N iterations, all GPUs compute full attention. Memory per GPU is O(n/N), enabling linear scaling.
    Trade-off: requires N rounds of all-to-all communication and synchronization.'
- id: 73c741c4-22e4-43d2-addb-0e57fa61e4b3
  type: flip
  front: How does paged attention reduce memory fragmentation for variable-length sequences in a KV cache?
  back: 'Paged attention allocates KV in fixed-size pages (e.g., 16 tokens/page) on-demand, not upfront
    as contiguous blocks. A 100-token sequence uses 7 pages; a 101-token sequence uses 7 + 1 token in
    page 8. Pages can be scattered in GPU memory. This avoids over-allocating contiguous space and allows
    pages to be shared across sequences (e.g., shared prompt prefix). Result: higher GPU memory utilization
    and support for longer sequences.'
- id: 89947bd2-0c86-41cc-b290-815e54a6bcde
  type: mcq
  front: Ring attention is advantageous for which inference scenario?
  back: 'Correct: (b). Ring attention requires inter-GPU synchronization and all-to-all communication,
    adding latency per token. For single requests, this overhead is unjustified. For batches of long sequences,
    latency is amortized: high throughput (tokens/second) masks per-token latency. (a) is a poor fit due
    to synchronization cost. (c) and (d) are unrelated.'
  choices:
  - key: a
    text: Single-request, low-latency inference requiring fast per-token generation
    correct: false
  - key: b
    text: High-throughput batching with long sequences, where latency is amortized across requests
    correct: true
  - key: c
    text: CPU-only inference without GPU acceleration
    correct: false
  - key: d
    text: Reducing model parameters through pruning
    correct: false
- id: abb2fc74-46d6-47df-9b93-216f5fe24ae2
  type: flip
  front: If you're serving a 32K-token sequence on an 80GB A100 GPU, would ring attention or paged attention
    be more suitable and why?
  back: Paged attention is more suitable for a single GPU because it reduces memory fragmentation and
    enables efficient page allocation within available GPU memory. If 32K tokens fit in 80GB, paged attention
    is sufficient and avoids inter-GPU communication overhead. Ring attention is better for even longer
    sequences (>200K tokens) distributed across multiple GPUs where single-GPU memory is insufficient.
    For 32K on one GPU, paged attention is optimal.
---

## Intuition
KV cache for long sequences exceeds GPU memory. Ring attention and paged attention enable inference on much longer contexts by distributing or paging KV across multiple GPUs/memory regions. Ring attention cycles KV blocks through GPUs in a ring topology, requiring only O(1) GPU memory per GPU. Paged attention (vLLM) pages KV in GPU memory like virtual memory. Together, they enable serving 10-100x longer contexts than naive approaches.

## Detail
**Ring Attention:** Partition KV into blocks, distribute across N GPUs (or memory pages). Each GPU holds one KV block. During attention:
1. Compute Q@K_local (attention within local block)
2. All-to-all rotate: send KV to next GPU, receive from previous
3. Repeat N times to compute attention over all KV blocks
Totally, O(n^2) computation is preserved (full attention), but memory per GPU is O(n/N), enabling O(N) scaling. Requires inter-GPU communication (all-reduce).

**Paged Attention:** Within a single GPU, partition KV cache into logical pages (e.g., 16-token blocks). Allocate pages to physical memory on-demand. Pages for different sequences can be scattered. Gather pages during attention computation. Reduces fragmentation and enables sequence length scaling up to GPU memory size.

VLLM combines both: use paged attention within GPU, ring attention across GPUs for extreme scale.

## Common gotchas / interview framings
- **Communication overhead:** Ring attention requires all-to-all communication; for many GPUs, communication time may dominate computation.
- **Paging granularity:** Block size trades off page lookup overhead vs. memory fragmentation; typically 8-64 tokens.
- **Not for inference latency:** Ring attention's all-to-all requires synchronization; good for throughput (batching), bad for per-token latency.
- **Combining strategies:** Ring + Paged + FlashAttention + GQA together enable 10-100x throughput improvements on long contexts.
- **Heterogeneous hardware:** Ring attention assumes homogeneous GPUs; bandwidth variation can bottleneck.

## See also
- [[paged-attention-vllms-pagedattention]]
- [[kv-cache-in-autoregressive-generation]]
- [[efficient-attention-flashattention-flashattention-2-flashattention-3]]

## Sources
See frontmatter `sources:`.

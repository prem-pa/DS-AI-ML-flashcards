---
id: 04b88231-82b5-4558-ad1c-03d55b5ae78d
title: Leading Embedding Models (BGE-M3, E5, Voyage, Cohere, OpenAI)
track: ai-llm-engineer
topic: embeddings-retrieval
difficulty: 3
tags:
- embedding-models
- proprietary
- open-source
- production
aliases:
- state-of-the-art embeddings
- embedding model comparison
- top embedding models
sources:
- url: https://huggingface.co/spaces/mteb/leaderboard
  label: MTEB Leaderboard
- url: https://blog.voyageai.com/2024/09/18/voyage-3/
  label: Voyage AI v3 Blog
- url: https://docs.cohere.com/docs/rerank
  label: Cohere Embed Docs
- url: https://platform.openai.com/docs/guides/embeddings
  label: OpenAI Embeddings Docs
cards:
- id: ce54087d-6f79-426b-96ca-5120f25d15b7
  type: flip
  front: 'Compare BGE-M3 and E5-Mistral on three dimensions: self-hosting, cost, and multilingual support.'
  back: '**BGE-M3**: Alibaba open-source, self-host for free, small (1024 dims), supports 100+ languages
    well. **E5-Mistral**: Microsoft open-source, self-host (7B parameter compute cost), larger (768-1024
    dims depending on variant), English-focused (supports some multilingual via fine-tuning). For production,
    BGE-M3 is faster and multilingual; E5-Mistral excels where precision matters and you have compute
    budgets.'
- id: 716ba1ba-ac0c-4dc4-bd22-8748fb18672f
  type: mcq
  front: You need to embed 1 billion documents at <$10k/month. OpenAI text-3-large costs $0.02/1M tokens
    at inference. Which model is better?
  back: At billion-scale, proprietary API costs dominate. Voyage-3-large is 2.2x cheaper than OpenAI while
    outperforming by ~7.55% on MTEB. BGE-M3 is best for truly massive scale (self-host) but requires infrastructure.
    Cohere's compression helps but pricing is similar to OpenAI. For this budget, Voyage-3-large or self-hosted
    BGE-M3 wins.
  choices:
  - key: a
    text: OpenAI text-3-large; it's the most reliable
    correct: false
  - key: b
    text: Voyage-3-large; it costs 2.2x less and performs better
    correct: true
  - key: c
    text: BGE-M3; self-host it to avoid API costs entirely
    correct: false
  - key: d
    text: Cohere embed-v4 with compression
    correct: false
- id: 18f7cde3-ff66-46f2-bb5b-59d5eb1708ee
  type: flip
  front: What is Matryoshka representation learning? Name one embedding model using it.
  back: Matryoshka representation learning trains embeddings so that truncating to smaller dimensions
    (e.g., 256 from 3072) retains 90%+ of similarity information. Useful for reducing latency and storage
    without retraining. **OpenAI text-embedding-3-large** uses this, allowing users to truncate from 3072
    to 256 dims on-the-fly with minimal quality loss, reducing query latency and storage costs significantly.
- id: fe607e76-0241-4003-893a-6d8bef6cf82e
  type: flip
  front: Why does BGE-M3 support both dense and sparse retrieval in a single model? How does this enable
    hybrid search?
  back: 'BGE-M3 outputs both dense embeddings (learned semantic representation) and sparse vectors (learned
    BM25-like term weights). Dense captures semantic meaning; sparse captures term frequency and rare
    words. Hybrid search merges both: dense finds semantically similar items, sparse catches exact-match
    keywords missed by dense. This dual output reduces need for separate [[hybrid-search-dense-sparsebm25]]
    systems.'
- id: 796b85ba-4f5c-45d6-8611-4add487992b5
  type: flip
  front: Name three trade-offs between open-source embeddings (BGE-M3, E5) and proprietary models (OpenAI,
    Voyage, Cohere).
  back: '1. **Cost vs. Control**: Proprietary APIs are pay-per-call (no upfront infrastructure); open-source
    requires self-hosting (compute + latency management). 2. **Performance vs. Simplicity**: Open-source
    can match proprietary models on MTEB but need fine-tuning for domains; proprietary models are general-purpose
    out-of-box. 3. **Customization vs. Support**: Open-source allows full control and fine-tuning; proprietary
    models offer vendor support, guaranteed SLAs, and updates.'
---

## Intuition
A handful of embedding models dominate production: open-source champions (BGE-M3, E5-Mistral) offer flexibility and self-hosting; proprietary leaders (OpenAI text-embedding-3-large, Cohere embed-v4, Voyage-3) trade cost for reliability and support. Choosing the right model is a trade-off between MTEB performance, latency, cost, and domain fit.

## Detail

**BGE-M3** (Alibaba, open-source): 8K token context, 1024 dims, supports dense, sparse, and multi-vector hybrid retrieval. MTEB score ~63, excels at multilingual (100+ languages). Sparse vectors enable BM25-like term matching within the model.

**E5-Mistral** (Microsoft, open-source): Built on Mistral-7B, highest-performing open-source model for high-precision tasks. Larger compute footprint (7B parameters); best when accuracy outweighs latency.

**Voyage-3 & Voyage-3-Lite**: Proprietary (VoyageAI). Voyage-3-large outperforms OpenAI text-3-large by ~7.55% across domains while costing 2.2x less and using 3x smaller dimensions (1024 vs 3072). Voyage-3-Lite for cost-sensitive use cases.

**Cohere embed-v4**: Proprietary, MTEB 65.2. Includes built-in dimension compression (reduce storage 30–75% without quality loss). Strong for enterprise search and reranking pipelines paired with [[re-ranking-and-cross-encoder-models]].

**OpenAI text-embedding-3-large**: Proprietary, MTEB 64.6. 3072 dims, Matryoshka representation (can truncate to 256 dims with minimal quality loss). Widely used in production, excellent API reliability.

## Common gotchas / interview framings
- **Dimension size tradeoff:** Smaller dims (256, 512) are faster but less nuanced; 3072 is highest precision but costlier. Use [[leading-embedding-models-bge-m3-e5-voyage-cohere-openai-text-embedding-3]] with dimension truncation if possible.
- **Context window limits:** Most embeddings default to 512 or 8K tokens; [[chunk-size-and-overlap-in-document-splitting]] must account for this.
- **Proprietary cost at scale:** OpenAI at billion-scale vectors gets expensive; open-source offsets compute.
- **Multi-lingual models underperform on English:** [[multilingual-and-cross-lingual-embeddings]] often score lower on English-only tasks than English-specific models.
- Interviewers ask: "Voyage-3 vs OpenAI—which scales better?" Answer: Voyage-3 is cheaper; both are production-ready. BGE-M3 for self-hosting, Cohere for reranking pipelines.

## See also
- [[dense-embeddings-and-contrastive-learning]]
- [[mteb-benchmark-and-evaluation]]
- [[multilingual-and-cross-lingual-embeddings]]
- [[specialized-embeddings-medical-code-domain-specific]]
- [[hybrid-search-dense-sparsebm25]]

## Sources
See frontmatter `sources:`.

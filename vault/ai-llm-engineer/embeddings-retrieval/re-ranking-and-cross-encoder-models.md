---
id: 4274fb3b-5fd2-482b-b8b3-d32e1351341d
title: Re-Ranking and Cross-Encoder Models
track: ai-llm-engineer
topic: embeddings-retrieval
difficulty: 3
tags:
- reranking
- cross-encoders
- ranking
- neural-ranking
aliases:
- reranker models
- cross-encoder ranking
- relevance ranking
sources:
- url: https://docs.cohere.com/docs/rerank
  label: Cohere Rerank Docs
- url: https://cohere.com/blog/rerank-4
  label: Introducing Rerank 4
- url: https://galileo.ai/blog/mastering-rag-how-to-select-a-reranking-model
  label: 'Mastering RAG: Selecting Reranking Models'
cards:
- id: 44ce32f3-289d-4fec-850b-f5cb991fde75
  type: flip
  front: Explain the difference between bi-encoder and cross-encoder. Why is cross-encoder slower?
  back: '**Bi-encoder**: Encodes query once (10ms), encodes each doc independently (5ms each). No joint
    modeling. Fast (10 + 5k ms for k docs). **Cross-encoder**: Encodes (query, doc) pair jointly (5–10ms
    per pair). Captures query-doc interactions. Slower (5k–10k ms for k docs). Why slower: Every doc requires
    a separate forward pass through the full model with the query; bi-encoder amortizes query encoding
    once.'
- id: e14064d4-f12a-4771-8997-8be41da40039
  type: flip
  front: You have top-100 retrieved documents. Should you rerank all 100 or just top-20? What are the
    trade-offs?
  back: '**Rerank all 100** (5ms × 100 = 500ms latency): Highest chance of surfacing a missed relevant
    doc from position 50–100. Cons: 5x slower. **Rerank top-20** (100ms): Fast, likely covers most relevant
    docs (law of diminishing returns). Cons: May miss edge cases deep in retriever ranking. Recommendation:
    Start with top-50; if recall is sufficient, reduce to top-20 for latency. Measure on your data.'
- id: 710c1e00-7ef0-4e35-be0f-1b5f7df424b6
  type: mcq
  front: 'Your RAG system: dense retriever (70% recall) → rerank top-100 (90% recall) → LLM. End-to-end
    QA accuracy is 60%. Bottleneck?'
  back: 'Reranker is achieving 90% recall (good). The 30% gap between reranker recall (90%) and QA accuracy
    (60%) suggests the LLM isn''t using retrieved documents effectively. Issues: (1) Lost-in-the-middle
    (too many docs), (2) LLM hallucinating, (3) context overload. Solutions: Limit reranker output to
    top-5, refine prompt, or use stronger LLM.'
  choices:
  - key: a
    text: Reranker is missing relevant docs; increase reranking depth
    correct: false
  - key: b
    text: Reranker is working (90% recall); LLM is failing to answer based on context
    correct: true
  - key: c
    text: Dense retriever is weak; upgrade model
    correct: false
  - key: d
    text: Add more documents
    correct: false
- id: 437c748d-59e1-49ae-b378-9f07e22d83b8
  type: flip
  front: Compare Cohere Rerank v4, Cohere Rerank 3.5, and BGE-reranker-large on cost, accuracy, and deployment.
  back: '**Cohere Rerank v4**: Best accuracy, 32K context, multilingual. Proprietary (API cost ~$0.001–0.003
    per rerank). Fully managed. **Cohere Rerank 3.5**: Good accuracy, 4K context, multilingual. Older
    version, cheaper. **BGE-reranker-large**: Open-source, comparable accuracy, self-hosted (free software,
    GPU cost ~$500/month shared). Trade-off: Cohere v4 for best-in-class accuracy and minimal ops; BGE
    for cost-sensitive, large-scale systems.'
- id: db4e0da7-ec0e-4a17-adcd-8379ed6197a2
  type: flip
  front: Why does Cohere Rerank v4's 32K context window matter compared to Rerank 3.5's 4K?
  back: 'Rerank v3.5 (4K context) truncates documents longer than 4K tokens. A 8K-token document loses
    the last 4K. Rerank v4 (32K context) accommodates longer documents without truncation, enabling more
    complete document understanding. Useful for: long-form documents (research papers, legal contracts),
    multi-document ranking, and scenarios where document beginning and end are both important for relevance
    judgment.'
---

## Intuition
Retrieval (dense or BM25) retrieves fast but roughly. Reranking takes those rough candidates and reorders them with a more sophisticated model that jointly considers query + document (not independently). Cross-encoders (e.g., Cohere Rerank v4) are the modern approach, trading latency for accuracy. Most RAG systems use dense retrieval → cross-encoder reranking → LLM reader.

## Detail

**Bi-encoder vs Cross-encoder**:
- **Bi-encoder** (dense retrieval): Encode query independently, encode each doc independently, compute similarity. Fast (10ms), but doesn't capture query-doc interactions. Recall ~60%.
- **Cross-encoder**: Encode (query, doc) as a pair. Model sees both simultaneously, learns fine-grained interactions. Accuracy ~90%, but slow (5–100ms per doc, proportional to doc count).

**Why reranking works**: Retrievers optimize for speed; rerankers optimize for accuracy. Rank retrieved docs (50–100) with cross-encoder; top-10 output has high precision.

**Leading rerankers (2025–2026)**:
- **Cohere Rerank v4**: SOTA (as of early 2026). 32K context window, self-learning, multilingual. Variants: Fast (speed-optimized), Pro (accuracy-optimized).
- **Cohere Rerank 3.5**: Previous SOTA. Smaller context (4K), multilingual.
- **BGE-reranker-large**: Open-source, good quality, self-hostable. ~1000x cheaper than APIs but requires compute.
- **DPR + ColBERT**: Research models, not production-ready (slow, older architecture).

**Reranking cost**: Cross-encoding top-100 docs at 5ms/doc = 500ms latency. Typical RAG: retrieve (100ms) + rerank top-100 (500ms) = 600ms retrieval + 5–10s LLM = ~6–11s total.

## Common gotchas / interview framings
- **Law of diminishing returns**: Reranking top-100 improves precision; top-1000 has minimal ROI and huge cost. Most RAGs rerank top-50 to top-100.
- **Context window matters**: Cohere Rerank v4 has 32K context, enabling reranking very long documents. Rerank v3 (4K context) may truncate. [[leading-embedding-models-bge-m3-e5-voyage-cohere-openai-text-embedding-3]].
- **Open vs proprietary**: Cohere (proprietary) is best but costs per-API-call; BGE-reranker (open) is free but requires GPU compute (~$1/month on shared GPU, $500/month on dedicated).
- **Reranking is not deduplication**: Two documents with same meaning get different scores (cross-encoder distinguishes subtle differences). Deduplication is a separate concern.
- Interviewers ask: "Dense retrieval alone vs dense + reranker?" Answer: Reranker improves precision (70% → 90%); cost is ~500ms. Worth it for most RAG systems.

## See also
- [[rag-pipeline-architecture-retriever-ranker-reader]]
- [[hybrid-search-dense-sparsebm25]]
- [[retriever-design-bm25-dense-hybrid-multi-hop]]

## Sources
See frontmatter `sources:`.

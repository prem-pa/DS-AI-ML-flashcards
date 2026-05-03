---
id: e7a778f3-6b37-4928-b7da-b9d09eda436f
title: Hybrid Search (Dense + Sparse/BM25)
track: ai-llm-engineer
topic: embeddings-retrieval
difficulty: 3
tags:
- hybrid-search
- bm25
- sparse-retrieval
- ranking
aliases:
- dense + BM25
- combining dense and sparse
- multi-vector retrieval
sources:
- url: https://www.pinecone.io/learn/series/faiss/vector-indexes/
  label: Vector Indexes (Pinecone)
- url: https://blog.premai.io/best-embedding-models-for-rag-2026-ranked-by-mteb-score-cost-and-self-hosting/
  label: Best Embedding Models 2026
cards:
- id: b7be4414-480a-4d89-9d2b-1f45de8c251a
  type: flip
  front: Why does combining dense and sparse (BM25) retrieval improve recall compared to either alone?
  back: 'Dense retrieval captures semantic meaning but misses exact keywords (e.g., product SKU "XYZ-123"
    might not appear in training). BM25 captures exact terms but misses synonyms and paraphrases. A query
    for "blue shoes" with BM25 alone might miss documents mentioning "navy footwear." Hybrid retrieval:
    dense finds "navy footwear" (semantic match), BM25 finds "blue shoes" (exact match). Merging both
    result sets recovers both semantic and keyword-based relevance.'
- id: 0f9bf555-eba0-4949-9876-b662bab77c86
  type: flip
  front: Describe three strategies for combining dense and sparse retrieval results.
  back: '1. **Reciprocal Rank Fusion (RRF)**: Merge ranked lists using $\frac{1}{\text{rank}_1 + \text{rank}_2}$.
    Simple, rank-order agnostic. 2. **Weighted average**: $0.5 \cdot \text{dense\_score} + 0.5 \cdot \text{bm25\_score}$.
    Weights tuned on held-out dev set; better empirically. 3. **Learned fusion (reranker)**: Feed (query,
    dense_score, bm25_score) to a cross-encoder [[re-ranking-and-cross-encoder-models]] to predict final
    relevance. Best for complex interactions but requires labeled data.'
- id: 69e21943-93ff-431e-96ed-b16fcb7edba3
  type: mcq
  front: Your RAG system searches a technical documentation corpus. Queries often include exact terms
    ("REST API v3.2") and semantic paraphrases ("how to call HTTP endpoints"). Which retrieval should
    you use?
  back: 'Technical docs benefit from hybrid search. Dense embeddings struggle with version numbers ("v3.2"
    is rare in training, low semantic signal). BM25 retrieves exact matches directly. Hybrid catches both:
    dense finds conceptual matches ("HTTP endpoints" ↔ "REST API"), BM25 finds exact terms ("v3.2", "GET",
    "POST"). BGE-M3''s built-in sparse vectors also solve this elegantly in a single model.'
  choices:
  - key: a
    text: Dense-only; it captures both
    correct: false
  - key: b
    text: BM25-only; it's designed for exact matches
    correct: false
  - key: c
    text: Hybrid (dense + BM25); dense for semantic paraphrases, BM25 for exact terms like version numbers
    correct: true
  - key: d
    text: Dense with BGE-M3's sparse vector support
    correct: false
- id: b99f6369-b6d5-40f5-9430-e4d9df7e874f
  type: flip
  front: What is the computational cost of hybrid search vs dense-only retrieval?
  back: 'Hybrid search runs two retrievals (dense + BM25) sequentially or in parallel, then merges results.
    Computational cost: ~2x dense-only (two forward passes + merge overhead). At scale (million queries/day),
    this doubles infrastructure cost. For latency-sensitive systems, parallel execution helps, but peak
    latency is dominated by the slower retrieval (usually BM25 for large corpora). Trade-off: +100% cost
    for ~10–20% recall improvement in mixed-query workloads.'
- id: 8f7f3dbc-e1cc-49d4-a0c0-f7c733247952
  type: flip
  front: How does BGE-M3's multi-vector output simplify hybrid search compared to separate dense + BM25
    systems?
  back: 'BGE-M3 outputs three vectors per text: dense (semantic), sparse (BM25-like term weights), and
    colbert (token-level interaction vectors). All three come from one forward pass, eliminating the need
    to run separate BM25 indexing. Hybrid search is unified: query BGE-M3 → get dense + sparse vectors
    → search both in single vector DB → merge. Simpler deployment, no separate BM25 index, better maintained
    model coherence.'
---

## Intuition
Dense embeddings excel at semantic matching but miss exact keyword matches. BM25 (sparse retrieval) excels at keyword-driven queries but ignores semantic meaning. Hybrid search combines both: dense for semantic relevance, sparse for exact-term matching. The combination recovers precision lost by either alone.

## Detail

**Dense retrieval**: Query and documents encoded to vectors. Cosine similarity finds semantically similar documents. Works well for paraphrases and conceptual relevance. Fails on exact terms (proper nouns, product names, codes).

**Sparse retrieval (BM25)**: Term-based ranking. BM25 score: $\text{BM25}(q, d) = \sum_{i=1}^{n} \text{IDF}(q_i) \cdot \frac{f(q_i, d) \cdot (k_1 + 1)}{f(q_i, d) + k_1 \cdot (1 - b + b \cdot \frac{|d|}{\text{avgdl}})}$ where $f(q_i, d)$ is term frequency, IDF is inverse document frequency, $k_1$ and $b$ are tuning parameters. Excels when queries have exact terms but misses synonyms and paraphrases.

**Hybrid approaches**:
1. **Post-hoc fusion**: Retrieve top-k from dense, top-k from BM25, merge rankings (RRF, weighted average).
2. **Learned fusion**: Train a ranker on combined dense + sparse scores [[re-ranking-and-cross-encoder-models]].
3. **Single model (sparse+dense)**: BGE-M3 outputs both dense and sparse vectors in one forward pass, enabling unified hybrid search.

## Common gotchas / interview framings
- **Fusion weighting**: Equal weighting (0.5 dense + 0.5 BM25) is naive. Optimal weights depend on query type. Use [[re-ranking-and-cross-encoder-models]] to learn weights.
- **Sparse vector size**: BM25 produces vectors with vocabulary size dimensions (~100k for English). Storing alongside dense is memory-expensive; compression helps.
- **Query type variability**: Keyword-heavy queries favor BM25; semantic queries favor dense. Hybrid handles both but adds latency (two retrievals).
- **Vocabulary mismatch**: BM25 depends on exact term matching; misspellings, stemming differences hurt recall.
- Interviewers ask: "Dense or BM25 for product search?" Answer: Hybrid. Dense catches synonyms ("shoe" ↔ "footwear"); BM25 catches exact product names.

## See also
- [[dense-embeddings-and-contrastive-learning]]
- [[leading-embedding-models-bge-m3-e5-voyage-cohere-openai-text-embedding-3]]
- [[retriever-design-bm25-dense-hybrid-multi-hop]]

## Sources
See frontmatter `sources:`.

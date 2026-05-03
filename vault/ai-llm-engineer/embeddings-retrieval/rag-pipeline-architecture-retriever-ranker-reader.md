---
id: c72260c8-2718-4033-8ccf-4d6023d038a9
title: RAG Pipeline Architecture (Retriever, Ranker, Reader)
track: ai-llm-engineer
topic: embeddings-retrieval
difficulty: 3
tags:
- rag
- pipeline
- architecture
- system-design
aliases:
- RAG system design
- retrieval pipeline
- RAG architecture
sources:
- url: https://blog.premai.io/best-embedding-models-for-rag-2026-ranked-by-mteb-score-cost-and-self-hosting/
  label: Best Embedding Models for RAG 2026
cards:
- id: 80121422-3dc1-4293-92c2-6e676128a744
  type: flip
  front: Draw the three-stage RAG pipeline and explain why each stage is needed.
  back: 'Stage 1: **Retriever** (query → dense embed → vector search → top-k docs). Fast (10–100ms), rough
    recall (~60–80%). Stage 2: **Ranker** (top-k docs + query → cross-encoder → rerank → top-k''). Slower
    (100–500ms), high precision (~90%+). Stage 3: **Reader** (top-k'' docs + query → LLM → answer). Slow
    (1–10s), generates grounded answer. Why: Dense is fast but misses semantics. Cross-encoder is accurate
    but slow on large sets. LLM is slow but excellent at generation. Three stages optimize cost, latency,
    quality.'
- id: 5c043ae6-33d6-4cb8-8892-4f693fcf4c8c
  type: mcq
  front: Your RAG system has 99% retriever recall (dense embedding) but 60% end-to-end question-answering
    accuracy. The bottleneck is likely?
  back: High retriever recall (99%) with low QA accuracy (60%) indicates the ranker is missing relevant
    docs or the LLM is failing on large contexts. Since retriever is working well, the issue is downstream.
    Add a cross-encoder ranker [[re-ranking-and-cross-encoder-models]] to reorder and filter top-10 docs.
    If ranker is present, the issue is LLM context overload ("lost-in-the-middle"); limit to top-3 reranked
    docs.
  choices:
  - key: a
    text: Retriever quality; add more retrieval diversity
    correct: false
  - key: b
    text: Ranker quality or missing ranker; add cross-encoder reranking
    correct: true
  - key: c
    text: LLM quality; use a stronger model
    correct: false
  - key: d
    text: Chunk size; split documents differently
    correct: false
- id: 02f0189f-af0b-4fce-9d82-87cddd41295d
  type: flip
  front: Why is latency for the retriever stage (10–100ms) much lower than the ranker stage (100–500ms)?
  back: '**Retriever** uses fast approximate nearest neighbor search (HNSW, IVF) on vectors; only one
    forward pass through embedding model (10ms) + vector DB lookup (5–50ms). **Ranker** uses cross-encoders
    which score each (query, doc) pair independently, requiring one forward pass per doc. Scoring top-100
    docs takes 100 × 5–10ms = 500–1000ms. Rankers are slower because they''re more accurate (cross-encoder
    > bi-encoder) but must process each doc individually.'
- id: b5cf5cda-44a3-441f-ac94-1c0a616e8835
  type: flip
  front: Explain the 'lost-in-the-middle' problem in RAG and a solution.
  back: 'When passing 50 documents to an LLM with a prompt "Answer this question based on these docs,"
    the LLM often ignores middle documents, focusing on the first and last few. This is due to attention
    patterns in transformers. Solutions: (1) Pass fewer docs (top-3 to top-5) from a good ranker. (2)
    Use retrieval-augmented generation with iterative refinement: answer question once, identify gaps,
    retrieve more focused docs. (3) Integrate reranker [[re-ranking-and-cross-encoder-models]] to surface
    only the most relevant docs.'
- id: a00b30ea-2c32-457b-993c-38181838b809
  type: flip
  front: Compare single-hop vs multi-hop RAG retrieval. When should you use each?
  back: '**Single-hop**: Query → retrieve once → generate answer. Simple, fast (100–200ms), works for
    direct questions ("What is X?"). **Multi-hop**: Query → retrieve → generate partial answer → identify
    missing info → retrieve again → refine answer. Slower (1–2s), better for complex questions ("Compare
    X and Y, given the relationship between Z"). Use single-hop for most use cases; multi-hop when single-hop
    fails (low end-to-end recall). Multi-hop requires loop detection to avoid infinite retrieval.'
---

## Intuition
RAG (Retrieval-Augmented Generation) pipelines have three stages: (1) **Retriever** finds candidate documents fast but roughly, (2) **Ranker** reorders candidates with a more sophisticated model for accuracy, (3) **Reader** (LLM) reads top-k reranked documents and generates grounded answers. This three-stage design balances speed, accuracy, and cost.

## Detail

**Stage 1: Retriever** (speed-prioritized)
- Query → embed with dense model [[leading-embedding-models-bge-m3-e5-voyage-cohere-openai-text-embedding-3]] or BM25 [[hybrid-search-dense-sparsebm25]].
- Search [[vector-database-landscape-pinecone-qdrant-weaviate-milvus-pgvector-lancedb]] for top-k (typically 50–1000).
- Latency budget: 10–100ms. Recall: ~60–80% OK; ranker improves.

**Stage 2: Ranker** (accuracy-prioritized)
- Takes top-k from retriever. 
- Re-ranks using cross-encoder [[re-ranking-and-cross-encoder-models]] (e.g., Cohere Rerank v4).
- Typically re-ranks top-100 to top-10 or top-20.
- Latency budget: 100–500ms. Recall after reranking: 90%+.

**Stage 3: Reader** (generation)
- Takes top-k reranked documents.
- Feed to LLM with in-context prompt: "Given these documents, answer the question."
- LLM generates grounded answer.
- Latency: 1–10 seconds (LLM generation time dominates).

**Why three stages?**
- Dense embeddings are fast (10ms) but semantic-only; dense retrievers recall ~60% of relevant docs.
- Cross-encoders are slower (50–100ms per doc) but very accurate. Ranking top-100 is feasible; ranking 1M is not.
- LLMs are slow (1s+) but excellent at generating from context. Need ranker to ensure context is high-quality.

**Cost/latency/quality tradeoffs**:
- **Dense-only**: Fast (10ms), cheap, lower quality (~70% end-to-end recall).
- **Dense + Ranker**: Moderate (100–200ms), higher quality (~90% recall).
- **Dense + Ranker + LLM**: Slow (2s+), highest quality, most expensive.

## Common gotchas / interview framings
- **Bottleneck varies**: In early RAGs, retriever was bottleneck (poor recall). Modern models shift bottleneck to ranker (latency) or LLM (cost). Profile your system.
- **Ranker overfitting**: Training rankers on your data can overfit if data is small. Use [[re-ranking-and-cross-encoder-models]] from vendors or pretrain on large data.
- **Lost-in-the-middle**: Passing 50 documents to LLM and asking "answer based on these" can degrade quality. Reranker + limiting to top-3 to top-5 helps.
- **Multi-hop vs single-hop**: Simple RAG retrieves once. Multi-hop iterates: answer question → retrieve based on answer to refine.
- Interviewers ask: "Why not just rerank all docs with cross-encoder?" Answer: Cost. Cross-encoding 1M docs × 500ms each = 500k GPU-seconds. Dense retrieval + rerank top-100 is 100x faster.

## See also
- [[dense-embeddings-and-contrastive-learning]]
- [[vector-database-landscape-pinecone-qdrant-weaviate-milvus-pgvector-lancedb]]
- [[retriever-design-bm25-dense-hybrid-multi-hop]]
- [[re-ranking-and-cross-encoder-models]]
- [[chunk-size-and-overlap-in-document-splitting]]

## Sources
See frontmatter `sources:`.

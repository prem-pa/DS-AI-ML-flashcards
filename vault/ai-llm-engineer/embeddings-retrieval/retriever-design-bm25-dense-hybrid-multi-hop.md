---
id: c76dac06-409f-483f-b038-72ae1cab939b
title: Retriever Design (BM25, Dense, Hybrid, Multi-Hop)
track: ai-llm-engineer
topic: embeddings-retrieval
difficulty: 3
tags:
- retriever
- bm25
- dense-search
- multi-hop
aliases:
- retrieval methods
- search strategies
- query strategies
sources:
- url: https://www.pinecone.io/learn/series/faiss/vector-indexes/
  label: Vector Indexes (Pinecone)
cards:
- id: 9a96c3f7-9648-48a2-a783-6740b714f644
  type: flip
  front: Describe the strengths and weaknesses of BM25, dense, and hybrid retrievers.
  back: '**BM25**: Exact keyword matching. Fast, no embeddings needed. Fails on synonyms, paraphrases.
    Strength: rare terms, structured data. Weakness: semantic understanding. **Dense**: Embeddings, semantic
    matching. Handles paraphrases. Weakness: misses exact keywords (SKUs, names). Strength: open-domain
    QA. **Hybrid**: Combines both. Covers keywords + semantics. Weakness: 2x latency, fusion complexity.
    Strength: mixed query types.'
- id: be18505d-83d8-4ca4-a76c-55c508c9fa40
  type: mcq
  front: Your retriever is used for product search (queries like "best blue running shoes size 10"). BM25
    alone scores 75% recall. Dense alone scores 65%. Which should you use?
  back: 'Product search mixes keywords (size, color, brand) and semantics ("running shoes" ↔ "athletic
    footwear"). BM25 alone misses semantic variants; dense alone misses exact attributes. Hybrid retrieval:
    BM25 retrieves by size/color, dense retrieves by shoe type. Merging both yields ~80%+ recall. Multi-hop
    is overkill for single-turn product search.'
  choices:
  - key: a
    text: BM25 alone; it has higher recall
    correct: false
  - key: b
    text: Dense alone; it's more semantic
    correct: false
  - key: c
    text: Hybrid (BM25 + dense); BM25 catches exact "size 10", dense catches "running" ↔ "jogging"
    correct: true
  - key: d
    text: Multi-hop retrieval
    correct: false
- id: 704b7171-329d-40d9-8ee4-d8e19c14b363
  type: flip
  front: Explain multi-hop retrieval and when it should be used over single-hop.
  back: '**Single-hop**: Query once → retrieve documents → answer. **Multi-hop**: (1) Query → retrieve
    (2) Generate partial answer (3) Identify gaps/missing info (4) Retrieve again based on gaps (5) Repeat
    until confident. Use multi-hop for: complex questions requiring reasoning across multiple documents,
    follow-up questions, hypothetical reasoning. Example: "How would X policy affect Y given the relationship
    between Z and W?" Single-hop retrieval may miss Z-W relationship; multi-hop iterates to find it.'
- id: b42bb286-3635-4eaf-b6e1-3e1fb65ccd84
  type: flip
  front: What is query expansion and when should you use it?
  back: 'Query expansion augments the original query with synonyms, paraphrases, and related terms before
    retrieval. Example: "What is RAG?" → "What is retrieval-augmented generation? How do RAG systems work?
    RAG vs fine-tuning." Benefits: Higher recall, more robust to query variations. Costs: Increased computation,
    potential noise (expanded terms may introduce false matches). Use when: Dense retrieval recall is
    insufficient and computational budget allows; avoid when: computation is constrained or query is already
    comprehensive.'
- id: 724d29b4-1bd3-4c48-bd23-4cf2600e6caf
  type: flip
  front: What should a multi-hop retriever do to avoid infinite loops?
  back: 'Add termination conditions: (1) **Max iterations** (e.g., max 5 retrievals). (2) **Convergence
    check**: Stop if new information is minimal (e.g., >80% overlap with previous retrieval). (3) **Explicit
    relevance**: Stop if retrieved documents directly answer the query (LLM judges relevance). (4) **Loop
    detection**: Track retrieved document IDs; stop if re-retrieving same docs. Recommended: Combine max
    iterations + convergence check for robustness.'
---

## Intuition
Retriever design determines recall and speed. Dense retrievers (embeddings) are semantic but miss keywords. BM25 is keyword-centric but ignores meaning. Hybrid combines both. Multi-hop iterates retrieval to handle complex questions. Each has trade-offs in latency, recall, and cost.

## Detail

**BM25 Retriever**: Rank documents by term relevance. BM25 score $= \sum_i \text{IDF}(q_i) \cdot \frac{f(q_i, d) \cdot (k_1 + 1)}{f(q_i, d) + k_1(1 - b + b |d|/\text{avgdl})}$. Pros: Exact term matching, no embeddings needed. Cons: Fails on synonyms, paraphrases, semantic relevance. Best for: Keyword-driven queries, structured data with names/IDs.

**Dense Retriever**: Embed query and docs; find nearest neighbors. Pros: Semantic understanding, handles paraphrases. Cons: Misses exact keywords (rare terms, proper nouns). Latency: 10–100ms. Best for: Open-domain QA, semantic search.

**Hybrid Retriever**: Combine BM25 + dense. Retrieve top-k from each, merge rankings (RRF, learned fusion). Pros: Covers both keywords and semantics. Cons: 2x latency, complexity in fusion. Best for: Mixed query types (exact terms + semantic understanding).

**Multi-hop Retriever**: Iterative retrieval. (1) Answer query with initial retrieval. (2) Identify missing info. (3) Retrieve based on partial answer + gaps. (4) Repeat until confident. Pros: Handles complex reasoning, multi-document reasoning. Cons: 5–10x latency, complexity. Best for: Complex questions requiring reasoning across many docs.

**Query expansion** [[query-expansion-and-rewriting]]: Augment query with synonyms, paraphrases, related terms before retrieval. Improves recall but increases computation.

## Common gotchas / interview framings
- **Retriever recall vs end-to-end QA recall**: High retriever recall (relevant docs are retrieved) != high QA recall (LLM answers correctly). Ranker and LLM contribute downstream.
- **Multi-hop complexity**: Naive multi-hop can loop forever (retrieve → answer → retrieve → ...). Add termination condition: max iterations, convergence threshold, or explicit relevance check.
- **Fusion weights**: Equal weighting (0.5 BM25 + 0.5 dense) is suboptimal. Learn weights on dev set or use learned ranker [[re-ranking-and-cross-encoder-models]].
- **Query rewriting ambiguity**: Expanding a query can introduce noise. "Apple" → "Apple computer, apple fruit, Apple Inc." increases retrieval but may add noise. Use selective expansion.
- Interviewers ask: "Dense or BM25 for patent search?" Answer: Hybrid. Patents have exact claims (keywords) and conceptual novelty (semantics).

## See also
- [[dense-embeddings-and-contrastive-learning]]
- [[hybrid-search-dense-sparsebm25]]
- [[query-expansion-and-rewriting]]
- [[chunk-size-and-overlap-in-document-splitting]]

## Sources
See frontmatter `sources:`.

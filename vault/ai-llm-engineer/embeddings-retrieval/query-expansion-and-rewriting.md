---
id: 4b41c410-4058-4c02-a7de-30d82c3ec669
title: Query Expansion and Rewriting
track: ai-llm-engineer
topic: embeddings-retrieval
difficulty: 3
tags:
- query-optimization
- retrieval
- nlp
- semantic-search
aliases:
- query refinement
- query augmentation
- search term expansion
sources:
- url: https://blog.premai.io/best-embedding-models-for-rag-2026-ranked-by-mteb-score-cost-and-self-hosting/
  label: Best Embedding Models for RAG 2026
cards:
- id: 8532dbbd-dce3-492a-b3fc-85f09d3bbcd1
  type: flip
  front: Explain query expansion. What are two strategies (synonym-based vs LLM-based)?
  back: 'Query expansion adds related terms to improve retriever recall. **Synonym-based**: Use a thesaurus
    or synonym dictionary. "Apple" → "apple, apple fruit, apple tree". Fast (lookup only), deterministic,
    limited to pre-defined synonyms. **LLM-based**: Ask LLM to generate paraphrases. "How to learn Python"
    → "Python tutorial, Python for beginners, getting started with Python, Python learning guide". Slower
    (LLM inference), flexible, context-aware but potentially noisy.'
- id: afcf829c-2b45-4b15-9970-803ad12756c0
  type: flip
  front: What is query rewriting and how does it differ from expansion?
  back: '**Expansion** adds new terms; original query remains. **Rewriting** reformulates the query itself.
    Example expansion: "apple" + "apple inc". Example rewriting: "apple" → "apple inc" (domain-aware clarification).
    Rewriting is more aggressive; it replaces rather than augments. Use rewriting for: domain normalization
    ("amt" → "amount"), disambiguating terms, matching document conventions.'
- id: c4d79080-926c-4218-ac73-adb2149abf25
  type: mcq
  front: Your retriever gets query "How to fix Ctrl+S not working?" Zero relevant docs. Why expansion
    might help?
  back: Original query "Ctrl+S" is technical jargon; documents might use "save shortcut", "keyboard hotkey",
    "save command". Expansion bridges this vocabulary gap. Expanding to synonyms/related terms ("save
    shortcut", "keyboard shortcut", "Windows hotkey") increases the chance of matching relevant documents
    even if they use different terminology.
  choices:
  - key: a
    text: Expansion adds noise; not useful
    correct: false
  - key: b
    text: Expand to "Ctrl+S, save shortcut, keyboard shortcut issue, Windows/Mac save" to find documents
      on keyboard shortcuts
    correct: true
  - key: c
    text: Expansion is only for synonyms
    correct: false
  - key: d
    text: The query is fine; database is incomplete
    correct: false
- id: aabea459-1c23-44ba-9760-78c4dc7746d2
  type: flip
  front: When should you expand the user's query vs expanding documents at indexing time?
  back: '**Expand user query**: Runtime latency cost (multiple searches), handles dynamic user intent
    variations, no reindexing. **Expand documents at indexing**: One-time cost, consistent, no query-time
    latency. Recommendation: Expand documents if you control the corpus and expansion is deterministic
    (synonyms, domain normalization). Expand user queries if user intent is variable or real-time adaptation
    is needed. Bidirectional expansion (both) is best but costliest.'
- id: 58abd273-1df0-47e7-8b1a-e4e227add619
  type: flip
  front: How can you reduce noise from query expansion while maintaining recall gains?
  back: '1. **Semantic similarity threshold**: Only add expansions with >0.8 cosine similarity to original
    query. Filters out distant terms. 2. **Selective expansion**: Expand only ambiguous terms (proper
    nouns, rare terms) where expansion helps. 3. **Merge with ranking**: Retrieve via expanded query,
    rerank results with cross-encoder [[re-ranking-and-cross-encoder-models]] focusing on relevance to
    original (unmodified) query. Top-k results are high-quality despite expansion noise. 4. **Human-in-the-loop**:
    For critical queries, validate expansions before search.'
---

## Intuition
Queries are often terse or ambiguous ("Apple", "how to learn Python"). Query expansion augments queries with synonyms, paraphrases, or clarifications to improve retriever recall. Query rewriting reformulates the query to match document language better. Both trade computation for recall.

## Detail

**Query expansion**: Add related terms to the original query.
- **Synonym expansion**: "Apple" → "Apple, apple fruit, apple tree".
- **Paraphrase expansion**: "How to learn Python" → "How to learn Python, Python tutorial, Python for beginners, getting started with Python".
- **LLM-based expansion**: Use an LLM to generate alternative phrasings of the query. "What is RAG" → "What is retrieval-augmented generation? How does RAG work? RAG systems explained."

Expanded query is used for retrieval: search for original + all expanded terms, merge results.

**Query rewriting**: Reformulate query to match document language or domain conventions.
- **Domain normalization**: "doc" → "document", "amt" → "amount".
- **Clarification**: "apple" → "apple inc" (context-aware disambiguation).
- **Logical expansion**: "fast AND accurate" → boolean query logic.

**Impact**:
- **Pros**: Higher recall (more matching candidates), handles user spelling variations.
- **Cons**: Increased latency (multiple searches), potential noise (irrelevant expansions), higher cost.

**Implementation**:
1. **Static expansion**: Predefined thesaurus or synonyms. Fast, deterministic. Limited coverage.
2. **LLM-based**: Query to LLM → get expansions → search all. Flexible, handles context. Slower (LLM latency 500ms–1s).
3. **Learned expansion**: Train model to generate good expansions. Middle ground.

## Common gotchas / interview framings
- **Expansion noise**: Blindly expanding can introduce false positives. "Apple" → "Apple tree, apple juice, Apple Inc." confuses product search. Use selective expansion (semantic similarity threshold).
- **Latency cost**: Expanding a query 5x means 5x retrieval searches (if done serially). Implement in parallel and merge results for latency.<br/>
- **Diminishing returns**: First few expansions help; beyond 5–10 variations, recall plateaus. Stop when recall stops improving.
- **Domain specificity**: Generic expansions (synonyms) fail in specialized domains. Legal "contract" ≠ general "agreement". Use domain-aware models.
- Interviewers ask: "Should you expand user queries or document queries?" Answer: Often better to expand both (bidirectional expansion) or just documents at indexing time.

## See also
- [[retriever-design-bm25-dense-hybrid-multi-hop]]
- [[rag-pipeline-architecture-retriever-ranker-reader]]
- [[dense-embeddings-and-contrastive-learning]]

## Sources
See frontmatter `sources:`.

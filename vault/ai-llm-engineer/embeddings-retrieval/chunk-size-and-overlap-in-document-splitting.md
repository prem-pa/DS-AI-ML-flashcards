---
id: 973c7c5e-d8ef-495c-a792-96a8f2a10cd0
title: Chunk Size and Overlap in Document Splitting
track: ai-llm-engineer
topic: embeddings-retrieval
difficulty: 3
tags:
- document-processing
- chunking
- preprocessing
- rag
aliases:
- document splitting
- text chunking
- chunk strategy
sources:
- url: https://blog.premai.io/best-embedding-models-for-rag-2026-ranked-by-mteb-score-cost-and-self-hosting/
  label: Best Embedding Models for RAG 2026
cards:
- id: 1c16c21f-4bf8-4568-b0b8-7191b6d4ca3c
  type: flip
  front: Explain how chunk size and overlap affect vector database size and retrieval latency.
  back: 'Smaller chunks = more chunks per document. If 1000-page doc is split into 512-token chunks, you
    get ~2000 chunks (assuming ~256 tokens per page). Each chunk needs embedding and storage. More chunks
    = larger vector DB = slower retrieval (more similarity computations) and higher storage. Overlap multiplies
    this: 25% overlap = 1.25x more chunks. Trade-off: smaller chunks = higher recall (granular matching),
    larger chunks = lower cost (fewer embeddings).'
- id: 1fa643a9-bb7b-45ae-b1ee-c3df0f23fdc6
  type: mcq
  front: Your embedding model has 512 token context window. Should you use 1024 token chunks?
  back: If a chunk is 1024 tokens and your embedding model context is 512, the encoder truncates or ignores
    the last 512 tokens. This loses semantic information from the second half of the chunk. Always set
    chunk size ≤ embedding model context window. If you need longer chunks, use a model with larger context
    (e.g., BGE-M3 with 8K tokens) or split differently.
  choices:
  - key: a
    text: Yes; use the full model capacity
    correct: false
  - key: b
    text: No; chunks larger than context window truncate information at encoding
    correct: true
  - key: c
    text: Yes; the model will handle overflow
    correct: false
  - key: d
    text: No; you should use 128 token chunks
    correct: false
- id: d329eeff-4663-453f-853e-ffe966035619
  type: flip
  front: Compare sentence-level, paragraph-level, and fixed-size chunking. When should you use each?
  back: '**Sentence-level** (10–20 tokens): High granularity, clean semantic units. Best for open-domain
    QA. Risk: Lost context (sentences isolated). **Paragraph-level** (50–200 tokens): Natural units, balanced.
    Best for general documents. Risk: Variable size. **Fixed-size** (256, 512 tokens): Uniform, simple.
    Best for consistency. Risk: Ignores semantics. Use sentences for high-precision retrieval, paragraphs
    for balance, fixed-size for simplicity and consistency at scale.'
- id: dab2c1e8-a6b5-4fee-b164-5b06fc1e831e
  type: flip
  front: What is chunk overlap and why should you use it?
  back: 'Chunk overlap is repeating tokens at boundaries between consecutive chunks. Example: Chunk 1
    = tokens [0–256], Chunk 2 = tokens [200–456] (200 token overlap). Overlap preserves context lost at
    hard chunk boundaries. Without overlap, semantic information split across boundary is lost (Chunk
    1 ends, Chunk 2 begins, middle meaning is severed). Typical overlap: 10–25%. Higher overlap (50%+)
    maximizes context but doubles embedding cost.'
- id: 4ce807a9-9fc8-4400-bbfa-54f890cc8539
  type: flip
  front: Your RAG system splits documents into 256-token chunks with no overlap. Retriever recall is 70%.
    How would you improve it?
  back: 'Try: (1) **Increase overlap** to 10–25%; bridges context lost at boundaries. (2) **Increase chunk
    size** to 512 tokens; provides more context per chunk, may reduce boundary fragmentation. (3) **Use
    semantic chunking** instead of fixed-size; respects natural semantic units. Start with overlap (simplest,
    no recomputation needed); measure impact. If still low, reprocess with larger chunks or semantic splitting.'
---

## Intuition
Documents are too long to embed or pass to LLMs whole. You split them into chunks (sentences, paragraphs, fixed-length spans), embed each, and retrieve relevant chunks. Chunk size and overlap control granularity, recall, and retrieval quality. Too small = noisy fragments, lost context. Too large = semantic dilution, retrieval failures. No overlap = context breaks; high overlap = redundancy.

## Detail

**Chunk size options**:
- **Sentence-level** (10–20 tokens): Semantic units, clean boundaries. High recall (many candidates). Cons: Lost context, small chunks may lack meaning in isolation.
- **Paragraph-level** (50–200 tokens): Natural semantic units. Balance. Cons: Variable size, some paragraphs too long/short.
- **Fixed size** (256, 512, 1024 tokens): Consistent, predictable. Pros: Simple to implement, consistent embedding. Cons: May split mid-sentence, semantics ignored.
- **Semantic chunking**: Split on semantic boundaries (e.g., topic changes, NLP-detected section breaks). Pros: Respect semantic units. Cons: Slower to compute, variable chunk size.

**Chunk overlap**: Repeat tokens from adjacent chunks. Example: Chunk 1 tokens [1–256], Chunk 2 tokens [128–384] (128 token overlap). Overlap bridges context lost at chunk boundaries.
- **No overlap (0%)**: Highest efficiency, lowest context. Relevant info at boundary lost.
- **Moderate overlap (10–25%)**: Standard choice. Balances context and efficiency.
- **High overlap (>50%)**: Maximum context, more redundancy, increased embedding cost.

**Impact on retrieval**:
- Chunk size directly affects latency: more chunks = more embeddings to store, slower search (large indices).
- Overlap increases index size: 2x overlap = 2x more chunks, 2x more storage.
- Chunk quality affects recall: semantic chunks improve retriever ranking; random boundaries introduce noise.

## Common gotchas / interview framings
- **Context window mismatch**: If embedding model has 512 token context but chunks are 1024 tokens, information is lost at encoding. Match [[leading-embedding-models-bge-m3-e5-voyage-cohere-openai-text-embedding-3]] context window to chunk size.
- **Lost-in-the-middle with long chunks**: Passing 1000-token chunks to LLM may trigger "lost-in-the-middle" effect [[rag-pipeline-architecture-retriever-ranker-reader]].
- **No silver bullet**: Optimal chunk size is task/domain-dependent. Single paragraph for legal documents vs code blocks for documentation. Always measure on representative data.
- **Iterative tuning**: Start with 512 tokens, 10% overlap. Measure retriever recall. If low, reduce chunk size (more granularity); if noisy, increase size (more context).
- Interviewers ask: "What chunk size should we use?" Answer: "Start with 512 tokens, 10% overlap. Measure retriever recall. Iterate based on domain."

## See also
- [[rag-pipeline-architecture-retriever-ranker-reader]]
- [[retriever-design-bm25-dense-hybrid-multi-hop]]
- [[leading-embedding-models-bge-m3-e5-voyage-cohere-openai-text-embedding-3]]

## Sources
See frontmatter `sources:`.

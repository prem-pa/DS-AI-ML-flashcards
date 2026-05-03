---
id: 4e4f5c3e-0d42-48e6-a684-24b8d69c38f2
title: Vector Database Selection Criteria (Scale, Latency, Cost)
track: ai-llm-engineer
topic: embeddings-retrieval
difficulty: 3
tags:
- vector-databases
- system-design
- trade-offs
- evaluation
aliases:
- choosing a vector database
- database trade-offs
- vector DB selection
sources:
- url: https://liquidmetal.ai/casesAndBlogs/vector-comparison/
  label: Vector DB Comparison 2025
- url: https://callsphere.ai/blog/vector-database-benchmarks-2026-pgvector-qdrant-weaviate-milvus-lancedb
  label: Vector DB Benchmarks 2026
- url: https://tensorblue.com/blog/vector-database-comparison-pinecone-weaviate-qdrant-milvus-2025
  label: Vector DB Performance & Price 2025
cards:
- id: 4b820bf3-6a0d-4185-942d-04ec608544ea
  type: flip
  front: For each vector count range, recommend a vector database and justify.
  back: '**< 10M vectors**: Pinecone (no ops overhead, simplicity wins). **10M–100M**: pgvector (100x
    cheaper, still manageable with Postgres). **100M–1B**: Qdrant (balance of cost, performance, ops).
    **> 1B**: Milvus (only option that doesn''t bankrupt you). At 1B, Pinecone costs $250k+/month; Milvus
    on AWS/GCP is $5–10k/month.'
- id: 86861621-1965-4cd7-9b9f-d264e56fcf1a
  type: mcq
  front: Your startup has 50M vectors, $500/month infrastructure budget, p99 latency target of 100ms.
    What's the best choice?
  back: At 50M vectors, Pinecone costs $5–10k/month (outside budget). pgvector on a $100/month Postgres
    instance fits budget and HNSW achieves <100ms p99 latency. Qdrant would cost ~$500/month in EC2/K8s
    (tight budget, no buffer). Milvus is overkill at 50M. pgvector is the pragmatic choice.
  choices:
  - key: a
    text: Pinecone; it's the easiest
    correct: false
  - key: b
    text: pgvector with HNSW index; it fits your budget and latency
    correct: true
  - key: c
    text: Qdrant because it's open-source
    correct: false
  - key: d
    text: Milvus for future-proofing
    correct: false
- id: 5354c2fa-1b5f-424a-9e52-84f965c25b41
  type: flip
  front: Explain the latency-scale trade-off between HNSW and IVF indices.
  back: '**HNSW**: Fixed latency (~20–100ms at any scale) but memory grows 2–3x with vectors. At 1B vectors,
    HNSW uses 3–6TB. **IVF**: Memory scales linearly with vectors (1TB for 1B), but latency varies: few
    clusters (fast, low recall), many clusters (slow, high recall). Trade-off: HNSW = consistent latency,
    high memory; IVF = tunable latency, low memory. Choose HNSW if latency is strict; IVF if memory/cost
    is strict.'
- id: 2d59c71b-ca3c-444f-a7ae-bf753a62667f
  type: flip
  front: What hidden costs should you budget for when self-hosting Qdrant or Milvus?
  back: '1. **Infrastructure**: EC2/GCP instances, storage, network (~$1k–5k/month). 2. **DevOps/SRE**:
    Monitoring, alerting, backups, disaster recovery, upgrades (~10–20% engineer time). 3. **Scaling labor**:
    Tuning indices, rebalancing shards, capacity planning. 4. **Data migration**: If you need to switch
    later, moving billions of vectors is expensive. Budget $100k–500k in engineering + downtime. Pinecone''s
    per-vector pricing is high, but DevOps overhead of self-hosting often matches or exceeds it for growing
    systems.'
- id: 8902f482-fc55-4163-a92f-59a1f61c227c
  type: flip
  front: When would you choose Weaviate over Qdrant despite Qdrant's superior latency?
  back: 'When your use case requires knowledge graph + vector search (not pure vector search). Example:
    search documents and entities, rank by relationship to a query entity, filter by entity type. Weaviate''s
    GraphQL and structural filtering excel here. Qdrant is faster for pure semantic search. Trade-off:
    Weaviate for complex relational queries (lower latency still acceptable >100ms), Qdrant for speed-sensitive
    pure vector retrieval (<50ms p99).'
---

## Intuition
No single vector database wins on all axes. Pinecone is easiest but most expensive. pgvector is cheapest but requires Postgres expertise. Qdrant balances simplicity and cost. The right choice depends on your scale, latency requirements, cost sensitivity, and operational maturity.

## Detail
Key trade-offs when selecting a vector database:

**Scale (vectors stored)**:
- < 10M: Any option works. Pinecone simplicity often wins.
- 10M–100M: pgvector or Qdrant recommended. Pinecone becomes costly (~$1–10k/month).
- 100M–1B: pgvector or Milvus. Pinecone prohibitively expensive ($10–100k/month).
- 1B+: Milvus, self-hosted Qdrant, or pgvector at scale. Pinecone not viable.

**Latency (p99 query time)**:
- < 50ms: HNSW index (Qdrant, pgvector with HNSW).
- 50–200ms: IVF index or Qdrant default.
- > 200ms: Latency not a constraint; optimize for cost (Milvus, LanceDB).

**Cost ($/month)**:
- Managed (Pinecone): $100–100k depending on vector count and queries.
- Self-hosted (pgvector): $50–1000/month (Postgres server cost).
- Open-source (Qdrant, Milvus): $0 software + infrastructure (EC2, Kubernetes, ~$500–5000/month for mid-scale).

**Operational complexity**:
- Pinecone: None (managed).
- pgvector: Moderate (Postgres admin).
- Qdrant: Moderate (Docker/K8s).
- Milvus: High (distributed system, many moving parts).

## Common gotchas / interview framings
- **Hidden costs at scale**: Pinecone's per-vector pricing compounds; a 1B vector system costs $250k/month+.
- **Latency-scale coupling**: HNSW scales to 1B vectors but memory explodes; IVF is more memory-efficient but latency varies with cluster tuning.
- **Operational burden underestimation**: Self-hosting Qdrant/Milvus requires DevOps. Budget 10–20% engineering time for scaling, monitoring, disaster recovery.
- **Metadata filtering overhead**: Heavy metadata queries can negate ANN benefits. Measure on realistic queries before committing.
- Interviewers ask: "Pinecone vs pgvector?" Answer: "It depends on scale and budget. Pinecone < 50M vectors, unlimited budget. pgvector > 10M vectors, cost-sensitive."

## See also
- [[vector-database-landscape-pinecone-qdrant-weaviate-milvus-pgvector-lancedb]]
- [[exact-vs-approximate-nearest-neighbor-search-hnsw-ivf]]
- [[indexing-strategies-and-performance-tuning]]

## Sources
See frontmatter `sources:`.

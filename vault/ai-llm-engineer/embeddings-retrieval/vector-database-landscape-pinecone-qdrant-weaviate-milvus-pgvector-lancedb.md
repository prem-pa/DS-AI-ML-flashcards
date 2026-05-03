---
id: 75477fde-8357-4c28-9451-d93dda039535
title: Vector Database Landscape (Pinecone, Qdrant, Weaviate, Milvus, pgvector, LanceDB)
track: ai-llm-engineer
topic: embeddings-retrieval
difficulty: 3
tags:
- vector-databases
- infrastructure
- storage
- scalability
aliases:
- vector DB options
- vector storage
- embedding storage
sources:
- url: https://liquidmetal.ai/casesAndBlogs/vector-comparison/
  label: Vector DB Comparison 2025
- url: https://encore.dev/articles/best-vector-databases
  label: Best Vector Databases 2026
- url: https://callsphere.ai/blog/vector-database-benchmarks-2026-pgvector-qdrant-weaviate-milvus-lancedb
  label: Vector DB Benchmarks 2026
cards:
- id: 432320c3-5a30-4df4-8d22-653b063d8c12
  type: flip
  front: Compare Pinecone and pgvector on cost, operational overhead, and when to choose each.
  back: '**Pinecone**: Managed, pay per vector (~$0.25 per 100k vectors/month). No ops overhead. Scales
    trivially. Expensive at billion-scale. Best for startups/teams avoiding infrastructure. **pgvector**:
    Open-source Postgres extension, one-time setup, same cost as Postgres (typically $50–500/month for
    mid-scale). Requires Postgres knowledge. 100–1000x cheaper than Pinecone at scale. Best for teams
    with Postgres, cost-sensitive workloads. For <10M vectors with unlimited budget: Pinecone. For >100M
    vectors or cost-sensitive: pgvector.'
- id: c559f65d-4d44-4981-b293-559b53e21503
  type: mcq
  front: You're building a multi-tenant RAG system for 50 customers, each with private vectors. Which
    DB enables the cleanest isolation with minimal ops?
  back: Pinecone's namespaces provide built-in multi-tenancy isolation without extra ops. Milvus and Qdrant
    require custom namespace/deployment logic. pgvector with separate schemas is viable but adds Postgres
    management. Pinecone's abstraction and SLAs make it ideal for multi-tenant systems where simplicity
    > cost.
  choices:
  - key: a
    text: Milvus with per-customer namespaces
    correct: false
  - key: b
    text: Pinecone with namespaces
    correct: true
  - key: c
    text: pgvector with separate schemas
    correct: false
  - key: d
    text: Qdrant with per-customer deployments
    correct: false
- id: 63eb4312-71e2-4db8-9141-beba051793ab
  type: flip
  front: Why would you choose Qdrant or Milvus over Pinecone despite higher operational complexity?
  back: '**Cost at scale**: At 1B vectors, Pinecone costs $250k+/month; Qdrant/Milvus cost 10–50x less.
    **Data privacy**: No vectors sent to external service. **Customization**: Full control over indexing,
    replication, hardware. **Performance tuning**: [[indexing-strategies-and-performance-tuning]] can
    be optimized for your specific latency/recall tradeoff. Use Pinecone for simplicity <100M vectors;
    use open-source for massive scale or strict cost/privacy requirements.'
- id: 87d502c6-cf9c-47f2-a86e-1ec8c50c9154
  type: flip
  front: Explain the trade-off between Weaviate and Qdrant for a search system requiring both vector similarity
    and complex filtering.
  back: '**Weaviate**: Knowledge graph + vector DB. Excels at multi-hop queries and complex relational
    filtering. GraphQL interface is expressive. Lower throughput and latency compared to Qdrant. **Qdrant**:
    Pure vector search, simpler filtering. Excellent latency and throughput; BM25 integration for hybrid
    search. Use Weaviate for complex relationships (documents linked to entities, ontologies); Qdrant
    for pure vector/hybrid search performance.'
- id: fdb38d6a-5b7e-48e6-a8e3-f63d94e9ee44
  type: flip
  front: What is LanceDB and when is it appropriate for a vector search system?
  back: 'LanceDB is an embedded in-process vector DB built on Lance columnar format. Vectors and data
    live in the same zero-copy storage, enabling fast local search without a separate server. Ideal for:
    laptop/edge inference, single-machine embedding pipelines, local RAG without network overhead. Not
    suitable for multi-user remote access or high-concurrency systems. Use for development, offline search,
    or client-side search; use Qdrant/Pinecone for production multi-user systems.'
---

## Intuition
Vector databases store embeddings and enable fast similarity search at scale. Each trades off ease-of-use, cost, latency, and operational overhead differently. Pinecone offers managed simplicity; Qdrant offers open-source control; pgvector adds search to Postgres; LanceDB offers embedded/local search.

## Detail

**Pinecone** (managed): Fully hosted, zero-ops, auto-scaling. Pay per vector stored and queries. Best for teams avoiding infrastructure; costly at billion-scale. HTTP API, metadata filtering, namespaces for multi-tenancy.

**Qdrant** (open-source + cloud): High-performance ANN search in Rust. Excellent latency and throughput. Flexible deployment: Docker, Kubernetes, managed Qdrant Cloud. Strong filtering, built-in HNSW indexing. Best for teams comfortable with infrastructure.

**Weaviate** (open-source + cloud): Knowledge graph + vector search. GraphQL interface, complex filtering, multi-hop queries. Better for use cases requiring relational reasoning alongside semantic search.

**Milvus** (open-source): Deployed at billion+ vector scale. Supports multiple index types (IVF, HNSW, Annoy). Complex operational setup; recommended for massive production workloads with infrastructure teams.

**pgvector** (Postgres extension): Add vector search to existing Postgres. Vectors and metadata in same ACID transaction. No separate infrastructure; supports both IVFFlat and HNSW indexing. Best for teams already using Postgres, moderate scale (<100M vectors).

**LanceDB** (embedded): In-process vector DB built on Lance columnar format. Zero-copy data access, excellent for local/laptop development and edge search. Not suitable for remote multi-user access.

## Common gotchas / interview framings
- **Cost at scale**: Managed databases (Pinecone) are simple but 10–100x more expensive than pgvector or Milvus at billion-scale.
- **Operational complexity**: Qdrant and Milvus require DevOps; pgvector requires Postgres expertise; Pinecone requires none but API dependency.
- **Index selection matters**: pgvector HNSW is faster than IVFFlat but larger; trade latency vs. memory [[exact-vs-approximate-nearest-neighbor-search-hnsw-ivf]].
- **Metadata filtering overhead**: Heavy filtering can negate ANN speedup; design your schema carefully [[vector-database-selection-criteria-scale-latency-cost]].
- Interviewers ask: "Pinecone vs pgvector for 100M vectors?" Answer: pgvector is 100x cheaper; Pinecone for managed simplicity only if budget allows.

## See also
- [[exact-vs-approximate-nearest-neighbor-search-hnsw-ivf]]
- [[indexing-strategies-and-performance-tuning]]
- [[vector-database-selection-criteria-scale-latency-cost]]

## Sources
See frontmatter `sources:`.

---
id: 34fa2aa3-6acd-4513-9a9d-8d5915d78a83
title: Exact vs Approximate Nearest Neighbor Search (HNSW, IVF)
track: ai-llm-engineer
topic: embeddings-retrieval
difficulty: 3
tags:
- indexing
- algorithms
- approximate-search
- ann
aliases:
- ANN search
- nearest neighbor algorithms
- HNSW vs IVF
sources:
- url: https://www.pingcap.com/article/approximate-nearest-neighbor-ann-search-explained-ivf-vs-hnsw-vs-pq/
  label: 'ANN Search Explained: IVF vs HNSW vs PQ'
- url: https://www.myscale.com/blog/hnsw-vs-ivf-explained-powerful-comparison/
  label: HNSW vs IVF Comparison
- url: https://www.pinecone.io/learn/series/faiss/vector-indexes/
  label: Vector Indexes (Pinecone)
cards:
- id: e99de33a-ac51-4635-b92d-6d335803b83e
  type: flip
  front: Explain the core idea of HNSW. Why is it so fast despite having to search vectors?
  back: HNSW (Hierarchical Navigable Small World) builds a multi-layer graph where vectors are connected
    to nearby neighbors. Search navigates the graph, jumping between layers, following edges toward the
    query's nearest neighbors. Because the graph is hierarchical and each layer halves the search space,
    reaching the target takes O(log N) steps instead of O(N). This logarithmic traversal is why HNSW achieves
    sub-100ms latency on billion-scale vectors.
- id: b3b85f9b-1572-44e5-8893-6fd261f238f3
  type: flip
  front: What is the main difference between HNSW and IVF in how they partition the search space?
  back: '**HNSW**: Graph-based. Vectors are nodes in a multi-layer graph; edges connect nearby neighbors.
    Partitioning is implicit (neighbors form clusters). **IVF**: Partition-based. Vectors are explicitly
    divided into k clusters via k-means. Search: coarse quantization identifies relevant clusters, then
    exact search within clusters. HNSW explores via graph navigation; IVF explores via cluster membership.'
- id: d7d873d3-a314-4945-b182-a557047a3caa
  type: mcq
  front: You have 1B vectors. Memory is $10/GB, query latency budget is 100ms. HNSW uses 3TB, IVF uses
    1TB. Which scales better?
  back: 'At billion scale, memory cost dominates. IVF''s 1TB vs HNSW''s 3TB is $20k/month difference.
    Both can meet 100ms latency: IVF via cluster count, HNSW via default settings. IVF scales better economically.
    HNSW is better for offline indexing with high recall requirements; IVF wins for cost-sensitive, update-heavy
    systems.'
  choices:
  - key: a
    text: HNSW; it has better recall
    correct: false
  - key: b
    text: IVF; it has 2TB less memory cost ($20k/month savings) and meets latency budget with cluster
      tuning
    correct: true
  - key: c
    text: They scale equally; choose based on data distribution
    correct: false
  - key: d
    text: HNSW; it's faster to build
    correct: false
- id: 37a9b96a-b507-4161-bec1-e190ecd52811
  type: flip
  front: How does tuning the number of clusters (k) in IVF affect recall and latency? What's the trade-off?
  back: 'Larger k (more clusters): Each cluster is smaller, so per-cluster recall improves, but you must
    search more clusters to achieve the same recall. Trade-off: **Fewer clusters** (k=100) = low latency,
    lower recall (~80%). **More clusters** (k=10000) = high latency, higher recall (~95%). At query time,
    you can tune how many clusters to search, adjusting recall vs latency dynamically. This tunability
    is IVF''s advantage over HNSW (which has fixed recall at index build time).'
- id: 01399cbe-c7c0-4351-81a6-3a8a906aead1
  type: flip
  front: Why is HNSW poor for systems with frequent vector updates, and how does IVF handle updates better?
  back: 'HNSW''s multi-layer graph structure requires rebuilding to maintain performance after many insertions/deletions.
    Rebalancing the graph is expensive (hours for billion-scale). IVF handles updates more gracefully:
    new vectors are assigned to their nearest cluster and inserted incrementally, requiring minimal re-indexing.
    For streaming data or evolving corpora, IVF is significantly more efficient than HNSW.'
---

## Intuition
Exact nearest neighbor (brute-force) is slow at scale (O(n) per query). Approximate nearest neighbor (ANN) trades small accuracy loss for massive speedup. HNSW and IVF are the two dominant ANN algorithms, each with different speed/accuracy/memory trade-offs.

## Detail

**Exact search (brute-force)**: Compare query vector to all N database vectors via cosine similarity or L2 distance. Time: O(N·d) per query. For N=1M, d=1024, ~1B operations per query → 1–10s latency. Unacceptable for production.

**Approximate Nearest Neighbor (ANN)**: Sacrifice small accuracy loss (~0–5%) for sub-second search.

**HNSW (Hierarchical Navigable Small World)**: Graph-based algorithm. Builds a multi-layer hierarchical graph where each vector connects to nearby neighbors. Search navigates the graph, jumping between layers, reaching the target in log(N) steps. Properties:
- **Latency**: Very fast (1–100ms at 1B scale).
- **Memory**: Higher (graph overhead ~2–3x vector size).
- **Index build time**: Slow (hours for 1B vectors).
- **Recall**: Very high (95%+ at default settings).
- **Update handling**: Adding/removing vectors is slow and memory-intensive.

**IVF (Inverted File)**: Partition-based algorithm. Divide vector space into k clusters (via k-means), build inverted index. During query: search only relevant clusters via coarse quantization, then re-rank within clusters. Properties:
- **Latency**: Fast (10–500ms depending on cluster count).
- **Memory**: Compact (minimal overhead).
- **Index build time**: Fast (k-means is fast).
- **Recall**: Medium (80–95%, depends on cluster quality and cluster search count).
- **Update handling**: Fast; adding vectors to existing clusters is efficient.
- **Data dependency**: Performance varies with data distribution; poorly clustered data degrades recall.

## Common gotchas / interview framings
- **Index size trade-off**: HNSW uses 2–3x more memory than IVF. For 1B vectors, HNSW might need 3–6TB vs IVF's 1–2TB.
- **Recall vs latency tuning**: HNSW recall is fixed at index build; IVF recall is tunable via cluster count at query time (higher cluster count = higher recall, higher latency).
- **Update patterns**: HNSW struggles with frequent updates (re-indexing costly); IVF handles incremental updates well.
- **Clustering assumption**: IVF assumes data has natural clusters; high-dimensional random data or non-Euclidean distributions hurt IVF.
- Interviewers ask: "HNSW or IVF for a real-time search system?" Answer: IVF (faster updates); for offline indexing: HNSW (better recall).

## See also
- [[vector-database-landscape-pinecone-qdrant-weaviate-milvus-pgvector-lancedb]]
- [[indexing-strategies-and-performance-tuning]]
- [[vector-database-selection-criteria-scale-latency-cost]]

## Sources
See frontmatter `sources:`.

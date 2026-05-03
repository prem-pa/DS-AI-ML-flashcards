---
id: 103021a2-021f-4e96-9bbe-2dcc9fa7fe7d
title: Embedding-based retrieval at scale
track: verticals
topic: recommender-systems
difficulty: 5
tags:
- embedding-retrieval
- approximate-nearest-neighbor
- faiss
- scann
- quantization
- hashing
- distributed-search
- scalability
aliases:
- ann-search
- vector-search
- similarity-search
- large-scale-retrieval
sources:
- url: https://github.com/facebookresearch/faiss
  label: 'FAISS: Facebook AI Similarity Search'
- url: https://github.com/google-research/google-research/tree/master/scann
  label: 'ScaNN: Google Scalable Nearest Neighbors'
- url: https://blog.twitter.com/engineering/en_us/topics/insights/2018/ads-recommendations-machine-learning
  label: Twitter Ads Recommendation at Scale
cards:
- id: 951a1088-e14f-4771-b54e-17421912dc2f
  type: flip
  front: Why can't naive similarity search (computing dot product with all n items) scale to billions
    of items?
  back: Naive search is O(n·d) where n=items, d=embedding dimension. With n=billions, d=hundreds, this
    requires trillions of operations per query. At typical speeds (1B ops/sec), that's 1000+ seconds per
    query—infeasible. ANN algorithms reduce to O(d log n) or O(d·polylog(n)) via indexing.
- id: 0cec4619-4cb3-44fe-bea4-8d93cf7aad8e
  type: mcq
  front: What is the core idea behind Product Quantization (PQ) for efficient similarity search?
  back: 'PQ splits embedding space: d=128 dims → 8 subspaces of 16 dims. Each subspace is quantized to
    k centroids (e.g., 256). Each vector stored as 8 bytes (one index per subspace) instead of 512 bytes.
    Query computes similarity via subspace lookups. Tradeoff: 16x compression vs some recall loss from
    quantization.'
  choices:
  - key: a
    text: Train multiple similarity models and ensemble their predictions
    correct: false
  - key: b
    text: Decompose the d-dimensional space into m subspaces, quantize each to k centroids
    correct: false
  - key: c
    text: Hash items into buckets based on locality-sensitive hash functions
    correct: false
  - key: d
    text: Build a graph index where nodes are items and edges connect neighbors
    correct: false
- id: 97e6b8f1-c46b-4ee7-bf0a-7bc0f3f97d31
  type: flip
  front: How does ScaNN (Scalable Nearest Neighbors) improve upon Product Quantization?
  back: 'ScaNN uses Anisotropic Vector Quantization (AQ): quantization codebook is adapted to the query
    distribution, not fixed. Also uses a two-stage approach: coarse search with AQ (find candidates),
    fine-grained reranking with original embeddings. This achieves higher recall at same latency compared
    to standard PQ.'
- id: 9ef9e33f-8e2a-4063-8a9e-46dccb5773f6
  type: flip
  front: What is the recall-latency tradeoff in production ANN systems, and how is it typically tuned?
  back: 'Higher recall (find true nearest neighbors) requires more computation: more clusters (IVF), more
    layers (HNSW), more hash functions (LSH) → increased latency. Production systems typically target
    90-98% recall at 10-50ms latency for serving SLAs. Parameters tuned based on hardware, query batch
    size, and business tradeoffs.'
- id: 80d404df-386f-4406-86c9-42e3bf68864c
  type: mcq
  front: Why is embedding staleness a problem in large-scale ANN retrieval systems?
  back: 'If embeddings are retrained but the ANN index isn''t rebuilt, queries use new embeddings against
    old indexed embeddings → mismatch → degraded recall. Mitigation: periodic index refreshes (daily/weekly),
    incremental updates (add new embeddings, remove old ones), or maintaining multiple index versions
    during canary rollouts.'
  choices:
  - key: a
    text: Old embeddings have lower dimensionality
    correct: false
  - key: b
    text: If user/item towers retrain, the index is built from old embeddings and becomes suboptimal
    correct: false
  - key: c
    text: Stale embeddings always have zero recall
    correct: false
  - key: d
    text: Embedding staleness only affects batch processing
    correct: false
---

## Intuition

Given a user embedding, finding the top-k most similar items from a catalog of billions is a nearest-neighbor search problem. Naive approach (compute similarity to all items) is O(n) time and infeasible. Approximate nearest neighbor (ANN) algorithms trade recall for speed, enabling sub-second latency at scale.

## Detail

**Core problem:**
Given query embedding q ∈ ℝ^d and database D = {x_1, ..., x_n} ⊂ ℝ^d, find k items with highest similarity(q, x_i). Naive: O(n·d) similarity computations. With n = billions and d = hundreds, this is prohibitive.

**ANN approaches:**

1. **Locality-Sensitive Hashing (LSH):**
   - Hash items and query into buckets; retrieve only bucket items
   - Hash functions preserve similarity: nearby vectors hash to same bucket with high probability
   - Tradeoff: tuning hash function length (more hashes → higher recall but slower)
   - Used for minhash (Jaccard similarity), random projection (cosine similarity)

2. **Product Quantization (PQ):**
   - Decompose d-dimensional space into m subspaces (e.g., d=128 → 8 subspaces of 16 dims)
   - Quantize each subspace to k centroids (e.g., 256 clusters per subspace)
   - Index stores only quantized values (8 bytes instead of 512 bytes per vector)
   - Query: compute similarity between query subspaces and each centroid subspace, reconstruct top candidates
   - Tradeoff: compression ratio vs recall loss

3. **Hierarchical Navigable Small World (HNSW):**
   - Graph-based index; each item is node; edges connect to nearest neighbors
   - Multi-layer structure: top layer has few nodes (approximate search), bottom layers dense (refinement)
   - Query: start from top, greedily navigate to nearest neighbor, descend to lower layers
   - Fast construction O(n log n); query O(log n) with small constants
   - Used in libraries like Faiss, Hnswlib

4. **ScaNN (Scalable Nearest Neighbors, Google):**
   - Combines ANISOTROPIC vector quantization (AQ) with reranking
   - AQ: Learn quantization codebook adapted to query distribution (unlike PQ which is fixed)
   - Two-stage: coarse search with AQ (find candidates), fine-grained search with original embeddings
   - Empirically achieves higher recall than PQ at same latency

**Production systems (FAISS, Hnswlib, Vespa):**

- **FAISS (Facebook):** Open-source library supporting multiple index types (PQ, HNSW, IVF-Flat, etc.)
  - IVFFlat: Inverted file index, partition space into k clusters, search relevant clusters
  - Composite index: combine multiple strategies (e.g., IVF + PQ)
  - Distributed: index sharded across machines, search parallelized

- **ScaNN (Google):** Focus on latency with high recall; tuned for dynamic workloads

**Recall-latency tradeoff:**
- Parameter tuning: increase number of clusters (IVF) / layers (HNSW) / hash functions (LSH) → higher recall, slower
- Typical production: 90-98% recall @ 10-50 ms latency

**Embedding dimensionality:**
- Common: 64-512 dims (higher dim → more expressive but slower, needs more memory)
- Tradeoff: model accuracy vs latency

## Common gotchas / interview framings

- **Cold-start:** New items without embeddings or insufficient embedding history. Fall back to content features or random exploration.
- **Embedding drift:** If user/item towers retrain frequently, index becomes stale. Need refresh strategy (periodic rebuild vs incremental updates).
- **Dimensionality curse:** High-dimensional spaces are sparse; nearest neighbors less meaningful. Require dimensionality reduction (PCA, learned compression) or re-embedding.
- **Query-specific optimization:** LSH, PQ, ScaNN all optimize differently. Choice depends on hardware (CPU vs GPU), query patterns (batch vs streaming), and SLA.
- **Recall vs diversity:** Top-k ANN results are highly correlated (all similar to query). May need post-processing (diversity reranking) to avoid filter bubbles.
- **Index synchronization:** Embedding databases must stay in sync with model versions. Managing multiple index versions/canary deployments is non-trivial.
- **Distributed scalability:** Single-machine indices (HNSW) don't scale to billions; require sharding strategies, distributed graph construction, and careful rebalancing.

## See also
- [[approximate-nearest-neighbor-search]]
- [[faiss-facebook]]
- [[scann-google]]
- [[locality-sensitive-hashing]]
- [[product-quantization]]
- [[indexing-structures]]
- [[distributed-similarity-search]]
- [[recall-latency-tradeoff]]

## Sources
See frontmatter `sources:`.

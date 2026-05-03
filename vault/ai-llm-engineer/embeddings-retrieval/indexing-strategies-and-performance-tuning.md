---
id: 506b2a2d-b8cc-4f93-846c-0620513218d0
title: Indexing Strategies and Performance Tuning
track: ai-llm-engineer
topic: embeddings-retrieval
difficulty: 5
tags:
- indexing
- optimization
- performance
- tuning
aliases:
- index tuning
- performance optimization
- vector indexing
sources:
- url: https://www.myscale.com/blog/hnsw-vs-ivf-explained-powerful-comparison/
  label: HNSW vs IVF Deep Dive
- url: https://www.pingcap.com/article/approximate-nearest-neighbor-ann-search-explained-ivf-vs-hnsw-vs-pq/
  label: ANN Search Explained
- url: https://medium.com/@bavalpreetsinghh/pgvector-hnsw-vs-ivfflat-a-comprehensive-study-21ce0aaab931
  label: pgvector HNSW vs IVFFlat Study
cards:
- id: 807c5524-0acc-4606-a6e7-7926beba8b18
  type: flip
  front: Explain the HNSW parameters M, efConstruction, and ef. Which is tunable post-indexing?
  back: '**M**: Maximum degree per node in the graph. Fixed at index build. Higher M (32 vs 8) = higher
    recall, more memory, slower queries. **efConstruction**: Neighbor check budget during index building.
    Fixed at build. Higher (1000 vs 200) = better quality, slower build. **ef**: Neighbor check budget
    at query time (dynamic). Can be tuned post-index without rebuild. Increase ef for higher recall (slower
    queries); decrease for latency.'
- id: 69e2b132-700f-4639-9ece-3e86476eac85
  type: mcq
  front: 'Your HNSW index with M=16, ef=200 has p99 query latency of 500ms (budget: 100ms). How should
    you tune?'
  back: ef is tunable post-index. Decreasing ef from 200 to 50 will significantly reduce latency (log
    reduction in neighbor checks) while moderately reducing recall. This is the fast fix. Rebuilding with
    M=8 is slower and permanent. nprobe is an IVF parameter. For HNSW, decrease ef first; if recall drops
    below acceptable, then consider rebuilding with lower M or switching to IVF.
  choices:
  - key: a
    text: Decrease ef to 50; trade recall for latency
    correct: true
  - key: b
    text: Decrease M to 8; rebuild the index
    correct: false
  - key: c
    text: Increase nprobe; it'll speed up queries
    correct: false
  - key: d
    text: Use IVF instead of HNSW
    correct: false
- id: 4523c6fb-3346-4c43-a72a-af71c6e04c66
  type: flip
  front: What is the relationship between IVF's nlist and nprobe? How do they control the latency-recall
    trade-off?
  back: '**nlist**: Number of clusters (fixed at index build). Typical 100–10k. **nprobe**: How many nearest
    clusters to search per query (tuned post-index). Relationship: more clusters (higher nlist) = smaller
    clusters, faster per-cluster search, but you must probe more clusters to maintain recall. With nlist=1000
    and nprobe=10, you search 10 of 1000 clusters (~1%). Higher nprobe (10→50) = higher recall, higher
    latency. Tune nprobe post-index to hit your latency budget.'
- id: 66a78522-2f6b-488b-835f-67d0d849ad03
  type: flip
  front: Describe a systematic approach to benchmarking and tuning vector indices for production.
  back: '1. **Representative sample**: Use 10k–100k vectors from production distribution. 2. **Latency-recall
    grid**: Test parameter combinations (M=[8,16,32], ef=[50,200,400]) on 1k queries. Measure recall@10
    and latency. 3. **Plot curves**: Plot latency vs recall for each config. 4. **Pick the knee**: Choose
    the parameter set maximizing recall while staying under latency budget. 5. **Warm-up & cache**: Cache
    index in memory, warm up on 100 queries before measuring (HNSW has cache effects). 6. **A/B test in
    production**: Test your tuned params on a subset of traffic before full rollout.'
- id: 00f1fb66-afaf-42e2-8a83-4c7650e6cac0
  type: flip
  front: When and why should you use quantization (PQ, scalar quantization) in vector indices?
  back: 'Quantization reduces vector size (float32 → int8, or Product Quantization to 4–16 bits per chunk)
    for massive memory savings (4–16x) at the cost of ~1–5% recall loss. Use when: (1) memory is the constraint
    (billion-scale indices), (2) network bandwidth is bottleneck (distributed systems), (3) you have latency
    budget to trade for memory. Don''t use if accuracy is critical or if recall loss is unacceptable.
    Example: 1B vectors, 1024 dims, float32 = 4TB; with PQ8 = 250GB.'
---

## Intuition
Building and tuning vector indices is engineering. Small knob tweaks (HNSW M, efConstruction, IVF nlist, nprobe) can 10x latency or halve recall. Advanced practitioners optimize indices for their specific data distribution, query patterns, and latency-recall-memory constraints via systematic benchmarking.

## Detail

**HNSW tuning parameters**:
- **M (max neighbors per node)**: Default 16. Higher M = higher recall, more memory, slower indexing. Typical range: 8–48. Set M=32 for high recall, M=8 for low memory.
- **efConstruction**: Number of nearest neighbors to check during index building. Default 200. Higher = better index quality, slower build. Typical: 200–1000.
- **ef (query parameter)**: Analogous to efConstruction but at query time. Higher ef = higher recall, slower query. Tune post-index for latency-recall tradeoff.

**IVF tuning parameters**:
- **nlist (cluster count)**: Number of cluster centers. IVF builds k-means index with nlist clusters. Typical: 100–10000. Higher nlist = more clusters, shorter scan per cluster, less recall per probe. Trade-off: more clusters = shorter per-cluster search but need to probe more clusters for same recall.
- **nprobe (clusters to search)**: How many nearest clusters to search at query time. Default ~nlist/8. Higher nprobe = higher recall, higher latency. Tune this post-index for latency-recall tradeoff.
- **Factory string (FAISS)**: Encodes index configuration, e.g., "IVF100,PQ64" (100 clusters, 64-bit product quantization). Complex configurations trade memory/accuracy/speed.

**Quantization** (advanced): Reduce vector size (float32 → float8 or binary). 4–16x memory savings, small accuracy loss. Example: PQ (Product Quantization) decomposes 1024-dim vector into chunks, quantizes each. Used in Milvus, FAISS.

**Best practices**:
1. **Profile your data**: Measure recall@k for different parameter settings on a sample.
2. **Latency-recall curves**: Plot latency vs recall for different M, ef, nprobe values. Choose the knee of the curve for your latency budget.
3. **Warm-up queries**: Cache/warm HNSW index before serving; first queries are slower.
4. **Monitor in production**: Measure actual p50, p95, p99 latencies; recall on real query distributions may differ from benchmark.

## Common gotchas / interview framings
- **Overfitting to benchmark**: Tuning indices on standard benchmarks (SIFT, GIST) may not generalize to your data. Always benchmark on representative data.
- **Stateless parameter tuning**: HNSW's ef and IVF's nprobe are query-time parameters; you can A/B test different values without rebuilding the index.
- **Memory explosion with HNSW**: HNSW at M=64 on 1B vectors uses 10TB+; easily exceeds available RAM. Monitor memory during indexing.
- **IVF cluster imbalance**: If clusters are imbalanced (some huge, some tiny), nprobe-based recall is inconsistent. k-means initialization matters.
- Interviewers ask: "My HNSW query is too slow." Answer: Decrease ef (faster, lower recall). If recall is already bad, you need lower M and rebuild.

## See also
- [[exact-vs-approximate-nearest-neighbor-search-hnsw-ivf]]
- [[vector-database-landscape-pinecone-qdrant-weaviate-milvus-pgvector-lancedb]]
- [[vector-database-selection-criteria-scale-latency-cost]]

## Sources
See frontmatter `sources:`.

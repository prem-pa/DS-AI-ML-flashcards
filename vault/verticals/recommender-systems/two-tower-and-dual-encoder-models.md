---
id: 076dd234-aa2f-42d8-85e2-fea34c1bcdb8
title: Two-tower and dual-encoder models
track: verticals
topic: recommender-systems
difficulty: 3
tags:
- two-tower
- dual-encoder
- candidate-generation
- embedding-retrieval
- approximate-nearest-neighbor
- scalability
- youtube
- pinterest
aliases:
- dual-tower
- siamese-networks
- embedding-based-retrieval
- neural-collaborative-filtering
sources:
- url: https://cloud.google.com/blog/products/ai-machine-learning/scaling-deep-retrieval-tensorflow-two-towers-architecture
  label: 'Google Cloud: Scaling Deep Retrieval with Two Towers'
- url: https://towardsdatascience.com/two-tower-networks-and-negative-sampling-in-recommender-systems-fdc88411601b/
  label: Two-Tower Networks and Negative Sampling (Towards Data Science)
- url: https://www.shaped.ai/blog/the-two-tower-model-for-recommendation-systems-a-deep-dive
  label: 'The Two-Tower Model: Deep Dive (Shaped)'
cards:
- id: 59fff318-4ab7-4437-9867-756ef8800c8d
  type: flip
  front: Why are two-tower models essential in production recommendation systems at scale?
  back: Two-tower models enable efficient candidate generation from massive catalogs (billions of items).
    User embeddings are compared against precomputed item embeddings using approximate nearest neighbor
    (ANN) search, reducing O(n) complexity to O(d log n). This makes it feasible to retrieve top candidates
    in milliseconds.
- id: b399ecb7-d327-4072-a168-1adafd9e1e0f
  type: mcq
  front: How is the two-tower model typically trained?
  back: 'Two-tower uses contrastive learning: maximize sim(user_i, item_i) while minimizing sim(user_i,
    item_j) for negatives. Loss is typically cross-entropy with in-batch negatives or sampled softmax.
    Hard negative mining (sampling difficult negatives) improves training efficiency.'
  choices:
  - key: a
    text: Minimize MSE between user and item embeddings
    correct: false
  - key: b
    text: Maximize inner product of positive pairs while minimizing inner product of negative pairs (contrastive
      learning)
    correct: false
  - key: c
    text: Ranking loss on pairwise item comparisons
    correct: false
  - key: d
    text: Classification loss on binary click prediction
    correct: false
- id: f4e73dc2-bcfa-4b83-9543-f31f2f7d9c3d
  type: flip
  front: What is the role of approximate nearest neighbor (ANN) search in two-tower inference?
  back: During inference, instead of computing similarities between user embedding and all n items (O(n)),
    ANN search retrieves top-k nearest items in O(d log n) time. Libraries like FAISS and ScaNN index
    precomputed item embeddings for fast lookup, making real-time retrieval feasible.
- id: 8e439c92-a017-41d1-ad35-a9da625b4912
  type: flip
  front: What is the 'embedding collapse' problem in two-tower training, and how is it mitigated?
  back: 'Embedding collapse occurs when both towers learn to output nearly identical embeddings for all
    items, destroying discriminative power. Mitigation: (1) Hard negative mining to force distinction,
    (2) Regularization on embeddings, (3) Proper batch size to ensure diverse negative samples, (4) In-batch
    negatives strategy, (5) Temperature scaling in similarity scores.'
- id: 8fee7b13-d87c-450c-84ea-a1a80618ce33
  type: mcq
  front: Why is there often a gap between two-tower training and ranking model inference in production
    pipelines?
  back: 'Two-tower is trained on logged interactions (clicks) which have position bias: items ranked higher
    get more clicks. Ranker uses a smaller subset of candidates and can incorporate position-correction
    signals (IPW, causal inference) for unbiased ranking. Also, two-tower embeddings may not reflect real-time
    trending, while ranker can use fresh signals.'
  choices:
  - key: a
    text: Two-tower is trained on implicit feedback with position bias, while ranker sees unbiased data
    correct: false
  - key: b
    text: Ranker requires more real-time signals than two-tower
    correct: false
  - key: c
    text: Two-tower embeddings become stale after training
    correct: false
  - key: d
    text: Two-tower uses different features than ranker
    correct: false
---

## Intuition

Two-tower models enable efficient large-scale candidate generation by learning separate dense embeddings for users and items. During inference, retrieve items via approximate nearest neighbor (ANN) search in embedding space. In production pipelines: two-tower for candidate generation (millions → thousands), then ranking models for final ordering.

## Detail

**Architecture:**
- **User tower:** Neural network consuming user features/context (id, history, demographics, context)
- **Item tower:** Neural network consuming item features (id, metadata, embeddings)
- **Output:** user_embedding (d-dim) and item_embedding (d-dim)
- **Similarity:** Inner product or cosine similarity score_ij = user_i · item_j^T

**Training objective:**
Max likelihood with in-batch negatives or sampled negatives:
- Positive: (user_i, item_i) with label 1
- Negatives: (user_i, item_j where j != i) with label 0
- Loss: Cross-entropy or triplet loss
- Hard negative mining: Sample negatives similar to positive to harder optimization

**Inference pipeline:**
1. Encode all items offline: item_embeddings = item_tower(item_features) → shape (n_items, d)
2. Quantize/index embeddings using FAISS, ScaNN, or other ANN libraries
3. For a user query: user_embedding = user_tower(user_features)
4. ANN search: retrieve top-k items by nearest neighbors (O(d log n) vs O(n) naive)
5. Pass top-k to ranker for final scoring

**Production at scale (Pinterest, YouTube, Meta):**
- Two-tower + ANN retrieves hundreds/thousands from billions of items
- Supports sequence models in user tower (e.g., GRU over recent history)
- Can modify loss for diversity: decrease negative sampling probability for retrieved items to increase variety
- Handles position bias in ranking stage separately

**Negative sampling strategies:**
- **Random:** Simple but inefficient
- **In-batch negatives:** Use other positive pairs in batch as negatives (cheap)
- **Hard negatives:** Mine difficult examples (similar embeddings, false negatives) → better optimization
- **Sampled softmax:** Efficient approximation of full softmax

## Common gotchas / interview framings

- **Embedding collapse:** Both towers learn to output similar embeddings for all items → no discrimination. Mitigated by regularization, hard negative mining, or batch size tuning.
- **Training-serving gap:** Two-tower trained on implicit feedback (logged interactions) with position bias. Ranker should handle position-corrected scoring.
- **Embedding staleness:** User/item embeddings precomputed → real-time behavior (e.g., trending item) requires online encoding or refresh.
- **ANN approximation error:** Approximate search (FAISS, ScaNN) may miss true nearest neighbors, reducing coverage. Tradeoff: speed vs recall.
- **Calibration:** Inner product assumes embedding norms are calibrated. If not, similar items can have very different scores. Consider normalizing embeddings or using cosine similarity.
- **Cold-start:** New items/users without embedding history. Mitigate with content-based features or hybrid warm-start.

## See also
- [[two-tower-model]]
- [[dual-encoder-architecture]]
- [[approximate-nearest-neighbor-search]]
- [[candidate-generation]]
- [[embedding-space]]
- [[inner-product-similarity]]
- [[cosine-similarity]]
- [[scann]]
- [[faiss]]

## Sources
See frontmatter `sources:`.

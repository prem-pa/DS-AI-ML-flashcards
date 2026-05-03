---
id: bbc4c6b7-d622-4c82-b679-357d1e68ef11
title: Content-based filtering and features
track: verticals
topic: recommender-systems
difficulty: 3
tags:
- content-based-filtering
- feature-engineering
- item-features
- semantic-similarity
- cold-start
- hybrid-systems
aliases:
- content-driven recommendations
- feature-based filtering
- semantic recommendation
sources:
- url: https://en.wikipedia.org/wiki/Recommender_system
  label: 'Wikipedia: Recommender Systems Overview'
- url: https://towardsdatascience.com/recommendation-system-basics-with-collaborative-filtering-and-content-based-filtering-8dbb1925c2f4
  label: Content-based vs Collaborative Filtering Tutorial
cards:
- id: 40fd2ae7-fa9a-4ad3-899c-68a2d8502dbe
  type: flip
  front: What is the fundamental difference between collaborative filtering and content-based filtering?
  back: 'CF uses patterns in user-user or item-item interactions: ''users with similar histories like
    similar items.'' Content-based uses item features: ''recommend items similar to ones you liked.''
    CF requires interaction data; content-based requires item metadata.'
- id: cd00f90b-3411-48c6-9b3b-fb092dab9292
  type: mcq
  front: Why is content-based filtering particularly useful for the cold-start problem?
  back: 'New items have no user interactions, so CF cannot work. Content-based filtering works immediately
    if item features (metadata, embeddings) are available, making it ideal for handling new items. This
    is why most production systems use hybrid: CF for ranked users, content-based for new items.'
  choices:
  - key: a
    text: It trains faster than collaborative filtering
    correct: false
  - key: b
    text: It can recommend new items with zero interaction history using their features
    correct: false
  - key: c
    text: It requires fewer labeled examples
    correct: false
  - key: d
    text: It always produces more accurate recommendations
    correct: false
- id: b733aedf-3a7f-4077-9150-ca7ba78eadd2
  type: flip
  front: How would you construct a user preference profile in content-based filtering?
  back: 'Common methods: (1) Average features of all liked items, (2) Weighted average using ratings as
    weights (higher ratings → more influence), (3) Learned preference vectors from logistic regression
    on liked/disliked items, (4) Recency-weighted average (recent interactions weighted higher).'
- id: fd9e0683-d597-4d3d-bd59-1ec09f6290cb
  type: flip
  front: What is the main limitation of content-based filtering regarding recommendation diversity?
  back: It tends to recommend items very similar to past liked items, creating filter bubbles. Users get
    trapped in narrow recommendation silos (e.g., someone who watches one action movie gets only action
    recommendations). CF is better for serendipitous discoveries.
- id: c9cc559e-4b6e-43f5-80eb-cacbf0ae97b8
  type: mcq
  front: Which similarity metric is most appropriate for comparing dense learned embeddings in content-based
    systems?
  back: Cosine similarity is the standard for dense vector embeddings. It measures angle between vectors,
    ignoring magnitude. For categorical one-hot features, Jaccard is better. For TF-IDF vectors, cosine
    is also optimal. Euclidean distance can work but cosine is more numerically stable for high-dimensional
    sparse embeddings.
  choices:
  - key: a
    text: Jaccard similarity
    correct: false
  - key: b
    text: Cosine similarity
    correct: false
  - key: c
    text: Manhattan distance
    correct: false
  - key: d
    text: Hamming distance
    correct: false
---

## Intuition

Content-based filtering recommends items similar to those a user has liked in the past. Instead of leveraging collaborative patterns, it relies on item features/metadata and user preference profiles. If a user watched action movies, recommend similar action movies.

## Detail

**Approach:**
1. Extract item features (genre, director, cast, keywords, plot embeddings, etc.)
2. Build user preference profiles from their rated/interacted items
3. Compute similarity between user profile and candidate items
4. Recommend top-k most similar items

**Feature representations:**
- **Categorical:** One-hot/multi-hot encoding (genres, language, rating)
- **TF-IDF:** Text features from titles, descriptions, tags
- **Embeddings:** Learned representations from deep models (plot text → BERT embeddings)
- **Metadata:** Release date, duration, budget, ratings
- **Derived:** Trending scores, engagement metrics

**Similarity metrics:**
- Cosine similarity (most common for embeddings)
- Euclidean distance
- Jaccard similarity (categorical features)

**User profile construction:**
- Average of liked items' features
- Weighted average (using ratings as weights)
- Explicit preference vectors (learned from interaction)

**Hybrid approaches:** Combine CF + content by:
- Learning embeddings jointly on interactions + metadata
- Ensemble predictions: α·CF_score + (1-α)·content_score
- Factorization machines incorporating both signal types

## Common gotchas / interview framings

- **Cold-start advantage:** New items with no interaction history can be recommended via content features (unlike CF). This is crucial for platforms with high item churn.
- **Limited discovery:** Tends to recommend similar items; may create filter bubbles. User who likes one action movie gets recommended only action movies.
- **Feature engineering burden:** Success depends heavily on feature quality. Poor features → poor recommendations. Requires domain knowledge.
- **Scalability:** Feature extraction and similarity computation scales with feature dimensionality and catalog size.
- **Serendipity:** Less likely to surface unexpected/diverse recommendations compared to CF.
- **Popularity bias in metadata:** If metadata reflects past engagement, content filtering inherits popularity bias.

## See also
- [[content-based-filtering]]
- [[feature-engineering]]
- [[item-metadata]]
- [[semantic-similarity]]
- [[hybrid-recommenders]]
- [[cold-start-problem]]
- [[tfidf]]
- [[embedding-similarity]]

## Sources
See frontmatter `sources:`.

---
id: fa52b52f-f9b8-41ed-91ec-691a442eb1cc
title: Embedding-based features
track: ml-engineer
topic: feature-engineering-selection
difficulty: 3
tags:
- categorical-encoding
- embeddings
- dimensionality-reduction
- neural-networks
- representation-learning
aliases:
- learned-embeddings
- categorical-embeddings
- embedding-layers
- dense-representations
sources:
- url: https://sebastianraschka.com/faq/docs/feature_sele_categories.html
  label: Sebastian Raschka - Feature Selection Categories
- url: https://www.blog.trainindata.com/feature-selection-with-wrapper-methods/
  label: Train in Data - Feature Selection and Embeddings
- url: https://www.frontiersin.org/journals/big-data/articles/10.3389/fdata.2025.1624507/pdf
  label: Frontiers - Feature Selection and Deep Learning
cards:
- id: a646f7f0-0f7f-4753-9d53-d0bfad196d54
  type: flip
  front: Why are embeddings superior to one-hot encoding for high-cardinality categoricals (e.g., 100k
    distinct items)?
  back: 'One-hot creates 100k sparse binary features; embeddings use 10-50 dense dimensions. Benefits:
    (1) far fewer features, (2) capture similarity (nearby vectors = similar categories), (3) smoother
    gradients, (4) reduce data sparsity and improve generalization.'
- id: a8bc13b9-889d-4128-838e-9077853243aa
  type: flip
  front: How would you handle a new category during inference if you trained embeddings on historical
    data?
  back: 'Several approaches: (1) pre-assign rare categories to a single ''unknown'' embedding, (2) use
    a hash-based bucketing strategy to assign new categories to consistent embeddings, (3) use a default
    initialization (e.g., mean of existing embeddings), (4) treat as a missing value and impute.'
- id: 6a3eb3c1-875f-4001-8230-4f090acbb0fa
  type: mcq
  front: When should you use pre-trained embeddings (e.g., from word2vec or collaborative filtering) vs.
    learning embeddings end-to-end?
  back: Pre-trained embeddings transfer knowledge and are ideal for low-data regimes. End-to-end learning
    optimizes for your specific task when you have enough data. Both approaches work; choice depends on
    data volume and domain.
  choices:
  - key: a
    text: Always learn end-to-end; pre-training is only for NLP
    correct: false
  - key: b
    text: Use pre-trained if you have limited training data or the embedding task (e.g., product similarity)
      is well-understood; learn end-to-end for large data or novel tasks
    correct: true
  - key: c
    text: Pre-trained embeddings are incompatible with tree models, so only use for neural nets
    correct: false
  - key: d
    text: End-to-end learning always overfits; pre-training is mandatory
    correct: false
- id: 82a26748-b552-41d4-9bfa-713da3b25c8d
  type: flip
  front: What's the trade-off in choosing embedding dimension (e.g., 10 vs. 50 vs. 128)?
  back: 'Small dimensions (10): faster, less overfitting, but may lose category nuance. Large dimensions
    (128): more expressive, captures fine-grained similarity, but requires more data and increases overfitting
    risk. Rule of thumb: ~√(cardinality) or cross-validate.'
- id: cb1f0a45-3421-4c7f-be73-5974c377c423
  type: mcq
  front: In a tabular model with 1M users and 100k products, would you recommend one-hot or embeddings
    for product_id?
  back: Embeddings are ideal here. 100k one-hot features cause sparsity and overfitting. 32-dim embeddings
    reduce noise, capture product similarity, and improve generalization.
  choices:
  - key: a
    text: One-hot; it's simpler and works fine for tabular data
    correct: false
  - key: b
    text: Embeddings; 100k features from one-hot is excessive; embeddings yield ~32 features and capture
      similarity
    correct: true
  - key: c
    text: Both; concatenate one-hot and embeddings for redundancy
    correct: false
  - key: d
    text: Neither; drop the product_id and use only user features
    correct: false
---

## Intuition

Embeddings replace high-cardinality categorical variables (e.g., product ID with 100k values) with learned dense vectors. Instead of one-hot encoding (100k sparse features), an embedding maps each category to a 10-50 dimensional dense vector. This reduces sparsity, captures semantic similarity, and often improves generalization. Neural networks learn embeddings end-to-end; for tree models, you can pre-train embeddings separately (e.g., via Word2Vec on sequences of product IDs) and use the dense vectors as input features.

## Detail

Embedding layers in neural nets learn representations optimized for the downstream task. A product embedding captures product similarity; users who buy product A are more likely to buy products similar in embedding space.

Pre-trained embeddings (from NLP or recommendation systems) transfer knowledge: word2vec embeddings for text categories, embedding models from collaborative filtering. For tabular data, fastText can embed categorical columns unsupervised.

Benefit over one-hot: drastically fewer features (10 vs. 100k), automatic similarity capture, smoother gradients for gradient descent. Drawback: less interpretable than one-hot, requires careful initialization and regularization to avoid overfitting in low-data regimes.

```python
# Keras embedding layer in a model
from tensorflow.keras.layers import Embedding, Input, Flatten, Dense

input_ids = Input(shape=(1,))
embed = Embedding(input_dim=100000, output_dim=32)(input_ids)  # 100k categories -> 32-dim vectors
flat = Flatten()(embed)
output = Dense(1, activation='sigmoid')(flat)
```

## Common gotchas / interview framings
- "Why not just use one-hot encoding for categorical variables?"
- "When should you pre-train embeddings vs. learn them end-to-end?"
- "How do you handle new categories not seen during training with embeddings?"
- "Compare embedding dimensions: too small (loses info), too large (overfits). How do you choose?"

## See also
- [[embeddings]]
- [[one-hot-encoding]]
- [[target-encoding]]
- [[categorical-variables]]
- [[neural-networks]]
- [[dimensionality-reduction]]

## Sources
See frontmatter `sources:`.

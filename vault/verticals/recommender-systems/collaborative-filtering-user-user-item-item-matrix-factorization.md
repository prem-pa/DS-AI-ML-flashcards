---
id: 6a83370c-e51a-4485-8e23-6952b8c0f2ba
title: Collaborative filtering (user-user, item-item, matrix factorization)
track: verticals
topic: recommender-systems
difficulty: 3
tags:
- collaborative-filtering
- user-similarity
- item-similarity
- matrix-factorization
- latent-factors
- netflix-prize
aliases:
- memory-based filtering
- model-based filtering
- SVD
- SVD++
sources:
- url: https://datajobs.com/data-science-repo/Recommender-Systems-%5BNetflix%5D.pdf
  label: Netflix Matrix Factorization Techniques Paper
- url: https://en.wikipedia.org/wiki/Matrix_factorization_(recommender_systems)
  label: 'Wikipedia: Matrix Factorization in Recommender Systems'
- url: https://link.springer.com/article/10.1007/s10115-024-02315-z
  label: Collaborative Filtering Through Netflix Prize Lens
cards:
- id: 52a18b57-1696-4c0e-a501-93bf57cbf14f
  type: flip
  front: What is the core assumption in collaborative filtering that makes it work?
  back: Users with similar past preferences will like similar future items. This is leveraged by either
    finding similar users (user-user CF) or similar items (item-item CF) based on interaction patterns.
- id: 3f440b44-ec97-4883-868b-946cccfc8521
  type: mcq
  front: In matrix factorization R ≈ U × V^T, what do U and V represent?
  back: U is the user latent factor matrix (m users × k latent dimensions), V is the item latent factor
    matrix (n items × k latent dimensions). The dot product U_i · V_j predicts user i's preference for
    item j.
  choices:
  - key: a
    text: U = user-item interactions, V = item features
    correct: false
  - key: b
    text: U = user latent factors (m × k), V = item latent factors (n × k), where k is the rank
    correct: false
  - key: c
    text: U = singular values, V = eigenvectors
    correct: false
  - key: d
    text: U = user embeddings, V = video embeddings (YouTube-specific)
    correct: false
- id: 13e1d9d2-f416-459d-895c-3e7674ed8935
  type: flip
  front: Why is SVD++ an improvement over basic SVD in collaborative filtering?
  back: 'SVD++ incorporates implicit feedback (clicks, skips) and explicit ratings together. Netflix Prize
    results showed SVD++ → timeSVD++ progression: RMSE SVD < SVD++ < timeSVD++. The improvement of SVD++
    over SVD was smaller than timeSVD++ over SVD++ because temporal dynamics captured significant preference
    drift.'
- id: 559d23e3-44a9-4db9-bc2d-724d077d448e
  type: flip
  front: What is the main limitation of user-user collaborative filtering compared to matrix factorization?
  back: User-user CF requires computing similarity between all user pairs, which is O(m²) complexity.
    For platforms with millions of users, this is computationally prohibitive. Matrix factorization is
    O(mnk) and can scale better with distributed training.
- id: e06526fd-28be-4ca5-b702-9bdecc2cf374
  type: mcq
  front: Which regularization term prevents overfitting in the MF objective ||R - UV^T||² + λ(||U||² +
    ||V||²)?
  back: The λ(||U||² + ||V||²) term is L2 regularization on latent factors. It penalizes large factor
    values, preventing overfitting. Typical λ values range from 0.01 to 0.1. Combined with early stopping
    and appropriate k selection, this controls model complexity.
  choices:
  - key: a
    text: '||R - UV^T||²'
    correct: false
  - key: b
    text: λ(||U||² + ||V||²)
    correct: false
  - key: c
    text: The learning rate in SGD
    correct: false
  - key: d
    text: Early stopping only
    correct: false
---

## Intuition

Collaborative filtering predicts user preferences by leveraging patterns in user-item interaction history. The core insight: if two users rated similar items similarly, they likely have similar tastes. Matrix factorization decomposes the sparse user-item interaction matrix into low-rank latent factor matrices, revealing hidden preference patterns.

## Detail

**Three main approaches:**

1. **User-user CF:** Find similar users, recommend items liked by neighbors. Computationally expensive for large user bases.

2. **Item-item CF:** Find similar items based on co-rating patterns. More stable than user-user as item relationships are stable.

3. **Matrix Factorization (MF):** Decomposes user-item matrix R into R ≈ U × V^T, where U is (m × k) user latent factors, V is (n × k) item latent factors, k << min(m,n). Learned via SGD minimizing: ||R - UV^T||² + λ(||U||² + ||V||²).

**Advanced variants:**
- **SVD:** Basic matrix factorization with regularization
- **SVD++:** Incorporates implicit feedback and temporal dynamics
- **timeSVD++:** Adds time-aware components for evolving preferences

**Netflix Prize (2006-2009):** Demonstrated SVD improvements from ~0.95 RMSE (Cinematch baseline) to 0.857 RMSE (ensemble). SVD++ → timeSVD++ showed consistent gains with more latent factors.

## Common gotchas / interview framings

- **Cold-start:** MF fails for new users/items with no interaction history. Requires content-based hybrid approach.
- **Sparsity:** Real-world matrices are 99%+ sparse; efficient computation needed (only factorize known entries).
- **Scalability:** User-user similarity O(m²), item-item O(n²) can be prohibitive; MF is O(mnk) and more scalable.
- **Implicit vs explicit:** MF can handle implicit feedback (clicks, watches) differently—common in production.
- **Overfitting:** Heavy regularization and early stopping critical; k selection is hyperparameter tuning problem.

## See also
- [[collaborative-filtering]]
- [[matrix-factorization]]
- [[singular-value-decomposition]]
- [[latent-factor-model]]
- [[user-item-interactions]]
- [[sparsity-problem]]
- [[netflix-competition]]

## Sources
See frontmatter `sources:`.

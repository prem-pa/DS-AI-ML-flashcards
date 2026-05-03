---
id: 906e1c83-f23e-4f26-8b93-1f34a2f968c2
title: Learning-to-rank and ranking models
track: verticals
topic: recommender-systems
difficulty: 3
tags:
- learning-to-rank
- listwise-loss
- pairwise-loss
- pointwise-loss
- dcn
- dlrm
- ctr-prediction
- feature-interaction
aliases:
- LTR
- ranking-optimization
- rank-learning
- pointwise-ranking
sources:
- url: https://arxiv.org/abs/2008.13535
  label: 'DCN V2: Improved Deep & Cross Network Paper'
- url: https://arxiv.org/pdf/1906.00091
  label: 'DLRM: Deep Learning Recommendation Model (Facebook)'
- url: https://engineering.fb.com/2024/11/19/data-infrastructure/sequence-learning-personalized-ads-recommendations/
  label: 'Meta: Sequence Learning for Ads Recommendations (2024)'
cards:
- id: 53cbc4ee-6e8e-455f-8564-5780c04b909e
  type: flip
  front: What is the key difference between pointwise and listwise learning-to-rank loss functions?
  back: 'Pointwise: Treats each query-item pair independently, predicting relevance score. Simple but
    ignores ranking order.

    Listwise: Optimizes the entire ranking list jointly, directly optimizing ranking metrics like NDCG.
    More complex but better ranking quality. LambdaMART is a popular listwise approach.'
- id: 96e381e6-1e90-4c0f-85e0-632d2402f1cd
  type: mcq
  front: What is the core architectural innovation of DCN (Deep & Cross Network)?
  back: DCN's key innovation is the cross network, which explicitly models feature interactions via r_l
    = r_{l-1} ⊙ (w_l · r_0) + r_{l-1}. This is more efficient than dense MLPs for learning crossing patterns.
    The deep tower learns non-linear patterns. DCN-V2 improved expressiveness and made it more efficient
    for web-scale systems.
  choices:
  - key: a
    text: Using two separate towers for user and item embeddings
    correct: false
  - key: b
    text: Combining a deep MLP with an explicit cross network layer for feature interactions
    correct: false
  - key: c
    text: Replacing dense layers with embeddings
    correct: false
  - key: d
    text: Using attention mechanisms for feature weighting
    correct: false
- id: e624e018-b613-4afe-a5de-5b38592e3ff1
  type: flip
  front: In DLRM, why is the factorization machine (FM) layer important?
  back: 'FM layer computes pairwise feature interactions: ∑_i<j (e_i ⊙ e_j). This captures how sparse
    feature embeddings interact (e.g., user_id × item_id interaction). DLRM''s core hypothesis is that
    these interactions are paramount for recommendations, so FM-based interaction modeling is essential.'
- id: 79372b16-5835-4c99-88c2-89ae340b43a9
  type: mcq
  front: Why does position bias pose a challenge in learning-to-rank for recommender systems?
  back: 'Position bias: items ranked higher get more clicks due to visibility, not relevance. When training
    LTR on logged clicks, the model learns position bias as relevance signal. Mitigation: inverse propensity
    weighting (IPW), controlled experiments with randomized ranking, or causal inference approaches.'
  choices:
  - key: a
    text: Higher-ranked items have naturally better features
    correct: false
  - key: b
    text: Higher-ranked items receive more clicks regardless of relevance, biasing LTR training
    correct: false
  - key: c
    text: Lower-ranked items are always clicked less frequently
    correct: false
  - key: d
    text: Position bias only affects implicit feedback systems
    correct: false
- id: 173ffed2-ba97-49fe-96b1-ee397bcced51
  type: flip
  front: What major shift did Meta announce in 2024 for their recommendation system architecture?
  back: Meta moved from DLRM with manual feature engineering to sequence learning models. The new approach
    learns representations directly from engagement and conversion event sequences, replacing the traditional
    DLRM neural network architecture. This achieved 2-4% conversion improvement on select segments.
---

## Intuition

Learning-to-rank (LTR) optimizes the ranking order of items given a query/user context. Unlike binary classification (clicked/not-clicked), LTR directly optimizes ranking metrics: NDCG, MAP, MRR. Two main approaches: pointwise (predict individual scores) and listwise (optimize ranking order directly).

## Detail

**Three loss formulations:**

1. **Pointwise:** Treat each query-item pair independently. Predict relevance score or click probability.
   - Loss: Cross-entropy for binary (click/no-click) or MSE for relevance score
   - Pros: Simple, works with any classifier
   - Cons: Ignores ranking order; high-scored items at bottom still incur loss

2. **Pairwise:** Compare relative rankings of item pairs
   - Loss: BPR (Bayesian Personalized Ranking), margin-based ranking loss
   - Optimize: P(item_i ranked > item_j | clicked(i) AND not clicked(j))
   - Pros: More ranking-aware
   - Cons: O(n²) pair comparisons

3. **Listwise:** Optimize entire ranking list jointly
   - Loss: LambdaMART, LambdaRank (differentiable approximations of NDCG, MAP)
   - Optimize ranking metrics directly via gradient of rank loss
   - Pros: Directly optimizes ranking quality
   - Cons: Complex, less stable gradient signals

**Modern architectures:**

- **DCN (Deep & Cross Network):** Proposed by Google for web-scale LTR. Two parallel towers: deep MLP learns non-linear patterns; cross network explicitly models feature interactions via r_l = r_{l-1} ⊙ (w_l · r_0) + r_{l-1}. DCN-V2 improves expressiveness and training efficiency.

- **DLRM (Deep Learning Recommendation Model):** Facebook's architecture. Hypothesis: feature interactions are paramount. Sparse features → embeddings, dense features normalized, then factorization machine (FM) layer computes pairwise interactions: ∑_i<j (e_i ⊙ e_j), concatenated with raw embeddings into MLP for final score.

**In production (Meta 2024):**
Meta transitioned from manual feature engineering in DLRM to **sequence learning models** (replacing DLRM architecture). New approach: learn representations directly from engagement/conversion events in sequences. Achieved 2-4% conversion lift on select segments.

## Common gotchas / interview framings

- **Metric mismatch:** Optimizing cross-entropy (pointwise) ≠ optimizing NDCG (ranking quality). Especially problematic when tail items matter.
- **Position bias:** Higher-ranked items get more clicks regardless of relevance. LTR must use inverse propensity weighting or counterfactual approaches.
- **Feature engineering:** DLRM's original success relied on thousands of manually engineered features. Now shifting toward learned representations.
- **Training-serving skew:** LTR models trained on logged interactions (with position bias) but deployed ranking fresh items with no logged clicks.
- **Computational cost:** DCN-V2 and DLRM are expressive but require careful optimization at scale (distributed training, batching).
- **Scalability of interaction learning:** Factorization machine component O(kn) where k is embedding dim, n is number of features.

## See also
- [[learning-to-rank]]
- [[dcn-deep-cross-network]]
- [[dlrm-deep-learning-recommendation]]
- [[ctr-prediction]]
- [[feature-interaction]]
- [[loss-functions]]
- [[lambdamart]]
- [[gradient-boosting]]

## Sources
See frontmatter `sources:`.

---
id: 3a19ef46-aaea-495b-81ac-401459271b0c
title: K-Nearest Neighbors (KNN)
track: data-scientist
topic: classical-machine-learning
difficulty: 1
tags:
- classification
- regression
- instance-based-learning
- lazy-learning
aliases:
- instance-based
- memory-based
- distance-weighted KNN
sources:
- url: https://scikit-learn.org/stable/modules/neighbors.html
  label: scikit-learn Nearest Neighbors
- url: https://www.coursera.org/learn/machine-learning
  label: Andrew Ng ML Specialization
cards:
- id: 3af4c01b-de6f-4f82-8b4a-09439b983703
  type: flip
  front: KNN algorithm. Why 'lazy'?
  back: Stores data, defers computation to prediction. No model fitting during training.
- id: c5fd94f0-c9b7-4c8f-930e-3fae1e95197a
  type: flip
  front: $k$ affects bias-variance?
  back: 'Low $k$: memorization (low bias, high variance). High $k$: smooth boundary (high bias, low variance).'
- id: 20a06d5f-8d5a-4a70-b2d4-3105510cdb7b
  type: flip
  front: Curse of dimensionality in KNN?
  back: 'High-dim: points nearly equidistant, distance meaningless. Volume scales as $r^d$; neighborhood
    tiny. Solution: PCA first.'
- id: bf9b5647-665f-4da9-be37-9686b0ce0c6c
  type: flip
  front: Why scale features before KNN?
  back: 'Distance-based. Unscaled: large-range features dominate. Standardize.'
---

## Intuition
Simplest: store data, predict via majority vote / average of $k$ nearest. No training; slow prediction. Sensitive to scaling and curse of dimensionality.

## Detail
**Algorithm:** Store training data. Query: compute distance to all points, find $k$ nearest, vote/average. **$k$ effect:** Low=high variance, high=high bias. **Curse:** High-dim: all equidistant.

## See also
- [[curse-of-dimensionality]]
- [[distance-metrics]]
- [[feature-scaling]]

## Sources
See frontmatter `sources:`.

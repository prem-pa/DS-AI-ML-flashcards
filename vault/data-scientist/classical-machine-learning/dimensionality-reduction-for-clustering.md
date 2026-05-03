---
id: 209523d9-1167-48ec-9aff-a0202289633c
title: Dimensionality reduction for clustering
track: data-scientist
topic: classical-machine-learning
difficulty: 3
tags:
- clustering
- unsupervised-learning
- curse-of-dimensionality
- preprocessing
aliases:
- curse of dimensionality
- high-dimensional clustering
- feature reduction
sources:
- url: https://scikit-learn.org/stable/modules/clustering.html
  label: scikit-learn Clustering
- url: https://www.coursera.org/learn/machine-learning
  label: Andrew Ng ML Specialization
cards:
- id: e1908d92-7c04-4bcb-9133-6ae57084b9e6
  type: flip
  front: Curse dimensionality in clustering?
  back: 'High-dim: all nearly equidistant, density meaningless. Reduce via PCA/selection to restore structure.'
- id: b1ebb78e-99e9-4100-8b30-aec2f139f72b
  type: flip
  front: Apply dimensionality reduction? When skip?
  back: Always if $d$ large (>100) or quality poor. Skip if $d$ small, features all relevant, or interpretability
    critical.
- id: 95089caf-d4d0-4009-b3d1-6e99b48931b5
  type: flip
  front: PCA vs feature selection pre-clustering?
  back: 'PCA: fast, retains variance, uninterpretable. Selection: interpretable, slower. For clustering:
    PCA often sufficient.'
- id: 07ad58c0-3dea-4112-a863-1ec337f382bc
  type: flip
  front: Why NOT use t-SNE for clustering input?
  back: t-SNE distorts distances (prioritizes visualization). Use for viz post-clustering only, not input.
---

## Intuition
High-dim: all equidistant, density meaningless. Reduce via PCA/feature selection to restore local structure.

## Detail
**Curse:** Volume $\propto r^d$; in 100D, data sparse. **Solutions:** PCA (fast, variance), feature selection (interpretable), nonlinear (t-SNE: viz only).

## See also
- [[curse-of-dimensionality]]
- [[pca]]
- [[feature-selection]]

## Sources
See frontmatter `sources:`.

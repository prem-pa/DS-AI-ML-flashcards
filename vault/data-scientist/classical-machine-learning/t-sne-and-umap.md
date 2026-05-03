---
id: fbc51fdc-c257-47bc-9301-ef02d29b29cc
title: t-SNE and UMAP
track: data-scientist
topic: classical-machine-learning
difficulty: 3
tags:
- dimensionality-reduction
- visualization
- manifold-learning
- nonlinear-reduction
aliases:
- manifold learning
- nonlinear reduction
- t-distributed stochastic neighbor embedding
sources:
- url: https://scikit-learn.org/stable/modules/manifold.html
  label: scikit-learn Manifold Learning
- url: https://www.coursera.org/learn/machine-learning
  label: Andrew Ng ML Specialization
cards:
- id: 5ad973d1-60ab-4821-9ec1-f85b9559a536
  type: flip
  front: t-SNE mechanism. Preserve local?
  back: Converts high-dim distances to probabilities (Gaussian), low-dim (Student-t). Minimizes KL divergence.
    Preserves k-NN (local) but distorts global.
- id: 1beba47c-e580-44db-a338-00e624e8bc5a
  type: flip
  front: UMAP vs t-SNE?
  back: 'UMAP: faster, better global structure (distances more meaningful). Both excellent for visualization.
    UMAP preferred if speed/global matter.'
- id: 7b250c4a-faa6-46c2-8e9a-3d73e568e92e
  type: flip
  front: Why NOT cluster on t-SNE/UMAP output?
  back: Both distort distances. Clustering detects visualization artifacts, not true clusters. Cluster
    on original/PCA; viz after.
- id: 98a3684a-8264-4169-ae47-227762d9e9a6
  type: flip
  front: 'Hyperparameters: t-SNE, UMAP?'
  back: 't-SNE: perplexity (5-50, effective neighbors). UMAP: n_neighbors (k-NN), min_dist (spacing).
    Tune interactively.'
---

## Intuition
Nonlinear reduction for visualization. Preserve local structure (k-NN). t-SNE distorts global; UMAP better global. Both slow, non-deterministic.

## Detail
**t-SNE:** Convert distances to neighborhood probabilities (Gaussian→Student-t), minimize KL divergence. **UMAP:** Fuzzy topological metric, faster, better global.

## See also
- [[manifold-learning]]
- [[local-vs-global-structure]]
- [[visualization]]

## Sources
See frontmatter `sources:`.

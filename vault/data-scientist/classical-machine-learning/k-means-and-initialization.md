---
id: 5ddffdff-bc86-4c7c-a0a1-c4b5ea36a440
title: K-means and initialization
track: data-scientist
topic: classical-machine-learning
difficulty: 3
tags:
- clustering
- unsupervised-learning
- centroid-based
- local-optima
aliases:
- k-means++
- Lloyd's algorithm
- within-cluster variance
sources:
- url: https://scikit-learn.org/stable/modules/clustering.html#k-means
  label: scikit-learn K-means
- url: https://www.coursera.org/learn/machine-learning
  label: Andrew Ng ML Specialization
cards:
- id: 3875dd07-924e-4b9d-853f-eb63bc45198e
  type: flip
  front: K-means objective. Why 'distortion'?
  back: $J=\sum||x_i-\mu_{c(i)}||^2$. Spread of clusters. Lower=tighter.
- id: 0be59d17-9992-49ca-9d7c-dd77c962e5d2
  type: flip
  front: Lloyd's algorithm. Global optimum guaranteed?
  back: 'Iterate: assign, update. $J$ non-increasing; convergence to local (not global) minimum. Good
    init via k-means++ improves.'
- id: b2420a98-c8a8-4546-ac93-ad6187595ad1
  type: flip
  front: k-means++ init. Why better?
  back: Pick first random. Then probability $\propto D^2$ (distance squared to nearest). Spreads seeds.
    Avoids bad clustering.
- id: 3dc00f05-f628-4bff-8f83-6f7390139133
  type: flip
  front: Choose $k$. Elbow limitations?
  back: 'Elbow method: plot distortion vs $k$, look for kink. Subjective. Silhouette score: objective.
    Domain knowledge.'
---

## Intuition
Partition $k$ clusters by minimizing within-cluster variance. Iterate: assign to nearest centroid, update centroid as mean. Sensitive to init; k-means++ spreads seeds apart.

## Detail
**Objective:** $J=\sum||x_i-\mu_{c(i)}||^2$. **Lloyd:** (1) assign, (2) update. **k-means++:** Choose first random, then subsequent with probability $\propto D^2$ (distance to nearest).

## See also
- [[clustering]]
- [[expectation-maximization]]
- [[silhouette-score]]

## Sources
See frontmatter `sources:`.

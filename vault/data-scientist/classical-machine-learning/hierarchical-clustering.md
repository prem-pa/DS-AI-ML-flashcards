---
id: dba8b05b-4b5a-4e8b-b041-121d8606c383
title: Hierarchical clustering
track: data-scientist
topic: classical-machine-learning
difficulty: 3
tags:
- clustering
- unsupervised-learning
- dendrogram
- agglomerative
aliases:
- agglomerative clustering
- dendrogram
- linkage methods
sources:
- url: https://scikit-learn.org/stable/modules/clustering.html#hierarchical-clustering
  label: scikit-learn Hierarchical Clustering
- url: https://www.coursera.org/learn/machine-learning
  label: Andrew Ng ML Specialization
cards:
- id: 604ce289-236d-407d-9817-c3921c41eb11
  type: flip
  front: Agglomerative clustering. Dendrogram?
  back: 'Bottom-up: start each point, merge closest. Dendrogram: tree (leaves=points, nodes=merges, height=distance).
    Cut at height for $k$.'
- id: 077c9013-6f7e-4617-a55d-f9daa26e2319
  type: flip
  front: Four linkage criteria. Which 'chaining'?
  back: 'Single (min): chaining. Complete (max): compact. Average (mean): balanced. Ward (variance): k-means-like.'
- id: 27a153f2-e15e-42c3-9082-4c70796b1772
  type: flip
  front: Hierarchical computational cost?
  back: 'Naive: $O(n^3)$ (distances $O(n^2)$, updates). Space: $O(n^2)$ (distance matrix). Impractical
    for large $n$.'
- id: 685296ca-95b0-4f2e-bdc8-2ef7a68dc3c1
  type: flip
  front: Choose $k$ from dendrogram?
  back: Cut at desired height. Look for gap (elbow). Or use silhouette.
---

## Intuition
Bottom-up: each point starts as cluster, iteratively merge nearest pairs. Dendrogram visualizes merges; cut at height for $k$ clusters.

## Detail
**Algorithm:** (1) Compute distances. (2) Merge closest pair (by linkage). (3) Update distances. Repeat. **Linkage:** Single (min), complete (max), average (mean), Ward (variance).

## See also
- [[dendrograms]]
- [[linkage-criteria]]
- [[distance-matrix]]

## Sources
See frontmatter `sources:`.

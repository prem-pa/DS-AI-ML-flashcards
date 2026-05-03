---
id: 4891ff1e-ff9a-44c6-879b-bdd2b68621a5
title: DBSCAN
track: data-scientist
topic: classical-machine-learning
difficulty: 3
tags:
- clustering
- density-based
- non-convex-clusters
- outlier-detection
aliases:
- density-based spatial clustering
- eps-neighborhood
- core points
sources:
- url: https://scikit-learn.org/stable/modules/clustering.html#dbscan
  label: scikit-learn DBSCAN
- url: https://www.coursera.org/learn/machine-learning
  label: Andrew Ng ML Specialization
cards:
- id: 4891e3bc-3ccd-4b04-bde8-bfdf1e5cba17
  type: flip
  front: Core, border, noise points?
  back: 'Core: enough neighbors. Border: near core, not core. Noise: isolated. Clusters: connected core
    points.'
- id: 8a963144-4509-4ba2-8ab6-4fb17de22157
  type: flip
  front: Density-reachability? Non-convex clusters?
  back: 'Reachable: chain of core points. Forms arbitrary shapes via density chains, not spherical.'
- id: 506a49c8-7b07-431b-96cb-808df2721077
  type: flip
  front: Choose eps, min_samples?
  back: 'eps: k-distance graph, plot sorted distances, elbow = threshold. min_samples: typically $d+1$
    or $2d$, default 5.'
- id: 746217bd-00ab-4a72-8173-36f5f9e2e18c
  type: flip
  front: Varying density limitation?
  back: Single eps doesn't capture different densities. One cluster sparse (labeled noise), another dense
    (split). Use HDBSCAN.
---

## Intuition
Density-based. Identifies dense regions, marks isolated as noise. Non-convex clusters, auto $k$. Two params: eps (radius), min_samples (neighbors).

## Detail
**Core point:** $|N_\epsilon(x)| \geq$ min_samples. **Border point:** near core but not core. **Noise:** neither. **Algorithm:** For each core, grow cluster via density-reachability.

## See also
- [[density-based-clustering]]
- [[outliers]]
- [[eps-parameter]]

## Sources
See frontmatter `sources:`.

---
id: 60b8a516-08ef-4aeb-b400-fcbffc434eea
title: Variance explained and scree plots
track: data-scientist
topic: classical-machine-learning
difficulty: 1
tags:
- dimensionality-reduction
- pca
- model-selection
- interpretability
aliases:
- cumulative variance
- elbow method
- scree plot
sources:
- url: https://scikit-learn.org/stable/modules/decomposition.html#pca
  label: scikit-learn PCA
- url: https://www.coursera.org/learn/machine-learning
  label: Andrew Ng ML Specialization
cards:
- id: 118cee49-8af4-49a5-ba6a-28578890c044
  type: flip
  front: Variance explained per component, cumulative?
  back: 'Component $i$: $\lambda_i / \sum \lambda_j$. Cumulative $k$: sum first $k$. Choose $k$ at desired
    threshold (e.g., 0.95).'
- id: dacdfafd-274d-4570-9a2c-9c73f067e575
  type: flip
  front: Scree plot utility. Choose $k$?
  back: Plot $\lambda_i$ vs $i$. Look for elbow (point where level off). Components before=signal, after=noise.
- id: df820b87-ed7a-41a0-865a-a2832d7341ee
  type: flip
  front: Cumulative variance threshold method?
  back: Plot cumulative variance vs $k$. Choose $k$ at threshold (e.g., 95%). Objective vs elbow.
- id: f2296593-5a59-46fc-9a3e-af23013ff173
  type: mcq
  front: Last eigenvalues spuriously large (small $n$)?
  back: 'Small $n$: sampling variance spuriously inflates tail. Use CV or permutation to validate $k$.'
  choices:
  - key: a
    text: PCA inflates.
    correct: false
  - key: b
    text: Random noise in tail.
    correct: true
  - key: c
    text: Unimportant features.
    correct: false
  - key: d
    text: No issue.
    correct: false
---

## Intuition
Variance explained: information retained per component. Scree plot: eigenvalues vs component. Elbow suggests number of components. Heuristic: retain ~95% variance.

## Detail
**Variance $i$:** $\lambda_i / \sum \lambda_j$. **Cumulative $k$:** $(\sum_{i=1}^k \lambda_i) / \sum \lambda_j$. **Scree plot:** Look for elbow. **Threshold:** Choose $k$ reaching 95%.

## See also
- [[pca]]
- [[eigenvalues]]
- [[dimensionality-reduction]]

## Sources
See frontmatter `sources:`.

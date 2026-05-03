---
id: 6bc33ca8-4c3e-45d5-8967-9ea6cbfa8a22
title: Principal Component Analysis (PCA)
track: data-scientist
topic: classical-machine-learning
difficulty: 3
tags:
- dimensionality-reduction
- unsupervised-learning
- variance-maximization
- orthogonal
aliases:
- principal components
- eigendecomposition
- variance-preserving reduction
sources:
- url: https://scikit-learn.org/stable/modules/decomposition.html#pca
  label: scikit-learn PCA
- url: https://www.coursera.org/learn/machine-learning
  label: Andrew Ng ML Specialization
cards:
- id: 9fa41f25-574b-463d-862e-fcc155bcc403
  type: flip
  front: PCA objective. Principal components?
  back: 'Find orthogonal directions maximizing variance. Solution: eigenvectors of covariance matrix,
    ordered by eigenvalues.'
- id: 56da0692-12c3-4c59-a1c7-f7a80127147a
  type: flip
  front: Compute PCA. Eigendecomposition steps?
  back: 'Standardize. Covariance $\Sigma$. Eigendecomposition: $\Sigma w_i = \lambda_i w_i$. Project:
    $X_{proj} = X w_{1:k}$.'
- id: 777a0f63-d253-46cb-af5e-e72f2eba3181
  type: flip
  front: Choose $k$ components. Elbow method?
  back: Plot cumulative variance vs $k$. Elbow = threshold. Alternatively, threshold (e.g., 95%).
- id: a33507be-275d-4630-96a0-5820f1f04b05
  type: flip
  front: Why standardize before PCA?
  back: 'PCA maximizes variance, scale-dependent. Unstandardized: large-range features dominate. Standardize
    for equal contribution.'
---

## Intuition
Find orthogonal directions maximizing variance. 1st component: max variance. 2nd: max remaining (orthogonal to 1st). Useful for visualization, noise reduction, speedup.

## Detail
**Solution:** Eigendecomposition $\Sigma w = \lambda w$ (covariance). Eigenvectors (ordered by $\lambda$): principal components. **Variance explained:** $\lambda_i / \sum_j \lambda_j$.

## See also
- [[eigenvalues]]
- [[covariance-matrix]]
- [[scree-plot]]
- [[variance-explained]]

## Sources
See frontmatter `sources:`.

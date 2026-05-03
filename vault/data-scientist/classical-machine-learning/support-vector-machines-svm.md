---
id: c1ef6833-ba02-41f2-8fd9-f8dcc505b227
title: Support Vector Machines (SVM)
track: data-scientist
topic: classical-machine-learning
difficulty: 3
tags:
- classification
- max-margin
- kernel-trick
- nonlinear-boundaries
aliases:
- kernel SVM
- max-margin classifier
- support vectors
sources:
- url: https://scikit-learn.org/stable/modules/svm.html
  label: scikit-learn SVM
- url: https://www.coursera.org/learn/machine-learning
  label: Andrew Ng ML Specialization
cards:
- id: 1ef880b3-6187-4685-b1d1-6f8d43018e03
  type: flip
  front: Margin definition. Why maximize?
  back: 'Margin=2/||w||. Max margin: buffer zone robust to perturbations, simpler boundary, better generalization.'
- id: 4fae58b0-b9e8-4414-9265-f9b9f7bcae62
  type: flip
  front: Soft margin $C$ control?
  back: 'High $C$: strict margin (overfit). Low $C$: soft (underfit). Tradeoff.'
- id: bae68165-582f-48cb-803e-119c795c25df
  type: flip
  front: Kernel trick. Why nonlinearity without explicit mapping?
  back: '$K(x,x'')=\phi(x)^T\phi(x'')$ computes inner product in high-dim without forming $\phi$. RBF:
    infinite-dim, cheap $K$.'
- id: c2f7190c-989d-467c-82ab-c99430430bcb
  type: flip
  front: Three kernels. When each?
  back: 'Linear: high-dim or separable (fast). Polynomial: moderate nonlinearity. RBF: low-dim nonlinear
    (flexible).'
---

## Intuition
SVM maximizes margin (distance to nearest points). Kernel trick enables nonlinearity without high-dim mapping.

## Detail
**Soft margin:** $\min \frac{1}{2}||w||^2 + C\sum\xi_i$ s.t. $y_i(w^Tx_i+b) \geq 1-\xi_i$. **Kernel:** $K(x,x')=\phi(x)^T\phi(x')$. **Decision:** $\hat{y}=\text{sign}(\sum\alpha_i y_i K(x_i,x)+b)$.

## See also
- [[kernel-trick]]
- [[max-margin]]
- [[hinge-loss]]
- [[dual-formulation]]

## Sources
See frontmatter `sources:`.

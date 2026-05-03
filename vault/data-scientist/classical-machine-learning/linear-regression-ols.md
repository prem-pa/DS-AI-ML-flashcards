---
id: f5864e5a-6580-4130-a1ad-17080ac97062
title: Linear regression (OLS)
track: data-scientist
topic: classical-machine-learning
difficulty: 1
tags:
- regression
- supervised-learning
- statistical-inference
- baseline
aliases:
- ordinary-least-squares
- linear-model
- OLS
sources:
- url: https://scikit-learn.org/stable/modules/linear_model.html
  label: scikit-learn Linear Models
- url: https://www.coursera.org/learn/machine-learning
  label: Andrew Ng ML Specialization
- url: https://www.deeplearning.ai/courses/machine-learning-specialization/
  label: DeepLearning.AI ML
cards:
- id: ff8848a5-afbe-4a02-940c-f8a587a1736f
  type: flip
  front: Derive the closed-form OLS solution from the loss function.
  back: 'Minimize $||y - X\beta||_2^2$. Gradient: $\frac{\partial}{\partial\beta}(y - X\beta)^T(y - X\beta)
    = -2X^T(y - X\beta) = 0$. Solve: $\hat{\beta} = (X^T X)^{-1} X^T y$. Requires $X^T X$ invertible.'
- id: 703896f4-8f8a-47a5-8519-eb31a7846bf9
  type: flip
  front: Why is OLS equivalent to MLE under Gaussian noise?
  back: If $y = X\beta + \epsilon$ with $\epsilon \sim N(0, \sigma^2)$, then $\log p(y|X,\beta) = -\frac{1}{2\sigma^2}||y
    - X\beta||_2^2 + \text{const}$. Maximizing likelihood ≡ minimizing MSE. Justifies OLS from probabilistic
    perspective.
- id: 67dc9097-8b2c-4608-b5d5-625199ec9a5a
  type: mcq
  front: When $X$ has multicollinearity, what happens to $(X^T X)^{-1}$?
  back: Multicollinearity makes $X^T X$ poorly conditioned, leading to unstable $\hat{\beta}$ with inflated
    variances. Ridge regression (adding $\lambda I$) regularizes.
  choices:
  - key: a
    text: Stable.
    correct: false
  - key: b
    text: Singular or near-singular, causing large coefficient variance.
    correct: true
  - key: c
    text: Auto-adjusts.
    correct: false
  - key: d
    text: Intercept absorbs it.
    correct: false
- id: fd15a31c-00c6-433a-bad1-075a0f0fda57
  type: mcq
  front: Is OLS high bias or high variance in bias-variance tradeoff?
  back: 'OLS: few features → high bias; many features → high variance. Best understood via learning curves.'
  choices:
  - key: a
    text: High bias.
    correct: false
  - key: b
    text: High variance (small $n$).
    correct: false
  - key: c
    text: Low both (ideal).
    correct: false
  - key: d
    text: Depends entirely on number of features.
    correct: true
---

## Intuition
Linear regression finds the best-fit line through data by minimizing the sum of squared residuals. It's the workhorse baseline for supervised learning.

## Detail
Closed-form solution: $\hat{\beta} = (X^T X)^{-1} X^T y$
$$\text{minimize } ||y - X\beta||_2^2$$
Unbiased estimator under classical assumptions. Cost: $O(nd^2)$.

**Mean Squared Error as MLE:** If $\epsilon \sim N(0, \sigma^2)$:
$$\mathcal{L}(\beta) \propto \exp\left(-\frac{1}{2\sigma^2}||y - X\beta||_2^2\right)$$

## Common gotchas
- **Singularity:** Multicollinearity → $(X^T X)^{-1}$ undefined. Ridge regression adds $\lambda I$.
- **Extrapolation:** Unreliable outside training data range.
- **Assumption violations:** Use diagnostics to detect.

## See also
- [[bias-variance-tradeoff]]
- [[mse-loss]]
- [[normal-distribution]]
- [[residuals]]

## Sources
See frontmatter `sources:`.

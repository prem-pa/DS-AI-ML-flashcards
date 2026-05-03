---
id: 8c55fb35-21c5-4662-bc73-bf4b3c74bd3d
title: Regularization (Ridge, Lasso, Elastic Net)
track: data-scientist
topic: classical-machine-learning
difficulty: 3
tags:
- regression
- regularization
- overfitting
- hyperparameter-tuning
aliases:
- L1/L2 penalty
- weight decay
- shrinkage methods
sources:
- url: https://scikit-learn.org/stable/modules/linear_model.html
  label: scikit-learn Linear Models
- url: https://www.coursera.org/learn/machine-learning
  label: Andrew Ng ML Specialization
cards:
- id: e6721afd-0878-4120-ae27-15c473cba445
  type: flip
  front: Write Ridge, Lasso, Elastic Net penalties. Conceptual differences?
  back: 'Ridge: $\lambda||\beta||_2^2$ (shrinks smoothly). Lasso: $\lambda||\beta||_1$ (zeros out). Elastic:
    both. Ridge never zeros; Lasso sparse; Elastic balances.'
- id: ab77688e-ba40-4bd2-855b-19811e7bc0f0
  type: flip
  front: Derive Ridge closed-form solution. How handles multicollinearity?
  back: $\hat{\beta}_{ridge} = (X^T X + \lambda I)^{-1} X^T y$. Adding $\lambda I$ ensures invertibility
    even if $X^T X$ singular. Stabilizes, reduces variance at cost of bias.
- id: bb884b73-b0ac-4b0b-9658-88de59d4dcbe
  type: flip
  front: Why Lasso sparse, Ridge not?
  back: $L_1$ has corner at zero (subgradient condition), reachable exactly. $L_2$ smooth (gradient never
    zero), only approaches zero asymptotically.
- id: 57ab3b61-8b7c-4a87-99ed-3c5d8d202edc
  type: flip
  front: When use Elastic Net over Ridge/Lasso?
  back: 'When: (1) features highly correlated, (2) want sparsity + stability, (3) Lasso unstable across
    CV. Default $\alpha=0.5$ (50-50 L1/L2).'
---

## Intuition
Regularization penalizes large coefficients to reduce overfitting. Ridge (L2) shrinks; Lasso (L1) zeros out; Elastic Net blends both.

## Detail
**Ridge:** $||y - X\beta||_2^2 + \lambda||\beta||_2^2$ → $\hat{\beta} = (X^T X + \lambda I)^{-1} X^T y$
**Lasso:** $||y - X\beta||_2^2 + \lambda||\beta||_1$ → sparse (zeros coefficients)
**Elastic Net:** $||y - X\beta||_2^2 + \lambda_1||\beta||_1 + \lambda_2||\beta||_2^2$ → balances both

## See also
- [[ridge-regression]]
- [[lasso-regression]]
- [[elastic-net]]
- [[feature-selection]]

## Sources
See frontmatter `sources:`.

---
id: d75f158d-f199-4f12-a62f-8149590c1f1a
title: Covariance and correlation
track: data-scientist
topic: foundational-statistics-probability
difficulty: 3
tags:
- association
- linear-relationship
- feature-engineering
- dimensionality-reduction
- multicollinearity
aliases:
- joint-variation
- linear-dependence
- association-strength
sources:
- url: https://www.statlearning.com/
  label: James et al. An Introduction to Statistical Learning
- url: https://link.springer.com/book/10.1007/978-0-387-21736-9
  label: Wasserman, L. All of Statistics (Ch. 5)
cards:
- id: 6e6fce2d-a614-49ba-80c3-42a190407990
  type: flip
  front: Prove that correlation coefficient ρ ∈ [-1, 1] using Cauchy-Schwarz inequality.
  back: ''
- id: 8156ae36-be2b-46ed-a41a-ed52c9506cc2
  type: flip
  front: Two features have Pearson r = 0.92. How does this affect linear regression and what remedies
    exist?
  back: ''
- id: d86a1056-5b37-413c-983d-feea8a7b8d44
  type: flip
  front: You compute r = 0 between X and Y. Can you conclude they are independent?
  back: ''
- id: 54158b53-c7d1-4852-ab97-29f93b4db14f
  type: flip
  front: 'Explain how Simpson''s Paradox can occur in correlation: r is positive overall but negative
    within subgroups.'
  back: ''
- id: dada2e47-caf9-4ca8-a3ca-ff41ccaffe64
  type: mcq
  front: Which correlation is robust to outliers?
  back: ''
  choices:
  - key: a
    text: Pearson
    correct: false
  - key: b
    text: Spearman rank
    correct: false
  - key: c
    text: Kendall tau
    correct: false
  - key: d
    text: Both B and C
    correct: true
---

## Intuition
Covariance measures how two variables move together (units: product of variable units). Correlation standardizes this to [-1, 1], making it scale-free and interpretable: +1 = perfect positive, -1 = perfect negative, 0 = independent (for linear relationships). In ML, high correlation signals redundant features; high covariance between features and target signals predictive power.

## Detail
Covariance: $\text{Cov}(X, Y) = E[(X - \mu_X)(Y - \mu_Y)] = E[XY] - E[X]E[Y]$. Pearson correlation: $\rho = \frac{\text{Cov}(X,Y)}{\sigma_X \sigma_Y}$. Sample versions use $n-1$ divisors: $r = \frac{\sum(x_i - \bar{x})(y_i - \bar{y})}{\sqrt{\sum(x_i - \bar{x})^2 \sum(y_i - \bar{y})^2}}$.

Key properties: Cov is bilinear (linear in each argument); Var(X) = Cov(X, X); Cov(aX, bY) = ab Cov(X,Y). Correlation only captures linear dependence—two variables can be strongly nonlinearly related (e.g., Y = X²) yet have ρ ≈ 0. High multicollinearity ($|r| > 0.9$) inflates regression standard errors and destabilizes coefficients.

## Common gotchas / interview framings
- Correlation ≠ causation; confounding variables can induce spurious correlation
- Correlation is zero does not imply independence (e.g., $(X, X^2)$ are dependent but uncorrelated if X ~ symmetric)
- Outliers dramatically affect sample correlation; use Spearman's rank correlation for robustness
- High feature correlation requires feature selection or regularization (Lasso, Ridge) to stabilize estimates

## See also
- [[covariance]]
- [[pearson-correlation]]
- [[multicollinearity]]
- [[feature-selection]]
- [[causation]]

## Sources
See frontmatter `sources:`.

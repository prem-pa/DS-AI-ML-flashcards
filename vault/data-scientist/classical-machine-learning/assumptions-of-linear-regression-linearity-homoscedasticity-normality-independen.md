---
id: 497aa191-2359-4889-87de-68e2266e5f01
title: Assumptions of linear regression (linearity, homoscedasticity, normality, independence)
track: data-scientist
topic: classical-machine-learning
difficulty: 3
tags:
- regression
- diagnostics
- statistical-assumptions
- inference
aliases:
- Gauss-Markov assumptions
- LINO
- OLS assumptions
sources:
- url: https://scikit-learn.org/stable/modules/linear_model.html
  label: scikit-learn Linear Models
- url: https://www.coursera.org/learn/machine-learning
  label: Andrew Ng ML Specialization
cards:
- id: eb22e705-a6f1-45c0-b14d-549eaf50f1e2
  type: flip
  front: What are LINO assumptions and which ones are needed for inference vs prediction?
  back: 'LINO: Linearity, Independence, Normality, homOscedasticity. For prediction: linearity + independence.
    For p-values/CIs: all four. Non-normality forgivable for large $n$ (CLT).'
- id: 32aaa393-ba16-47a0-a5f7-13a3180101ca
  type: flip
  front: State Gauss-Markov Theorem. What does it guarantee?
  back: Under linearity, independence, homoscedasticity, OLS is BLUE (Best Linear Unbiased Estimator—lowest
    variance among linear unbiased). Normality not required. Ridge adds bias but can reduce total error.
- id: e4cb4f4d-131d-4f9d-a812-54153cb56b8b
  type: flip
  front: Residual plot shows funnel shape. What violation? How to fix?
  back: 'Heteroscedasticity: variance grows with fitted values. Fixes: (1) transform $y$ (log), (2) weighted
    least squares, (3) White''s standard errors, (4) regularization.'
- id: 1ba1ada6-5d64-469b-bf3d-7dd821ddc3d2
  type: mcq
  front: Time-series violates independence. Consequence?
  back: Serial correlation leaves $\hat{\beta}$ unbiased but inflates $\text{Var}(\hat{\beta})$. SEs too
    small. Use Newey-West or ARIMA.
  choices:
  - key: a
    text: Coefficients biased.
    correct: false
  - key: b
    text: Coefficients unbiased, SEs wrong (underestimated).
    correct: true
  - key: c
    text: Model unusable.
    correct: false
  - key: d
    text: Intercept only affected.
    correct: false
---

## Intuition
OLS has nice properties (unbiased, minimum variance) only under four LINO assumptions. Violations invalidate inference even if prediction works.

## Detail
**LINO:** Linearity, Independence, Normality, homOscedasticity.
1. **Linearity:** $E[y|X] = X\beta$.
2. **Independence:** $\text{Cov}(\epsilon_i, \epsilon_j) = 0$ for $i \neq j$.
3. **Normality:** $\epsilon \sim N(0, \sigma^2)$.
4. **Homoscedasticity:** $\text{Var}(\epsilon|X) = \sigma^2$ (constant).

**Gauss-Markov:** Under linearity, independence, homoscedasticity (no normality), OLS is BLUE.

**Diagnostics:** Residual plot, Q-Q plot, Durbin-Watson test, scale-location plot.

## See also
- [[residual-plots]]
- [[q-q-plots]]
- [[heteroscedasticity]]
- [[autocorrelation]]

## Sources
See frontmatter `sources:`.

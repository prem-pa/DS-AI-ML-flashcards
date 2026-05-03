---
id: eaca939a-7cf8-49a8-b6d2-fa22df0aa86c
title: Measures of spread (variance, std-dev, IQR)
track: data-scientist
topic: foundational-statistics-probability
difficulty: 1
tags:
- descriptive-statistics
- dispersion
- variability
- outlier-detection
- scale-parameter
aliases:
- dispersion
- scale
- spread-metrics
sources:
- url: https://ocw.mit.edu/courses/18-05-introduction-to-probability-and-statistics-spring-2022/
  label: 'MIT 18.05: Introduction to Probability and Statistics'
- url: https://link.springer.com/book/10.1007/978-0-387-21736-9
  label: Wasserman, L. All of Statistics (Ch. 2)
cards:
- id: be7cbf1a-bdd2-4cb7-964f-15379b2df5f0
  type: flip
  front: Show that Var(X) = E[X²] - (E[X])².
  back: ''
- id: 602b2e5f-3708-480f-a8fb-0ff25f0b66d8
  type: flip
  front: Why do we divide by n-1 instead of n when computing sample variance?
  back: ''
- id: 06d5ee2d-d0d3-4bbb-9a5b-0e32085c1e6b
  type: flip
  front: 'You measure two datasets: A has σ = 5, B has σ = 20. What inference challenge might Dataset
    B present?'
  back: ''
- id: c021bcaa-72d4-4bb8-83f3-950f5b54441d
  type: flip
  front: Your regression model shows constant variance in residuals up to X=100, then doubles. What problem
    is this?
  back: ''
- id: 6ab7e6be-7d4b-4a57-a698-979884c9e904
  type: mcq
  front: Which spread measure is unaffected by extreme outliers?
  back: ''
  choices:
  - key: a
    text: Variance
    correct: false
  - key: b
    text: Standard deviation
    correct: false
  - key: c
    text: IQR
    correct: true
  - key: d
    text: Range
    correct: false
---

## Intuition
Variance and standard deviation measure how spread out data is around the mean. The IQR (interquartile range) measures spread using percentiles, making it robust to outliers. In ML, dispersion controls regularization strength, affects confidence interval width, and determines when you need stratified sampling or heteroscedasticity adjustments.

## Detail
Variance is $\sigma^2 = E[(X - \mu)^2] = E[X^2] - (E[X])^2$; the second form is useful computationally. Standard deviation $\sigma = \sqrt{\sigma^2}$ scales back to original units. The sample estimate $s^2 = \frac{1}{n-1}\sum_{i=1}^n(x_i - \bar{x})^2$ divides by $n-1$ (Bessel correction) to be unbiased.

The IQR = Q₃ - Q₁ (75th percentile minus 25th) contains the middle 50% and is unaffected by values beyond 1.5 × IQR (standard outlier cutoff). Low variance ($\sigma^2 \to 0$) means tight clustering; high variance signals dispersed data requiring larger sample sizes for inference.

## Common gotchas / interview framings
- Variance increases with squared units (hard to interpret); use std-dev for comparability
- Sample variance divides by $n-1$, not $n$, to correct for estimating the mean (loss of 1 degree of freedom)
- IQR can hide bimodality: check both dispersion metrics and the actual distribution shape
- Heteroscedasticity (variance changing across the domain) violates OLS assumptions and inflates type-I error rates

## See also
- [[variance]]
- [[standard-deviation]]
- [[iqr]]
- [[quartiles]]
- [[outlier-detection]]

## Sources
See frontmatter `sources:`.

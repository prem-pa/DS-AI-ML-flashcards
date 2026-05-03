---
id: 67c60a14-e9b4-4777-a007-bcb1a01ee113
title: Normal distribution and its properties
track: data-scientist
topic: foundational-statistics-probability
difficulty: 1
tags:
- gaussian
- probability-distribution
- inference-foundation
- regression-assumption
- bell-curve
aliases:
- gaussian-distribution
- gaussian-curve
- standard-normal
sources:
- url: https://ocw.mit.edu/courses/18-05-introduction-to-probability-and-statistics-spring-2022/
  label: 'MIT 18.05: Introduction to Probability and Statistics'
- url: https://link.springer.com/book/10.1007/978-0-387-21736-9
  label: Wasserman, L. All of Statistics (Ch. 2-3)
cards:
- id: bea3debb-11ea-44c3-9833-fb8dcea73404
  type: flip
  front: A dataset follows N(100, 15²). Using the empirical rule, approximately what fraction of data
    fall between 85 and 115?
  back: ''
- id: 2bbad85c-ea82-489c-bafb-8d1fed14779e
  type: flip
  front: Write the normal distribution's PDF and explain why it's symmetric around μ.
  back: ''
- id: eb78ff77-0789-462b-8107-d89eb4d875c5
  type: flip
  front: Linear regression output shows 'Residuals approximately normal.' Is the raw dependent variable
    also approximately normal?
  back: ''
- id: 7c82585f-549c-4c8a-b7d9-f622fb2ff8ec
  type: flip
  front: Your data are N(0, 1), and you observe X = 6. The normal density at 6 is very low. Does this
    mean the observation is impossible?
  back: ''
- id: 3a0e5a81-3a2a-42a1-b248-082eda6abac2
  type: mcq
  front: 'If X ~ N(μ, σ²), then Z = (X - μ)/σ follows:'
  back: ''
  choices:
  - key: a
    text: N(μ, σ²)
    correct: false
  - key: b
    text: N(0, 1)
    correct: true
  - key: c
    text: Uniform(0, 1)
    correct: false
  - key: d
    text: Exponential(1)
    correct: false
---

## Intuition
The normal distribution (Gaussian) is the most important in statistics because (1) many natural phenomena approximate it, (2) the CLT says sample means are normal, and (3) it is mathematically tractable. It is defined by two parameters: mean $\mu$ and standard deviation $\sigma$. The empirical rule: ~68% of data within $\mu \pm \sigma$, ~95% within $\mu \pm 2\sigma$, ~99.7% within $\mu \pm 3\sigma$. Used in linear regression inference, confidence intervals, and hypothesis testing.

## Detail
Probability density function: $f(x) = \frac{1}{\sigma\sqrt{2\pi}} \exp\left(-\frac{(x-\mu)^2}{2\sigma^2}\right)$. Standardizing: $Z = \frac{X - \mu}{\sigma} \sim N(0,1)$ (standard normal). Key properties: (1) sum of independent normals is normal; (2) linear combinations of independent normals are normal; (3) maximum entropy distribution with given mean and variance; (4) invertible transformation via Box-Muller to generate normal samples. CDF $\Phi(z)$ and quantiles are tabled and used throughout statistics.

Importance: Linear regression assumes residuals are $N(0, \sigma^2)$; this enables exact inference (not just asymptotic). Multivariate normal $N(\mu, \Sigma)$ is foundational for multivariate inference and graphical models. Heavy-tailed alternatives (t-distribution, Cauchy) are used when normality fails.

## Common gotcha / interview framings
- Not all bell-shaped distributions are normal (e.g., t-distribution, logistic)
- Empirical rule (~95% within 2σ) only applies to normal; other distributions may differ significantly
- Transformation (log, sqrt, Box-Cox) can improve normality, but transforming the response variable in regression changes the interpretation of predictions
- Normality of residuals ≠ normality of raw data; what matters for regression inference is the former

## See also
- [[normal-distribution]]
- [[standard-normal]]
- [[z-scores]]
- [[tail-probabilities]]
- [[empirical-rule]]

## Sources
See frontmatter `sources:`.

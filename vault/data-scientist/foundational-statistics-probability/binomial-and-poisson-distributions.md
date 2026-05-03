---
id: 26ac4a16-c866-432c-bf28-9ca07479c43e
title: Binomial and Poisson distributions
track: data-scientist
topic: foundational-statistics-probability
difficulty: 3
tags:
- discrete-distribution
- count-data
- success-failure
- rare-events
- conversion-modeling
aliases:
- bernoulli-trials
- discrete-counts
- failure-rate
sources:
- url: https://link.springer.com/book/10.1007/978-0-387-21736-9
  label: Wasserman, L. All of Statistics (Ch. 3)
- url: https://www.statlearning.com/
  label: James et al. An Introduction to Statistical Learning (Ch. 4)
cards:
- id: 90634b18-5189-4ab1-b09e-88f56c20f8d6
  type: flip
  front: 'An A/B test has 500 users in each group. Group A: 45 conversions (9%). Group B: 60 conversions
    (12%). Is the difference significant at α = 0.05?'
  back: ''
- id: afbd58d9-c36f-4e07-a925-8c0591ab62c9
  type: flip
  front: Derive the mean and variance of a Binomial(n, p) distribution.
  back: ''
- id: d563e348-0f53-4659-9db9-c449ac8fd47f
  type: flip
  front: Emails arrive at a rate of 30 per hour on average. Model the count in a 10-minute window as Poisson.
    What is the probability of receiving exactly 3 emails?
  back: ''
- id: a6380178-8417-402d-ae7e-28218673e206
  type: flip
  front: Your Poisson regression model for count data has mean = 8 but residual variance = 25. What problem
    might this indicate?
  back: ''
- id: ba481c4b-2d46-47b3-9fb8-3e9459e54cde
  type: mcq
  front: When is the Poisson approximation to the Binomial valid?
  back: ''
  choices:
  - key: a
    text: n large, p large
    correct: false
  - key: b
    text: n large, p small, np = λ
    correct: true
  - key: c
    text: n small, p any value
    correct: false
  - key: d
    text: p = 0.5 always
    correct: false
---

## Intuition
The **binomial distribution** models the number of successes in n independent Bernoulli trials (coin flips). Used in A/B testing (conversions), classification (positive class count), quality control. The **Poisson distribution** models counts of rare events in a fixed interval (time, space), assuming events are independent and occur at a constant rate. Used in call center modeling, web traffic, disease incidence. Both are discrete (integer-valued); both have manageable formulas and are computationally friendly.

## Detail
**Binomial**: $P(X = k) = \binom{n}{k} p^k (1-p)^{n-k}$, where X ~ Binomial(n, p) has mean $\mu = np$ and variance $\sigma^2 = np(1-p)$. For large n and moderate p, Binomial → Normal (normal approximation). When np and n(1-p) both > 5, the normal approximation is excellent.

**Poisson**: $P(X = k) = \frac{\lambda^k e^{-\lambda}}{k!}$, where X ~ Poisson(λ) has mean and variance both equal to λ. Arises as the limit of Binomial(n, p) when n → ∞, p → 0, np → λ (rare events limit). Used for count data; common in generalized linear models (GLM) with log link.

Connection: Poisson is the binomial limit when p is very small and n very large; Poisson approximates binomial when n > 20 and p < 0.05. Both are used in hypothesis tests (exact binomial test) and confidence intervals for proportions.

## Common gotchas / interview framings
- Poisson assumes mean = variance; overdispersion (variance > mean) signals unobserved heterogeneity or mixture of Poissons
- Binomial assumes independence and constant p; violation (clustering, time-dependence) requires mixed models
- Normal approximation fails if p near 0 or 1, or n small; use exact binomial or continuity correction
- In logistic regression, each observation is implicitly Bernoulli(p); aggregated data (Y = number of successes out of n) uses binomial likelihood

## See also
- [[binomial-distribution]]
- [[poisson-distribution]]
- [[discrete-probability]]
- [[logistic-regression]]
- [[poisson-regression]]

## Sources
See frontmatter `sources:`.

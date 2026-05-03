---
id: 9642a791-0838-47c9-abce-15bc8aac6ab5
title: Beta distribution
track: data-scientist
topic: foundational-statistics-probability
difficulty: 3
tags:
- continuous-distribution
- proportion-prior
- conjugate-prior
- empirical-bayes
- online-learning
aliases:
- beta-prior
- proportion-distribution
- conjugate-family
sources:
- url: https://link.springer.com/book/10.1007/978-0-387-21736-9
  label: Wasserman, L. All of Statistics (Ch. 3, 10)
- url: https://www.statlearning.com/
  label: James et al. An Introduction to Statistical Learning
cards:
- id: ec36fc2a-eaa3-4710-a755-750372b4ca5c
  type: flip
  front: You use a Beta(2, 2) prior on a coin's bias p. This is equivalent to observing 1 head and 1 tail
    before seeing any real data. Why?
  back: ''
- id: b8875895-93b2-4e52-a562-119a2623d7e5
  type: flip
  front: Show that Beta(α, β) is the conjugate prior for Binomial(n, p) likelihood.
  back: ''
- id: 2aeebfb7-90cf-486d-9b65-979c865948be
  type: flip
  front: In Thompson sampling for A/B testing, why do we sample from the posterior Beta distribution rather
    than just using the posterior mean?
  back: ''
- id: ec2d6dd4-cb0e-408e-8e8e-80ded8551108
  type: flip
  front: You observe 100 clicks out of 1000 impressions. Beta(1,1) prior gives posterior Beta(101, 901).
    Beta(10,10) prior gives posterior Beta(110, 910). Are the posteriors the same?
  back: ''
- id: c8bc6024-6631-473e-9c5d-c85a793b3413
  type: mcq
  front: Which prior produces the most uniform distribution on [0,1]?
  back: ''
  choices:
  - key: a
    text: Beta(0.5, 0.5)
    correct: false
  - key: b
    text: Beta(1, 1)
    correct: true
  - key: c
    text: Beta(5, 5)
    correct: false
  - key: d
    text: Beta(100, 100)
    correct: false
---

## Intuition
The Beta distribution is defined on [0, 1], making it ideal for modeling proportions, probabilities, and rates. It is the conjugate prior for the Binomial likelihood: if you use Beta(α, β) as a prior on a success probability p and observe k successes out of n trials, the posterior is Beta(α + k, β + n - k)—still a Beta. This conjugacy enables efficient sequential updating and is used in online learning, Thompson sampling, and empirical Bayes. The shape parameters α and β control the distribution's skew and concentration.

## Detail
PDF: $f(p) = \frac{\Gamma(\alpha + \beta)}{\Gamma(\alpha)\Gamma(\beta)} p^{\alpha-1} (1-p)^{\beta-1}$. Mean: $E[p] = \frac{\alpha}{\alpha + \beta}$. Variance: $\text{Var}(p) = \frac{\alpha\beta}{(\alpha+\beta)^2(\alpha+\beta+1)}$. Special cases: Beta(1,1) = Uniform[0,1]; Beta(α, α) is symmetric around 0.5; large α, β concentrates around the mean.

**Conjugacy**: Binomial likelihood $P(k|n, p) \propto p^k(1-p)^{n-k}$ times Beta(α, β) prior gives Beta(α+k, β+n-k) posterior. This is computationally elegant: update sequential data by incrementing α and β. Used in click-through rate estimation, conversion optimization (Thompson sampling), and hierarchical Bayesian models (empirical Bayes hyperparameters).

## Common gotchas / interview framings
- Beta(α, β) is not uniform for all α, β; it requires the Gamma function and is not simple to sample without specialized methods
- Large α, β relative to sample size can dominate the posterior (regularization); small α, β let data speak louder
- Thompson sampling uses the Beta posterior to balance exploration-exploitation in online A/B tests
- Empirical Bayes: estimate α, β from data (marginal likelihood) rather than specifying them subjectively

## See also
- [[beta-distribution]]
- [[conjugate-prior]]
- [[binomial-beta-conjugacy]]
- [[empirical-bayes]]
- [[bayesian-ab-testing]]

## Sources
See frontmatter `sources:`.

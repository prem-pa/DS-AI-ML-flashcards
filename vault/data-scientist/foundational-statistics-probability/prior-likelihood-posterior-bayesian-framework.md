---
id: e987b7e1-9f95-4e05-8478-5f44ff9454ca
title: Prior, likelihood, posterior (Bayesian framework)
track: data-scientist
topic: foundational-statistics-probability
difficulty: 3
tags:
- bayesian-inference
- probabilistic-modeling
- update-mechanism
- generative-models
- probabilistic-programming
aliases:
- bayesian-updating
- posterior-distribution
- bayesian-computation
sources:
- url: https://link.springer.com/book/10.1007/978-0-387-21736-9
  label: Wasserman, L. All of Statistics (Ch. 10)
- url: https://ocw.mit.edu/courses/18-05-introduction-to-probability-and-statistics-spring-2022/
  label: 'MIT 18.05: Bayesian Inference (Lecture Notes)'
cards:
- id: 039437a3-69e5-4694-8f9c-956314b3142d
  type: flip
  front: You place a Beta(1,1) prior (uniform) on a coin's bias p and observe 10 heads, 0 tails. What
    is the posterior distribution?
  back: ''
- id: 101720f5-5862-4a97-af7e-a5a58750b7e4
  type: flip
  front: 'Compare two scenarios: (A) weak prior + weak evidence, (B) strong prior + weak evidence. Which
    posterior is closer to the prior?'
  back: ''
- id: e3b2637f-1f0a-42de-b962-74c0acd57cac
  type: flip
  front: Derive the posterior for a Gaussian likelihood N(μ, σ²) with Gaussian prior N(μ₀, τ²) on μ. What
    is the posterior mean?
  back: ''
- id: 10ea19fe-6f5e-4c9f-b9a1-54437f98970e
  type: flip
  front: Your Bayesian model has posterior P(θ|data) = Beta(0.5, 0.5), which is U-shaped with high density
    at the extremes. Does this mean θ is likely at 0 or 1?
  back: ''
- id: a8859379-8a72-4735-b848-9fa17263cf1b
  type: mcq
  front: What is the relationship between regularization in frequentist methods and priors in Bayesian
    methods?
  back: ''
  choices:
  - key: a
    text: They are unrelated
    correct: false
  - key: b
    text: Ridge regression (L2 penalty) is equivalent to Gaussian priors
    correct: false
  - key: c
    text: Lasso (L1 penalty) is equivalent to Laplace priors
    correct: false
  - key: d
    text: Both B and C
    correct: true
---

## Intuition
Bayesian inference treats unknown parameters as random variables with beliefs encoded in a **prior** $P(\theta)$. Given data, we compute the **likelihood** $P(\text{data}|\theta)$ (how probable the data under each $\theta$), then **update** via Bayes' rule to get the **posterior** $P(\theta|\text{data})$. The posterior combines prior belief with evidence from data. This is the foundation for Bayesian neural networks, empirical Bayes, A/B testing, and online learning algorithms.

## Detail
Bayes' rule: $P(\theta|\text{data}) = \frac{P(\text{data}|\theta)P(\theta)}{P(\text{data})}$ where $P(\text{data}) = \int P(\text{data}|\theta)P(\theta)d\theta$ (marginal likelihood, the normalizing constant). The prior encodes domain knowledge or regularization (e.g., Gaussian prior on regression coefficients ~ Ridge regression). The likelihood is determined by the model and data. The posterior is a distribution reflecting residual uncertainty about $\theta$ after observing data.

Key insight: posterior $\propto$ likelihood × prior. A weak prior (flat, diffuse) lets data dominate; a strong prior pulls inference toward the prior mean. **Conjugacy**: when prior and likelihood form a conjugate pair, the posterior is in the same family as the prior (e.g., Beta prior on a binomial parameter → Beta posterior). This enables efficient sequential updating and is used in many practical algorithms.

## Common gotchas / interview framings
- Priors are not arbitrary; they reflect prior belief or regularization. Weak priors can behave like frequentist methods asymptotically
- Posterior is not the likelihood; posterior is normalized probability distribution over $\theta$, likelihood is not
- Marginal likelihood $P(\text{data})$ is hard to compute (integral over parameter space); MCMC and variational inference are approximation strategies
- Empirical Bayes: use data to estimate the prior (e.g., hierarchical hyperparameters), mixing frequentist and Bayesian ideas

## See also
- [[prior-distribution]]
- [[likelihood-function]]
- [[posterior-distribution]]
- [[bayesian-inference]]
- [[conjugacy]]

## Sources
See frontmatter `sources:`.

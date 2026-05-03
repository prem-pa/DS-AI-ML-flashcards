---
id: 07cda402-2498-4bd3-ba8e-355e24bba222
title: Joint, marginal, and conditional distributions
track: data-scientist
topic: probability-bayesian-thinking
difficulty: 3
tags:
- probabilistic-foundations
- graphical-models
- factorization
- marginalization
- conditional-probability
- inference
aliases:
- joint distribution
- marginal distribution
- conditional probability
sources:
- url: https://en.wikipedia.org/wiki/Bayes_theorem
  label: Bayes' Theorem - Wikipedia
- url: https://machinelearningmastery.com/bayes-theorem-for-machine-learning/
  label: Bayes Theorem for Machine Learning - MLM
cards:
- id: 2deb1545-dc09-4de0-b318-e89d6a249362
  type: flip
  front: Write out the chain rule decomposition of $P(A,B,C,D)$ and explain how it enables efficient inference
    in DAGs.
  back: $P(A,B,C,D) = P(A)P(B|A)P(C|A,B)P(D|A,B,C)$. In a DAG, the chain rule respects causal/temporal
    ordering. Each factor depends only on its parents, enabling dynamic programming algorithms (belief
    propagation, variable elimination) to compute marginals in time polynomial in the number of parents
    per node, not exponential in the total number of nodes.
- id: b4cb0fda-a247-4a9b-bc96-4b8e210c1194
  type: mcq
  front: If $P(X,Y,Z) = P(X)P(Y|X)P(Z|Y)$, which of these is true?
  back: Given the factorization, once $Y$ is known, $Z$'s distribution depends only on $Y$, not on $X$.
    So $P(Z|X,Y)=P(Z|Y)$, confirming conditional independence. Marginally, $X$ influences $Z$ through
    $Y$, so they are not independent.
  choices:
  - key: a
    text: $X$ and $Z$ are marginally independent
    correct: false
  - key: b
    text: $X$ and $Z$ are conditionally independent given $Y$
    correct: true
  - key: c
    text: Both A and B
    correct: false
  - key: d
    text: Neither; $X$ and $Z$ are always dependent
    correct: false
- id: d2a4a8b0-872c-42f8-b2f5-08991f0d4a71
  type: flip
  front: In a Gaussian graphical model, explain how to recover the conditional distribution $P(X_1|X_2)$
    from the joint $P(X_1,X_2)$ when both are Gaussian.
  back: 'If $(X_1,X_2) \sim \mathcal{N}(\mu, \Sigma)$, the conditional $P(X_1|X_2)$ is also Gaussian with
    mean $\mu_1 + \Sigma_{12}\Sigma_{22}^{-1}(X_2-\mu_2)$ and variance $\Sigma_{11}-\Sigma_{12}\Sigma_{22}^{-1}\Sigma_{21}$.
    The precision matrix (inverse covariance) encodes conditional structure: zeros in $\Lambda$ indicate
    conditional independence.'
- id: d660d5f0-f60e-4da5-af0f-d1f018769757
  type: mcq
  front: Which operation preserves the most information from a joint distribution?
  back: The joint $P(X,Y)$ contains full information. Given $P(X,Y)$, you can recover both $P(X)$ and
    any $P(X|Y=y)$. Conversely, $P(X)$ and $P(Y|X)$ uniquely determine $P(X,Y)=P(Y|X)P(X)$. The product
    $P(X)P(Y)$ loses dependence structure, so it's strictly less informative.
  choices:
  - key: a
    text: 'Taking a marginal: $P(X) = \sum_y P(X,Y=y)$'
    correct: false
  - key: b
    text: 'Taking a conditional: $P(X|Y=y)$'
    correct: false
  - key: c
    text: 'Taking a product: $P(X)P(Y)$'
    correct: false
  - key: d
    text: All preserve equal information
    correct: true
- id: 82d0f8e1-f3c3-43ff-9152-33c0caa37e30
  type: flip
  front: What is the computational complexity of marginalizing a variable from a general factor graph,
    and why does the order matter?
  back: Marginalizing variable $X_i$ from a factor graph requires summing $X_i$ out of all factors touching
    it, creating a new factor. If $X_i$ has domain size $d$ and appears in $k$ factors each of size $m$,
    complexity is $O(d \cdot m^k)$. The order of marginalization matters because eliminating a variable
    can create large factors (fill-in), degrading future steps. Variable elimination algorithms choose
    elimination orders to minimize total fill-in.
---

## Intuition
Think of a joint distribution as the complete description of uncertainty over all variables together. Marginal distributions are obtained by "summing out" variables we don't care about. Conditional distributions describe one variable's uncertainty given knowledge of others. These three objects interlock: $P(X,Y) = P(X|Y)P(Y) = P(Y|X)P(X)$.

## Detail
For random variables $X$ and $Y$:
- **Joint distribution** $P(X,Y)$ specifies probability for every combination of values
- **Marginal distribution** $P(X) = \sum_y P(X,Y=y)$ obtained by summing/integrating out other variables
- **Conditional distribution** $P(X|Y) = P(X,Y)/P(Y)$ is the relative likelihood of $X$ given a fixed $Y$

In practice, inference tasks require moving between these representations. Computing marginals (inference) is NP-hard in general graphs. Conditional independence encoded in factorized distributions enables efficient computation. The chain rule decomposes any joint: $P(X_1,...,X_n) = \prod_i P(X_i | X_{i+1},...,X_n)$.

## Common gotchas / interview framings
- Forgetting that $P(X|Y)$ and $P(Y|X)$ are completely different; Bayes' rule relates them
- Assuming variables are independent when they appear in different factors (they may be marginally independent but conditionally dependent, or vice versa)
- Attempting exact inference on dense graphical models without recognizing the computational barrier
- Confusing the order of conditioning: $P(X,Y|Z) \neq P(X|Y,Z)P(Y|Z)$ in general

## See also
- [[bayes_theorem]]
- [[conditional_probability]]
- [[probability_distribution]]
- [[graphical_models]]
- [[factor_graphs]]
- [[belief_propagation]]

## Sources
See frontmatter `sources:`.

---
id: b23697da-4659-4b26-910a-74f93b2877e8
title: Frequentist vs Bayesian interpretation
track: data-scientist
topic: foundational-statistics-probability
difficulty: 3
tags:
- inference-paradigm
- probability-interpretation
- hypothesis-testing
- confidence-intervals
- credible-intervals
aliases:
- frequentism
- bayesianism
- inference-philosophy
sources:
- url: https://link.springer.com/book/10.1007/978-0-387-21736-9
  label: Wasserman, L. All of Statistics (Ch. 9-10)
- url: https://ocw.mit.edu/courses/18-05-introduction-to-probability-and-statistics-spring-2022/
  label: 'MIT 18.05: Probability and Statistics (Bayesian material)'
cards:
- id: fea63337-d819-454b-9a53-c8540f681c6c
  type: flip
  front: Explain the correct meaning of a 95% frequentist confidence interval for the mean.
  back: ''
- id: d4e9ebaa-9761-42e2-b920-3edf0cb5f979
  type: flip
  front: Explain the correct meaning of a 95% Bayesian credible interval for the mean.
  back: ''
- id: 08cf69da-bb41-468b-bd7f-8c3dea4e33b5
  type: flip
  front: 'A researcher reports: ''Our result is significant (p = 0.03), so the null hypothesis is false
    with 97% confidence.'' What''s wrong?'
  back: ''
- id: f398ca31-1174-4fdc-9110-b85b5af2bf43
  type: flip
  front: Write the Bayesian formula relating prior, likelihood, and posterior probabilities.
  back: ''
- id: e6d39c90-c171-4792-a8d1-97876ca955bc
  type: mcq
  front: In frequentist inference, which statement is correct?
  back: ''
  choices:
  - key: a
    text: The parameter is a random variable
    correct: false
  - key: b
    text: The parameter is fixed; the data is random
    correct: true
  - key: c
    text: The p-value is P(H₀|data)
    correct: false
  - key: d
    text: All statements are equivalent
    correct: false
---

## Intuition
Frequentist: probability is the long-run frequency of repeating an experiment; parameters are fixed unknowns; a 95% CI means if you repeat the procedure infinitely, 95% of intervals contain the true parameter. Bayesian: probability is a degree of belief; parameters are random variables; a 95% credible interval is the interval where the posterior probability is 95%. The choice affects how you report uncertainty, interpret results, and design experiments.

## Detail
Frequentist inference conditions on the **true parameter** (fixed): construct estimators (e.g., MLE) and quantify sampling variability via confidence intervals, standard errors, and p-values. A p-value is $P(\text{data}|H_0)$, not $P(H_0|\text{data})$. Bayesian inference puts a prior on the parameter and updates to a posterior given data: $P(\theta|\text{data}) \propto P(\text{data}|\theta)P(\theta)$. The posterior integrates uncertainty about $\theta$.

Confidence intervals are frequentist (coverage guarantee); credible intervals are Bayesian (probability the true value is inside). A 95% CI means: if you repeated the experiment, 95% of computed CIs would bracket the true value. A 95% credible interval means the posterior probability that $\theta$ lies in the interval is 0.95—a direct probability statement about the unknown parameter.

## Common gotchas / interview framings
- A 95% CI does NOT mean P(θ ∈ [a,b]) = 0.95 (frequentist CI is not a Bayesian credible interval)
- The p-value is NOT the probability the null hypothesis is true; it's the probability of observing the data (or more extreme) assuming the null
- Bayesian priors can be subjective, but they make assumptions explicit; frequentist methods hide assumptions in study design
- Multiple testing inflation (p-hacking) affects frequentist Type-I error; Bayesian methods are more robust if priors are set a priori

## See also
- [[frequentist-inference]]
- [[bayesian-inference]]
- [[confidence-intervals]]
- [[credible-intervals]]
- [[p-values]]

## Sources
See frontmatter `sources:`.

---
id: 0e65ae90-19a1-435e-8c2e-1e194377cc0c
title: Posterior estimation and credible intervals
track: data-scientist
topic: hypothesis-testing-inference
difficulty: 3
tags:
- Bayesian inference
- posterior
- credible intervals
- prior
- posterior mean
aliases:
- posterior distribution
- Bayesian CI
- credible interval
- Bayesian estimation
sources:
- url: https://stat20.berkeley.edu/spring-2024/4-generalization/04-hypothesis-tests/notes.html
  label: 'Berkeley Stat 20: Bayesian Inference'
- url: https://online.stat.psu.edu/statprogram/reviews/statistical-concepts/hypothesis-testing/p-value-approach
  label: 'Penn State STAT: Bayesian Methods'
- url: https://physiology.med.cornell.edu/people/banfelder/qbio/resources_2008/1.5_Bonferroni_FDR.pdf
  label: 'Cornell: Bayesian Approaches in Statistical Inference'
cards:
- id: c3dac5dc-05c1-4a59-9578-522a62eea91e
  type: flip
  front: State Bayes' theorem and explain each component in the context of parameter estimation.
  back: 'Bayes'' theorem: $P(\theta | \text{data}) = \frac{P(\text{data} | \theta) \cdot P(\theta)}{P(\text{data})}$.
    **Prior** $P(\theta)$: pre-data belief. **Likelihood** $P(\text{data} | \theta)$: how well the data
    fit parameter $\theta$. **Posterior** $P(\theta | \text{data})$: updated belief combining prior and
    data. **Evidence** $P(\text{data})$: normalizing constant (marginal likelihood), same for all $\theta$.'
- id: 6c1b3cf3-45ab-4d13-8495-8f52de1cf84f
  type: flip
  front: Compare Bayesian credible intervals to frequentist confidence intervals. How do interpretations
    differ?
  back: '**Credible interval**: "Given the data and prior, there is a 95% probability the parameter is
    in this interval." Direct probability statement about the parameter. **Confidence interval**: "If
    we repeated sampling and CI construction infinitely, 95% of intervals would contain the parameter."
    Frequentist statement about the procedure, not the parameter. Bayesian interpretation is more intuitive
    but requires a defensible prior.'
- id: 05c6072d-01bf-4bd0-a0cb-5b343290cbaf
  type: mcq
  front: You specify a prior and compute a posterior. The posterior mean differs from the sample mean.
    Why?
  back: ''
  choices:
  - key: a
    text: The posterior mean is always biased; prefer the sample mean.
    correct: false
  - key: b
    text: The posterior mean balances the likelihood (data) and the prior (belief). With an informative
      prior, the posterior mean shrinks toward the prior mean.
    correct: true
  - key: c
    text: The posterior was computed incorrectly.
    correct: false
  - key: d
    text: This only happens with non-informative priors.
    correct: false
- id: 9998b130-b6dc-4e3a-90ad-d4c6b5e5e718
  type: flip
  front: 'Design scenario: You estimate the conversion rate for a new product feature using a Bayesian
    approach. What prior would you choose and why?'
  back: 'Use a **Beta prior** (conjugate to binomial likelihood): $\theta \sim \text{Beta}(\alpha, \beta)$.
    For a weakly informative prior, use $\alpha = 1, \beta = 1$ (uniform). For domain knowledge (e.g.,
    historical rate of 5%), center the prior at 0.05, e.g., $\text{Beta}(5, 95)$. The posterior combines
    the prior belief with observed data. With little data, the prior pulls the estimate; with abundant
    data, the likelihood dominates.'
- id: f84eac5e-644a-4cfd-b2ec-3cf0782c691e
  type: flip
  front: Explain what "the posterior is proportional to likelihood times prior" means and why it's useful.
  back: '$$P(\theta | \text{data}) \propto P(\text{data} | \theta) \cdot P(\theta)$$

    This simplifies Bayes'' theorem by ignoring the evidence (normalizing constant). Computationally,
    you compute the product of likelihood and prior, then normalize to get the posterior. This is essential
    in MCMC methods (Metropolis-Hastings), which sample proportional to this product without computing
    the full evidence.'
---

## Intuition

Bayesian inference updates a prior belief about a parameter using observed data to obtain a **posterior distribution**. Unlike frequentist approaches, the posterior directly represents the probability distribution of the parameter *given* the observed data. A **credible interval** is the Bayesian analog of a confidence interval: a range believed to contain the parameter with a specified probability (e.g., 95%).

## Detail

Bayes' theorem:
$$P(\theta | \text{data}) = \frac{P(\text{data} | \theta) \cdot P(\theta)}{P(\text{data})}$$
The posterior is proportional to likelihood times prior:
$$P(\theta | \text{data}) \propto P(\text{data} | \theta) \cdot P(\theta)$$

**Components**:
- **Prior** $P(\theta)$: Represents belief before seeing data (e.g., uniform, normal, expert opinion).
- **Likelihood** $P(\text{data} | \theta)$: How likely the data are given $\theta$.
- **Posterior** $P(\theta | \text{data})$: Updated belief combining prior and data.

Point estimates from the posterior include:
- **Posterior mean**: $E[\theta | \text{data}] = \int \theta \cdot P(\theta | \text{data}) d\theta$.
- **Posterior median** or **mode** (MAP, maximum a posteriori).

A **95% credible interval** is any interval $[a, b]$ such that $P(a \leq \theta \leq b | \text{data}) = 0.95$. A common choice is the **highest posterior density (HPD)** interval: the shortest interval containing 95% of posterior mass.

## Common gotchas / interview framings

- **Credible interval interpretation**: "There is a 95% probability the true parameter is in this interval." This is valid and intuitive, unlike frequentist CIs—but only if your prior is reasonable.
- **Prior dependence**: Results depend on the choice of prior. Sensitivity analysis (varying the prior) is essential. Informative priors can bias results if misspecified.
- **"Objective" (flat) priors**: Uniform priors are not always non-informative and can lead to improper posteriors. Jeffreys' prior or other principled choices are more robust.
- **Computational burden**: Computing posteriors requires integration, often infeasible for complex models. MCMC methods (Gibbs sampling, Metropolis-Hastings) are approximations.
- **Small sample advantage**: Bayesian methods can incorporate prior information; useful when data are limited. Trade-off: prior assumptions must be defensible.

## See also
- [[posterior-distribution]]
- [[credible-interval]]
- [[bayes-theorem]]
- [[prior-distribution]]
- [[likelihood]]
- [[posterior-mean]]

## Sources
See frontmatter `sources:`.

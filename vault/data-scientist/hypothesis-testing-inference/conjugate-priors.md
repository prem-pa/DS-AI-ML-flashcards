---
id: f449f4de-5c8b-41e3-a069-ce24f8624545
title: Conjugate priors
track: data-scientist
topic: hypothesis-testing-inference
difficulty: 3
tags:
- conjugate priors
- closed-form posterior
- Bayesian computation
- beta-binomial
- normal-normal
aliases:
- conjugacy
- natural conjugate family
- posterior update
sources:
- url: https://stat20.berkeley.edu/spring-2024/4-generalization/04-hypothesis-tests/notes.html
  label: 'Berkeley Stat 20: Conjugate Priors and Bayesian Computation'
- url: https://online.stat.psu.edu/statprogram/reviews/statistical-concepts/hypothesis-testing/p-value-approach
  label: 'Penn State STAT: Conjugate Families'
- url: https://physiology.med.cornell.edu/people/banfelder/qbio/resources_2008/1.5_Bonferroni_FDR.pdf
  label: 'Cornell: Computational Bayesian Methods'
cards:
- id: dc83190f-8091-43a3-8540-2e3e1057e2d3
  type: flip
  front: Define a conjugate prior and explain why conjugacy simplifies Bayesian inference.
  back: 'A prior is conjugate to a likelihood if the posterior has the same functional form as the prior.
    **Why it matters**: (1) Closed-form posterior—no numerical integration needed. (2) Interpretable:
    prior hyperparameters often correspond to counts or pseudo-observations. (3) Sequential updating:
    posterior from one observation becomes the prior for the next, recursively. (4) Efficient computation.'
- id: c701e8cc-b32a-4595-8d38-a90d192fea10
  type: flip
  front: For a binomial likelihood with $n = 100$ trials and 30 successes, derive the posterior under
    a Beta$(2, 2)$ prior.
  back: 'Beta-Binomial conjugacy: Prior is Beta$(\alpha, \beta) = (2, 2)$. Likelihood contributes 30 successes
    and 70 failures. Posterior is Beta$(\alpha + 30, \beta + 70) = (32, 72)$. The posterior mean is $\frac{32}{32+72}
    = \frac{32}{104} \approx 0.308$. The prior added 4 pseudo-observations (2 successes, 2 failures),
    a weak influence; data dominate.'
- id: b5fb1b70-b0e3-46ef-bfd9-3a717977b1ad
  type: mcq
  front: You observe data $x_1, \ldots, x_n$ from a normal distribution with known variance $\sigma^2
    = 4$. Which prior is conjugate to the likelihood?
  back: ''
  choices:
  - key: a
    text: Beta(1, 1)
    correct: false
  - key: b
    text: Normal with mean $\mu_0$ and variance $\tau_0^2$
    correct: true
  - key: c
    text: Exponential distribution
    correct: false
  - key: d
    text: Uniform on $[−10, 10]$
    correct: false
- id: 324789b0-ce28-4372-9fc3-9855f80fa132
  type: flip
  front: Explain how to sequentially update a conjugate prior as new data arrive.
  back: '**Sequential update**: (1) Start with prior Beta$(\alpha_0, \beta_0)$. (2) Observe data (successes,
    failures), compute posterior Beta$(\alpha_0 + s, \beta_0 + f)$. (3) New data arrive; use the previous
    posterior as the new prior: Beta$(\alpha_1, \beta_1) = (\alpha_0 + s_1, \beta_0 + f_1)$. (4) Repeat.
    The final posterior is Beta$(\alpha_0 + s_{\text{total}}, \beta_0 + f_{\text{total}})$. All information
    accumulates without recomputing from scratch.'
- id: f9299957-5407-45fe-855b-50f00f993bcb
  type: flip
  front: 'Design scenario: You estimate click-through rate (CTR) sequentially as ads are shown. Why would
    Beta conjugacy be valuable here?'
  back: 'With Beta-Binomial conjugacy, as each impression is shown (success or failure), the posterior
    Beta distribution updates in closed form. No numerical integration needed. The prior hyperparameters
    ($\alpha, \beta$) encode belief: e.g., Beta$(1, 1)$ is neutral, Beta$(10, 90)$ favors low CTR based
    on historical data. As impressions accumulate, the posterior concentrates around the true CTR. Computational
    efficiency is critical for real-time decision-making.'
---

## Intuition

A **conjugate prior** is a prior distribution such that the posterior distribution (after observing data) belongs to the same family as the prior. This means the posterior has a closed-form solution, avoiding numerical integration or MCMC. Conjugacy greatly simplifies Bayesian computation and provides analytical insight into how data update beliefs.

## Detail

**Definition**: A prior $P(\theta)$ is conjugate to a likelihood $P(\text{data} | \theta)$ if the posterior $P(\theta | \text{data})$ has the same functional form as the prior.

**Classic examples**:
1. **Beta-Binomial**: Likelihood is binomial (number of successes out of $n$ trials). Prior is Beta$(\alpha, \beta)$. Posterior is Beta$(\alpha + \text{successes}, \beta + \text{failures})$. The prior parameters act as "pseudo-counts."
2. **Normal-Normal**: Likelihood is normal with known variance. Prior is normal. Posterior is normal with updated mean and variance.
3. **Gamma-Exponential**: Likelihood is exponential (rate model). Prior is Gamma. Posterior is Gamma.
4. **Dirichlet-Multinomial**: Likelihood is multinomial. Prior is Dirichlet. Posterior is Dirichlet.

**Why useful**: (1) Closed-form posterior simplifies inference. (2) Sequential updating: observe data, update prior to posterior; observe new data, use posterior as new prior. (3) Interpretable: prior parameters often correspond to imaginary prior counts or observations. (4) Computationally efficient.

## Common gotchas / interview framings

- **Not always available**: Conjugate priors exist for exponential family likelihoods but may not be flexible enough for complex models. Trade-off: convenience vs. modeling flexibility.
- **Choosing the right family**: For a binomial likelihood, Beta is conjugate. For normal, the normal prior is conjugate only if variance is known. Mismatched priors require numerical methods.
- **Informativeness imbalance**: A conjugate prior can be made weakly informative (e.g., Beta$(1, 1)$ is uniform) or informative (e.g., Beta$(50, 50)$ is concentrated near 0.5). Adjust hyperparameters to reflect confidence in prior beliefs.
- **Empirical Bayes**: You can estimate prior hyperparameters from data, then use the conjugate posterior. This is a data-driven approach but can inflate Type I error if the prior is too tuned to the data.
- **Why exponential family**: Conjugate priors exist for exponential family models (e.g., binomial, Poisson, exponential, normal). Understanding this connection helps you recognize conjugate structures.

## See also
- [[conjugate-prior]]
- [[beta-binomial]]
- [[normal-normal]]
- [[exponential-family]]
- [[closed-form-posterior]]
- [[natural-parameters]]

## Sources
See frontmatter `sources:`.

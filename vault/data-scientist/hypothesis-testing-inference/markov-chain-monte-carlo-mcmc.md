---
id: 7a5ba6b1-78eb-4b16-9e5a-c44bf8fb5f95
title: Markov Chain Monte Carlo (MCMC)
track: data-scientist
topic: hypothesis-testing-inference
difficulty: 5
tags:
- MCMC
- Metropolis-Hastings
- Gibbs sampling
- posterior sampling
- Bayesian computation
aliases:
- Metropolis-Hastings
- Gibbs sampling
- posterior simulation
- Hamiltonian MC
sources:
- url: https://stat20.berkeley.edu/spring-2024/4-generalization/04-hypothesis-tests/notes.html
  label: 'Berkeley Stat 20: MCMC and Bayesian Computation'
- url: https://online.stat.psu.edu/statprogram/reviews/statistical-concepts/hypothesis-testing/p-value-approach
  label: 'Penn State STAT: Markov Chain Monte Carlo'
- url: https://physiology.med.cornell.edu/people/banfelder/qbio/resources_2008/1.5_Bonferroni_FDR.pdf
  label: 'Cornell: Advanced Bayesian Computational Methods'
cards:
- id: 6be99c67-303f-484f-befa-010f43bf93c7
  type: flip
  front: Explain the Metropolis-Hastings algorithm in plain language.
  back: Start at a parameter value. Propose a new value (e.g., perturb slightly). Compute how likely the
    data are under the new vs. old value. Accept the new value with a probability reflecting the likelihood
    improvement (acceptance ratio). If accepted, move to the new value; if rejected, stay put. Repeat
    many times. The samples approximate the posterior distribution.
- id: a3aa47f2-bb00-4ebb-a475-9f01f20d7450
  type: flip
  front: What is the burn-in period and why is it important?
  back: Burn-in is the initial iterations of MCMC before the chain converges to the stationary (posterior)
    distribution. Starting from arbitrary initial values, the chain drifts toward high-posterior regions;
    early samples do not represent the posterior. Discard burn-in samples (e.g., first 1000 iterations)
    before analysis. Keeping them biases estimates toward the starting value.
- id: 7e7cd88d-9101-41c2-867f-7e19b46db224
  type: mcq
  front: In Metropolis-Hastings, what does the acceptance ratio control?
  back: ''
  choices:
  - key: a
    text: The variance of the posterior estimate.
    correct: false
  - key: b
    text: The probability of accepting a proposed parameter value, balancing exploration vs. convergence
      speed.
    correct: true
  - key: c
    text: The final posterior mean.
    correct: false
  - key: d
    text: Whether the chain converges to the correct distribution.
    correct: false
- id: ee139885-692a-42b1-baf3-399f2a08943f
  type: flip
  front: Explain effective sample size (ESS) and why MCMC samples are not independent.
  back: 'MCMC samples are autocorrelated: a proposed value depends on the current value, creating serial
    correlation. ESS is the number of independent samples equivalent to $N$ correlated samples: $ESS =
    N / \tau$ where $\tau$ is the autocorrelation time. E.g., 10,000 correlated samples might have $ESS
    = 1,000$ if autocorrelation is strong. This reduces precision; you need more samples for the same
    effective information.'
- id: d85bcea4-2658-4ec4-b72e-84274f0597e6
  type: flip
  front: 'Design scenario: You fit a hierarchical Bayesian model with random effects for patients and
    treatments. MCMC is slow. What strategies would you use?'
  back: 'Strategies: (1) **Gibbs sampling**: Sample patient effects, then treatment effects, from their
    full conditional distributions (often faster than general MH). (2) **Hamiltonian MC**: More efficient
    in high dimensions; uses gradient information. (3) **Parallel chains**: Run multiple chains and combine;
    better for diagnostics. (4) **Reparameterization**: Centered vs. non-centered parameterizations affect
    mixing. (5) **Approximate methods**: Variational inference, expectation-propagation for faster inference
    (approximate but scalable).'
---

## Intuition

Markov Chain Monte Carlo (MCMC) is a computational technique for sampling from complex, high-dimensional posterior distributions. When the posterior has no closed form (no conjugate prior), MCMC generates correlated samples that approximate the posterior, enabling estimation of posterior means, quantiles, and credible intervals without explicit integration.

## Detail

**Core idea**: Construct a Markov chain whose stationary distribution equals the target posterior $P(\theta | \text{data})$. Run the chain for many iterations; after convergence ("burn-in"), samples approximate the posterior.

**Metropolis-Hastings (MH) algorithm**:
1. Start with initial parameter value $\theta_0$.
2. Propose a new value $\theta^{*}$ from a proposal distribution $q(\theta^{*} | \theta_t)$.
3. Compute acceptance ratio: $\alpha = \min\left(1, \frac{P(\theta^{*} | \text{data}) \cdot q(\theta_t | \theta^{*})}{P(\theta_t | \text{data}) \cdot q(\theta^{*} | \theta_t)}\right)$.
4. Accept $\theta^{*}$ with probability $\alpha$; otherwise, stay at $\theta_t$.
5. Repeat; after many iterations, samples approximate the posterior.

**Gibbs sampling**: Special case where full conditional distributions $P(\theta_j | \theta_{-j}, \text{data})$ are known. Sample each parameter in turn from its conditional; often efficient for hierarchical and latent variable models.

**Key concepts**:
- **Burn-in**: Discard early iterations before convergence (transient behavior).
- **Thinning**: Keep every $k$-th sample to reduce autocorrelation and storage.
- **Convergence diagnostics**: Potential scale reduction factor ($\hat{R}$), trace plots, effective sample size (ESS).

## Common gotchas / interview framings

- **Dependence on starting values**: Poor initialization can lead to slow convergence; run multiple chains with different starting points.
- **Tuning proposal distributions**: MH acceptance rate should be ~20–50%; too high acceptance (small proposals) or too low (large proposals) wastes iterations. Adaptive MCMC can optimize this.
- **High-dimensional spaces**: Standard MH is inefficient in high dimensions (curse of dimensionality). Hamiltonian MC and other advanced methods are more efficient.
- **Not i.i.d. samples**: MCMC samples are autocorrelated; effective sample size (ESS) is much smaller than nominal sample count. Thinning reduces autocorrelation at a computational cost.
- **Model misspecification**: MCMC samples from the specified posterior, not the truth. If the model is wrong, posterior is misleading.

## See also
- [[mcmc]]
- [[metropolis-hastings]]
- [[gibbs-sampling]]
- [[convergence-diagnostics]]
- [[burn-in-period]]
- [[effective-sample-size]]

## Sources
See frontmatter `sources:`.

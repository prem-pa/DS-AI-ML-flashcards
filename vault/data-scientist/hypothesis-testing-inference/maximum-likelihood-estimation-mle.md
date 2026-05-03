---
id: 64b87da0-d7c9-4b53-8f97-feb152a5ffed
title: Maximum likelihood estimation (MLE)
track: data-scientist
topic: hypothesis-testing-inference
difficulty: 3
tags:
- MLE
- estimation
- likelihood
- efficiency
- asymptotic properties
aliases:
- ML estimation
- likelihood function
- log-likelihood
- parameter estimation
sources:
- url: https://online.stat.psu.edu/statprogram/reviews/statistical-concepts/hypothesis-testing/p-value-approach
  label: 'Penn State STAT: Maximum Likelihood Methods'
- url: https://stat20.berkeley.edu/spring-2024/4-generalization/04-hypothesis-tests/notes.html
  label: 'Berkeley Stat 20: Likelihood-Based Inference'
- url: https://www.colorado.edu/amath/sites/default/files/attached-files/lesson9_hyptests.pdf
  label: 'University of Colorado: Estimation and Hypothesis Testing'
cards:
- id: 8a0b305b-9f96-42a4-9d06-2f6fd9eed689
  type: flip
  front: Define the likelihood function and explain how it differs from a probability density.
  back: The **likelihood function** $L(\theta; \text{data})$ is the probability density (or mass) of the
    observed data as a function of the parameter $\theta$. It is *not* a probability distribution over
    $\theta$. For fixed data, the likelihood shows which parameter values are more or less consistent
    with what we observed. Maximizing the likelihood finds the parameter estimate.
- id: 517fd7c3-7259-4f2c-b5d3-dad2efb2c8ba
  type: flip
  front: You observe 10 coin flips resulting in 7 heads. Write the log-likelihood for the probability
    $p$ of heads and find the MLE.
  back: 'Log-likelihood: $\ell(p) = 7 \log(p) + 3 \log(1 - p)$. Taking the derivative: $\frac{d\ell}{dp}
    = \frac{7}{p} - \frac{3}{1-p} = 0$. Solving: $7(1-p) = 3p \Rightarrow 7 = 10p \Rightarrow \hat{p}
    = 0.7$. The MLE is the sample proportion of heads.'
- id: 7fd99ccb-3db5-45bc-9ee6-41534bf6a0c9
  type: mcq
  front: Under what conditions is the MLE asymptotically normal and efficient?
  back: ''
  choices:
  - key: a
    text: Always, for any sample size and model.
    correct: false
  - key: b
    text: When regularity conditions hold (e.g., continuous and twice-differentiable likelihood, information
      matrix is non-singular) and n → ∞.
    correct: true
  - key: c
    text: Only if the true distribution matches the assumed model exactly.
    correct: false
  - key: d
    text: When the likelihood has a unique global maximum and the sample is large.
    correct: false
- id: 0f92a1fb-0288-475d-837a-bab229e5ba02
  type: flip
  front: Why is the MLE biased for the sample variance, but this bias disappears asymptotically?
  back: The MLE for variance of a normal distribution is $\hat{\sigma}^2_{\text{MLE}} = \frac{1}{n} \sum
    (x_i - \bar{x})^2$ (divide by $n$). Its expected value is $\frac{n-1}{n} \sigma^2 < \sigma^2$, so
    it underestimates. But as $n \to \infty$, the ratio $\frac{n-1}{n} \to 1$, so the bias vanishes. In
    finite samples, use the unbiased estimator (divide by $n-1$) instead.
- id: a371aeb7-c850-4b4a-b70d-0e75d803c54a
  type: flip
  front: 'Design scenario: You model customer conversion as Bernoulli with parameter $p$. After fitting
    an MLE, you want a 95% CI for $p$. What methods would you use?'
  back: 'Methods: (1) **Asymptotic normality**: $\hat{p} \pm 1.96 \cdot \sqrt{\frac{\hat{p}(1-\hat{p})}{n}}$
    (valid for large $n$). (2) **Bootstrap**: Resample with replacement and compute $\hat{p}$ each time;
    use percentiles. (3) **Exact (Clopper-Pearson)**: Use the inverse binomial distribution (no asymptotic
    approximation). For small $n$ or $p$ near 0 or 1, avoid asymptotic normality; use bootstrap or exact
    method.'
---

## Intuition

Maximum likelihood estimation (MLE) is a principled method for estimating parameters of a probability distribution. The idea: given observed data, find the parameter values that make the data most likely. Formally, choose $\hat{\theta}$ that maximizes the **likelihood function** $L(\theta; \text{data})$, the joint probability density of the observations as a function of $\theta$.

## Detail

For a sample $x_1, \ldots, x_n$ from a distribution with parameter(s) $\theta$, the likelihood is:
$$L(\theta) = \prod_{i=1}^{n} f(x_i | \theta)$$
where $f$ is the probability mass function (discrete) or density (continuous).

To find the MLE, compute the **log-likelihood** $\ell(\theta) = \log L(\theta) = \sum_{i=1}^{n} \log f(x_i | \theta)$ and solve:
$$\frac{d\ell(\theta)}{d\theta} = 0$$
For multivariate $\theta$, solve the gradient $\nabla \ell(\theta) = 0$. Often, numerical optimization (Newton-Raphson, gradient descent) is needed.

**Asymptotic properties**: Under regularity conditions, as $n \to \infty$:
- **Consistency**: $\hat{\theta}_{\text{MLE}} \to \theta_0$ (true value) in probability.
- **Asymptotic normality**: $\sqrt{n}(\hat{\theta}_{\text{MLE}} - \theta_0) \xrightarrow{d} N(0, I(\theta_0)^{-1})$ where $I$ is the Fisher information matrix.
- **Efficiency**: MLE achieves the Cramér-Rao lower bound; it is the most efficient regular estimator asymptotically.

## Common gotchas / interview framings

- **MLE ≠ unbiased**: MLE may have bias in finite samples, though bias vanishes asymptotically. E.g., sample variance is biased; divide by $n-1$ for unbiased estimate.
- **Numerical optimization pitfalls**: Likelihood surface may be multimodal; optimization can get stuck at local maxima. Always check multiple starting points.
- **Misspecified model**: If the true distribution differs from the assumed model, MLE estimates are inconsistent for the true parameters but consistent for the pseudo-true parameters (Kullback-Leibler projection).
- **Computational burden**: For complex models (e.g., mixed-effects, latent variables), finding the MLE may be infeasible; approximate methods (EM algorithm, MCMC) are alternatives.
- **Standard errors**: Use the inverse Fisher information $I(\hat{\theta})^{-1}$ or bootstrap for confidence intervals and hypothesis tests.

## See also
- [[maximum-likelihood-estimation]]
- [[likelihood-function]]
- [[mle-asymptotic-normality]]
- [[fisher-information]]
- [[consistency]]
- [[efficiency]]

## Sources
See frontmatter `sources:`.

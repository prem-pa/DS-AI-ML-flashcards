---
id: 0b25e678-bf9c-4aa0-a84d-87717b3f60c1
title: Prior selection and sensitivity analysis
track: data-scientist
topic: probability-bayesian-thinking
difficulty: 5
tags:
- bayesian-practice
- prior-elicitation
- objective-priors
- robustness
- hyperparameter-tuning
aliases:
- prior specification
- informative prior
- weakly informative prior
sources:
- url: https://en.wikipedia.org/wiki/Prior_probability
  label: Prior Probability - Wikipedia
- url: https://vioshyvo.github.io/Bayesian_inference/hierarchical-models.html
  label: Hierarchical Models and Prior Selection - Chapter 6
cards:
- id: 0481792d-399c-4972-b31d-31c693952226
  type: flip
  front: For a Binomial likelihood $P(D|p) \propto p^k(1-p)^{n-k}$, derive the posterior if you use a
    Beta$(\alpha, \beta)$ prior. Why are Beta priors called "conjugate"?
  back: 'Prior: $P(p) \propto p^{\alpha-1}(1-p)^{\beta-1}$. Posterior: $P(p|D) \propto p^{k+\alpha-1}(1-p)^{n-k+\beta-1}$,
    which is Beta$(k+\alpha, n-k+\beta)$. The posterior has the same functional form as the prior (Beta
    family); hyperparameters update additively. Beta is conjugate to Binomial because the functional forms
    match. This enables closed-form inference and intuitive interpretation: $\alpha$ and $\beta$ act like
    pseudo-counts.'
- id: dbcdb310-1453-444d-a760-caf52a08c667
  type: mcq
  front: You have 100 observations of a quantity estimated to be 50 ± 10 (std dev). Which prior is more
    defensible for Bayesian inference?
  back: With 100 observations, the data is reasonably informative. A weakly informative prior (B) centered
    on the domain estimate and with width matching prior uncertainty provides gentle regularization without
    imposing the false confidence of a point mass (D) or the awkwardness of an improper/extremely diffuse
    prior (A, C).
  choices:
  - key: a
    text: $\theta \sim \text{Uniform}(-\infty, \infty)$ (flat)
    correct: false
  - key: b
    text: $\theta \sim \mathcal{N}(50, 10^2)$ (centered on estimate with std matching uncertainty)
    correct: true
  - key: c
    text: $\theta \sim \mathcal{N}(0, 100^2)$ (very diffuse)
    correct: false
  - key: d
    text: $\theta \sim \delta(50)$ (point mass at 50)
    correct: false
- id: 1c7799ef-ee87-4bff-a8ff-32113b80193c
  type: flip
  front: Explain why the Jeffreys prior is said to be invariant to reparameterization, and why this matters
    for prior selection.
  back: 'The Jeffreys prior is $P(\theta) \propto \sqrt{|I(\theta)|}$ where $I(\theta)$ is the Fisher
    information. Under a reparameterization $\phi = g(\theta)$, the Jeffreys prior on $\phi$ is derived
    the same way, and it transforms correctly: $P(\phi) = P(\theta(\phi)) |d\theta/d\phi|$, so the prior
    remains "non-informative" under change of variables. This matters because flat priors in one parameterization
    encode information in another; Jeffreys avoids this arbitrariness.'
- id: 1aad7f17-c382-4f84-b3e0-2cc375a7b330
  type: mcq
  front: In a sensitivity analysis, the posterior mean of a parameter shifts by 30% when you vary the
    prior from Beta(1,1) to Beta(10,10). What should you conclude?
  back: 'A 30% shift indicates the prior meaningfully impacts inference. This suggests: (1) sample size
    is small relative to prior strength, or (2) the two priors encode very different beliefs. Sensitivity
    analysis has done its job: flagging that prior choice is critical. You should either: justify the
    chosen prior on domain grounds, or report a range of posteriors under different priors.'
  choices:
  - key: a
    text: The posterior is insensitive; prior choice doesn't matter
    correct: false
  - key: b
    text: The prior is highly influential; results depend on prior specification
    correct: true
  - key: c
    text: The data is weak and should be collected
    correct: false
  - key: d
    text: Beta priors should not be used for this problem
    correct: false
- id: 47476ba4-2106-4eed-b263-d4f53deddc44
  type: flip
  front: In a hierarchical model $Y_i | \mu_i \sim \mathcal{N}(\mu_i, \sigma^2)$, $\mu_i | \tau \sim \mathcal{N}(\mu,
    \tau^2)$, explain what happens if you use a very weak prior on $\tau$ (hyperprior) vs. a strong one.
  back: '**Weak hyperprior on $\tau$** (e.g., $\tau \sim \text{Exponential}(0.1)$ with mode near 0): Posterior
    of $\tau$ may be estimated as nearly 0, inducing extreme shrinkage—all $\mu_i$ collapse toward $\mu$,
    losing group-level variation. This is the empirical Bayes pitfall: the point estimate of $\tau$ ignores
    uncertainty in $\tau$. **Strong hyperprior** (e.g., $\tau \sim \mathcal{N}(\mu_{\tau}, \sigma_{\tau}^2)$
    with reasonable bounds): Posterior accounts for uncertainty in $\tau$, yielding moderate shrinkage
    and better small-sample estimates. Fully Bayesian treatment properly averages over $\tau$''s posterior.'
---

## Intuition
Priors encode prior beliefs about parameters before seeing data. "Objective" (or noninformative) priors aim to let the data dominate, while subjective priors incorporate domain knowledge. Sensitivity analysis checks whether results depend critically on the prior choice—if they do, the prior is influential and conclusions may be fragile. The goal is to balance prior informativeness with data size and context.

## Detail
Common prior choices:
- **Noninformative (flat):** $P(\theta) \propto 1$. Not invariant under reparameterization; can lead to improper posteriors.
- **Weakly informative:** Centered at zero with moderate variance. Provides regularization without imposing strong beliefs. Example: $\theta \sim \mathcal{N}(0, \sigma^2)$ with $\sigma$ chosen to cover plausible range.
- **Conjugate priors:** Posterior has same form as prior. E.g., Beta prior on $p$ in Binomial gives Beta posterior. Enables closed-form posteriors and intuition about hyperparameter updates.
- **Empirical priors:** Estimated from data (typically hyperpriors). Useful in hierarchical models.

Sensitivity analysis varies the prior and reports how posterior inferences change. If posterior is robust to prior perturbations, the data dominates; if not, prior choice is critical and should be justified or explored.

## Common gotchas / interview framings
- Believing a "flat" prior is truly noninformative; flat in one parameterization is not flat under transformation
- Choosing conjugate priors solely for computational convenience without checking domain relevance
- Failing to check that priors are proper (integrate to 1) or that posteriors are proper when using noninformative priors
- In hierarchical models, underestimating the impact of the hyperprior; weak hyperpriors can induce strong shrinkage
- Assuming prior robustness without sensitivity checks, especially in small-sample or high-dimensional settings

## See also
- [[prior_probability]]
- [[bayes_theorem]]
- [[conjugate_prior]]
- [[dirichlet_distribution]]
- [[empirical_bayes]]
- [[model_selection]]

## Sources
See frontmatter `sources:`.

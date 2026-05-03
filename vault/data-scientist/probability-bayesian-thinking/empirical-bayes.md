---
id: ee3c936f-cc20-4d4a-9a27-e25efc574c5a
title: Empirical Bayes
track: data-scientist
topic: probability-bayesian-thinking
difficulty: 5
tags:
- hyperpriors
- shrinkage-estimation
- hierarchical-models
- marginal-likelihood
- empirical-estimation
aliases:
- empirical Bayes estimation
- type II maximum likelihood
- marginal likelihood
sources:
- url: https://en.wikipedia.org/wiki/Empirical_Bayes_method
  label: Empirical Bayes Method - Wikipedia
- url: https://hannig.cloudapps.unc.edu/STOR757Bayes/handouts/PetroneEtAl2014.pdf
  label: Empirical Bayes Methods in Classical and Bayesian Inference
cards:
- id: 34d2c89f-a3bf-4b11-9c60-14b61f4ca17e
  type: flip
  front: In James-Stein estimation, why does shrinking toward a common mean improve MSE compared to unrestricted
    maximum likelihood, even if the true means are different?
  back: 'James-Stein estimator shrinks each estimate $\bar{y}_i$ toward the grand mean $\bar{y}$ by a
    factor depending on sample size and dimension. Even if true $\mu_i$ are distinct, the shrinkage trades
    bias for lower variance. In high dimensions (dim $\geq 3$), the variance reduction outweighs the bias,
    yielding lower MSE than the MLE. This is an empirical Bayes insight: pooling information across groups
    (implicit in the shrinkage factor) reduces estimation error, especially in high-dim or small-sample
    regimes.'
- id: 77b19b22-7378-4194-af14-b5acde6bbd36
  type: mcq
  front: In empirical Bayes, $\hat{\alpha} = \arg\max_\alpha P(D|\alpha)$ is estimated from the same data
    used to compute the posterior. What is the main risk?
  back: 'Optimizing $\alpha$ on the same data used for inference can lead to overfitting: the marginal
    likelihood may peak at a hyperparameter that fits idiosyncratic noise in the data. A defensive practice
    is to use cross-validated marginal likelihood or to hold out a validation set for hyperparameter tuning.
    The empirical Bayes estimate is point-wise, ignoring uncertainty in $\alpha$, which compounds the
    issue.'
  choices:
  - key: a
    text: The posterior is doubly stochastic and hard to interpret
    correct: false
  - key: b
    text: The hyperparameter $\alpha$ is overfit; the marginal likelihood is optimized on the training
      data
    correct: true
  - key: c
    text: The prior becomes improper
    correct: false
  - key: d
    text: Shrinkage is always too weak
    correct: false
- id: 3a523101-903b-4994-80f5-95c00762bb27
  type: flip
  front: Explain the difference between empirical Bayes and fully Bayesian hierarchical inference. When
    would you prefer fully Bayesian?
  back: '**Empirical Bayes:** $\hat{\alpha} = \arg\max P(D|\alpha)$, then $P(\theta|D,\hat{\alpha})$.
    Fast, automatic shrinkage, but ignores uncertainty in $\alpha$ and risks overfitting $\alpha$. **Fully
    Bayesian:** Place a hyperprior on $\alpha$, compute $P(\theta|D) = \int P(\theta|D,\alpha)P(\alpha|D)d\alpha$.
    Slower (requires MCMC or variational inference), but properly propagates uncertainty and is less prone
    to overfitting. Prefer fully Bayesian when: (1) sample size is small (hyperparameter uncertainty is
    large), (2) hyperparameter has meaningful prior knowledge, or (3) robustness to prior specification
    is critical.'
- id: bef512fc-dc7c-4a21-ae73-16ac6d8ebb3f
  type: mcq
  front: In a problem with $n=30$ groups and 50 observations per group, empirical Bayes estimates a global
    variance $\hat{\tau}^2 \approx 0$ from the marginal likelihood. What is likely happening?
  back: With 30 groups, the MLE of between-group variance can be near-zero if groups happen to be similar
    in the sample, even if true variance is nonzero. Empirical Bayes treats $\hat{\tau}^2 \approx 0$ as
    fact, inducing complete pooling. A fully Bayesian approach would place a hyperprior on $\tau$ (e.g.,
    $\tau \sim \text{Half-Normal}(1)$) and average over its posterior, accounting for sampling variability
    and yielding more stable shrinkage.
  choices:
  - key: a
    text: The groups truly have no variance; the model is correct
    correct: false
  - key: b
    text: Empirical Bayes is collapsing the variance estimate by ignoring uncertainty in $\tau$, inducing
      extreme shrinkage
    correct: true
  - key: c
    text: The data are too noisy to estimate $\tau$ reliably
    correct: false
  - key: d
    text: A conjugate prior should be used instead
    correct: false
- id: 142bcbef-8f7c-432d-be2b-386bb1904b40
  type: flip
  front: 'Derive the empirical Bayes estimator for a Normal model: $y_i | \theta_i \sim \mathcal{N}(\theta_i,
    \sigma^2)$, $\theta_i | \mu, \tau \sim \mathcal{N}(\mu, \tau^2)$. Assume $\sigma^2$ is known.'
  back: 'Marginal likelihood: $P(y_i|\mu,\tau) = \int \mathcal{N}(y_i|\theta_i,\sigma^2)\mathcal{N}(\theta_i|\mu,\tau^2)d\theta_i
    = \mathcal{N}(y_i|\mu,\sigma^2+\tau^2)$. EB estimates $\hat{\mu}=\bar{y}$ and $\hat{\tau}^2$ by MLE
    of the marginal (via method of moments or numerical optimization). Then $E[\theta_i|y_i,\hat{\mu},\hat{\tau}]
    = \frac{\hat{\tau}^2}{\sigma^2+\hat{\tau}^2}y_i + \frac{\sigma^2}{\sigma^2+\hat{\tau}^2}\hat{\mu}$—a
    shrinkage estimator. Weight on $y_i$ depends on relative size of $\tau$ (between-group variance) to
    $\sigma$ (within-group noise).'
---

## Intuition
Empirical Bayes estimates hyperpriors (priors on priors) from the data, rather than specifying them a priori. The idea: use marginal likelihood $P(D | \alpha)$ to infer hyperparameters $\alpha$, then use those point estimates in the posterior. This provides automatic regularization and shrinkage toward a data-driven center, without requiring external domain expertise. It's a middle ground between pure Bayesian (fix hyperpriors) and pure frequentist (ignore priors).

## Detail
In a hierarchical model:
$$P(y_1,...,y_n | \alpha) = \int P(y_1,...,y_n | \theta_1,...,\theta_n) P(\theta_1,...,\theta_n | \alpha) d\theta$$

Empirical Bayes estimates $\hat{\alpha} = \arg\max_\alpha P(D|\alpha)$ (marginal likelihood), then computes $P(\theta|D, \hat{\alpha})$ treating $\hat{\alpha}$ as fixed. This shrinks each $\theta_i$ toward the common mean inferred from the data.

**Pros:** Automatic regularization, interpretable shrinkage, no hyperprior specification.
**Cons:** Underestimates posterior uncertainty (ignores uncertainty in $\alpha$), can overfit hyperparameters, may collapse variance estimates when true hyperparameter variance is small.

Fully Bayesian approach averages over hyperparameter uncertainty: $P(\theta|D) = \int P(\theta|D,\alpha)P(\alpha|D)d\alpha$, which is often more robust.

## Common gotchas / interview framings
- Empirical Bayes point estimates of $\alpha$ bias posteriors toward zero variance (complete pooling) when true hyperparameter variance is small
- Mistaking empirical Bayes for full Bayes; it ignores hyperparameter uncertainty
- Using marginal likelihood on the *training* set to select hyperparameters, leading to overfitting (should use cross-validation or held-out data)
- In high-dimensional settings, empirical Bayes shrinkage can be too aggressive, collapsing coefficients to zero when they should vary
- Confusing empirical Bayes with parametric bootstrap; they use data differently

## See also
- [[empirical_bayes_method]]
- [[hierarchical_bayes]]
- [[hyperparameter]]
- [[shrinkage_estimator]]
- [[empirical_risk_minimization]]

## Sources
See frontmatter `sources:`.

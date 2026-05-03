---
id: 52aa93be-f4bd-43a7-bc3e-d9e7285f738e
title: Model comparison and cross-validation
track: data-scientist
topic: probability-bayesian-thinking
difficulty: 3
tags:
- model-selection
- information-criteria
- cross-validation
- predictive-accuracy
- AIC
- BIC
- LOO-CV
aliases:
- AIC
- BIC
- WAIC
- leave-one-out cross-validation
- predictive performance
sources:
- url: https://en.wikipedia.org/wiki/Akaike_information_criterion
  label: Akaike Information Criterion - Wikipedia
- url: https://en.wikipedia.org/wiki/Bayesian_information_criterion
  label: Bayesian Information Criterion - Wikipedia
cards:
- id: dfd92cf1-46ea-4eff-83bb-1e8cfa7ddf28
  type: flip
  front: Explain why AIC is asymptotically equivalent to leave-one-out cross-validation (LOO-CV), even
    though one uses in-sample likelihood and the other uses hold-out test sets.
  back: 'AIC estimates the Kullback-Leibler divergence (distance from true model) using in-sample log-likelihood
    plus a $k$ penalty. By a remarkable result (Stone, 1977), AIC is asymptotically equivalent to LOO-CV
    for large $n$: the penalty $k$ in AIC exactly accounts for the average inflation of in-sample vs.
    held-out likelihood. However, finite-sample behavior differs: LOO-CV estimates generalization directly
    (model-free); AIC relies on asymptotic normality and the linear approximation. LOO-CV is more reliable
    for small-sample or extreme-distribution scenarios.'
- id: bb4c5a55-7088-4de7-acf7-b884a3bfd5d5
  type: mcq
  front: 'You compare two models using BIC: Model A (50 parameters) has $\text{BIC}=200$, Model B (10
    parameters) has $\text{BIC}=210$. Both were fit to $n=1000$ data. Which is better, and why is BIC''s
    penalty appropriate here?'
  back: 'Model A has lower BIC by 10 points, suggesting it is slightly preferred. However, BIC''s penalty
    term $k\log n = k \times 6.9$ is substantial: Model A pays $50 \times 6.9 \approx 345$ penalty, while
    Model B pays $10 \times 6.9 \approx 69$. The fact that A''s in-sample likelihood overcomes this massive
    penalty slightly suggests A may overfit. With large $n=1000$, BIC''s stringent penalty is appropriate
    for controlling false discoveries. Model B is safer; the 10-point BIC difference is small.'
  choices:
  - key: a
    text: Model A; lower BIC always wins
    correct: false
  - key: b
    text: Model B; it's simpler and BIC heavily penalizes complexity for large $n$
    correct: true
  - key: c
    text: Model A; the extra 40 parameters are justified by the 10-point BIC gain
    correct: false
  - key: d
    text: Neither; the difference is too small to decide
    correct: false
- id: 17c80891-fd61-4883-a14d-5e795839e3d4
  type: flip
  front: In Bayesian model comparison via Bayes Factors, why is the marginal likelihood $P(D|M)$ said
    to implement an "automatic Occam's razor"?
  back: 'Bayes Factor: $\text{BF}_{12} = P(D|M_1)/P(D|M_2) = \int P(D|\theta_1,M_1)P(\theta_1|M_1)d\theta_1
    / \int P(D|\theta_2,M_2)P(\theta_2|M_2)d\theta_2$. Model 1 with many parameters has a broader prior
    $P(\theta_1|M_1)$, spreading probability mass thinly over the parameter space. Unless the likelihood
    strongly peaks in the data-generating direction, the marginal likelihood integrates to something small.
    Conversely, a simpler model with fewer parameters has a concentrated prior and higher marginal likelihood
    if it fits reasonably well. This automatic down-weighting of complexity is "Occam''s razor" without
    explicit penalties.'
- id: 13b686de-6c6b-40fc-83a0-ff7e81eac4e4
  type: mcq
  front: For time series data with $n=100$ observations, which cross-validation strategy is most appropriate?
  back: 'Time series data have temporal dependencies; random partitioning (A, B, D) breaks the sequence
    and allows information leakage from future into the past. Forward-chaining (C) respects causality:
    you train on historical data and test on future data, mimicking the deployment scenario. This is the
    only valid cross-validation for time series.'
  choices:
  - key: a
    text: 5-fold cross-validation with random partition
    correct: false
  - key: b
    text: Leave-one-out cross-validation (LOO-CV)
    correct: false
  - key: c
    text: 'Forward-chaining: train on [1:t], test on [t+1], sliding window'
    correct: true
  - key: d
    text: Random-hold-out 20% test set, fit on 80%
    correct: false
- id: 43ec67ad-f9a0-4723-b460-4c24d3266457
  type: flip
  front: Explain the relationship between model complexity, marginal likelihood, and overfitting in Bayesian
    model selection. When would you prefer a complex model despite lower marginal likelihood?
  back: 'Complex models can have high marginal likelihood if the extra parameters meaningfully improve
    fit to the true data-generating process. However, if complexity is spurious (fitting noise), the marginal
    likelihood is low due to prior averaging. In practice: (1) if your domain knowledge strongly suggests
    complexity (e.g., physical models with mechanistic layers), use it despite lower marginal likelihood
    on current data; (2) if complexity is exploratory, trust marginal likelihood or LOO-CV; (3) use sensitivity
    analysis: vary prior on complexity (hyperprior scale) and check robustness. Marginal likelihood assumes
    the prior reflects your true uncertainty; misspecified priors can mislead.'
---

## Intuition
Model comparison answers: which model better predicts new data? Resampling methods like cross-validation directly estimate predictive accuracy by holding out test data. Information criteria (AIC, BIC) approximate cross-validation using in-sample likelihood and parameter count penalties. In Bayesian settings, model comparison via marginal likelihood ($P(D|M)$) weighs likelihood against complexity through Occam's razor. These tools balance fit and parsimony.

## Detail
**Information Criteria:**
- **AIC:** $-2\log\hat{L} + 2k$ where $k$ = # parameters. Penalizes complexity lightly; asymptotically equivalent to LOO-CV.
- **BIC:** $-2\log\hat{L} + k\log n$. Stronger complexity penalty; favors simpler models in large samples.
- **Bayes Factor:** Ratio of marginal likelihoods $P(D|M_1)/P(D|M_2)$. Incorporates prior on models; automatic Occam's razor via marginal likelihood.

**Cross-Validation:**
- **K-fold CV:** Partition data into $k$ folds, train on $k-1$, test on hold-out. Average test error estimates generalization.
- **LOO-CV:** Leave-one-out is efficient via Pareto Smoothed Importance Sampling (PSIS-LOO) without refitting.
- **WAIC:** Bayesian analogue to AIC using posterior predictive distribution.

## Common gotchas / interview framings
- Comparing models on training accuracy, not generalization accuracy (overfitting)
- Using BIC without noting it depends strongly on $n$ (penalty grows as $n \log n$)
- Ignoring multiple testing: comparing many models on the same data inflates false positives
- In time series, naively applying K-fold CV (breaks temporal structure); use time-series CV instead
- Bayes Factors are sensitive to priors; strong priors favor simpler models even if data favor complexity

## See also
- [[akaike_information_criterion]]
- [[bayesian_information_criterion]]
- [[cross-validation]]
- [[model_selection]]
- [[marginal_likelihood]]
- [[pareto_smoothed_importance_sampling]]

## Sources
See frontmatter `sources:`.

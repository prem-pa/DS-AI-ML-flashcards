---
id: 9695e322-438f-4d33-82c8-b73ecc2afdbc
title: Point forecasts and confidence intervals
track: verticals
topic: time-series-forecasting
difficulty: 3
tags:
- uncertainty
- quantile-regression
- confidence-intervals
- prediction-intervals
- probabilistic
aliases:
- prediction intervals
- quantile forecasting
- interval forecasts
- coverage
sources:
- url: https://otexts.com/fpp2/prediction-intervals.html
  label: 'Forecasting: Principles and Practice - Prediction Intervals'
- url: https://en.wikipedia.org/wiki/Quantile_regression
  label: Quantile Regression on Wikipedia
- url: https://arxiv.org/abs/1711.06026
  label: Deep Quantile Regression (Gasthaus et al.)
- url: https://scikit-learn.org/stable/modules/generated/sklearn.linear_model.QuantileRegressor.html
  label: Scikit-learn Quantile Regression
cards:
- id: 5df6cf67-1e5f-41b7-a240-f8a271c18168
  type: flip
  front: How does quantile regression differ from standard regression in terms of loss function and interpretation?
  back: Standard regression minimizes MSE, predicting the conditional mean; sensitive to outliers. Quantile
    regression minimizes pinball loss L_τ(y, ŷ) = (y - ŷ) × max(τ, τ - 1), directly predicting the τ-th
    quantile. For τ=0.5, this is the median (robust). For τ=0.05/0.95, this constructs a 90% interval
    without assuming normality or homoscedasticity.
- id: b18f4bed-22a4-430c-9de6-026675edd5c6
  type: mcq
  front: You train a quantile regression model for τ=[0.05, 0.5, 0.95]. At a test point, predictions are
    ŷ_0.05=8, ŷ_0.5=10, ŷ_0.95=9. What is the issue?
  back: 'Quantile crossing means quantile curves are not monotonic. It happens when τ-quantiles are trained
    independently without ordering constraints. Solutions: (1) train jointly with ordered loss, (2) post-hoc
    quantile sorting, or (3) use quantile regression forests (enforce monotonicity). Crossing intervals
    are invalid for decision-making.'
  choices:
  - key: a
    text: The median is too high
    correct: false
  - key: b
    text: 'Quantile crossing: ŷ_0.95 < ŷ_0.5 violates ordering τ₁ < τ₂ ⟹ ŷ_τ₁ ≤ ŷ_τ₂'
    correct: true
  - key: c
    text: The 90% interval is too narrow
    correct: false
  - key: d
    text: No issue; this is fine
    correct: false
- id: 495e3189-4f5b-4e67-8565-651b0bc23291
  type: flip
  front: Why is a 95% prediction interval often narrower than it should be for real data?
  back: 'Parametric intervals (e.g., ŷ ± 1.96σ) assume normality and constant variance. Real data is often
    heteroscedastic (variance depends on time/input) and non-normal (heavy tails, skew). These violations
    lead to undercoverage: true coverage <95%. Solution: use quantile regression or bootstrap methods
    which are non-parametric and adapt to local volatility.'
- id: 94999b70-8318-497a-9579-b252e37e8d21
  type: flip
  front: In what scenario would you use asymmetric quantiles (e.g., τ=0.8 instead of 0.5) for forecasting?
  back: 'When underforecasting is more costly than overforecasting. E.g., demand forecasting: underestimating
    demand risks stockouts (lost revenue); overestimating risks excess inventory (holding costs). Use
    τ > 0.5 to bias predictions upward. Similarly, for safety-critical systems (energy, water), use conservative
    τ near 0.9. This aligns forecasts with asymmetric business loss.'
---

## Intuition
Point forecasts (single values) are insufficient for decision-making; decision-makers need uncertainty estimates. Confidence/prediction intervals quantify forecast uncertainty: a 95% interval should contain the true value 95% of the time (coverage). Quantile regression directly predicts quantiles (e.g., 5th and 95th percentiles), enabling flexible, multimodal intervals.

## Detail
**Point Forecast:**
ŷ_t = E[y_t | history] (conditional mean), typically from MSE loss
- Simple but discards uncertainty information
- Can be suboptimal for asymmetric loss functions

**Prediction Interval (Classical):**
ŷ_t ± z_α/2 × σ̂
- z_α/2 from standard normal (e.g., 1.96 for 95%)
- σ̂ estimated from residual variance
- Assumes homoscedasticity (constant variance) and normality—often violated

**Quantile Regression:**
Minimize pinball loss: L_τ(y, ŷ) = (y - ŷ) × max(τ, τ - 1)
- For τ = 0.5: predict median (robust to outliers)
- For τ = 0.05, 0.95: predict 5th and 95th percentiles (90% interval)
- Loss asymmetric: underestimation penalized more at high τ, overestimation at low τ
- No distributional assumptions; handles heteroscedasticity

**Deep Quantile Forecasting:**
- Neural network outputs multiple quantiles: [ŷ_0.05, ŷ_0.5, ŷ_0.95]
- Train on quantile loss for each τ separately or jointly
- Captures variable uncertainty across time and input conditions
- Foundation models (Moirai uses mixture distributions) leverage this for flexible intervals

**Bootstrap Intervals:**
- Resample residuals and refit model to build empirical interval distribution
- Non-parametric; works without normality assumption
- Computationally expensive but robust

## Common gotchas / interview framings
- Point forecast minimizing MSE ≠ median quantile; they're different optimization targets
- Prediction intervals vs. confidence intervals: former for y values, latter for parameters
- Quantile crossing: predicted quantiles may violate τ₁ < τ₂ if trained independently; use ordered quantiles
- Coverage gaps: 95% interval from parametric assumption often <95% true coverage if distribution misspecified
- Wider intervals (more uncertainty) aren't always better; must balance coverage vs. width
- Asymmetric loss (e.g., underforecasting costly): use quantile >0.5 to bias high
- Foundation models (Chronos via quantiles, Moirai via mixture) produce probabilistic forecasts natively
- Probabilistic metrics (CRPS, pinball loss) often more relevant than MSE for decision-making

## See also
- [[quantile-regression]]
- [[prediction-interval]]
- [[coverage-probability]]
- [[pinball-loss]]
- [[bootstrap-intervals]]
- [[heteroscedasticity]]

## Sources
See frontmatter `sources:`.

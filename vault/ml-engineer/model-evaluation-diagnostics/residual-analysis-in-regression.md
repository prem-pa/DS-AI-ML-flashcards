---
id: 41296847-0686-49d1-9753-6b9fb47c3f65
title: Residual analysis in regression
track: ml-engineer
topic: model-evaluation-diagnostics
difficulty: 3
tags:
- residuals
- heteroscedasticity
- nonlinearity
- outliers
- diagnostic plots
aliases:
- error distribution
- residual plots
- assumption checking
sources:
- url: https://www.ml4devs.com/what-is/model-evaluation-metrics/
  label: 'ML4Devs: Model Evaluation Metrics'
- url: https://developers.google.com/machine-learning/crash-course/classification/accuracy-precision-recall
  label: Google ML Crash Course
cards:
- id: 72c2699a-07fe-4f22-9fd4-a661e4335813
  type: flip
  front: What does a funnel-shaped Residuals-vs-Fitted plot indicate? How would you fix it?
  back: 'Funnel indicates heteroscedasticity: variance increases with predicted value. Common in proportional/percentage
    data. Fixes: (1) log-transform y (if y > 0), (2) weighted least squares (inverse variance weights),
    (3) robust standard errors (HC3/HC4), (4) quantile regression. Diagnosis: Breusch-Pagan test p<0.05
    confirms.'
- id: 52c62d14-7cec-4a11-ab38-985352452632
  type: flip
  front: Your Q-Q plot shows heavy tails (S-shape). Does this invalidate linear regression?
  back: 'Not necessarily. By CLT, if n > 30, regression coefficients approximately normal even if residuals
    non-normal. Heavy tails matter for: (1) confidence intervals/hypothesis tests (unreliable), (2) prediction
    intervals (too narrow), (3) outlier sensitivity. Solution: robust regression (Huber loss, M-estimation)
    or investigate tail causes (outliers, data entry errors).'
- id: b6d731b9-16f8-412b-a123-cfd53844f01b
  type: mcq
  front: 'A U-shaped Residuals-vs-Fitted plot suggests:'
  back: 'U-shape is classic nonlinearity signal: model underfits a curved relationship. Errors positive
    at low/high fitted values, negative in middle. Solution: add polynomial terms, splines, or GAM. Funnel
    shape (A) indicates variance heteroscedasticity.'
  choices:
  - key: a
    text: Heteroscedasticity
    correct: false
  - key: b
    text: Nonlinearity (underfitting curvature)
    correct: true
  - key: c
    text: Independence violation
    correct: false
  - key: d
    text: Both A and B
    correct: false
- id: e97c87b7-655b-428e-a3df-dab6ee207032
  type: flip
  front: You identify a high-leverage, large-residual point in Residuals-vs-Leverage plot. Should you
    remove it?
  back: 'Not automatically. Investigate first: (1) data entry error? (2) real but extreme observation?
    (3) measurement outlier? Fit model with/without; if coefficient estimates/predictions change dramatically,
    it''s influential. Report robustness analysis. Removal justified only if data quality issue confirmed.'
- id: 377374f5-4904-4c63-9d44-c0f634dfa866
  type: flip
  front: Your time-series residuals show significant ACF at lag-1 (ρ=0.6). What's the issue?
  back: 'Residuals are autocorrelated → violates independence assumption. Standard errors are downward-biased
    (confidence intervals too narrow, p-values too small). Solution: (1) add lagged y or AR term, (2)
    ARIMA/GARCH model, (3) Newey-West robust SE for inference, (4) use differencing to remove trend. ACF
    p<0.05 at lag-k confirms.'
---

## Intuition
Residuals $r_i = y_i - \hat{y}_i$ reveal whether linear regression assumptions hold: normality, homoscedasticity, independence, no leverage. Plots expose model misspecification (nonlinearity), outliers, and data quality issues. Residual analysis is cheap diagnostics before complex troubleshooting.

## Detail
Key residual plots:
1. **Residuals vs Fitted**: y-axis = residuals, x-axis = predicted values. Ideally horizontal scatter (constant variance). Patterns indicate:
   - Funnel shape → heteroscedasticity (variance increases with $\hat{y}$; e.g., percent errors proportional to magnitude)
   - U-shape → nonlinearity; model underfits curvature
   - Horizontal band → good fit

2. **Q-Q Plot (Normal Probability)**: y-axis = sample quantiles, x-axis = theoretical normal. Linear → residuals normal. Heavy tails (S-shape) → outliers. Skew → asymmetry. Deviation ≠ disqualification; regression robust to non-normality if $n$ large.

3. **Scale-Location**: y-axis = $\sqrt{|\text{standardized residuals}|}$, x-axis = fitted. Tests homoscedasticity; same as Residuals-Fitted but standardized.

4. **Residuals vs Leverage**: identifies high-leverage outliers. Leverage = distance from feature space center. High leverage + large residual = influential point (can pivot fit).

5. **ACF plot** (time-series): autocorrelation of residuals. Significant lags → dependence (violates independence assumption).

## Common gotchas / interview framings
- Non-normal residuals acceptable if $n > 30$ (CLT); flag extreme violations (multimodal, heavy tails)
- Heteroscedasticity → standard errors biased → confidence intervals wrong (but coefficient estimates unbiased). Solution: robust SE, weighted regression, or log-transform y
- Outlier removal: never remove outliers without investigation; may be real signals. Robustness checks: fit with/without, report both
- Autocorrelation: critical for time series; invalidates standard errors. Use ARMA/GARCH or include lags
- Nonlinearity fix: polynomial features, splines, GAM, or transformation (log y, √y)

## See also
- [[heteroscedasticity]]
- [[residual-plot]]
- [[q-q-plot]]
- [[model-assumptions]]
- [[outlier-detection]]

## Sources
See frontmatter `sources:`.

---
id: 4d25dfa1-e00d-4e7a-a8ef-1ba024710db6
title: Regression metrics (MAE, MSE, RMSE, R-squared, quantile loss)
track: ml-engineer
topic: model-evaluation-diagnostics
difficulty: 3
tags:
- regression
- loss
- outliers
- asymmetric
- R-squared
- quantile
aliases:
- mean absolute error
- mean squared error
- heteroscedasticity
sources:
- url: https://www.ml4devs.com/what-is/model-evaluation-metrics/
  label: 'ML4Devs: Model Evaluation Metrics'
- url: https://developers.google.com/machine-learning/crash-course/classification/accuracy-precision-recall
  label: Google ML Crash Course
cards:
- id: 2ddbcf14-a6a2-4101-a9d4-e6119de9c623
  type: flip
  front: Why is RMSE more sensitive to outliers than MAE? Give a numerical example.
  back: 'Errors squared: a single error of 10 contributes 100 to MSE but only 10 to MAE sum. If most errors
    are 1 but one is 10: MAE = (99·1 + 1·10)/100 = 1.09, RMSE = √(99·1 + 1·100)/100 = √1.99 ≈ 1.41. Outliers
    dominate squared loss. Use MAE for noisy measurement, RMSE to penalize rare large mistakes.'
- id: 862fcfb4-ede8-45b1-9ac9-0c0ee5e2a26e
  type: flip
  front: What does R² = 0.7 mean? What about R² = -0.2?
  back: 'R² = 0.7: model explains 70% of variance; 30% unexplained (residual). R² = -0.2: model performs
    *worse than predicting the mean*—it''s adding noise. Negative R² signals model misspecification or
    overfitting to noise. Always check train/test R² split.'
- id: 8bc2cf84-32b9-4a80-a47c-8128d7bf6475
  type: mcq
  front: For demand forecasting, predicting too high (excess inventory) costs 2x more than predicting
    too low (stockout). Which metric?
  back: Quantile τ=0.33 penalizes overestimation (positive residual) 1x and underestimation 2x, matching
    costs. τ ∈ [0,1] where τ > 0.5 penalizes underestimation more (conservative). MSE/MAE symmetric; quantile
    is flexible for asymmetric loss.
  choices:
  - key: a
    text: MSE
    correct: false
  - key: b
    text: MAE
    correct: false
  - key: c
    text: Quantile loss with τ=0.33
    correct: true
  - key: d
    text: Quantile loss with τ=0.67
    correct: false
- id: 38c5390a-b515-43b8-b180-d6226e201e8b
  type: flip
  front: When would you prefer RMSE over MAE in regression evaluation?
  back: 'RMSE when: (1) large errors are especially costly (e.g., system failures, safety-critical), (2)
    outliers represent real signals, not noise, (3) residuals ~normal (RMSE has MLE interpretation). Prefer
    MAE when: (1) outliers are measurement errors, (2) interpretability matters (units of y), (3) data
    has heavy tails.'
- id: 4ab869f6-01ae-4421-8893-218afacdc2c0
  type: flip
  front: Your model has R² = 0.85 on train, R² = 0.4 on test. Diagnose the issue.
  back: 'Large train-test gap signals overfitting. Model memorized noise in training data. Diagnoses:
    (1) model complexity too high (reduce features, regularization), (2) insufficient training data (increase
    samples), (3) data leakage (temporal, information). Check residual plot for systematic patterns (nonlinearity,
    heteroscedasticity).'
---

## Intuition
Regression metrics measure prediction magnitude and direction of error. Mean-based metrics (MSE, RMSE) penalize outliers heavily; median-based (MAE) ignore them. R² measures fit quality relative to baseline. Quantile loss enables asymmetric costs (e.g., predicting low demand costs more than predicting high).

## Detail
For residuals $r_i = y_i - \hat{y}_i$:
- **MAE** = $\frac{1}{n}\sum|r_i|$ → robust to outliers, scale-independent interpretation
- **MSE** = $\frac{1}{n}\sum r_i^2$ → penalizes large errors quadratically
- **RMSE** = $\sqrt{\text{MSE}}$ → same units as $y$, emphasizes outliers harder than MAE
- **R²** = $1 - \frac{\sum r_i^2}{\sum(y_i - \bar{y})^2}$ → fraction of variance explained (0 to 1); R²=0.5 means model explains 50% of variance
- **Quantile loss (τ)** = $\sum \mathbb{1}[r_i \geq 0] \cdot \tau |r_i| + \mathbb{1}[r_i < 0] \cdot (1-\tau)|r_i|$ → asymmetric: overestimate costly if τ>0.5

RMSE vs MAE: RMSE increases nonlinearly with outlier magnitude; MAE linear. On clean data, RMSE and MAE rank models similarly. With outliers, MAE ranks differently—prefer MAE if outliers are real noise, RMSE if they're signals to minimize.

## Common gotchas / interview framings
- R² pitfall: R²<0 possible if model worse than horizontal baseline; unbounded above 1 with extrapolation
- Scale sensitivity: MSE and RMSE not comparable across different target scales; use normalized metrics (NRMSE, MAPE)
- Quantile loss asymmetry: τ=0.5 is MAE; τ=0.9 penalizes underestimation 9x more, overestimation 1x more
- Outlier bias: single outlier can dominate MSE/RMSE; invisible in MAE; use robust metrics (MAPE, Huber loss) or preprocess
- Learning curves: RMSE typically decreases with training size; if plateau, data quality or model capacity is bottleneck

## See also
- [[mean-absolute-error]]
- [[mean-squared-error]]
- [[outlier-robustness]]
- [[asymmetric-loss]]
- [[coefficient-of-determination]]

## Sources
See frontmatter `sources:`.

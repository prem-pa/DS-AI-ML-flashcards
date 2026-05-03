---
id: b76192d8-2c8e-4aba-84f8-c95b4a6efc65
title: ARIMA and exponential smoothing
track: verticals
topic: time-series-forecasting
difficulty: 3
tags:
- classical
- stationary
- autoregression
- moving-average
- forecasting
- decomposition
aliases:
- ARIMA(p,d,q)
- exponential smoothing
- state space
- SARIMA
sources:
- url: https://en.wikipedia.org/wiki/Autoregressive_integrated_moving_average
  label: ARIMA on Wikipedia
- url: https://www.otexts.com/fpp2/arima.html
  label: 'Forecasting: Principles and Practice - ARIMA'
- url: https://www.statsmodels.org/stable/generated/statsmodels.tsa.arima.model.ARIMA.html
  label: Statsmodels ARIMA
- url: https://www.otexts.com/fpp2/expsmooth.html
  label: Exponential Smoothing Methods
cards:
- id: d694b548-e4b1-4c91-8900-d605e15bf20f
  type: flip
  front: What does the 'd' component in ARIMA(p,d,q) represent and why is it needed?
  back: 'The ''d'' (integrated) component represents the number of times the series must be differenced
    to achieve stationarity. Differencing removes trends: ∇y_t = y_t - y_{t-1}. It''s needed because ARIMA''s
    AR and MA components assume a stationary series; non-stationary series violate this assumption, breaking
    parameter estimation and inference.'
- id: 9b5ff69b-9cbf-4497-91d4-dbf263aa5ec8
  type: mcq
  front: You fit ARIMA(1,1,1) to a revenue series with d=1. What does this mean about the series?
  back: d=1 means the original series is I(1)—integrated of order 1—and requires one differencing operation
    to stationarize. This typically indicates a random walk or trending behavior (unit root). d=0 would
    mean the series is already stationary; d=2 would require second-order differencing (rare).
  choices:
  - key: a
    text: The series is stationary without differencing
    correct: false
  - key: b
    text: The series requires one difference to become stationary; it has a unit root
    correct: true
  - key: c
    text: The series has 1 seasonal period
    correct: false
  - key: d
    text: The forecast is a simple average of past errors
    correct: false
- id: 9abee63d-a1c9-4382-9604-dcc048317317
  type: flip
  front: How does exponential smoothing differ from ARIMA in handling trend and seasonality?
  back: 'Exponential smoothing uses explicit components: Simple ES for level only, Holt''s method adds
    a trend component estimated recursively, Holt-Winters adds seasonality. ARIMA achieves the same via
    differencing (for trend) and AR/MA terms. ES produces natural confidence intervals from state-space
    formulation; ARIMA confidence intervals grow wider with horizon.'
- id: 1eb875bc-fb3b-4105-afeb-3d15eb2d014f
  type: flip
  front: Name one key limitation of ARIMA/exponential smoothing and how modern methods address it.
  back: 'Key limitation: assumes linear relationships and fails with structural breaks or nonlinearity.
    Modern deep learning methods (LSTM, Transformer, N-BEATS) learn nonlinear patterns from data. Foundation
    models (Chronos, Moirai) leverage pretraining on large diverse corpora to capture complex temporal
    dynamics zero-shot, without manual stationarity testing or hyperparameter selection.'
---

## Intuition
ARIMA and exponential smoothing are foundational classical methods for time series forecasting that assume linear patterns and stationarity (or require differencing to achieve stationarity). ARIMA decomposes dynamics into autoregressive (AR), integrated (I, differencing), and moving-average (MA) components. Exponential smoothing assigns exponentially decreasing weights to past observations, with variants handling trend and seasonality.

## Detail
**ARIMA(p,d,q):**
- **p (AR)**: Autoregressive order—forecast depends on previous values
- **d (I)**: Differencing order—how many times to difference to achieve stationarity
- **q (MA)**: Moving-average order—forecast depends on past forecast errors

**Key Steps:**
1. Check stationarity (ADF test)
2. Difference if needed (d ≥ 1)
3. Identify ACF/PACF patterns to select p, q
4. Fit and validate
5. SARIMA extends with seasonal components (P,D,Q,s)

**Exponential Smoothing:**
- Simple: ŷ_{t+1} = α·y_t + (1-α)·ŷ_t (α ∈ [0,1])
- Holt (trend): adds trend component
- Holt-Winters (seasonal): adds both trend and seasonal components
- Provides closed-form confidence intervals via state-space representation

## Common gotchas / interview framings
- ARIMA assumes linearity and stationarity; fails on structural breaks or strongly nonlinear data
- Over-differencing reduces forecast accuracy; d=1 or d=2 usually sufficient
- ACF/PACF selection of (p,q) is subjective; grid search + AIC/BIC more reliable
- Seasonal period *s* must be set correctly; monthly data typically s=12
- Exponential smoothing can produce unintuitive confidence bands if α chosen poorly
- Neither handles regime changes or external regressors naturally (ARIMAX for exogenous)
- Forecast accuracy degrades rapidly at long horizons due to error accumulation

## See also
- [[autoregressive-models]]
- [[stationarity]]
- [[differencing]]
- [[moving-averages]]
- [[seasonal-adjustment]]
- [[box-jenkins]]
- [[exponential-smoothing-variants]]
- [[sarima]]

## Sources
See frontmatter `sources:`.

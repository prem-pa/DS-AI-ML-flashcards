---
id: 70471acf-614a-453e-a818-71c1116daf5f
title: Seasonality and trend decomposition
track: verticals
topic: time-series-forecasting
difficulty: 3
tags:
- decomposition
- trend
- seasonality
- STL
- classical
- preprocessing
aliases:
- additive/multiplicative decomposition
- STL decomposition
- seasonal extraction
- trend-season-residual
sources:
- url: https://otexts.com/fpp2/classical-decomposition.html
  label: 'Forecasting: Principles and Practice - Decomposition'
- url: https://www.statsmodels.org/stable/generated/statsmodels.tsa.seasonal.STL.html
  label: Statsmodels STL
- url: https://en.wikipedia.org/wiki/Seasonal_decomposition
  label: Seasonal Decomposition on Wikipedia
- url: https://otexts.com/fpp2/stl.html
  label: STL Method for Time Series
cards:
- id: 03761fb8-5cf9-4c77-aaf6-a4d9b1dcce80
  type: flip
  front: When would you use additive vs. multiplicative decomposition?
  back: 'Additive (y = T + S + E) when seasonal variation is constant over time regardless of trend level.
    Multiplicative (y = T × S × E) when seasonal variation scales with the trend—common in economics,
    sales, and finance where higher baselines lead to larger seasonal swings. Plot the series: if seasonal
    peaks grow over time, use multiplicative.'
- id: 6d255a74-2056-4eee-95eb-3dd04adae282
  type: mcq
  front: You decompose a retail sales series and get trend T_t, seasonality S_t, and residuals E_t. What
    is the next reasonable step?
  back: Actually, (a) is correct—forecast each component independently (trend with ARIMA, seasonality
    with seasonal indices, residuals with a residual model or as noise) and add/multiply them back. This
    divide-and-conquer approach often outperforms single-series forecasting, especially for series with
    strong, stable seasonality. But standard practice focuses on modeling trend and seasonality; residuals
    are often white noise.
  choices:
  - key: a
    text: Forecast each component independently and recombine
    correct: false
  - key: b
    text: Use only T_t for forecasting; discard S_t and E_t
    correct: false
  - key: c
    text: Train one ARIMA on the full series ignoring decomposition
    correct: false
  - key: d
    text: Average T_t and S_t and ignore E_t
    correct: true
- id: 6b450b55-5081-4b44-93cf-190cf4c0b37b
  type: flip
  front: What is STL decomposition and when would you prefer it over classical decomposition?
  back: STL (Seasonal and Trend using Loess) iteratively applies LOWESS smoothing to extract trend and
    seasonality. It's more robust to outliers, handles variable seasonality, and allows flexible control
    via seasonal/trend window widths. Use it when your series has irregular patterns, outliers, or seasonality
    that changes over time. Classical decomposition is simpler but assumes constant seasonal pattern and
    is sensitive to outliers.
- id: 4870b5d6-58cc-4acc-9e4e-ab0c6e29338b
  type: flip
  front: How does decomposition relate to modern deep learning approaches for time series?
  back: Deep models (LSTM, Transformers, N-BEATS) implicitly learn trend and seasonality from data without
    explicit decomposition. N-BEATS in interpretable mode explicitly reconstructs trend (polynomial basis)
    and seasonality (harmonic basis) but in a learnable way. Foundation models (Chronos, Moirai) trained
    on massive diverse datasets implicitly capture seasonality patterns, making manual decomposition often
    unnecessary.
---

## Intuition
Many time series exhibit recurring patterns (seasonality) overlaid on a long-term trend, plus noise. Decomposition separates these components: y_t = T_t + S_t + E_t (additive) or y_t = T_t × S_t × E_t (multiplicative). Modeling each separately often yields better forecasts than fitting the original series directly.

## Detail
**Classical Decomposition:**
- Apply moving average to estimate trend T_t
- Compute detrended series: y_t - T_t
- Average seasonal indices by season to extract S_t
- Residuals: E_t = y_t - T_t - S_t
- Choose additive (constant seasonal magnitude) or multiplicative (seasonal magnitude proportional to trend)

**STL (Seasonal and Trend decomposition using Loess):**
- Iteratively applies LOWESS (locally weighted scatterplot smoothing) for robustness
- Parameters: seasonal window (controls seasonal smoothness), trend window (controls trend smoothness)
- Handles irregular seasonality better than classical method
- Robust to outliers

**Usage:**
1. Decompose series into components
2. Forecast each component independently (e.g., ARIMA for trend, seasonal indices for seasonality)
3. Recombine forecasts
4. Often reduces RMSE vs. modeling full series

## Common gotchas / interview framings
- Additive vs. multiplicative choice matters: plot series and check if seasonal amplitude grows with level
- Decomposition requires full historical seasonality; unreliable with <2 full seasonal cycles
- STL parameters require tuning; default values may smooth too much or too little
- Forecasting components separately doesn't guarantee best overall forecast if components interact
- Anomalies in one component can corrupt estimates of others; STL's robustness helps
- Deep models (Transformers, N-BEATS) implicitly learn decomposition; explicit decomposition redundant for them
- Modern foundation models handle seasonality through pretraining, avoiding manual extraction

## See also
- [[additive-models]]
- [[multiplicative-models]]
- [[stl-decomposition]]
- [[seasonal-adjustment]]
- [[x-11]]
- [[lowess]]
- [[trend-estimation]]
- [[residual-analysis]]

## Sources
See frontmatter `sources:`.

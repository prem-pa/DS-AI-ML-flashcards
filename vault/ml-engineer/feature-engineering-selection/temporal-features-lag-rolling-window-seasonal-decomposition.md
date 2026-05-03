---
id: 03b5129f-ed3f-4027-8af5-1c77f47fa378
title: Temporal features (lag, rolling window, seasonal decomposition)
track: ml-engineer
topic: feature-engineering-selection
difficulty: 3
tags:
- time-series
- temporal-patterns
- lagged-features
- rolling-aggregates
- seasonality
aliases:
- lag-features
- rolling-window-features
- seasonal-indicators
- time-based-features
sources:
- url: https://sebastianraschka.com/faq/docs/feature_sele_categories.html
  label: Sebastian Raschka - Feature Selection and Temporal Data
- url: https://www.enjoyalgorithms.com/blog/feature-selection-techniques/
  label: Enjoy Algorithms - Feature Selection Techniques
- url: https://www.frontiersin.org/journals/big-data/articles/10.3389/fdata.2025.1624507/pdf
  label: Frontiers - Feature Selection Review
cards:
- id: ebcbfc0c-03f4-44aa-bc0c-e97568f107e6
  type: flip
  front: Why should you use sin/cos encoding for cyclic features (hour, day-of-week) instead of categorical
    one-hot?
  back: 'Cyclical encoding preserves circularity: hour 23 and hour 0 are adjacent, not opposite. sin/cos
    pairs map cyclic features to a 2D plane where distance reflects temporal distance. One-hot ignores
    the wrap-around, creating spurious discontinuities.'
- id: 2c9b4540-fb72-4114-8ed2-bdf54b256235
  type: flip
  front: What is look-ahead bias in time-series feature engineering, and how do you avoid it?
  back: 'Look-ahead bias occurs if you use future information (e.g., lag_-1 instead of lag_1, or rolling
    average including future points). Avoid by: (1) only shift backward (shift(positive)), (2) use expanding
    windows, (3) strictly separate train/test by time cutoff.'
- id: 6d03a5f0-ccfe-41c2-8b56-6ba7d373eead
  type: mcq
  front: When engineering lag features for hourly data, why might lag_1 (1 hour) alone be insufficient?
  back: 'Time-series often have multiple periodicities: daily, weekly, seasonal. Lag_24 captures diurnal
    patterns, lag_168 captures weekly. Multiple lags + rolling windows + seasonal decomposition together
    model the full temporal structure.'
  choices:
  - key: a
    text: Lag_1 is always sufficient; lags beyond 1 introduce noise
    correct: false
  - key: b
    text: Hourly patterns may repeat at lag_24 (daily) and lag_168 (weekly); multiple lags capture these
      cycles
    correct: true
  - key: c
    text: Lag_1 is too slow; you need lag_0.5 for intra-hour patterns
    correct: false
  - key: d
    text: Lags destroy model interpretability
    correct: false
- id: 6b2cd4a4-0cc2-447a-92a3-d521cf6604c8
  type: flip
  front: How would you handle missing values created by lag features at the start of a time series?
  back: 'Options: (1) drop first k rows (accept data loss), (2) forward-fill or interpolate lags (introduces
    bias if many missing), (3) use seasonal naive lags only after sufficient history, (4) impute with
    mean/median of available history. Choose based on data volume and model sensitivity.'
- id: a5d6c4ad-4f41-47d6-95d2-7efe079fd9e4
  type: mcq
  front: A sales forecast uses lag_1 but still misses spikes. Which feature would you add next?
  back: Rolling statistics capture trend and volatility, which lags alone miss. Lag_1 is a point estimate;
    rolling mean/std smooth and reveal patterns. Together they model both short-term momentum and recent
    context.
  choices:
  - key: a
    text: Lag_2 and lag_3 to capture higher-order dependencies
    correct: false
  - key: b
    text: Rolling mean and std to capture recent trend and volatility
    correct: true
  - key: c
    text: Random polynomial features
    correct: false
  - key: d
    text: A second lag_1 column (duplicate)
    correct: false
---

## Intuition

Time-series models depend on history and patterns. Lags (prior values), rolling statistics (7-day average), and seasonal indicators encode temporal dependencies that raw timestamps don't capture. A sales forecast needs yesterday's sales; a traffic prediction needs the same hour from last week; a utility load model needs seasonal (summer vs. winter) and cyclical (weekday vs. weekend) patterns.

## Detail

**Lag features**: y_t-1, y_t-7 (autoregressive structure). Essential for modeling momentum and inertia.

**Rolling windows**: rolling mean/std/min/max over k periods (7-day, 30-day). Captures trends and volatility.

**Seasonal indicators**: hour-of-day, day-of-week, month, holidays (one-hot or cyclical encoding). Captures repeating patterns.

**Seasonal decomposition**: trend, seasonal, residual components via STL or classical decomposition. Use trend and seasonal as engineered features.

**Cyclical encoding**: Convert cyclic features (hour 0-23) to sin/cos pairs to preserve circularity: sin(2π*hour/24), cos(2π*hour/24).

```python
import pandas as pd
from statsmodels.tsa.seasonal import seasonal_decompose

# Lag features
df['sales_lag_1'] = df['sales'].shift(1)
df['sales_lag_7'] = df['sales'].shift(7)

# Rolling features
df['sales_rolling_7'] = df['sales'].rolling(7).mean()
df['sales_rolling_std'] = df['sales'].rolling(7).std()

# Cyclical encoding
df['hour_sin'] = np.sin(2 * np.pi * df['hour'] / 24)
df['hour_cos'] = np.cos(2 * np.pi * df['hour'] / 24)

# Seasonal decomposition
decomposed = seasonal_decompose(df['sales'], model='additive', period=7)
df['trend'] = decomposed.trend
df['seasonal'] = decomposed.seasonal
```

## Common gotchas / interview framings
- "How do you prevent look-ahead bias when engineering lag features?"
- "Why is cyclical encoding (sin/cos) better than categorical encoding for hour-of-day?"
- "What happens to lags at the start of a time series (missing values)?"
- "Describe a situation where a single lag feature failed and rolling statistics succeeded."

## See also
- [[time-series]]
- [[autocorrelation]]
- [[seasonal-decomposition]]
- [[stationarity]]
- [[forecast-horizons]]
- [[feature-engineering]]

## Sources
See frontmatter `sources:`.

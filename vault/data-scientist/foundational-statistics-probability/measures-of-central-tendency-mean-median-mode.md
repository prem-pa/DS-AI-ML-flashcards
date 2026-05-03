---
id: eeff23aa-a424-4d05-a3fb-86a4210372e6
title: Measures of central tendency (mean, median, mode)
track: data-scientist
topic: foundational-statistics-probability
difficulty: 1
tags:
- descriptive-statistics
- location-parameter
- data-summary
- robust-statistics
- outliers
aliases:
- central-location
- point-of-centrality
- central-values
sources:
- url: https://ocw.mit.edu/courses/18-05-introduction-to-probability-and-statistics-spring-2022/
  label: 'MIT 18.05: Introduction to Probability and Statistics'
- url: https://link.springer.com/book/10.1007/978-0-387-21736-9
  label: Wasserman, L. All of Statistics (Ch. 2)
cards:
- id: c91dd6c4-916b-4dca-8f43-cd089297572e
  type: flip
  front: Why is the sample mean $\bar{x}$ the optimal unbiased estimator under squared loss?
  back: ''
- id: 18504e12-bede-480b-a547-d54e23f437b7
  type: flip
  front: When analyzing salary data, the mean is \$95k but the median is \$65k. What does this tell you?
  back: ''
- id: 32d39f18-a72e-4048-ae6b-4c0813497ab0
  type: flip
  front: Explain Simpson's Paradox with central tendency measures.
  back: ''
- id: 991057b5-52a0-4b65-84f7-5fe6325db08c
  type: mcq
  front: Which measure of central tendency is least affected by a single extreme outlier?
  back: ''
  choices:
  - key: a
    text: Mean
    correct: false
  - key: b
    text: Median
    correct: true
  - key: c
    text: Mode
    correct: false
  - key: d
    text: Range
    correct: false
- id: 3bc1d2e8-ad18-48e8-aa6e-879144940109
  type: mcq
  front: For a bimodal distribution, which statistic might be misleading?
  back: ''
  choices:
  - key: a
    text: Mode
    correct: false
  - key: b
    text: Median
    correct: false
  - key: c
    text: Mean
    correct: true
  - key: d
    text: All of the above
    correct: false
---

## Intuition
Measures of central tendency answer: where is the "center" of the data? The mean is the arithmetic average (sensitive to outliers); the median is the middle value (robust to extremes); the mode is the most frequent value (useful for categorical data). In ML contexts, choosing the right measure determines whether you're detecting typical behavior or protecting against asymmetric distributions.

## Detail
The **mean** $\bar{x} = \frac{1}{n}\sum_{i=1}^{n} x_i$ is optimal under squared loss and is unbiased but can be pulled by extreme values. The **median** partitions ordered data at the 50th percentile, minimizing absolute deviations and having breakdown point 50% (more robust). The **mode** identifies the highest-density region—critical for multimodal distributions where mean/median mask the true structure.

For skewed distributions (e.g., income, time-to-event), mean > median (right skew) or mean < median (left skew). This tells you whether your model's expectations (often using mean) align with typical observations (median).

## Common gotchas / interview framings
- The mean can be non-representative in skewed data; Simpson's paradox shows aggregated means can reverse direction from within-group means
- Median is not affected by extreme values but is harder to work with mathematically
- "Center" depends on your loss function: L₂ → mean, L₁ → median, L₀ → mode

## See also
- [[mean]]
- [[median]]
- [[mode]]
- [[outlier-detection]]
- [[robust-statistics]]

## Sources
See frontmatter `sources:`.

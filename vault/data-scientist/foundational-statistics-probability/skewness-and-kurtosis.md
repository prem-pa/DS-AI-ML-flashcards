---
id: 3165f4c9-fab6-49d8-bbb8-d8c1b4005c14
title: Skewness and kurtosis
track: data-scientist
topic: foundational-statistics-probability
difficulty: 3
tags:
- distribution-shape
- higher-moments
- non-normality
- model-diagnostics
- tail-behavior
aliases:
- asymmetry
- tail-weight
- distribution-moments
sources:
- url: https://www.statlearning.com/
  label: James et al. An Introduction to Statistical Learning
- url: https://hastie.su.domains/ElemStatLearn/
  label: Hastie, Tibshirani, Friedman. Elements of Statistical Learning (Ch. 2)
cards:
- id: 3364f66e-aae4-46c9-bab9-347b72e40853
  type: flip
  front: A dataset has skewness = 1.5 and kurtosis = 4. Is it close to normal? What issues might arise
    in hypothesis testing?
  back: ''
- id: f1233584-a322-4f7c-ad98-8fea30b215b0
  type: flip
  front: Write the formula for skewness and explain why it's zero for a symmetric distribution.
  back: ''
- id: a937c0b2-cac1-4e4f-ada1-7d008e161e29
  type: flip
  front: You fit a linear regression and the residuals show excess kurtosis = 5. What does this indicate?
  back: ''
- id: 011980c9-4ade-4df1-9fe7-b14f71a4a628
  type: mcq
  front: Right-skewed income data often has skewness > 0 and high kurtosis. Which transformation typically
    helps?
  back: ''
  choices:
  - key: a
    text: Square
    correct: false
  - key: b
    text: Exponential
    correct: false
  - key: c
    text: Log
    correct: true
  - key: d
    text: Inverse
    correct: false
- id: 1130e2de-e04b-4677-89a6-7eb632239a27
  type: mcq
  front: 'Excess kurtosis > 0 means:'
  back: ''
  choices:
  - key: a
    text: Lighter tails than normal
    correct: false
  - key: b
    text: Fatter tails than normal
    correct: true
  - key: c
    text: More symmetric
    correct: false
  - key: d
    text: Bimodal
    correct: false
---

## Intuition
Skewness measures asymmetry: right-skewed distributions have a long tail on the right (mean > median), left-skewed have it on the left. Kurtosis measures tail weight relative to a normal: high kurtosis means fatter tails (more extreme values), low kurtosis means lighter tails. Both shape hypothesis test validity and affect model assumptions (e.g., linear regression assumes normality).

## Detail
Skewness: $\gamma_1 = E\left[\left(\frac{X - \mu}{\sigma}\right)^3\right]$. For normal data, $\gamma_1 \approx 0$; right skew $> 0$; left skew $< 0$. Excess kurtosis: $\gamma_2 = E\left[\left(\frac{X - \mu}{\sigma}\right)^4\right] - 3$. Normal has $\gamma_2 = 0$; high $\gamma_2$ (leptokurtic, fat tails); low $\gamma_2$ (platykurtic, thin tails).

These are 3rd and 4th standardized moments. Real datasets often deviate from normality: income and firm sizes are right-skewed; financial returns have high kurtosis (crash risk). Non-normality can invalidate t-tests, confidence intervals, and regression inference unless sample size is large (CLT rescue).

## Common gotchas / interview framings
- Skewness $\neq$ multimodality; a unimodal distribution can be heavily skewed
- High kurtosis is often called "leptokurtic" (peaked + fat tails) vs. normal's mesokurtic shape
- Sample skewness/kurtosis are noisy estimators; use formal tests (Shapiro-Wilk, Anderson-Darling) for small n
- Transformations (log, square root, Box-Cox) can reduce skewness and improve model fit

## See also
- [[skewness]]
- [[kurtosis]]
- [[normality-tests]]
- [[higher-moments]]
- [[tail-risk]]

## Sources
See frontmatter `sources:`.

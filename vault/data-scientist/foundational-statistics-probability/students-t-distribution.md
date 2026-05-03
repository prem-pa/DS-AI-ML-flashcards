---
id: 8eeb9e61-9306-4905-adb3-661379119d88
title: Student's t-distribution
track: data-scientist
topic: foundational-statistics-probability
difficulty: 3
tags:
- inference-distribution
- small-sample
- t-test
- robust-estimator
- degrees-of-freedom
aliases:
- t-distribution
- student-t
- heavy-tails
sources:
- url: https://ocw.mit.edu/courses/18-05-introduction-to-probability-and-statistics-spring-2022/
  label: 'MIT 18.05: Introduction to Probability and Statistics'
- url: https://link.springer.com/book/10.1007/978-0-387-21736-9
  label: Wasserman, L. All of Statistics (Ch. 7-9)
cards:
- id: eebbbd2d-79a1-4597-94d9-2829c4a5a796
  type: flip
  front: When should you use t-test vs. z-test for comparing a sample mean to a population mean?
  back: ''
- id: 1e97b6b7-db78-4b16-a139-fdbe95ae3cf3
  type: flip
  front: Explain why T = (X̄ - μ)/(s/√n) follows a t-distribution with n-1 degrees of freedom.
  back: ''
- id: 112f5b27-eab6-47f5-8e3f-a3ac428bf8b0
  type: flip
  front: 'You compare two groups: n₁ = 10, n₂ = 50. Should you use standard t-test or Welch''s t-test?'
  back: ''
- id: d16e060c-85ae-41cb-be9c-472a2e6dcd99
  type: flip
  front: A one-sample t-test on n = 5 observations gives t = 2.0, p-value = 0.10 (two-tailed). What does
    this mean for practical significance?
  back: ''
- id: 17104d01-0fb0-4f7a-8db8-8435536ccb00
  type: mcq
  front: 'As sample size n increases, the t-distribution approaches:'
  back: ''
  choices:
  - key: a
    text: Chi-square
    correct: false
  - key: b
    text: Normal
    correct: true
  - key: c
    text: Exponential
    correct: false
  - key: d
    text: Uniform
    correct: false
---

## Intuition
The t-distribution is a fat-tailed version of the normal, used for inference when sample size is small and variance is estimated. If you have small n, the sample standard deviation s is noisy (underestimates σ), so normal-based confidence intervals are too narrow. The t-distribution has heavier tails (higher probability of extreme values) to account for this extra uncertainty. As n → ∞, t → normal. It is the basis of t-tests (comparing means) and t-based confidence intervals, which are valid even for non-normal data if n is moderately large (CLT).

## Detail
Student's t with ν degrees of freedom: $f(t) = \frac{\Gamma(\frac{\nu+1}{2})}{\sqrt{\nu\pi}\Gamma(\frac{\nu}{2})} \left(1 + \frac{t^2}{\nu}\right)^{-(\nu+1)/2}$. Mean = 0, variance = $\nu/(\nu-2)$ for $\nu > 2$. As $\nu \to \infty$, variance → 1 (standard normal).

Arises when sampling from normal: if $X_1, \ldots, X_n \sim N(\mu, \sigma^2)$ are iid, then $T = \frac{\bar{X} - \mu}{s/\sqrt{n}} \sim t_{n-1}$ where $s^2 = \frac{1}{n-1}\sum(X_i - \bar{X})^2$. This is the one-sample t-test statistic. The degrees of freedom ν = n - 1 account for estimating the mean (loss of 1 degree of freedom).

Two-sample t-test for comparing means of independent groups uses $t_{n_1+n_2-2}$ (or Welch's adjustment for unequal variances). Paired t-test uses differences and has ν = n - 1 (n pairs).

## Common gotchas / interview framings
- t-test assumes normality; for large n, CLT rescues you (Central Limit Theorem applies), but for small n and severe non-normality, use Mann-Whitney U (nonparametric)
- Degrees of freedom (ν) determine tail weight; ν = 1 (Cauchy, undefined mean/variance), ν = 30 (very close to normal). Small ν gives wider CIs, protecting against small-sample noise
- Unequal variances violate t-test assumption; use Welch's t-test (adjusts SE for unequal variance) instead of standard t-test
- Multiple t-tests (p-hacking) inflate type-I error; use Bonferroni or false discovery rate (FDR) correction

## See also
- [[t-distribution]]
- [[t-test]]
- [[degrees-of-freedom]]
- [[confidence-intervals]]
- [[normality]]

## Sources
See frontmatter `sources:`.

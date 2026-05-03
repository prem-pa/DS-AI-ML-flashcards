---
id: 2c001793-6d2f-4b95-b4ea-9504655e9d71
title: Bootstrap methods
track: data-scientist
topic: hypothesis-testing-inference
difficulty: 3
tags:
- bootstrap
- resampling
- nonparametric
- confidence intervals
- bias estimation
aliases:
- resampling
- bootstrap confidence intervals
- percentile bootstrap
sources:
- url: https://stat20.berkeley.edu/spring-2024/4-generalization/04-hypothesis-tests/notes.html
  label: 'Berkeley Stat 20: Bootstrap and Resampling Methods'
- url: https://online.stat.psu.edu/statprogram/reviews/statistical-concepts/hypothesis-testing/p-value-approach
  label: 'Penn State STAT: Nonparametric Bootstrap'
- url: https://stats.libretexts.org/Bookshelves/Applied_Statistics/Mikes_Biostatistics_Book_(Dohm)/08:_Inferential_Statistics/8.1:_The_null_and_alternative_hypotheses
  label: 'LibreTexts: Bootstrap Estimation Methods'
cards:
- id: 617a6419-88ba-42de-b88e-cd0ccd6ea383
  type: flip
  front: Explain the bootstrap resampling procedure in plain language.
  back: 'From your sample of $n$ observations: (1) Draw $n$ observations *with replacement*, creating
    a bootstrap sample. (2) Compute your statistic (mean, median, etc.) on the bootstrap sample. (3) Repeat
    steps 1–2 many times (e.g., 10,000). (4) Look at the distribution of bootstrap statistics—it approximates
    how the true statistic varies across samples. This mimics what would happen if you collected many
    real samples.'
- id: 213cc18d-83d7-44c3-927c-39bbf683f2f2
  type: flip
  front: Why is the bootstrap "distribution-free" and what does this mean for practice?
  back: The bootstrap does not assume the data follow a normal, exponential, or any other parametric distribution.
    Instead, it uses the empirical distribution of the observed data as a proxy for the true population
    distribution. In practice, this means you can compute CIs and standard errors for any statistic (mean,
    median, correlation, regression slope) without worrying about whether assumptions like normality hold.
- id: 38f710a9-40d0-4256-ad41-7cf06b10f9ff
  type: mcq
  front: You compute a 95% bootstrap CI using the percentile method and get [0.35, 0.72] for the median.
    You notice the distribution of bootstrap medians is right-skewed. Which statement is true?
  back: ''
  choices:
  - key: a
    text: The CI is symmetric around the observed median; bootstrap doesn't preserve shape.
    correct: false
  - key: b
    text: The CI may be asymmetric around the point estimate, which is correct and reflects the skewness
      of the sampling distribution.
    correct: true
  - key: c
    text: The percentile method is invalid for skewed data; use parametric normality instead.
    correct: false
  - key: d
    text: The bootstrap has failed; collect more data.
    correct: false
- id: fb21cf0e-a0d8-4b8c-9649-826ec73c8f91
  type: flip
  front: When would you use bootstrap vs. parametric methods for confidence intervals?
  back: 'Use **bootstrap** when: (1) data are non-normal, (2) the statistic of interest is complex (e.g.,
    trimmed mean, median, ratio), or (3) parametric assumptions are unclear. Use **parametric** when:
    (1) strong prior knowledge supports a distribution (e.g., binary outcome → binomial), (2) theoretical
    efficiency is needed, (3) sample size is very small ($n < 10$, bootstrap unreliable). For $n \geq
    30$ and complex statistics, bootstrap is often safer.'
- id: 4315e1f0-2a27-465e-89c3-78159d72a6ac
  type: flip
  front: 'Design scenario: You estimate the median customer lifetime value (CLV) from 100 transactions.
    Why is bootstrap ideal here, and what potential caveat should you mention?'
  back: 'Bootstrap is ideal because CLV may be right-skewed (a few high-value customers), violating normality.
    Bootstrap CIs automatically adapt to this skew without assuming a parametric form. **Caveat**: Your
    CI is only as good as your sample; with only 100 transactions, you may not capture rare high-value
    outliers. If you suspect extreme events, consider longer observation windows or robust methods.'
---

## Intuition

The bootstrap is a powerful, distribution-free resampling method that estimates the sampling distribution of a statistic without assuming a parametric form for the data. The idea: repeatedly resample *with replacement* from the observed data and compute the statistic each time. The resulting distribution of bootstrap statistics approximates the true sampling distribution.

## Detail

**Bootstrap procedure**:
1. Collect original sample of size $n$.
2. Resample $n$ observations *with replacement* from the sample, creating a bootstrap sample.
3. Compute the statistic (mean, median, correlation, regression coefficient, etc.) on the bootstrap sample.
4. Repeat steps 2–3 a large number of times (e.g., 10,000 iterations).
5. The empirical distribution of bootstrap statistics estimates the sampling distribution.

**Confidence intervals**: Extract percentiles from the bootstrap distribution. For a 95% CI of the mean, use the 2.5th and 97.5th percentiles of bootstrap means. This is the **percentile method**, simple and works for any estimator.

**Advantages**: (1) Works for any estimator, (2) makes no distributional assumptions, (3) captures the actual shape of the sampling distribution (asymmetry, multiple modes), (4) computational, not theoretical.

**Assumptions**: Data are independent and identically distributed (i.i.d.); resampling with replacement preserves this structure under $H_0$.

## Common gotchas / interview framings

- **Not a magic bullet**: Bootstrap still requires i.i.d. assumption; it cannot rescue time-series data or clustered structures without modification (e.g., block bootstrap).
- **Small samples, large bootstraps**: With $n = 20$, you cannot resample very unique patterns. Bootstrap works best with $n \geq 30$; smaller samples may lead to biased CIs.
- **Percentile vs. bias-corrected methods**: Basic percentile bootstrap can be biased for skewed distributions; BCa (bias-corrected and accelerated) bootstrap corrects this.
- **Multiple estimation targets**: Bootstrapping correlation, regression coefficients, etc. works, but interpret with care if there are underlying parametric assumptions (e.g., linearity in regression).
- **Computational cost**: For large datasets and many iterations, bootstrap can be slow; vectorized implementations or parallel computing help.

## See also
- [[bootstrap]]
- [[resampling-methods]]
- [[empirical-distribution]]
- [[percentile-method]]
- [[bca-bootstrap]]
- [[nonparametric-inference]]

## Sources
See frontmatter `sources:`.

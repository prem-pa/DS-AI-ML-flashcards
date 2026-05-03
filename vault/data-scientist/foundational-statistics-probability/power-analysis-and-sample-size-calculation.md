---
id: ae515bd7-39a0-4114-8f63-5565859b5f03
title: Power analysis and sample size calculation
track: data-scientist
topic: foundational-statistics-probability
difficulty: 3
tags:
- experimental-design
- statistical-power
- type-II-error
- effect-size
- sample-planning
aliases:
- statistical-power
- type-II-error
- sample-size-planning
sources:
- url: https://ocw.mit.edu/courses/18-05-introduction-to-probability-and-statistics-spring-2022/
  label: 'MIT 18.05: Introduction to Probability and Statistics'
- url: https://www.statlearning.com/
  label: James et al. An Introduction to Statistical Learning (Ch. 13)
cards:
- id: e3f3b16b-753e-415d-bfd3-de76fa513605
  type: flip
  front: You design an A/B test to detect a 5% lift in conversion rate. Baseline = 10%, Target = 10.5%.
    How does sample size scale with desired power?
  back: ''
- id: a1552ddb-834b-441a-8d9b-1c127bf8ab2b
  type: flip
  front: Sketch the relationship between sample size n, effect size δ, significance level α, and power
    (1 - β) for a t-test.
  back: ''
- id: ec468b48-9eba-4c3c-abd6-703052bc58f5
  type: flip
  front: Your underpowered study (80% power for medium effect) finds a significant result (p = 0.02).
    Should you trust the effect size estimate?
  back: ''
- id: 84d65032-1374-413e-896d-87a0039ef577
  type: flip
  front: You run an A/A test (same group vs. itself) with n = 1,000 and find p = 0.30 (not significant).
    The post-hoc power is 5%. What does this mean?
  back: ''
- id: 43b9dc51-1fb8-4b6b-a83f-60e082930ddd
  type: mcq
  front: 'To detect a smaller effect size while maintaining 80% power, you must:'
  back: ''
  choices:
  - key: a
    text: Increase sample size
    correct: true
  - key: b
    text: Increase α
    correct: false
  - key: c
    text: Decrease α
    correct: false
  - key: d
    text: Increase Type-II error
    correct: false
---

## Intuition
Power is the probability of detecting a true effect (rejecting H₀ when it's false): Power = 1 - β, where β is Type-II error (missing a true effect). Before running an experiment, you must determine the minimum sample size needed to achieve desired power (typically 80%) for a target effect size. Underpowered studies (small n, large effect size needed) waste time and resources; overpowered studies waste money. Power analysis trades off Type-I error (α, false positives), Type-II error (β, false negatives), effect size (δ), and sample size (n).

## Detail
For a one-sample t-test: power depends on (1) significance level α (usually 0.05), (2) effect size δ = (μ₁ - μ₀)/σ (standardized difference), (3) sample size n, (4) one-tailed vs. two-tailed. Larger δ and n → higher power. Larger α → higher power (but increases false positive risk). For two-sample tests, power also depends on the ratio of sample sizes; balanced designs (n₁ = n₂) are most efficient.

Effect size benchmarks (Cohen): small δ = 0.2, medium δ = 0.5, large δ = 0.8 (for t-tests). Prospectively, choose target effect size based on practical significance, then compute n to achieve desired power. Post-hoc power calculations are discouraged (biased, not predictive) but often requested; instead, report confidence intervals and effect sizes.

Sample size formulas: one-sample t-test with power (1 - β) and effect size δ requires $n \approx 2(z_{\alpha/2} + z_{\beta})^2 / \delta^2$ (for large n). Two-sample tests require 2× this. Higher power or smaller effect size increases n quadratically.

## Common gotchas / interview framings
- Post-hoc power is not informative; it's determined by the p-value (low p-value → high post-hoc power by definition)
- Statistical significance ≠ practical significance; a large n can detect a tiny effect that doesn't matter
- Multiple testing inflates Type-I error; Bonferroni correction requires smaller p-values, reducing power for individual tests
- Underpowered studies produce inflated effect sizes (winners' curse) due to selection bias (only large effects reach significance)

## See also
- [[statistical-power]]
- [[type-i-error]]
- [[type-ii-error]]
- [[effect-size]]
- [[sample-size]]

## Sources
See frontmatter `sources:`.

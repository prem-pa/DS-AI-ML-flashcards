---
id: a12cfa68-bbbd-439b-ba27-95e6c891c9c6
title: Confidence intervals and their interpretation
track: data-scientist
topic: hypothesis-testing-inference
difficulty: 3
tags:
- confidence intervals
- estimation
- interpretation
- frequentist
- coverage
aliases:
- CI
- confidence level
- margin of error
- interval estimation
sources:
- url: https://online.stat.psu.edu/statprogram/reviews/statistical-concepts/hypothesis-testing/p-value-approach
  label: 'Penn State STAT: Confidence Intervals and Interpretation'
- url: https://stat20.berkeley.edu/spring-2024/4-generalization/04-hypothesis-tests/notes.html
  label: 'Berkeley Stat 20: Estimation and Intervals'
- url: https://stats.libretexts.org/Bookshelves/Applied_Statistics/Mikes_Biostatistics_Book_(Dohm)/08:_Inferential_Statistics/8.1:_The_null_and_alternative_hypotheses
  label: 'LibreTexts: Interval Estimation'
cards:
- id: 0611baec-30af-4e28-912a-04fddfc9305f
  type: flip
  front: What does a 95% confidence interval actually mean? What is a common misinterpretation?
  back: '**Correct**: If we repeatedly sample and construct CIs using the same method, approximately 95%
    of the intervals will contain the true parameter. This is about the procedure''s long-run coverage.
    **Misinterpretation**: "There is a 95% probability the true parameter is in this specific interval."
    Once an interval is computed, it either contains the parameter or it does not; probability is not
    about a fixed interval.'
- id: 9d5ce43c-8ad1-4c48-a8f6-163f71850451
  type: flip
  front: How does sample size affect the width of a confidence interval?
  back: Width is proportional to $1/\sqrt{n}$. Doubling sample size reduces width by $\sqrt{2}$ (i.e.,
    ~30% narrower). Larger samples yield tighter intervals, higher precision, more actionable estimates.
    This is why sample size calculations are critical before studies begin.
- id: 5215a052-16f2-429c-899d-138fd6edfbc8
  type: mcq
  front: 'You compute a 95% CI for the difference in means between two groups: [−2.5, 1.2]. A colleague
    says, "The true difference is 95% likely to be in this interval." How do you correct this?'
  back: ''
  choices:
  - key: a
    text: That statement is correct; a 95% CI guarantees the true difference is in the interval with 95%
      probability.
    correct: false
  - key: b
    text: The true difference either is or is not in the interval (we don't know which). The 95% confidence
      refers to the long-run coverage of the procedure, not this specific interval.
    correct: true
  - key: c
    text: The CI is too wide; collect more data.
    correct: false
  - key: d
    text: The colleague is correct, and the interval is uninformative because it includes zero.
    correct: false
- id: 0823a675-bea1-4b8b-9cbb-9b8de2a28ae3
  type: flip
  front: Explain the relationship between a 95% CI and a two-sided hypothesis test at $\alpha = 0.05$.
  back: 'A 95% CI for parameter $\theta$ excludes the null value (e.g., 0) if and only if a two-sided
    test of $H_0: \theta = 0$ rejects at $\alpha = 0.05$. The CI and test are dual: the CI is the set
    of null values not rejected by the test. This duality allows moving between CIs and hypothesis tests.'
- id: b98f2d7a-5cf8-4962-b9ca-0e245fdf104f
  type: flip
  front: 'Design scenario: You estimate the click-through rate (CTR) of an ad and compute a 95% CI of
    [0.042, 0.058]. The product team wants a narrower CI to make a decision. What are your options?'
  back: 'Options: (1) Increase sample size (run the experiment longer); width $\propto 1/\sqrt{n}$. (2)
    Decrease confidence level (use 90% CI instead of 95%; narrower but less conservative). (3) Pre-register
    a hypothesis and reduce to one-sided test (narrower CI but directional). Always communicate the tradeoff:
    narrower CIs come from larger samples or lower confidence, not from shrinking the data.'
---

## Intuition

A **confidence interval** (CI) is a range of values computed from a sample that is believed to contain the true population parameter with a specified probability (confidence level). For instance, a 95% CI for the mean is constructed so that, if we repeatedly sample and compute CIs the same way, approximately 95% of those intervals will contain the true mean.

## Detail

For a parameter $\theta$ estimated by $\hat{\theta}$, a 100(1-$\alpha$)% CI has the form:
$$\hat{\theta} \pm z_{\alpha/2} \cdot SE(\hat{\theta})$$
where $SE$ is the standard error and $z_{\alpha/2}$ is the critical value (e.g., 1.96 for 95% CI under normality).

For a sample mean with known population standard deviation: $\bar{x} \pm 1.96 \cdot \frac{\sigma}{\sqrt{n}}$. With unknown $\sigma$, use the t-distribution: $\bar{x} \pm t_{\alpha/2, n-1} \cdot \frac{s}{\sqrt{n}}$.

**Key interpretation**: If we repeated the sampling and CI-construction process infinitely many times, 95% of the intervals would contain the true $\theta$. This is a statement about the *long-run frequency* of the procedure, not about a single observed interval.

## Common gotchas / interview framings

- **Not "95% chance the parameter is in the interval"**: Once computed, the parameter either is or is not in a specific observed interval. The probability is about the procedure.
- **Narrower CI ≠ higher confidence**: A 90% CI is narrower than a 95% CI for the same data; confidence level and width are distinct concepts.
- **Multiple CIs**: If you compute 100 independent 95% CIs, expect ~5 to miss the true value due to sampling variation.
- **Asymmetry in bootstrapped CIs**: For skewed distributions, percentile bootstrap CIs may be asymmetric around the point estimate, which is correct and informative.
- **Relationship to hypothesis tests**: A 95% CI for $\mu$ excludes 0 (the null value) if and only if a two-sided t-test rejects $H_0: \mu = 0$ at $\alpha = 0.05$.

## See also
- [[confidence-interval]]
- [[frequentist-inference]]
- [[coverage-probability]]
- [[margin-of-error]]
- [[standard-error]]
- [[bootstrap-ci]]

## Sources
See frontmatter `sources:`.

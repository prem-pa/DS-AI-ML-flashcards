---
id: 6c9d0dd5-bd1d-47ba-aac3-2ed53b5a861d
title: Nonparametric tests (Mann-Whitney, Kruskal-Wallis, permutation tests)
track: data-scientist
topic: hypothesis-testing-inference
difficulty: 3
tags:
- nonparametric
- distribution-free
- rank-based
- permutation
- robustness
aliases:
- Wilcoxon test
- Mann-Whitney U
- Kruskal-Wallis H
- randomization tests
- rank tests
sources:
- url: https://stat20.berkeley.edu/spring-2024/4-generalization/04-hypothesis-tests/notes.html
  label: 'Berkeley Stat 20: Nonparametric and Permutation Methods'
- url: https://statistics.laerd.com/statistical-guides/hypothesis-testing-3.php
  label: 'Laerd Statistics: Nonparametric Alternatives'
- url: https://online.stat.psu.edu/statprogram/reviews/statistical-concepts/hypothesis-testing/p-value-approach
  label: 'Penn State: Distribution-Free Methods'
cards:
- id: 040e37f7-dbf7-45e0-8541-b62e2a4c5d7e
  type: flip
  front: When would you use the Mann-Whitney U test instead of a t-test?
  back: 'Use Mann-Whitney U when: (1) data are severely non-normal (heavy tails, skew), (2) sample sizes
    are small ($n < 30$) and normality is questionable, (3) data are ordinal (rankings) rather than continuous,
    or (4) outliers are present and cannot be removed. Mann-Whitney compares distributions via ranks,
    not means, so it is more robust.'
- id: c46270b4-d62a-4dba-ac14-0d5f3ccad6af
  type: flip
  front: Explain how the Mann-Whitney U test works conceptually.
  back: Combine all observations from both groups and assign ranks 1, 2, ..., $n$ (smallest to largest).
    Compute the sum of ranks in group 1 (or group 2). If $H_0$ is true (identical distributions), the
    sums should be roughly proportional to group sizes. Extreme rank sums indicate a difference in distribution
    location. The test statistic $U$ is derived from these ranks.
- id: d55b7403-54bc-4320-9bcf-ec6f9835832c
  type: mcq
  front: You compare 4 groups using Kruskal-Wallis H test. H = 8.1, p = 0.04. What is your next step?
  back: ''
  choices:
  - key: a
    text: Conclude all groups differ significantly.
    correct: false
  - key: b
    text: Stop; the test is complete and answers your question.
    correct: false
  - key: c
    text: Conduct pairwise Mann-Whitney tests (with Bonferroni or FDR correction) to identify which groups
      differ.
    correct: true
  - key: d
    text: Compute confidence intervals for each group mean.
    correct: false
- id: 7188a953-bd6a-4972-b3c8-4efbd461ee61
  type: flip
  front: Describe a permutation test framework. Why is it valuable despite computational cost?
  back: 'A permutation test works by: (1) Compute test statistic $T$ on observed data. (2) Generate all
    possible rearrangements of the data under $H_0$ (exchangeability). (3) Compute $T$ for each permutation.
    (4) p-value = proportion of permutations yielding $T$ as extreme as observed. **Valuable because**:
    (a) Exact (not asymptotic), (b) applies to any statistic, (c) no distributional assumptions. Modern
    computers handle even large datasets.'
- id: c56a98c5-542b-4da3-9bf2-9ae0ad39ba6c
  type: flip
  front: A nonparametric test shows p = 0.08 (not significant at $\alpha = 0.05$). Would switching to
    a parametric t-test increase the chance of significance? When would this be appropriate?
  back: Possibly, because parametric tests are more powerful when assumptions hold. However, switching
    only because the nonparametric result is not significant ("significance shopping") is p-hacking and
    inflates Type I error. Only switch if you have strong *prior* justification (e.g., performed normality
    test before choosing test, or collect more data to improve normality). Document your testing strategy
    pre-analysis.
---

## Intuition

Nonparametric tests make few or no assumptions about the underlying distribution of data. Instead of testing population parameters (like means), they test properties of distributions (like medians or ranks). These tests are robust to outliers and violations of normality, making them ideal when assumptions for parametric tests fail.

## Detail

**Mann-Whitney U test**: Nonparametric alternative to the independent t-test. Tests whether two independent samples come from the same distribution. The test compares ranks rather than raw values: rank all observations across both groups, then compute the test statistic based on the sum of ranks in one group. Robust to outliers and non-normality.

**Kruskal-Wallis H test**: Extension to 3+ groups (nonparametric ANOVA). Tests $H_0$: all groups have the same distribution. The statistic $H$ is based on ranks; if significant, use pairwise Mann-Whitney tests (with multiple testing correction) to identify differing pairs.

**Permutation tests** (randomization tests): A distribution-free approach that computes p-values by generating all (or a large sample of) possible rearrangements of the data under $H_0$. The p-value is the proportion of permutations yielding a test statistic as extreme as observed. Advantages: (1) Valid for any test statistic, (2) exact (not asymptotic), (3) transparent computational framework. Computationally intensive but feasible with modern computers.

## Common gotchas / interview framings

- **Loss of power**: Nonparametric tests sacrifice power when parametric assumptions hold. Use only when necessary.
- **Median vs. mean**: Mann-Whitney tests the median (or general distribution location), not the mean; conclusions differ if only means are of interest.
- **Assumption still present**: Nonparametric tests assume independence and identical distributions; they do NOT assume normality but are not free of all assumptions.
- **Permutation validity**: Permutation tests assume exchangeability under $H_0$; violated if data are clustered or correlated.
- **Multiple testing correction**: Post-hoc pairwise comparisons require Bonferroni or FDR correction, reducing power further.

## See also
- [[nonparametric-tests]]
- [[rank-based-statistics]]
- [[permutation-test]]
- [[wilcoxon-signed-rank]]
- [[mann-whitney-u]]
- [[kruskal-wallis]]

## Sources
See frontmatter `sources:`.

---
id: b417e45c-6aa8-4a00-b2a2-4fcb36469aa1
title: Parametric tests (t-test, ANOVA, chi-squared)
track: data-scientist
topic: hypothesis-testing-inference
difficulty: 3
tags:
- parametric
- t-test
- ANOVA
- chi-squared
- test selection
aliases:
- t-test
- ANOVA F-test
- chi-square test
- dependent sample testing
sources:
- url: https://www.pnw.edu/wp-content/uploads/2020/03/lecturenotes10-9.pdf
  label: 'PNW: Hypothesis Tests and Parametric Methods'
- url: https://www2.stat.duke.edu/courses/Fall11/sta10/STA10lecture21.pdf
  label: 'Duke: Hypothesis Testing Examples and Case Studies'
- url: https://uwaterloo.ca/scholar/sites/ca.scholar/files/gydong/files/afm_113_week_8_lessons.pdf
  label: 'University of Waterloo: Hypothesis Testing Fundamentals'
cards:
- id: b609b071-6264-4ce7-839f-cc453da8d6ca
  type: flip
  front: When would you use a t-test vs. ANOVA? What is the key distinction?
  back: Use **t-test** to compare means of exactly 2 groups (independent or paired samples). Use **ANOVA**
    when comparing means across 3 or more groups. ANOVA tests the omnibus hypothesis that all means are
    equal; if significant, post-hoc tests identify which pairs differ. You could run multiple t-tests
    instead, but this inflates Type I error (multiple testing problem).
- id: 994aae1f-9d41-401f-8bdd-709d102e66f2
  type: flip
  front: You have data from 2 groups with unequal sample sizes and unequal variances. Which variant of
    the t-test should you use?
  back: Use **Welch's t-test**, which does not assume equal population variances. It adjusts the degrees
    of freedom and is more robust than Student's t-test when variances differ. The test statistic and
    p-value are computed differently to account for heteroscedasticity.
- id: 0cf40f17-daa8-4c51-9788-70841d95ffa1
  type: mcq
  front: You conduct a 3-group ANOVA and obtain F = 4.2, p = 0.02. What does this result tell you?
  back: ''
  choices:
  - key: a
    text: Group 1 and Group 2 differ significantly; the effect is practically large.
    correct: false
  - key: b
    text: At least one pair of group means differs significantly; conduct post-hoc tests to identify which.
    correct: true
  - key: c
    text: All three group means are significantly different from each other.
    correct: false
  - key: d
    text: The overall variance in the data is 4.2 times the expected value.
    correct: false
- id: 18e07aec-818d-4380-812c-f674a726aaa8
  type: flip
  front: Describe a scenario where the assumptions for a t-test might be violated. How would you proceed?
  back: 'Violations occur with: (1) non-normal data (heavy tails, skew), (2) unequal variances (Levene''s
    test), (3) small sample sizes ($n < 30$). Options: (a) Use Welch''s t-test (handles variance inequality).
    (b) Transform data (e.g., log, square root) to normalize. (c) Use a nonparametric test like Mann-Whitney
    U. (d) With large $n$, t-test is robust due to CLT.'
- id: 1f4bd8c7-8392-44c4-a0a3-0ee52d854894
  type: flip
  front: In a chi-squared test for independence, what do the expected counts represent and why is the
    assumption about them important?
  back: Expected counts are the cell frequencies predicted under $H_0$ (independence), calculated as $E
    = \frac{(\text{row total}) \times (\text{column total})}{\text{grand total}}$. If expected counts
    are too small (< 5), the $\chi^2$ distribution approximation breaks down and the p-value is unreliable.
    Combine categories or use Fisher's exact test as alternatives.
---

## Intuition

Parametric tests assume the data follow a specific distribution (usually normal). When assumptions hold, they are the most powerful tests—they extract maximum information from data. Common parametric tests include:
- **t-test**: Compares means of two groups (independent or paired).
- **ANOVA** (F-test): Compares means across 3+ groups.
- **Chi-squared test**: Tests independence or goodness-of-fit for categorical data.

## Detail

**t-test** assumes normality and tests $H_0: \mu_1 = \mu_2$. The test statistic is $t = \frac{\bar{x}_1 - \bar{x}_2}{SE(\bar{x}_1 - \bar{x}_2)}$, following a t-distribution with degrees of freedom $n_1 + n_2 - 2$ (unpaired). Use a paired t-test when observations are linked (e.g., before/after).

**ANOVA** extends the t-test to $k \geq 3$ groups, testing $H_0: \mu_1 = \mu_2 = \ldots = \mu_k$. The F-statistic is the ratio of between-group variance to within-group variance: $F = \frac{MS_{\text{between}}}{MS_{\text{within}}}$. Large F rejects $H_0$. Post-hoc tests (Tukey, Bonferroni) compare pairs if ANOVA is significant.

**Chi-squared test** ($\chi^2$) applies to categorical data, testing independence via: $\chi^2 = \sum \frac{(O - E)^2}{E}$ where $O$ is observed count, $E$ is expected count under $H_0$.

## Common gotchas / interview framings

- **Violating normality**: t-test is robust to moderate violations with $n \geq 30$ (CLT) but risky for small $n$ or heavy-tailed data. Use Shapiro-Wilk or Q-Q plots to check.
- **Unequal variances**: Use Welch's t-test (doesn't assume equal variances) instead of Student's t. Levene's test checks variance homogeneity.
- **Multiple comparisons after ANOVA**: A significant ANOVA does not tell you *which* pairs differ; pairwise comparisons require correction (Bonferroni, Tukey).
- **Low cell counts in chi-squared**: If expected counts < 5, results are unreliable; use Fisher's exact test (2×2) or combine categories.
- **Design of experiments**: Blocking, randomization, and power calculation must precede test selection.

## See also
- [[t-test]]
- [[anova]]
- [[chi-squared-test]]
- [[normality-assumption]]
- [[variance-homogeneity]]
- [[parametric-vs-nonparametric]]

## Sources
See frontmatter `sources:`.

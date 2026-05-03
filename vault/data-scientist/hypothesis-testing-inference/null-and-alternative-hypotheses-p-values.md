---
id: 9791e85a-1803-470a-9400-519581658fac
title: Null and alternative hypotheses, p-values
track: data-scientist
topic: hypothesis-testing-inference
difficulty: 1
tags:
- testing
- hypothesis
- p-values
- statistics
- inference
aliases:
- H0 and Ha
- significance testing
- p-value interpretation
sources:
- url: https://stats.libretexts.org/Bookshelves/Applied_Statistics/Mikes_Biostatistics_Book_(Dohm)/08:_Inferential_Statistics/8.1:_The_null_and_alternative_hypotheses
  label: 'LibreTexts: Null and Alternative Hypotheses'
- url: https://online.stat.psu.edu/statprogram/reviews/statistical-concepts/hypothesis-testing/p-value-approach
  label: 'Penn State STAT: P-Value Approach'
- url: https://stat20.berkeley.edu/spring-2024/4-generalization/04-hypothesis-tests/notes.html
  label: 'Berkeley Stat 20: Hypothesis Testing'
cards:
- id: e565cd5b-5eab-4bdb-aecb-7da0ef262dee
  type: flip
  front: Define the null hypothesis ($H_0$) and explain its role in hypothesis testing.
  back: The null hypothesis is a baseline claim—typically "no effect," "no difference," or "status quo."
    It is assumed true unless the data provide strong evidence to the contrary. Its role is to define
    what "no signal" looks like; we test whether the data contradict this assumption.
- id: b071086e-b5f1-4ab6-9f12-1295adcdb08f
  type: flip
  front: What does a p-value of 0.03 actually mean? What does it NOT mean?
  back: '**Means**: If the null hypothesis were true, there is a 3% probability of observing a test statistic
    as extreme or more extreme than what we observed. **Does NOT mean**: (1) 3% chance the null is true,
    (2) 97% confidence in the alternative, (3) the effect size is large, or (4) the result is practically
    important.'
- id: 81920311-54c0-4eb0-9814-f4a28f688ec2
  type: mcq
  front: You run a hypothesis test and obtain a p-value of 0.12 at $\alpha = 0.05$. Which statement is
    correct?
  back: ''
  choices:
  - key: a
    text: We reject $H_0$; there is 12% chance the null is true.
    correct: false
  - key: b
    text: We fail to reject $H_0$; this means $H_0$ is definitely true.
    correct: false
  - key: c
    text: We fail to reject $H_0$; the data are not sufficiently extreme under $H_0$ to conclude a statistically
      significant effect.
    correct: true
  - key: d
    text: We reject $H_0$ because p > 0.05 indicates a strong alternative.
    correct: false
- id: f75289eb-c9a0-4c0e-a573-34eb525f48ac
  type: flip
  front: Distinguish between one-sided and two-sided hypothesis tests. When is each appropriate?
  back: '**One-sided test** ($H_a: \mu > \mu_0$ or $\mu < \mu_0$): Tests for effect in a specific direction;
    more powerful but requires directional hypothesis pre-specified. **Two-sided test** ($H_a: \mu \neq
    \mu_0$): Tests for any difference; weaker but does not assume direction. Use two-sided unless theory
    or domain knowledge strongly justifies directionality (decided *before* data collection).'
- id: d4c2f8fe-e070-4e86-b50c-ee8aba737541
  type: flip
  front: Explain the multiple testing problem and why it matters.
  back: 'When testing many hypotheses independently at $\alpha = 0.05$, the probability of at least one
    false positive grows. For $m$ independent tests, the family-wise error rate ≈ $1 - (1 - \alpha)^m$.
    Example: 20 tests give ~64% chance of ≥1 false positive. This inflates Type I errors and requires
    correction (e.g., Bonferroni, FDR).'
---

## Intuition

Hypothesis testing is a formal framework for making decisions about populations using sample data. We start with a **null hypothesis** ($H_0$)—a claim of "no effect" or "status quo"—and test whether evidence sufficiently contradicts it. The **alternative hypothesis** ($H_a$) is what we would conclude if $H_0$ is rejected.

A **p-value** quantifies the strength of this evidence: it is the probability of observing a test statistic as extreme as (or more extreme than) what we observed, *assuming $H_0$ is true*. Small p-values suggest the data are unlikely under $H_0$, leading us to reject it.

## Detail

Formally, given a test statistic $T$ computed from the sample:
- The p-value = $P(T \geq t_{\text{obs}} | H_0 \text{ is true})$ for a one-sided test (or analogous for two-sided).
- A significance level $\alpha$ (often 0.05) is chosen before the test; if p-value $< \alpha$, we reject $H_0$.
- Rejection means: "The data provide sufficient evidence against $H_0$ at the $\alpha$ level."

**Critical Misconception**: A p-value is *not* "the probability that $H_0$ is true" or "the probability the result is due to chance." Rather, it conditions on $H_0$ being true and measures how compatible the data are with that assumption.

## Common gotchas / interview framings

- **P < 0.05 does NOT mean**: the effect is true with 95% probability, or the effect size is large, or the result is practically important.
- **P-hacking**: Repeating analyses, changing hypotheses post-hoc, or selective reporting until p < 0.05 inflates false positives.
- **File drawer problem**: Studies with non-significant results go unpublished, creating selection bias.
- **One-sided vs. two-sided**: Always specify before testing; one-sided tests are more powerful but directional.
- **Multiple testing**: Testing 20 independent hypotheses at $\alpha = 0.05$ expects ~1 false positive by chance alone.

## See also
- [[type-i-error]]
- [[statistical-significance]]
- [[null-hypothesis]]
- [[p-value]]
- [[alternative-hypothesis]]
- [[rejection-region]]

## Sources
See frontmatter `sources:`.

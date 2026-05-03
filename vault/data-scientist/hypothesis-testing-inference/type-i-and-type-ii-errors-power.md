---
id: 9e2652c3-83e4-4fb9-8798-aa8b32b906ba
title: Type I and Type II errors, power
track: data-scientist
topic: hypothesis-testing-inference
difficulty: 3
tags:
- errors
- power
- alpha
- beta
- test design
aliases:
- false positives
- false negatives
- sensitivity
- specificity
sources:
- url: https://www.southampton.ac.uk/~mb1a10/stats/FEEG6017_5.pdf
  label: 'Southampton: Hypothesis Testing and Type I/II Errors'
- url: https://mcpanalytics.ai/whitepapers/whitepaper-bonferroni-correction
  label: 'MCP Analytics: Type I and Type II Error Analysis'
- url: https://www.colorado.edu/amath/sites/default/files/attached-files/lesson9_hyptests.pdf
  label: 'University of Colorado: Hypothesis Tests and Power'
cards:
- id: 59322665-5265-4182-bdaa-41b9e9ed1577
  type: flip
  front: Define Type I and Type II error. Which is controlled by $\alpha$?
  back: '**Type I error** (false positive): Rejecting $H_0$ when it is true. Probability = $\alpha$ (significance
    level). **Type II error** (false negative): Failing to reject $H_0$ when it is false. Probability
    = $\beta$. The significance level $\alpha$ directly controls Type I; Type II ($\beta$) is influenced
    by sample size, effect size, and $\alpha$.'
- id: 37fa59cd-f68b-49de-9b3e-e159868a8aeb
  type: flip
  front: What is statistical power and how does it relate to Type II error?
  back: Statistical power = $1 - \beta$ is the probability of correctly rejecting $H_0$ when the alternative
    hypothesis is true (detecting a true effect). A power of 0.80 means an 80% chance of detecting the
    effect if it exists. Higher power reduces the risk of Type II error (missing a real effect).
- id: 6d00f834-600d-476c-b524-5c21e8fbb143
  type: mcq
  front: In an A/B test, increasing the significance level from $\alpha = 0.05$ to $\alpha = 0.10$ does
    which of the following?
  back: ''
  choices:
  - key: a
    text: Decreases Type I error rate and increases power
    correct: false
  - key: b
    text: Increases Type I error rate and increases power
    correct: true
  - key: c
    text: Decreases both Type I error and Type II error
    correct: false
  - key: d
    text: Increases Type II error and decreases power
    correct: false
- id: bac9d6bd-97ba-4981-8181-730d1daca392
  type: flip
  front: 'Design scenario: A company tests whether a new algorithm reduces fraud. False positives block
    legitimate transactions (cost: customer frustration); false negatives miss fraud (cost: financial
    loss). How would you set $\alpha$ and power?'
  back: Since false negatives (undetected fraud) are likely more costly, aim for high power ($\beta$ low,
    e.g., 0.10 for 90% power) to minimize Type II errors. Depending on fraud cost, $\alpha$ could remain
    0.05 or be slightly higher (0.10) to further increase power. Always justify the choice by the asymmetric
    costs of errors.
- id: aa1eb396-92f0-48ed-83ec-e15c97b874da
  type: flip
  front: Explain the relationship between sample size, effect size, and power in a single-sentence formula
    (conceptually).
  back: 'Power increases with larger sample size (more information), larger true effect size (easier to
    detect), and larger $\alpha$ (looser rejection criterion). Roughly: Power $\propto \sqrt{n \cdot \delta^2}$
    where $n$ is sample size and $\delta$ is standardized effect size.'
---

## Intuition

When testing a hypothesis, two types of mistakes are possible:
- **Type I error**: Rejecting $H_0$ when it is actually true (false positive). Probability is controlled by significance level $\alpha$.
- **Type II error**: Failing to reject $H_0$ when it is false (false negative). Probability is $\beta$.

**Statistical power** = $1 - \beta$ is the probability of correctly rejecting $H_0$ when the alternative is true—the ability to detect a true effect.

In business contexts, the costs of these errors differ: Type I (launching a bad feature, running a fraudulent campaign) vs. Type II (missing a good opportunity, failing to detect disease). This tradeoff fundamentally shapes test design.

## Detail

For a given sample size and effect size, there is an inverse tradeoff between $\alpha$ and $\beta$: decreasing $\alpha$ (stricter rejection criterion) increases $\beta$ (harder to detect true effects). Power depends on:
- **Effect size** ($\delta$, Cohen's $d$): Larger effects are easier to detect; requires domain knowledge or pilot studies.
- **Sample size** ($n$): Larger samples increase power; proportional to $\sqrt{n}$.
- **Significance level** ($\alpha$): Smaller $\alpha$ reduces Type I but increases Type II.

Power analysis involves choosing sample size pre-study to achieve target power (e.g., 80%) given an expected effect size and $\alpha$.

## Common gotchas / interview framings

- **Not specifying the tradeoff**: Interview: "Should we reduce alpha to 0.01?" Answer requires discussing increased Type II cost.
- **Ignoring multiple testing**: Correction (Bonferroni) increases $\beta$ and reduces power; must account when designing sample size.
- **Confusing $\beta$ with p-value**: $\beta$ is chosen pre-study; p-values are observed post-study.
- **Assuming equal costs**: Defaulting to $\alpha = 0.05$ ignores asymmetric business costs of errors.
- **Power of 0.5 tests**: Underpowered studies produce unreliable, non-replicable results.

## See also
- [[type-i-error]]
- [[type-ii-error]]
- [[statistical-power]]
- [[significance-level]]
- [[effect-size]]
- [[sample-size]]

## Sources
See frontmatter `sources:`.

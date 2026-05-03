---
id: f1c36aec-21b3-4192-b6fd-d89924ec5131
title: Confidence intervals vs Bayesian posterior
track: data-scientist
topic: ab-testing-experimentation
difficulty: 3
tags:
- statistical-inference
- confidence-intervals
- bayesian
- interpretation
- posterior-distribution
aliases:
- CI vs Bayesian
- frequentist vs Bayesian
- credible interval
- posterior distribution
sources:
- url: https://netflixtechblog.com/a-b-testing-and-beyond-improving-the-netflix-streaming-experience-with-experimentation-and-data-5b0ae9295bdf
  label: 'Netflix: A/B Testing Frameworks'
- url: https://research.netflix.com/research-area/experimentation-and-causal-inference
  label: 'Netflix: Experimentation Research'
cards:
- id: 93aed152-be8e-4e0e-b87f-113ee15aae69
  type: flip
  front: ''
  back: ''
- id: dd6bbe94-3d1f-4910-adde-5affe0030f90
  type: flip
  front: ''
  back: ''
- id: 0874e568-7439-4149-8fb9-fcf35862384c
  type: mcq
  front: ''
  back: ''
  choices:
  - key: a
    text: 95% (if mean is 2 SD from 0)
    correct: false
  - key: b
    text: 97-99% (roughly)
    correct: false
  - key: c
    text: Depends on the prior used
    correct: false
  - key: d
    text: Can't answer without more info
    correct: false
- id: b5218ebe-606b-4b9e-ba2b-13177a32bfda
  type: flip
  front: ''
  back: ''
---

## Intuition

**Confidence interval (frequentist)**: "If we repeated this experiment 100 times, the true effect would fall in this interval ~95 times."

**Bayesian posterior**: "Given the data and our prior belief, the effect is most likely in this range, and here's the probability distribution."

## Detail

**Confidence interval (95% CI)**:
- **Interpretation**: Frequentist. If you ran experiment infinitely, 95% of computed CIs would contain the true effect
- **Decision rule**: If CI doesn't cross 0, effect is significant (p < 0.05)
- **Limitation**: CI is either fully in one region or not; no probability statement about true effect
- **Advantage**: No prior needed; based only on data
- **Example**: CI = [1.2%, 3.5%]. We're 95% confident true effect is between 1.2% and 3.5%

**Bayesian posterior**:
- **Interpretation**: Bayesian. Distribution of the effect given data and prior
- **Decision rule**: If posterior is mostly > 0, effect is likely positive; report P(effect > 0)
- **Limitation**: Requires choosing a prior (can bias results)
- **Advantage**: Direct probability statement ("95% chance effect is >1%"); easier to interpret; can incorporate prior knowledge
- **Example**: Posterior = Normal(2.0%, 0.8%). 92% of posterior mass is >0%, so effect is likely positive

**Trade-offs**:
- **CI**: Mechanistic, repeatable, no prior bias. But binary decision (significant or not)
- **Bayesian**: Richer information (full distribution), but requires prior choice

**Practical use**:
- Most companies use CI + frequentist stats (standard, well-understood)
- Some use Bayesian for decision-making (posterior above threshold = go)
- Hybrid: use Bayesian for planning (priors) and frequentist for reporting (CIs)

## Common gotchas / interview framings
- "Our 95% CI is [-1%, 3%], it includes 0, so no effect." → Correct interpretation: we can't rule out zero (not significant). But effect could still be real (just uncertain)
- "Posterior shows 85% chance effect > 0%. Should we launch?" → Depends on your threshold. If threshold is 90%, no. If 80%, yes. Bayesian requires a decision threshold, not just significance
- "CI is [-1%, 3%] but posterior mean is 1%. Why the difference?" → Posterior is influenced by prior; CI is not. If prior was strong, posterior pulls toward prior
- "Can we compare two CIs to see which effect is bigger?" → Only if non-overlapping. Overlapping CIs might still show significantly different effects (depends on correlation). Better: test interaction directly

## See also
- [[sequential-testing-and-peek-penalties]]
- [[effect-size-estimation-and-practical-significance]]

## Sources
See frontmatter `sources:`.

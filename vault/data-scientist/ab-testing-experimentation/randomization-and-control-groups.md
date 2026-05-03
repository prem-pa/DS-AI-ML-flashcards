---
id: 91beb1b1-a928-4de2-af38-59e756a9e819
title: Randomization and control groups
track: data-scientist
topic: ab-testing-experimentation
difficulty: 1
tags:
- causal-inference
- experiment-design
- confounding
- treatment-assignment
- statistical-validity
aliases:
- RCT
- random assignment
- treatment allocation
sources:
- url: https://netflixtechblog.com/a-b-testing-and-beyond-improving-the-netflix-streaming-experience-with-experimentation-and-data-5b0ae9295bdf
  label: 'Netflix: A/B Testing and Beyond'
- url: https://research.netflix.com/research-area/experimentation-and-causal-inference
  label: 'Netflix Research: Experimentation & Causal Inference'
cards:
- id: 6d52add8-6d9b-4eb3-ad20-68c63f2b0d68
  type: flip
  front: ''
  back: ''
- id: 9fe749bb-7791-444a-b884-8a0e602cc7ce
  type: flip
  front: ''
  back: ''
- id: d2b7b601-e7db-408a-a5e4-07d3c5e4d9cf
  type: mcq
  front: ''
  back: ''
  choices:
  - key: a
    text: Compare all treated vs all control users
    correct: false
  - key: b
    text: Compare only adopters vs control
    correct: false
  - key: c
    text: Weight treated non-adopters as 0.5 treated
    correct: false
  - key: d
    text: Use propensity score matching on adoption
    correct: false
- id: 6fb8a85e-cf2f-497f-8004-c0e6771429f5
  type: flip
  front: ''
  back: ''
---

## Intuition

Randomization is the gold standard for causal inference because it breaks the link between confounders (unobserved variables) and treatment assignment. If users are randomly assigned to treatment or control, then on average across many experiments, any differences in outcomes are causally attributable to the treatment, not to pre-existing differences.

## Detail

When you randomly assign users to control (no treatment) or treatment, you ensure that:
- **Covariate balance**: Both arms start with similar characteristics on average
- **No selection bias**: Sicker users don't preferentially enter treatment
- **Valid statistical inference**: Standard t-tests, confidence intervals apply

Control groups can be:
1. **Business-as-usual**: The current experience (most common)
2. **Placebo**: A fake treatment (rare online, common in medicine)
3. **No-change**: Explicitly holding old behavior

Randomization must happen at the correct unit (user, session, account) before treatment is assigned. Post-treatment selection invalidates causality.

## Common gotchas / interview framings
- "If we randomize users but some never see treatment, is it still causal?" → ITT (intention-to-treat) remains causal; per-protocol is biased
- "Why not just compare treated users to others?" → Selection bias: treated users may be different (more engaged, different day-of-week)
- "Can we randomize at session level if users see both arms?" → Yes if session effects dominate; no if memory/network effects exist
- "What if randomization is imperfect?" → Use adjusted analysis (ANCOVA/CUPED) to correct imbalance

## See also
- [[causal-inference]]
- [[confounding-variables]]
- [[internal-validity]]
- [[randomized-controlled-trial]]

## Sources
See frontmatter `sources:`.

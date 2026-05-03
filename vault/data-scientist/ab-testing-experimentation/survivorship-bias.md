---
id: a572b88d-067c-4219-b3d0-2c5a69d434c7
title: Survivorship bias
track: data-scientist
topic: ab-testing-experimentation
difficulty: 3
tags:
- survivorship-bias
- attrition
- churn
- intent-to-treat
- selection-bias
aliases:
- survivor cohort bias
- attrition bias
- ITT vs per-protocol
sources:
- url: https://research.netflix.com/research-area/experimentation-and-causal-inference
  label: 'Netflix: Causal Inference in Experiments'
- url: https://netflixtechblog.com/a-b-testing-and-beyond-improving-the-netflix-streaming-experience-with-experimentation-and-data-5b0ae9295bdf
  label: 'Netflix: A/B Testing Methodologies'
cards:
- id: 2291cc2d-a712-41a2-a418-9920e77a64a3
  type: flip
  front: ''
  back: ''
- id: 58bdb465-d172-42ec-8ec1-81542dfdc275
  type: flip
  front: ''
  back: ''
- id: b6d88d8a-f249-4073-936a-dfc3214dd976
  type: mcq
  front: ''
  back: ''
  choices:
  - key: a
    text: Sample size mismatch (950 vs 990)
    correct: false
  - key: b
    text: 'Survivorship bias: treatment survivors are more engaged, not all users'
    correct: false
  - key: c
    text: Metric definition is wrong
    correct: false
  - key: d
    text: Both A and B
    correct: false
- id: c21f43a1-23a4-4f0e-ad48-0880436ae970
  type: flip
  front: ''
  back: ''
---

## Intuition

If treatment causes some users to churn, then only the less-churn-prone users remain in treatment. When you compare remaining treatment users to control, you're comparing different cohorts (survivors), not a fair apples-to-apples comparison.

## Detail

**Survivorship bias occurs when**:
1. Treatment causes user dropout/churn (e.g., features is so bad, users leave)
2. You only analyze remaining users (survivors)
3. Remaining treatment users are systematically different from remaining control users

**Example**:
- Control: 100 users, 10 churn → 90 remain
- Treatment: 100 users, 20 churn (treatment is annoying) → 80 remain
- Remaining treatment users are more engaged/patient (they stayed despite annoyance)
- Remaining control users are random (normal churn)
- Comparing 80 vs 90: treatment users *survivors* are *more engaged* → biased upward

**Solution: Intent-to-Treat (ITT)**:
- Analyze all randomized users, including those who churned
- Treat churned users as having a metric value of 0 (or their last observation)
- Maintains causal estimate even if treatment causes dropout

**Per-protocol analysis** (biased, don't use):
- Only analyze users who complied with assignment (e.g., stayed)
- Introduces survivor cohort bias

**Measurement options**:
1. **Engagement metric for remaining users**: P(engaged) = # active / # remaining
   - Biased if treatment changes churn
2. **Engagement metric for all users**: P(engaged) = # active / # randomized
   - ITT: causal, unbiased
3. **Lifetime value, including churn**: LTV counts revenue of users who stay
   - Better for detecting churn harm

## Common gotchas / interview framings
- "We tested a feature; users in treatment who stayed are more satisfied." → Check churn rate. If treatment increased churn, then remaining users are survivors (not representative of all treatment users). Use ITT
- "Should we exclude users who churned before the experiment ended?" → Depends on definition. If they churn during the test, include them (ITT). If they churned before randomization, exclude (that's different)
- "ITT is too conservative; it masks true effects." → ITT isn't conservative; it's unbiased. You might be measuring different effects: "effect of feature for all" (ITT) vs "effect for engaged users" (per-protocol, biased)
- "Our engagement metric increased but DAU decreased. Which do we trust?" → DAU decreased is the ITT effect; engagement (%) increased because survivors are more engaged. Trust DAU; it's the causal estimate

## See also
- [[randomization-and-control-groups]]
- [[novelty-effects-and-long-term-impact]]

## Sources
See frontmatter `sources:`.

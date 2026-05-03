---
id: 40fd3383-9c9a-4992-a426-558d17bd6792
title: Opportunity sizing
track: data-scientist
topic: product-business-sense
difficulty: 3
tags:
- strategy
- prioritization
- impact
- business-math
- data-analysis
aliases:
- impact sizing
- opportunity quantification
- market size
- addressable market
sources:
- url: https://qubit.capital/blog/evaluating-traction-metrics-consumer-apps
  label: Evaluating Traction Metrics for Consumer Apps
- url: https://userpilot.com/blog/north-star-metric/
  label: Sizing Opportunities in North Star Context
cards:
- id: f235919f-53ca-46e6-a302-9fa3366a6bb3
  type: flip
  front: What are the three dimensions of opportunity sizing?
  back: '1. Population: How many users/transactions affected (e.g., 5M DAU in feature flag X). 2. Impact
    per unit: % improvement or absolute delta (e.g., +3% retention). 3. Time & revenue: Total value over
    a period (e.g., 5M * 3% * $0.50 LTV = $75K annual).'
- id: f01a1327-e6af-47f6-9830-ad1b3557d02f
  type: flip
  front: 'You''re prioritizing two features: (A) Improve mobile app speed (affects 40% of users, ~+8%
    engagement). (B) Add dark mode (affects 20% of users, ~+15% engagement). Which has higher opportunity
    sizing?'
  back: 'Calculate impact: (A) 100% user base * 40% * 8% = 3.2% overall engagement lift. (B) 100% * 20%
    * 15% = 3% overall engagement lift. (A) is slightly higher (3.2% vs 3%). BUT, also consider: (A) affects
    power users (mobile), so if those drive more revenue, (A) is higher. (B) is aspirational (dark mode
    is nice but maybe not core). Size by affected DAU, apply LTV weight, and rank by revenue impact, not
    just engagement %.'
- id: e5c7f5c5-6e4c-40c5-9ebf-f8ffc041dd4d
  type: mcq
  front: You're considering reducing onboarding steps from 5 to 3. Currently, 100K weekly signups complete
    step 1, 70K complete step 2, 40K complete step 3, 30K complete step 4, 20K complete step 5. Simplification
    is expected to save 20% at steps 3-5. What's the opportunity?
  back: 'Steps 3-5 have 40K, 30K, 20K users (90K user-steps in total). Removing 2 steps and saving 20%
    attrition = ~18K saved, but simpler math: average of steps 3–5 is ~30K. 30K * 20% drop-off saved ≈
    6–10K. Conservative estimate: 10K.'
  choices:
  - key: a
    text: 4K users saved (20% of 20K at step 5)
    correct: false
  - key: b
    text: 10K users saved (20% of 50K from steps 3-5 combined)
    correct: false
  - key: c
    text: 14K users saved (summing 20% drop-off avoidance)
    correct: true
  - key: d
    text: 20K users gained (100% savings at step 5)
    correct: false
- id: 3fa71632-a7ba-4756-89d5-2e1a487eb253
  type: flip
  front: Why is cannibalization important in opportunity sizing?
  back: 'A feature might increase one metric (DAU, signups) but decrease another (ARPU, profit margin).
    If you launch a free tier, DAU might +30%, but ARPU might -25%. Net revenue impact depends on weighting:
    if ARPU drives 80% of revenue, -25% ARPU + 30% DAU is a net negative. Always size impact on the true
    north star (revenue, LTV, profit), not just vanity metrics.'
---

## Intuition
Opportunity sizing estimates the potential revenue or impact of solving a problem. Not all problems are worth solving—some affect 100 users, others affect 10M. Size the opportunity first to prioritize ruthlessly.

## Detail
A complete opportunity sizing has three dimensions:

1. **Population**: How many users or transactions are affected?
   - Total addressable market (TAM): e.g., "10M DAU"
   - Serviceable market: e.g., "5M DAU in the US, English-speaking"
   - Serviceable obtainable: e.g., "2M DAU who have Premium subscription"

2. **Impact per unit**: How much does solving it improve the metric?
   - "Reduce checkout friction: +2% conversion rate"
   - "Improve recommendations: +5% engagement hours"
   - "Launch in new country: +20% DAU"

3. **Time & revenue**: What's the economic value over a period?
   - "2M users * +2% conversion * $5 ARPU = $200K annual revenue"
   - "1M engaged users * +$1 LTV = $1M annual LTV increase"

Example: Reducing checkout abandonment
- Population: 500K weekly visitors, 30% add to cart = 150K cart-starters
- Current checkout completion: 60% (90K transactions)
- Opportunity: +5% checkout completion = 7.5K new transactions
- Revenue: 7.5K * $50 ARPU = $375K annual
- Cost to improve: 3 weeks engineering
- ROI: $375K / (3 weeks * $150/hr * 40 hrs) ≈ $375K / $18K ≈ 20x

Prioritization: Size multiple opportunities, rank by ROI or impact, work top-down.

## Common gotchas / interview framings
- **Overlap**: Two improvements might not stack. If you reduce checkout friction AND improve product quality, the combined impact isn't 2% + 3% = 5%; it's closer to 2% + 2.9% = 4.9% (diminishing returns on the same user).
- **Segment variance**: Improving mobile checkout helps mobile users (30% of traffic), but overall impact is 5% * 30% = 1.5%, not 5%.
- **Cannibalization**: Launching a cheaper tier might increase DAU but reduce ARPU. Net impact: DAU +10%, ARPU -20%. If ARPU is 80% of revenue, net revenue impact is negative.
- **Interview scenario**: "We want to reduce onboarding friction. Which improvement has the highest ROI: skip email verification (saves 10% at step 1) or shorten form to 3 fields (saves 5% at step 2)? Population: 100K weekly signups." → Skip email: 100K * 10% = 10K users saved. Shorten form: 90K (after email) * 5% = 4.5K saved. But email verification catches bots (reduce fraud by $2/user prevented). Include fraud savings in the ROI calculation.

## See also
- [[defining-success-metrics-and-north-star]]
- [[cost-benefit-analysis-and-roi]]
- [[funnel-analysis]]

## Sources
See frontmatter `sources:`.

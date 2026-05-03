---
id: b95fd69e-d722-49e0-8ef6-fae7ed0f1138
title: Cost-benefit analysis and ROI
track: data-scientist
topic: product-business-sense
difficulty: 1
tags:
- business
- decision-making
- investment
- experimentation
- prioritization
aliases:
- ROI
- cost-benefit
- value of information
- experiment cost
sources:
- url: https://qubit.capital/blog/evaluating-traction-metrics-consumer-apps
  label: Evaluating Traction Metrics for Consumer Apps
- url: https://www.northbeam.io/blog/what-is-mau-tracking-monthly-active-users-to-gauge-growth
  label: Tracking and Measuring MAU for Growth
cards:
- id: 63eb1c14-6b74-4962-829b-0ae3a7de1327
  type: flip
  front: Define ROI in the context of a product experiment. What three components go into the calculation?
  back: 'ROI = Net Benefit / Cost. (1) Cost: engineering time, infra, opportunity cost, analytics effort.
    (2) Benefit: expected lift (%) * user population * monetization impact * time horizon. (3) Risk: adjust
    for probability of success (most features don''t move the needle—default 30–50% success rate).'
- id: 4476b8ee-c839-4ef3-81ca-142697e9a5a8
  type: mcq
  front: Your team proposes a 4-week feature with estimated +1% LTV impact on 5M users ($1 annual LTV
    increase). The cost is 4 weeks of engineering (1 eng @ $150/hr, 160 hrs). What's the approximate ROI
    if 50% of features succeed?
  back: 'Benefit = 5M users * $1 * 1% = $50K annually. Cost = 160 * $150 = $24K. Unadjusted ROI = $50K
    / $24K ≈ 2.1x. With 50% success rate: $50K * 50% = $25K expected benefit. Expected ROI = $25K / $24K
    ≈ 1x (break-even). Realistic: only build if strategic or if success rate is higher than 50%.'
  choices:
  - key: a
    text: 0.3x (negative ROI, don't build)
    correct: false
  - key: b
    text: 1.6x (break-even with success discount)
    correct: true
  - key: c
    text: 3.1x (positive ROI, consider building)
    correct: false
  - key: d
    text: 8.3x (very positive, highest priority)
    correct: false
- id: afc224f2-f4e1-4aad-99b5-01e8cab51e50
  type: flip
  front: Your team has already spent $100K developing a feature (sunk cost), but early internal testing
    shows it likely won't move key metrics. Engineering wants to ship it anyway ("we're already in").
    How do you evaluate ROI?
  back: 'Ignore the $100K sunk cost. Evaluate forward: (1) Cost to ship and maintain (ongoing dev, support).
    (2) Benefit: likely nil if testing shows no metric improvement. (3) Risk: shipping a feature that
    doesn''t work erodes user trust and clutters the product. Forward-looking ROI is negative. Recommendation:
    Don''t ship. The sunk cost is already lost; shipping compounds the loss.'
- id: 2212c732-1e10-470a-be82-718e0e1b00a9
  type: flip
  front: Why is 'value of information' important in cost-benefit analysis?
  back: Sometimes testing is cheap relative to the decision. A $5K 2-week A/B test tells you whether a
    $500K feature investment is worthwhile. ROI of the test = (expected value of decision) / (test cost).
    If the test prevents a bad $500K investment, the test has 100x+ ROI.
---

## Intuition
Before running an experiment or launching a feature, estimate: Does the potential value (uplift * users affected * time horizon) exceed the cost (development, infra, opportunity cost)? ROI = Net Benefit / Cost. High-ROI bets get prioritized; low-ROI ideas are shelved or deprioritized.

## Detail
Cost-benefit analysis has two sides:

**Costs:**
- Engineering effort (weeks to build)
- Infra/maintenance (server, support)
- Opportunity cost (engineers not working on other projects)
- Experimentation cost (time to run A/B test, data analysis)

**Benefits:**
- Expected lift (e.g., +5% retention)
- Population size (e.g., 10M users)
- Monetization impact (e.g., $1 LTV increase)
- Time horizon (one-time vs. recurring annual benefit)

Example calculation:
- Feature: Improve onboarding. Cost: 2 weeks engineering = 80 hours. Benefit: +3% W1 retention on 50K weekly signups, +$0.50 LTV per user. Annual NPV = 50K * 52 weeks * 3% * $0.50 = $39K. ROI = $39K / (80 * $150/hr) = $39K / $12K ≈ 3.25x. If cost is $12K and benefit is $39K annually, ROI is positive.

Value of information: Sometimes the cost is not building, but testing. Running a 2-week A/B test costs ~$5K in engineering time but tells you if a $200K investment is worth it. That's high ROI (avoid $200K mistake).

## Common gotchas / interview framings
- **Sunk cost fallacy**: Don't justify continuing a low-ROI project because "we've already spent $100K." That $100K is gone; evaluate forward-looking ROI only.
- **Probability discount**: Not every experiment succeeds. 70% of features don't move the needle. Adjust expected value: $39K benefit * 30% success rate = $11.7K expected value. Then ROI = $11.7K / $12K ≈ 1x (break-even).
- **Cannibalization**: A new feature might increase overall engagement but shift time from higher-monetization features. Measure net revenue impact, not gross feature adoption.
- **Interview scenario**: "We can either spend 3 weeks optimizing checkout (estimated +2% conversion, +$500K annual) or build a new feature (unknown upside, 6 weeks). Which has higher ROI?" → Checkout: $500K / $18K (3 weeks) ≈ 27x (higher if you discount for 80% success rate). New feature: unknown upside, but likely won't pay back 6 weeks unless it's a bet-the-company move.

## See also
- [[defining-success-metrics-and-north-star]]
- [[opportunity-sizing]]
- [[data-driven-decision-culture]]

## Sources
See frontmatter `sources:`.

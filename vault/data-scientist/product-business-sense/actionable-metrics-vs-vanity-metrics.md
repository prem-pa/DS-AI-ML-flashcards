---
id: fcc389c9-6bbe-4d16-99c1-e2038dd0065c
title: Actionable metrics vs vanity metrics
track: data-scientist
topic: product-business-sense
difficulty: 1
tags:
- metrics
- analytics
- causality
- product-decision
- business-impact
aliases:
- vanity metric
- meaningful metric
- metric quality
- actionable KPI
sources:
- url: https://www.statsig.com/perspectives/active-users-dau-wau-and-mau-explained
  label: DAU, WAU, and MAU Explained
- url: https://andrewchen.com/dau-mau-is-an-important-metric-but-heres-where-it-fails/
  label: 'Andrew Chen: DAU/MAU and Its Limits'
cards:
- id: 0eecfc86-5cab-4ae7-85a9-5f2e7347cae9
  type: flip
  front: Define actionable metric. What two properties must it have?
  back: 1. You can directly influence it with product changes. 2. It correlates with (and ideally causes)
    business value (revenue, retention, lifetime value). A vanity metric looks impressive but lacks one
    or both properties.
- id: 0b3977b8-f8f5-4b31-944e-f17c05a4439d
  type: mcq
  front: Which metric is most actionable for an e-commerce platform?
  back: 'Checkout completion is actionable: you can A/B test forms, payment methods, etc., and it directly
    predicts revenue. Cumulative visitors ignores quality; pageviews don''t drive purchase; YTD revenue
    is lagged and depends on many factors outside product control.'
  choices:
  - key: a
    text: Total site visitors ever (cumulative)
    correct: false
  - key: b
    text: Checkout completion rate (% of cart-starters who purchase)
    correct: true
  - key: c
    text: Average daily pageviews per user
    correct: false
  - key: d
    text: Total revenue generated YTD
    correct: false
- id: 56719bae-39db-4506-9946-e31ae753aaba
  type: flip
  front: Your team launched a referral feature. A week later, you see 50K new signups (up 3x). But W1
    retention hasn't changed. What's the actionable insight?
  back: 'The growth is vanity. High-volume signups don''t mean product improvement unless retention improves.
    The referral feature may be attracting non-target users or driving low-quality invites. Next: Measure
    retention by acquisition channel and referral depth. If referral signups have sub-par retention, optimize
    incentives or targeting. If overall retention is flat, the feature isn''t working despite signup spikes.'
- id: 7dc950b7-f516-453e-a6de-9491f96ef046
  type: flip
  front: Why is DAU/MAU ratio (stickiness) more actionable than total DAU alone?
  back: 'Total DAU can grow by acquiring many new users without retention improvements (vanity). DAU/MAU
    ratio measures habituality: it tells you what % of monthly users return daily. A rising ratio signals
    product engagement improvement; flat/falling ratio despite DAU growth suggests poor retention, which
    predicts churn.'
---

## Intuition
Actionable metrics are ones you can influence with product changes and that correlate to business outcomes. Vanity metrics look good in a report but don't tell you whether your product is working.

## Detail
An actionable metric has two properties:
1. **You can influence it**: Your product team's decisions directly move it. (Engagement time is actionable; pageviews on auto-refresh is vanity.)
2. **It predicts business value**: When it moves, revenue or retention moves. (Checkout completion rate predicts revenue; total traffic pageviews don't.)

Vanity metric examples:
- Total cumulative signups (ignores quality, churn)
- Total pageviews (ignores depth, intent)
- Visits (passive, doesn't imply engagement)
- Total users ever registered (includes dead accounts)

Actionable metric examples:
- W1 retention (% of week-1 signups still active week 2) → predicts LTV
- Feature usage rate (% of users using feature X monthly) → guides feature roadmap
- Checkout completion by step → pinpoints conversion bottleneck
- DAU/MAU ratio (stickiness) → signals habituality

## Common gotchas / interview framings
- **Correlation trap**: Pageviews correlate with revenue in aggregate, but optimizing pageviews alone (click-baiting) can tank revenue. Ensure causality, not just correlation.
- **Survivor bias**: Monthly active users (MAU) includes churn you can't see. A cohort that looks sticky in MAU might be losing half its users month 1, then stabilizing.
- **Segment blindness**: Average DAU/MAU hides that half your users are power users (80% DAU/MAU) and half are dormant. Measure by segment.
- **Interview scenario**: "We launched a new feature. Users loved it—we got 10K new signups. Should we double down?" → Check: Do these signups retain? Do they engage the feature? Or is viral novelty inflating vanity metrics?

## See also
- [[defining-success-metrics-and-north-star]]
- [[feature-adoption-and-engagement-metrics]]
- [[stakeholder-alignment-and-communication]]

## Sources
See frontmatter `sources:`.

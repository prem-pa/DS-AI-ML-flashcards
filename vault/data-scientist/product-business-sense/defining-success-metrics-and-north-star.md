---
id: 9a970b8b-8bbc-4be7-92c4-b2cbd482a58d
title: Defining success metrics and north star
track: data-scientist
topic: product-business-sense
difficulty: 1
tags:
- metrics
- north-star
- kpi
- alignment
- product-strategy
aliases:
- north star metric
- primary metric
- success metric
- OKR
sources:
- url: https://userpilot.com/blog/north-star-metric/
  label: 'North Star Metric: How to Find Yours & Measure Progress'
- url: https://gopractice.io/product/the-product-managers-guide-to-north-star-metrics/
  label: Product Manager's Guide to North Star Metrics
- url: https://articles.sequoiacap.com/retention
  label: Sequoia Capital on Retention
cards:
- id: f2b152aa-decc-4c37-8094-22e34060962d
  type: flip
  front: What makes a metric a good North Star? Name three criteria.
  back: '1. Causality: Your work must directly influence it (not lagged or external). 2. Business correlation:
    It predicts revenue, retention, or long-term viability. 3. Comprehensiveness: It captures the full
    spectrum of value (not just acquisition).'
- id: 35880936-1b3e-477e-88c2-d9ac8fad58de
  type: flip
  front: Your company optimizes for weekly active users (WAU). Why is this risky as a sole North Star?
  back: 'WAU ignores quality and monetization. You might acquire cheap, low-LTV users or retain unprofitable
    cohorts. A better North Star: user LTV, gross margin, or 30-day retention. WAU is a useful tier-2
    metric, not the primary.'
- id: 9c2938f8-ef4d-4385-b6b2-fd8ce286fff8
  type: mcq
  front: Which statement about North Star metrics is true?
  back: Multiple North Stars create misalignment. A north star must be actionable (changing it is within
    team control). Neither retention nor acquisition is universally better—it depends on business stage
    and model.
  choices:
  - key: a
    text: You should have multiple North Stars to ensure balanced optimization.
    correct: false
  - key: b
    text: A North Star must be actionable by the product team within a sprint.
    correct: true
  - key: c
    text: Retention is always a better North Star than acquisition.
    correct: false
  - key: d
    text: North Star metrics should be reviewed and updated only if business model changes.
    correct: false
- id: b501df20-f931-4d7a-ba63-beac3c3c8644
  type: flip
  front: Signups to your app grew 50% YoY, but annual revenue is flat. Your CEO asks if something is wrong.
    What hypotheses would you test?
  back: '1. **Cohort quality shift**: Are newer cohorts lower-LTV? Check LTV by acquisition month. 2.
    **Pricing/monetization change**: Did monetization or pricing decrease? 3. **Mix shift**: Are signups
    coming from lower-ARPU segments? 4. **Retention cliff**: Do new cohorts churn faster? Run cohort retention
    curves. 5. **Time lag**: Revenue lags signups by months; is this a forecasting issue? Frame the answer:
    ''Which of these is most likely depends on our acquisition channels and product changes in the last
    6 months.'''
---

## Intuition
A North Star metric is the single primary metric that defines product success. It should directly tie to business value, be influenced by product decisions, and guide all teams. Without it, teams optimize locally and misalign.

## Detail
Choosing a North Star requires three things:
1. **Causality**: Does the team's work directly influence it? (Pageviews are vanity; engagement leading to purchases is north star)
2. **Business correlation**: Does it predict revenue, retention, or survival? (DAU for a social network; Gross Margin for a marketplace)
3. **Comprehensiveness**: Does it capture all you care about? (Signups alone miss churn; retention is more complete)

Common North Stars by business model:
- **SaaS**: MRR or retention (30-day, 90-day)
- **Marketplaces**: GMV per supplier or take rate
- **Social/Consumer**: DAU or engagement hours
- **Creator platforms**: Creator monthly revenue

Why not multiple metrics? Multiple metrics create misalignment. Product optimizes one, growth optimizes another. Use a primary North Star + supporting tier-2 metrics.

## Common gotchas / interview framings
- **Gaming the metric**: If North Star is signups, product adds dark patterns. Measure what you truly value or it will be gamed.
- **Lag problem**: If North Star lags (only visible in 90 days), you need leading indicators to steer faster.
- **Survivorship bias**: Retention at day 30 looks good, but cohorts from acquisition channel A have higher lifetime value—you're measuring the wrong cohort.
- **Interview scenario**: "Our signup metric grew 40% but revenue didn't. Why?" → Quality vs. quantity. Cheap CAC users might have poor LTV.

## See also
- [[actionable-metrics-vs-vanity-metrics]]
- [[leading-vs-lagging-indicators]]
- [[cost-benefit-analysis-and-roi]]
- [[stakeholder-alignment-and-communication]]

## Sources
See frontmatter `sources:`.

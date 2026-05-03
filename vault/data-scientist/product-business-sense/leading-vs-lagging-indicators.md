---
id: 458f6abb-5ac7-4d5c-afe9-4cf60e3175e7
title: Leading vs lagging indicators
track: data-scientist
topic: product-business-sense
difficulty: 3
tags:
- metrics
- forecasting
- predictive
- early-signals
- strategy
aliases:
- leading indicator
- lagging indicator
- leading metric
- predictive signal
sources:
- url: https://www.statsig.com/perspectives/understanding-daumau-key-metrics-for-product-success
  label: 'Understanding DAU/MAU: Key Metrics for Product Success'
- url: https://userpilot.com/blog/north-star-metric/
  label: Defining North Star Metrics
cards:
- id: b3d73767-c05d-408d-a903-52297011e401
  type: flip
  front: What's the difference between a leading and lagging indicator? Give one product example of each.
  back: 'Leading: Appears early, predicts future outcome. Example: Feature adoption % predicts feature
    stickiness weeks later. Lagging: Summarizes past behavior. Example: 30-day cohort retention (visible
    30 days after signup). Use leading to steer; use lagging to validate strategy.'
- id: 5a68557b-5831-434e-bbeb-aa08c53395df
  type: mcq
  front: Which is a leading indicator for 30-day subscription retention?
  back: Feature usage in week 1 predicts month-1 retention and beyond. Churn rate is lagging (you see
    it after churn happens). ARR and support tickets are lagging/aggregate metrics, not predictive of
    individual retention.
  choices:
  - key: a
    text: Churn rate in month 3
    correct: false
  - key: b
    text: Feature usage days in week 1
    correct: true
  - key: c
    text: Annual recurring revenue
    correct: false
  - key: d
    text: Number of support tickets resolved
    correct: false
- id: 71144f9b-e751-43ff-a438-ca10814f3e89
  type: flip
  front: You A/B test a new onboarding flow. After 3 days, the test variant shows higher DAU. After 30
    days, cohort retention is identical. What happened?
  back: 'DAU at day 3 is a leading signal, but it''s misleading here. The new flow likely increases early
    engagement (users are in the flow more) but doesn''t improve long-term retention. The leading indicator
    (DAU day 3) didn''t predict the lagging outcome (retention day 30) because the causal mechanism was
    wrong. Lesson: Validate that your leading indicator mechanistically predicts the lagging outcome,
    not just correlates in retrospect.'
- id: 0daf2e70-c8f8-4670-8da7-b413eb842973
  type: flip
  front: Why do you need both leading and lagging indicators?
  back: Lagging indicators (retention, revenue, churn) are true outcome measures but arrive weeks/months
    later, too late to course-correct mid-experiment. Leading indicators (engagement, feature usage) give
    you feedback within days, letting you pivot quickly. But leading indicators can be gamed or misaligned,
    so you validate with lagging metrics.
---

## Intuition
Leading indicators predict future outcomes; lagging indicators measure past results. To steer in real-time, you need leading indicators. Lagging indicators validate strategy but arrive too late to course-correct.

## Detail
Leading indicators appear first and predict lagging outcomes:
- **User engagement metrics** (DAU, feature engagement, time-in-app) → predict 30-day retention
- **Invites sent** → predict viral growth (leading K-factor)
- **Onboarding completion** → predicts W1 retention
- **Payment method on file** → predicts subscription renewal
- **Feature adoption rate** → predicts feature stickiness

Lagging indicators are the end result:
- **30-day cohort retention** (visible 30 days after signup)
- **Revenue / MRR** (aggregates many user behaviors from weeks/months back)
- **Churn rate** (you only know after a user churns)
- **LTV** (requires years of data)

Time lag examples:
- A/B test launches on day 1. Engagement signal (leading) moves day 2–7. Retention signal (lagging) visible day 30–35. Revenue signal visible day 60+.
- New feature release: usage rate is leading; impact on DAU/MAU visible days later; impact on MRR visible months later.

## Common gotchas / interview framings
- **Leading ≠ causal**: High onboarding completion correlates with retention, but if you force users through bad onboarding, correlation breaks. Leading indicators guide; they don't guarantee causation.
- **Segment mismatch**: Engagement metric for power users predicts *their* retention, not free-tier users. Ensure leading indicator matches the lagging population.
- **Diminishing predictivity**: A leading indicator strong last quarter might weaken this quarter if product or user mix shifts. Monitor and re-validate.
- **Interview scenario**: "We shipped a redesign on Monday. Tuesday we see no improvement in DAU. Should we revert?" → DAU is lagging; look for leading signals (session length, feature usage rate, early cohort retention). Week-long test window is too short for lagging validation.

## See also
- [[defining-success-metrics-and-north-star]]
- [[feature-adoption-and-engagement-metrics]]
- [[churn-prediction-and-retention-modeling]]
- [[data-driven-decision-culture]]

## Sources
See frontmatter `sources:`.

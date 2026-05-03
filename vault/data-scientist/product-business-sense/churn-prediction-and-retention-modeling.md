---
id: 24691ceb-9c91-450c-be74-7146f0e16ce7
title: Churn prediction and retention modeling
track: data-scientist
topic: product-business-sense
difficulty: 3
tags:
- predictive
- retention
- churn
- machine-learning
- lifecycle
- intervention
aliases:
- churn risk
- retention prediction
- at-risk users
- survival analysis
sources:
- url: https://articles.sequoiacap.com/retention
  label: Sequoia Capital on Retention and Churn
- url: https://clevertap.com/blog/viral-coefficient/
  label: Viral Coefficient and Customer Lifecycle
cards:
- id: ec3a4c65-36d8-4873-ad26-a6aafc4a2ff7
  type: flip
  front: Define churn prediction. What are three signals that predict a user is at risk of churning?
  back: 'Churn prediction: Use historical behavior to score each user''s risk of inactivity in a future
    period (e.g., next 30 days). Three signals: (1) Engagement decline: usage dropping over weeks. (2)
    Feature adoption gap: never tried key features (lower sticky). (3) Session metrics: short sessions
    or infrequent logins (low engagement). (Other: not monetizing, support tickets, cohort age.)'
- id: 8aea639b-2b84-421a-a88a-4ddc55d461cc
  type: mcq
  front: 'You built a churn model: 90% accuracy, 70% precision (of flagged at-risk users, 70% actually
    churn), 50% recall (of all churners, you caught 50%). Should you intervene on all flagged users?'
  back: 'Accuracy is misleading (class imbalance: most users don''t churn, so "predict no one churns"
    is 95% accurate but useless). Precision (70%) means 30% of interventions are wasted on false positives.
    Intervention ROI depends on: cost of intervention vs. LTV of retained user. If LTV = $100 and intervention
    = $5, even 30% FP is worth it. Must A/B test to confirm intervention actually prevents churn.'
  choices:
  - key: a
    text: Yes, 90% accuracy is excellent.
    correct: false
  - key: b
    text: No, 30% of interventions waste budget on false positives.
    correct: false
  - key: c
    text: Maybe, depends on intervention cost vs. LTV and whether intervention actually prevents churn
      (A/B test).
    correct: true
  - key: d
    text: Yes, 70% precision is high enough to warrant broad intervention.
    correct: false
- id: 0b0a221d-b713-450c-8925-41d1babecc43
  type: flip
  front: Your churn model flags 100K at-risk users. You send a $5 discount offer. 30% accept (30K), and
    70% of those (21K) would have churned anyway (vs. 30% baseline churn in control). Did the intervention
    work?
  back: 'Intervention prevented 21K churn. Cost: 100K * $5 = $500K. Savings: 21K users * $100 LTV (example)
    = $2.1M. ROI is positive (4x). BUT: 70% of intervention cost ($350K) went to false positives or users
    who would have stayed anyway. Real ROI: net 21K prevented churn / 100K users = 21pp lift, or $2.1M
    - $500K = $1.6M net gain. Decision: scale up, but test more targeted interventions to improve precision.'
- id: 9e242126-36d1-4450-aad2-661169dece9b
  type: flip
  front: Why is 'survivorship bias' a problem in churn prediction?
  back: If you only model users who survived to day 30, you miss day 1–7 churners. These early churners
    have different predictors (onboarding friction, wrong product-market fit for segment) than late churners.
    Build separate models for early churn (1–7 day) and late churn (30+) to avoid bias.
---

## Intuition
Churn prediction uses historical user behavior to identify users at risk of leaving soon. With early warning, you can intervene (discounts, re-engagement campaigns, support outreach) before they churn. This is far cheaper than acquisition and improves retention metrics.

## Detail
A churn prediction model typically:

1. **Define churn**: 30 days inactive (varies by product; daily-active apps may use 7 days, subscription may use 60 days)

2. **Feature engineering**: Build signals from user history
   - Engagement trend: Is usage rising, stable, or declining? (Declining predicts churn)
   - Feature adoption: Did user try key features? (Non-adopters churn more)
   - Session length / frequency: Short sessions or infrequent access signal risk
   - Monetization: Did user ever pay? (Paying users churn less)
   - Support signals: Open tickets, refund requests (predict churn)
   - Cohort age: When did they sign up? (Early cohorts more at-risk)

3. **Training data**: Label users as churned/not churned at a certain horizon (e.g., "user churned in days 30–60") and build a classifier

4. **Ranking users**: Score each active user 0–1 (churn risk). Intervene on top 10–20%

Intervention examples:
- **Email re-engagement**: "We noticed you haven't used X in a week. Here's what you're missing."
- **Discount offer**: "Come back for 20% off. Valid 7 days."
- **Support outreach**: "You flagged difficulty with feature Y. Our team can help."
- **Onboarding gap**: "You haven't tried personalized recommendations. Let's set it up."

Metrics:
- **Precision**: Of users flagged, what % actually churn? (If you're wrong, you waste discount offers)
- **Recall**: Of users who churn, what % did you catch? (Miss some, some interventions fail)
- **Lift**: Do at-risk users who receive intervention churn less than control? (A/B test to measure impact)

## Common gotchas / interview framings
- **Survivorship bias**: Only users who survived to day 30 are in your training data. Day 1 churners are invisible. Build separate models for early churn (day 1–7) and late churn (day 30+).
- **Stale data**: If prediction happens on weekly batch, there's a lag. By the time you score, user might already have churned. Real-time prediction requires streaming data and fast scoring.
- **Intervention paradox**: Targeting at-risk users might signal that the app knows you're losing interest (negative user experience). Frame positively: "New feature you'll love" vs. "Come back or lose access."
- **Interview scenario**: "You built a churn model; it's 85% accurate. Why wouldn't you use it to intervene on all high-risk users?" → (1) Precision matters: is 85% accuracy good precision (low false positives)? (2) Intervention ROI: cost of intervention vs. LTV of retained user. (3) A/B test impact: does intervention actually prevent churn? (4) Frequency: over-emailing causes more churn.

## See also
- [[leading-vs-lagging-indicators]]
- [[cohort-analysis-and-retention-curves]]
- [[feature-adoption-and-engagement-metrics]]

## Sources
See frontmatter `sources:`.

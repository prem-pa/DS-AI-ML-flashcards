---
id: 9f1c41a0-414b-4aa5-b1a0-cd0bd0e16c66
title: Feature adoption and engagement metrics
track: data-scientist
topic: product-business-sense
difficulty: 3
tags:
- analytics
- feature
- adoption
- engagement
- dau-mau
- product-health
aliases:
- feature adoption rate
- engagement metrics
- feature activation
- feature usage
sources:
- url: https://www.statsig.com/perspectives/active-users-dau-wau-and-mau-explained
  label: DAU, WAU, and MAU Explained
- url: https://www.statsig.com/perspectives/understanding-daumau-key-metrics-for-product-success
  label: Understanding DAU/MAU for Product Success
cards:
- id: b249d036-2a52-43b5-b16f-7b17180dff32
  type: flip
  front: Define DAU, MAU, and DAU/MAU ratio. What does a high ratio indicate?
  back: DAU = daily active users. MAU = monthly active users. DAU/MAU ratio = DAU / MAU (e.g., if 1M DAU
    and 5M MAU, ratio is 20%). High ratio (>50%) indicates habituality—users return daily. Low ratio (<10%)
    indicates usage is sporadic. Healthy ratio is 20–40% for social/consumer products.
- id: fd70648c-ee62-4e03-ab03-a641878d796b
  type: flip
  front: Your app's DAU is flat at 2M, but MAU grew from 5M to 7M. DAU/MAU dropped from 40% to 29%. What's
    happening?
  back: 'More users are signing up (MAU +40%) but aren''t returning regularly (DAU flat). This is acquisition
    without retention—classic sign of churn masking. Each cohort might be getting stickier, but older
    cohorts are churning fast. Next: (1) Check cohort retention curves—do new cohorts have better W1 retention?
    (2) Check DAU composition—is DAU still from early cohorts (bad) or new cohorts (good)? (3) Engagement
    issue: users try the app once but don''t return. Focus on onboarding and early engagement.'
- id: b933a8b9-8954-442a-898d-44e9f9292333
  type: mcq
  front: 'Feature X launched to 100% of users 4 weeks ago. Adoption: Week 1: 60%, Week 2: 55%, Week 3:
    50%, Week 4: 48%. Is this feature successful?'
  back: '60% adoption week 1, declining to 48%, suggests novelty factor or forced onboarding. Healthy
    feature adoption stabilizes (e.g., 60% → 55% → 55% → 55%). Declining adoption suggests the feature
    isn''t sticky. Context: If it''s a secondary feature (nice-to-have), 48% stable might be fine. If
    it''s primary, declining is bad and warrants investigation (is the feature valuable? Is it discoverable?).'
  choices:
  - key: a
    text: Yes, 60% adoption is very high.
    correct: false
  - key: b
    text: No, adoption is declining; the feature is losing interest.
    correct: false
  - key: c
    text: Unclear; need to know if the decline is normal (novelty wearing off) or bad (feature doesn't
      deliver value).
    correct: true
  - key: d
    text: Yes if adopters have higher LTV, no if they don't.
    correct: false
- id: 8ef2f639-a48a-4f9f-a73e-d38be31a9ec1
  type: flip
  front: Why is 'feature adoption rate' more actionable than 'total feature usage count'?
  back: Total usage count can grow by simply adding more users (DAU growth), masking that the feature
    is actually less popular per user. Adoption rate (% of DAU using feature) isolates the feature's appeal.
    If DAU grows 20% but adoption rate falls 10%, the feature is becoming less attractive per user—a signal
    to investigate quality or discoverability.
---

## Intuition
DAU (Daily Active Users) and MAU (Monthly Active Users) measure engagement breadth. Feature adoption measures depth: what % of users actually use a feature? Combined, these metrics show if your product is engaging and if new features drive stickiness.

## Detail
Key engagement metrics:

**Breadth metrics:**
- **DAU**: # unique users active (any interaction) in a day
- **MAU**: # unique users active in a month
- **DAU/MAU ratio (stickiness)**: DAU / MAU. >20% is healthy; >50% is exceptional.
- **WAU**: Weekly active users (in between)

**Feature-level:**
- **Feature adoption rate**: % of DAU who used feature X (e.g., 60% of users use search)
- **Feature depth**: % of DAU who used feature X + secondary feature (e.g., saved a search result)
- **Time to adoption**: Days until user first uses feature after signup
- **Feature engagement decay**: Usage over weeks—does it drop off or stabilize?

**Interpretation:**
- New feature with 40% adoption week 1, 35% week 2, 25% week 4 → early curiosity, low stickiness. Not core to product.
- Feature with 40% adoption week 1, 38% week 2, 37% week 4 → sticky. Driving habit formation.

Roadmap implications:
- High DAU/MAU + stable DAU → product is sticky, focus on monetization
- Rising DAU/MAU + rising DAU → product is improving engagement, growth is solid
- Flat DAU + low DAU/MAU + declining feature adoption → engagement crisis, need product refresh

## Common gotchas / interview framings
- **Adoption ≠ engagement**: 80% of users might try a feature once, but 10% use it regularly. Measure repeat usage, not just "ever used."
- **Time-of-day effects**: DAU can spike on certain days (weekends) or times (evenings). Use weekly or monthly aggregates to smooth.
- **Missing definition**: If "active" means "logged in" vs. "took an action," metrics diverge. Define clearly and consistently.
- **Feature cannibalization**: A new feature might increase engagement but shift time from older features (net engagement flat). Measure overall DAU/engagement time, not just feature adoption.
- **Interview scenario**: "Feature X launched 4 weeks ago; adoption is 50%. Is it successful?" → Context matters: (1) Is it sticky (adoption staying at 50% or declining)? (2) Do adopters have higher retention/revenue? (3) What's comparable to similar features? (4) Is it a primary or secondary feature? (50% adoption is great for a secondary feature, disappointing for core.)

## See also
- [[actionable-metrics-vs-vanity-metrics]]
- [[leading-vs-lagging-indicators]]
- [[cohort-analysis-and-retention-curves]]

## Sources
See frontmatter `sources:`.

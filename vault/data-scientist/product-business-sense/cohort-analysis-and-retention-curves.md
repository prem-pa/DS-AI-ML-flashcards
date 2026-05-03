---
id: ebe08a26-b043-42f8-ac4b-bbf7fe821dd3
title: Cohort analysis and retention curves
track: data-scientist
topic: product-business-sense
difficulty: 3
tags:
- analytics
- retention
- cohort
- lifecycle
- user-segmentation
aliases:
- cohort retention
- retention curve
- cohort tracking
- user lifecycle
sources:
- url: https://articles.sequoiacap.com/retention
  label: Sequoia Capital on Retention
- url: https://www.geckoboard.com/best-practice/kpi-examples/dau-mau-ratio/
  label: DAU/MAU Ratio and KPI Examples
cards:
- id: d1bc7ef3-ad21-49f0-b10a-0e953b7353fb
  type: flip
  front: Define a cohort and explain why cohort analysis is better than overall DAU/MAU trends.
  back: 'A cohort is a group of users sharing a common attribute (usually signup date). Cohort analysis
    tracks each cohort independently over time. It''s better than overall DAU/MAU because DAU can mask
    mixing effects: if old cohorts (low retention) shrink and new cohorts (high retention) grow, overall
    DAU looks flat when both are degrading. Cohorts isolate user lifecycle signals.'
- id: 1f924466-09db-4b92-ae16-cfc90d187888
  type: mcq
  front: You see overall MAU is flat month-over-month, but cohort analysis shows all cohorts declining
    in retention. What's happening?
  back: 'Flat MAU with declining cohort retention is classic mix-shift masking. New acquisition offsets
    churn, so DAU/MAU look stable, but long-term each cohort is healthier or worse. Real issue: declining
    retention. Investigate product changes, onboarding quality, or feature adoption.'
  choices:
  - key: a
    text: Product quality is improving (users staying longer)
    correct: false
  - key: b
    text: User acquisition volume increased, masking retention decay
    correct: true
  - key: c
    text: Seasonal trends are averaging out retention changes
    correct: false
  - key: d
    text: Revenue is increasing despite flat user counts
    correct: false
- id: 8a2d1de9-98f2-4bfb-a813-115cb3319972
  type: flip
  front: Week-1 retention for organic signups is 60%; paid ads is 40%. Same product. Why the difference,
    and what's your recommendation?
  back: 'Organic users likely have higher intent (they searched for you) and are better qualified. Paid
    ads may be attracting low-fit users. Options: (1) Improve onboarding to help paid-ads users. (2) Refine
    ad targeting/messaging to attract higher-intent users. (3) Measure LTV: if paid-ads users have lower
    W1 retention but higher LTV (they convert on monetization), the channel is still valuable. Benchmark:
    which channel is more profitable despite retention difference?'
- id: 3781c5d7-157b-465b-8063-1d38b51d71d8
  type: flip
  front: What's the difference between week-1 retention and month-1 retention, and why should you track
    both?
  back: Week-1 retention shows early engagement (onboarding effectiveness, initial product clarity). Month-1
    retention shows habit formation (is the product sticky enough to build routine usage?). Week-1 helps
    diagnose onboarding problems; month-1 helps validate product-market fit. Declining week-1 with stable
    month-1 suggests onboarding issue, not product issue.
---

## Intuition
Cohort analysis groups users by a shared characteristic (signup week, acquisition channel, feature adoption date) and tracks their behavior over time. Retention curves show how many users remain active at week 1, week 2, week 4, etc. This reveals user lifecycle patterns, identifies problem cohorts, and validates product improvements.

## Detail
A cohort is a group of users sharing one attribute, usually signup date ("week of Feb 2") or acquisition channel ("organic search" vs. "paid ads"). For each cohort, you calculate:
- **N0**: Users in cohort (e.g., 1,000 signups)
- **N1**: Users active week 1 after signup (900, so 90% retention)
- **N4**: Users active week 4 (500, so 50% retention)
- **N12**: Users active week 12 (300, so 30% retention)

Retention curves are plots of N_t / N_0 vs. time. Healthy patterns:
- **Day 1 retention**: 50–70% (half of signups don't return)
- **Week 1 retention**: 30–50% (major drop-off; if >70%, sticky product)
- **Month 1 retention**: 20–40%
- **Month 3 retention**: 5–20% (if still >10%, strong product-market fit)

Cohorts reveal:
- **Seasonal shifts**: Summer cohorts churn faster (school, vacation)
- **Acquisition quality**: Paid-search cohorts more engaged than display cohorts
- **Product changes**: Cohorts before/after feature launch show improvement
- **Geographic/demographic patterns**: Users from Asia vs. US have different lifecycle curves

## Common gotchas / interview framings
- **Selection bias**: Only tracked users retained at day 1 in some systems, so your W1 retention looks artificially high. Always include signup-to-day-1 drop-off.
- **Interval mismatch**: Measuring weekly but users engage on weekends differently. Use daily cohorts for weekly analysis, or weekly for monthly, to smooth.
- **Reactivation paradox**: A user inactive for 60 days comes back. Which cohort? Usually: original signup cohort, but flag as reactivation separately.
- **Interview scenario**: "Our month-1 retention dropped 5 points YoY. Investigate." → Run cohort analysis: Which acquisition channel(s) dropped? Which feature adoption dropped? Is it onboarding, feature quality, or mix shift? Segment by device, region, or new features.

## See also
- [[leading-vs-lagging-indicators]]
- [[feature-adoption-and-engagement-metrics]]
- [[churn-prediction-and-retention-modeling]]

## Sources
See frontmatter `sources:`.

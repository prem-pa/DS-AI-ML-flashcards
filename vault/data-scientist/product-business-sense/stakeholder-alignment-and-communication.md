---
id: 42def202-9f64-4cdc-9353-8d3f38b6cb8c
title: Stakeholder alignment and communication
track: data-scientist
topic: product-business-sense
difficulty: 1
tags:
- communication
- stakeholder
- framing
- narrative
- business-context
aliases:
- stakeholder management
- executive summary
- narrative framing
- audience adaptation
sources:
- url: https://articles.sequoiacap.com/retention
  label: Retention as a Stakeholder Metric
- url: https://gopractice.io/product/the-product-managers-guide-to-north-star-metrics/
  label: Metrics for Cross-Functional Alignment
cards:
- id: 0fdab2da-add8-4467-93a8-685b1898d36d
  type: flip
  front: Why does stakeholder communication matter in data science? How would you frame the same result
    differently for engineering vs. an exec?
  back: 'Different stakeholders optimize for different metrics. Engineering cares about technical feasibility
    and system health. Execs care about revenue and quarterly targets. Same finding ("feature reduces
    latency by 200ms"): Engineering frame: "Improves system efficiency, reduces load by 15%." Exec frame:
    "Faster experience drives +2% engagement, estimated +$500K annual impact."'
- id: 29b029cf-5c48-445a-9d1d-54c7126e1709
  type: flip
  front: 'You ran an A/B test: test variant has +5% engagement but -8% monetization. Revenue is net negative.
    How do you present this to (a) Product, (b) Execs?'
  back: '(a) Product: ''The feature increases engagement and is directionally good for retention, but
    monetization suffers in the short term. We recommend: (1) explore monetization levers (ads, paywalls)
    alongside this feature, or (2) use this as a low-monetization feature for free tier to increase DAU,
    monetizing separately.'' (b) Execs: ''Test shows +5% engagement but -8% monetization, resulting in
    -$X revenue impact over [timeframe]. Not recommended for ship in current form. Options: (1) refine
    to preserve monetization (add pricing lever), (2) use as acquisition feature (tier differentiation),
    or (3) shelf until we have monetization strategy.'''
- id: a280cbf0-d3ab-4aef-b204-fa8882bc8dbd
  type: mcq
  front: 'You present a finding to your CEO: ''W1 retention improved from 40% to 43%.'' The CEO asks,
    ''So we''ll make more money?'' What''s the most complete answer?'
  back: The CEO cares about money, not abstract metrics. You must bridge W1 retention to dollars. Higher
    W1 retention is a leading indicator of higher LTV, so estimate impact. But acknowledge uncertainty
    and other factors (churn might happen at week 8, monetization might be static). Option (a) is oversimplified;
    (c) is cautious but avoids the question; (d) is false.
  choices:
  - key: a
    text: Yes, higher retention means more users will pay.
    correct: false
  - key: b
    text: 'Likely yes. Higher W1 retention predicts higher LTV. Conservative estimate: +$X per cohort.
      Across 100K weekly signups, +$Y annual impact. But other factors (monetization, churn cliff) matter
      too.'
    correct: true
  - key: c
    text: We need to run a longer test to see revenue impact.
    correct: false
  - key: d
    text: Retention improvements always drive revenue growth.
    correct: false
- id: 899aa2d2-fb93-42b9-ac63-4db4420abcae
  type: flip
  front: What's the difference between a data-driven narrative and selective framing?
  back: 'Data-driven narrative: Show the full picture, including caveats and alternative explanations.
    (''Retention +3%, but churn curve shows cliff at week 8; product changes to address this are...'')
    Selective framing: Cherry-pick metrics that look good and hide bad ones. (''DAU +20% Q3!'' while hiding
    ''-40% cohort retention''). Selective framing erodes trust; narrative with caveats builds credibility.'
---

## Intuition
Different stakeholders care about different things. Engineers care about technical feasibility and debt. Product cares about user outcomes and roadmap. Execs care about revenue and quarterly targets. Communicate findings in language each audience understands, with metrics that matter to them.

## Detail
Frame results for different audiences:

**Engineering:**
- Lead with impact on technical debt and system health ("75% reduction in checkout latency")
- Quantify engineering effort: "2 weeks to implement, 0.5 weeks to monitor"
- Highlight stability wins: "Reduces error rate by 0.05%"

**Product:**
- Lead with user outcomes: "W1 retention improved from 40% to 43%"
- Explain feature impact: "New recommendations feature drives +8% engagement for power users"
- Tie to roadmap: "Unblocks 3 downstream features"

**Executive / Finance:**
- Lead with revenue or growth: "Feature expected to drive +$2M annual ARR"
- Provide confidence interval: "Conservative scenario: +$1M; optimistic: +$3M"
- Give payback period: "Breaks even in 6 months; 2.5x ROI in 12 months"

**Marketing / Growth:**
- Lead with acquisition or engagement: "Referral feature drives +15% viral coefficient"
- Segment by channel: "High-value users (LTV>$500) are 2x more likely to refer"

Common mistakes:
- Leading with technical details instead of business impact
- Using p-values and confidence intervals for non-technical audiences
- Showing raw data instead of stories ("DAU +150K" without context)
- Ignoring caveats or alternative explanations

## Common gotchas / interview framings
- **Selective framing**: Showing only the metric that looks good ("signups +40%") while hiding the bad one ("churn +15%") erodes trust. Communicate the full picture, even hard truths.
- **Jargon mismatch**: Saying "cohort retention" to an exec who only knows "LTV" confuses. Translate: "cohort retention predicts lifetime value."
- **Overconfidence**: "This feature will increase revenue by 20%" without caveats. Better: "Based on comparable products, we expect a +10–20% uplift; we'll test and measure."
- **Interview scenario**: "Present the same result (W1 retention +3%) to engineering, product, and the CFO." → Engineering: "Onboarding optimization reduced tutorial friction by 40%, latency impact negligible." Product: "Improved early user experience; retention cohorts 3pp higher across all channels." CFO: "W1 retention predictor of LTV; +3pp suggests +$X million annual LTV impact; ROI of improvement is Y."

## See also
- [[defining-success-metrics-and-north-star]]
- [[cost-benefit-analysis-and-roi]]
- [[data-driven-decision-culture]]

## Sources
See frontmatter `sources:`.

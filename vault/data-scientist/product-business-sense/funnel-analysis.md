---
id: 5068026e-6bfc-422b-aec6-405aec6fed09
title: Funnel analysis
track: data-scientist
topic: product-business-sense
difficulty: 1
tags:
- analytics
- conversion
- user-journey
- bottleneck
- optimization
aliases:
- conversion funnel
- drop-off analysis
- funnel step
- user flow
sources:
- url: https://www.wudpecker.io/blog/how-to-use-funnel-analysis-and-data-to-reduce-user-churn
  label: Funnel Analysis and User Churn Reduction
- url: https://www.geckoboard.com/best-practice/kpi-examples/dau-mau-ratio/
  label: KPI Examples and Funnel Metrics
cards:
- id: b5d68b23-eb0b-40ee-8d80-d20b3a74d62c
  type: flip
  front: Define funnel analysis. What does it reveal, and why should you focus on the biggest drop-off
    step?
  back: 'Funnel analysis tracks users through a series of steps toward a goal (e.g., signup → activation).
    It reveals drop-off at each step, showing where users abandon. Focus on biggest drop-off because:
    (a) it''s where most users are lost, (b) even small % improvement affects many users, (c) likely highest
    ROI (remove biggest friction first).'
- id: dc34696d-0e32-470d-a01d-1691049bd554
  type: mcq
  front: 'A mobile checkout funnel: 1M landing, 800K product page, 400K add-to-cart, 350K checkout start,
    315K complete. Which step should you optimize first?'
  back: 'Step 2→3 has 50% friction (400K / 800K), the highest %. Even 10% improvement = 40K users. Step
    1→2 looks big (200K), but it''s 25% of traffic (200K / 800K), typical for landing-to-product-page
    drop. Rule of thumb: optimize highest % friction first unless segment analysis reveals a cheaper fix
    elsewhere.'
  choices:
  - key: a
    text: 'Step 1→2: 200K users drop (highest absolute)'
    correct: false
  - key: b
    text: 'Step 2→3: 400K users drop (50% friction, highest %)'
    correct: true
  - key: c
    text: 'Step 4→5: 35K users drop (lowest absolute)'
    correct: false
  - key: d
    text: 'Step 3→4: depends on effort to optimize relative to impact'
    correct: false
- id: 8643f161-bf72-4a57-bf8b-753d5212f30b
  type: flip
  front: 'Desktop checkout completion: 70%. Mobile: 45%. Mobile traffic is 60% of total. What''s the weighted
    average, and where should you focus?'
  back: 'Weighted avg = 45% * 60% + 70% * 40% = 27% + 28% = 55%. Mobile is the bottleneck: 25pp gap vs.
    desktop. But before optimizing, confirm: (1) Is the gap real or data quality (tracking bug)? (2) Which
    mobile step is the problem (onboarding, payment entry, button size)? (3) What''s ROI: 60M mobile users
    * 10pp improvement * ARPU. If ROI is positive, mobile checkout is your priority.'
- id: c5f7a835-5b9f-4945-bf96-bd6b631738ea
  type: flip
  front: Why is segment analysis important in funnel analysis? Give an example.
  back: 'Overall funnel hides segment differences. Example: Overall signup-to-activation drop is 30%,
    but paid-ads cohorts are 50% while organic is 20%. Optimizing the average funnel won''t help—paid-ads
    users have different needs (onboarding, value prop clarity). Segment by acquisition channel, device,
    geography, and user type to find root causes.'
---

## Intuition
A funnel is a sequence of steps users must complete to reach a goal (signup → email verify → onboarding → activation, or browse → add-to-cart → checkout → purchase). Funnel analysis measures drop-off at each step, revealing bottlenecks and optimization opportunities.

## Detail
A typical e-commerce funnel:
- **Step 1**: Landing page viewed: 100K users
- **Step 2**: Product page viewed: 80K (20% drop)
- **Step 3**: Add to cart: 40K (50% drop)
- **Step 4**: Checkout started: 35K (12.5% drop)
- **Step 5**: Payment info entered: 30K (14% drop)
- **Step 6**: Purchase completed: 27K (10% drop)

Funnel metrics:
- **Drop-off**: # users at step N - # at step N+1 (e.g., 40K−35K = 5K drop at checkout)
- **Conversion rate**: Users completing step / entering step. (e.g., 27K / 100K = 27% overall; 27K / 35K = 77% from checkout start to purchase)
- **Friction**: % of users lost. (e.g., step 3 has 50% friction; 50% don't add to cart)

Where to focus:
- **Biggest drop-off**: Step 3 (50% friction). Even 10% improvement = 4K users, likely high ROI.
- **Unexplained drop**: Step 2 to 3 (50% jump from typical 20% churn). Suggests friction on product page, not elsewhere.
- **Segment analysis**: Mobile has 60% step 3 drop; desktop 40%. Mobile checkout is the bottleneck.

## Common gotchas / interview framings
- **Survivor bias**: Funnel only tracks users who enter. If someone lands on page but bounces before event fires, they're not in the funnel. The true step-1 count is higher.
- **Multiple paths**: Real funnels aren't linear (e.g., some users add to cart via wishlist, not product page). Multi-funnel analysis captures this.
- **Attribution**: If step 2→3 drop rises after a mobile redesign, is the redesign bad or did mobile traffic mix change? Segment by device and timing.
- **Interview scenario**: "Checkout completion dropped 5pp in the last week. Diagnose." → (1) When exactly (by day, time)? (2) Which devices/geographies? (3) Did you change payment provider, page design, or validation logic? (4) Segment funnel by cohort/device. If drop is mobile-only, it's a frontend issue; if all devices, it's backend/payment.

## See also
- [[cohort-analysis-and-retention-curves]]
- [[feature-adoption-and-engagement-metrics]]
- [[actionable-metrics-vs-vanity-metrics]]

## Sources
See frontmatter `sources:`.

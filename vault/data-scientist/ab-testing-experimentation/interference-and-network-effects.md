---
id: b5315340-cee0-4a96-ac83-447b98bc1b82
title: Interference and network effects
track: data-scientist
topic: ab-testing-experimentation
difficulty: 3
tags:
- interference
- spillover-effects
- network-effects
- sutva-violation
- treatment-assignment
aliases:
- spillover effects
- SUTVA violation
- peer effects
- cross-user effects
sources:
- url: https://netflixtechblog.com/a-b-testing-and-beyond-improving-the-netflix-streaming-experience-with-experimentation-and-data-5b0ae9295bdf
  label: 'Netflix: Handling Interference in Experiments'
- url: https://research.netflix.com/research-area/experimentation-and-causal-inference
  label: 'Netflix: Causal Inference Research'
cards:
- id: f3232beb-b2d5-45d0-9c6b-77cb51fb0ca8
  type: flip
  front: ''
  back: ''
- id: 661dc386-bda6-4a52-aba6-3cf1a12854fc
  type: flip
  front: ''
  back: ''
- id: 8064bc45-c7c3-4211-96ee-14bfaca9d038
  type: mcq
  front: ''
  back: ''
  choices:
  - key: a
    text: Yes, buyers are randomized independently
    correct: false
  - key: b
    text: No, buyers interact with both treated and untreated sellers (interference)
    correct: false
  - key: c
    text: Yes, if the marketplace is large enough
    correct: false
  - key: d
    text: No, pricing algorithm violates SUTVA
    correct: false
- id: b4b1e3b5-d306-47cf-8e2e-7664e4bf6011
  type: flip
  front: ''
  back: ''
---

## Intuition

**SUTVA** (Stable Unit Treatment Value Assumption) assumes treatment for one user doesn't affect outcomes of other users. But on networks (social, marketplaces), treated users affect untreated users (interference).

## Detail

**SUTVA violation**:
- Treatment to user A affects user B's outcomes
- Randomization no longer ensures causality
- Classic example: recommend treatment to user A → influences user B's feed → B's metrics change

**Types of interference**:
1. **Direct network effects**: User A is treated; affects friends of A directly
   - Example: recommendation algorithm change for A → A sends better recommendations → A's friends engage more
2. **Competitive effects**: User A is treated; affects competing users
   - Example: seller A gets new tools → sells more → buyer has fewer options from A → shifts to seller B
3. **Market effects**: Treated users change supply/demand for everyone
   - Example: price change for A → affects overall price equilibrium → affects untreated users' willingness to pay

**Detection**:
- **Spillover analysis**: Check if control users near treated users differ from control users far from treated users
- **Geographic/network analysis**: If treatment propagates (e.g., via recommendations), look for distance decay (effect fades with distance)

**Solutions**:
1. **Cluster randomization**: Randomize at network/marketplace/geography level (not user)
   - Pros: avoids interference
   - Cons: fewer units, less power

2. **Switchback experiments**: Randomize time periods, not users (for two-sided marketplaces like Doordash)
   - Pros: avoids user interference
   - Cons: temporal confounds (day-of-week effects)

3. **Ego-network randomization**: Keep treated user's network separate
   - Example: randomize at friend-group level
   - Pros: captures direct effects
   - Cons: complex implementation

4. **IPW (Inverse Probability Weighting) + causal graphs**: Model interference explicitly
   - Advanced technique, rarely used in practice

## Common gotchas / interview framings
- "We randomized users, so no interference." → Wrong. If treated users can message/recommend to untreated users, interference exists
- "Should we always use cluster randomization to be safe?" → No, cluster level might be too coarse. Better: diagnose interference first (spillover analysis), then decide
- "Switchback experiments avoid user-level interference?" → Yes, but introduce temporal confounds (Monday users ≠ Friday users). Use within-day switchbacks or carefully control day-of-week
- "How do we measure interference effect size?" → Compare control users near treated vs far treated. Difference ≈ spillover effect

## See also
- [[randomization-and-control-groups]]
- [[randomization-units-user-session-account]]

## Sources
See frontmatter `sources:`.

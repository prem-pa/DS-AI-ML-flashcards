---
id: e00c6d28-ddad-452e-b262-9c40e5ebb099
title: Treatment and control selection
track: data-scientist
topic: ab-testing-experimentation
difficulty: 3
tags:
- experiment-design
- treatment-arm
- control-arm
- placebo
- business-as-usual
aliases:
- arm selection
- control definition
- treatment vs control
sources:
- url: https://netflixtechblog.com/a-b-testing-and-beyond-improving-the-netflix-streaming-experience-with-experimentation-and-data-5b0ae9295bdf
  label: 'Netflix: A/B Testing Best Practices'
- url: https://www.abtasty.com/blog/sample-size-calculation/
  label: 'AB Tasty: Sample Size Calculation Guide'
cards:
- id: cd05b30b-8530-4640-a1f6-37fb8c385025
  type: flip
  front: ''
  back: ''
- id: e6da8d2e-78dd-4a0e-9810-35d65791a5bf
  type: flip
  front: ''
  back: ''
- id: c51750ec-93dc-46fa-83eb-d53ce5d0751b
  type: mcq
  front: ''
  back: ''
  choices:
  - key: a
    text: Selection bias (control users are less engaged)
    correct: false
  - key: b
    text: Novelty effect (treatment users are excited)
    correct: false
  - key: c
    text: Expectancy effect (control users are frustrated)
    correct: false
  - key: d
    text: Network interference (control affects treatment)
    correct: false
- id: f96f776f-32ca-4961-9705-040902b1e750
  type: flip
  front: ''
  back: ''
---

## Intuition

The control arm defines what you're measuring change *against*. A poorly chosen control makes your experiment useless: a bad control-treatment comparison doesn't isolate the feature's true effect.

## Detail

**Business-as-usual (BAU) control**: Users see the current experience. Most common online because:
- Directly measurable: no ambiguity about what users see
- Relevant to business: we care about vs. today, not vs. nothing
- No novelty bias: control isn't new, so no placebo effect

**Placebo control**: Users see a fake treatment (e.g., button relocation that does nothing). Rare online but useful for:
- Isolating psychological effects (e.g., "did the red button *really* work, or did users expect it to?")
- Medicine and psychology (expected effect vs. true pharmacology)

**No-exposure control**: Users see nothing (e.g., no notification). Risky because:
- Feels bad to users (they're denied)
- Unfair comparison if baseline is a notification

**Best practice**: Choose BAU. Be explicit about what "control" sees. If users are skeptical of your idea, use placebo to rule out expectancy.

## Common gotchas / interview framings
- "Should we use a smaller control (10%) to save traffic?" → No, fixed 50-50 is standard; use holdout rates for ramp-ups, not for power
- "Can we use historical data as control?" → Risky: temporal confounds, seasonality, user drift. Better: concurrent randomization
- "What if control is the 'old' product version?" → Works if product is stable. Problem: old code may have bugs, introducing unfair variance
- "Placebo: isn't that unethical?" → Ethical if duration is short and effect is known. Disclose to stakeholders.

## See also
- [[randomization-and-control-groups]]
- [[novelty-effects-and-long-term-impact]]
- [[instrumentation-and-logging]]

## Sources
See frontmatter `sources:`.

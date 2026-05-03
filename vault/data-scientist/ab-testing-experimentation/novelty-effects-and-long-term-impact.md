---
id: 2093362b-482e-4c21-becc-4adc484e6034
title: Novelty effects and long-term impact
track: data-scientist
topic: ab-testing-experimentation
difficulty: 3
tags:
- habituation
- novelty-bias
- long-term-effects
- sustained-change
- withdrawal-effects
aliases:
- novelty effect
- Hawthorne effect
- habituation
- long-term impact
sources:
- url: https://netflixtechblog.com/a-b-testing-and-beyond-improving-the-netflix-streaming-experience-with-experimentation-and-data-5b0ae9295bdf
  label: 'Netflix: A/B Testing Long-Term Effects'
- url: https://research.netflix.com/research-area/experimentation-and-causal-inference
  label: 'Netflix: Experimentation Research'
cards:
- id: 9e2fe9e4-ba60-4701-9c76-7545a6924d08
  type: flip
  front: ''
  back: ''
- id: a60d1580-d3c4-4843-a0fa-66bc053456e4
  type: flip
  front: ''
  back: ''
- id: 11bc203f-ec0f-4a73-9316-5f4a131e4ba6
  type: mcq
  front: ''
  back: ''
  choices:
  - key: a
    text: 'Novelty effect: real week-2+ effect might be lower'
    correct: false
  - key: b
    text: 'Confounding: 1 week might have seasonal anomaly'
    correct: false
  - key: c
    text: 'Measurement lag: 7-day metric is delayed'
    correct: false
  - key: d
    text: All of the above
    correct: false
- id: 292893ac-d387-4a2f-8bf6-02dafadab899
  type: flip
  front: ''
  back: ''
---

## Intuition

Users may respond to a new feature with initial excitement (novelty effect) that fades over time (habituation). Or they may get worse over time (withdrawal effect if old experience is removed).

## Detail

**Novelty effect**:
- Users are excited by new feature → short-term boost in engagement/satisfaction
- Effect fades over 2-4 weeks as they get used to it
- Example: new feed design, +10% engagement first week, +3% week 4
- Risk: launch based on week 1, but week 4 shows it's not actually good

**Withdrawal effect**:
- If old feature is removed (not just hidden), users who relied on it may get worse outcomes
- Example: removing an easy-access feature → users take longer to find it → engagement drops
- Opposite of novelty: starts neutral, gets worse

**Habituation**:
- Users adapt to change; impact diminishes
- E.g., "recommended for you" widget: first day +5% CTR, day 7 +2%, day 14 +1% (adaptation)

**Long-term impact**:
- True effect may only emerge after weeks/months
- Example: onboarding change helps retention, but 7-day cohort looks same (30-day cohort shows +3%)

**Detection**:
1. **Test duration**: Run for 4+ weeks to capture habituation (avoid 1-week tests)
2. **Holdout cohort**: Keep some users on old experience long-term, compare to updated users
3. **Cohort analysis**: Split results by week (week 1 vs week 4) to see if effect fades

**Mitigation**:
- Gradual rollout: ramp up new feature over 4 weeks (lets users adapt, less shock)
- Compare new-vs-new: if removing old feature, compare new to "improved old" not "no feature"
- Measure long-term metrics: retention, churn, LTV instead of just engagement

## Common gotchas / interview framings
- "Feature shows +5% lift week 1, we should launch now." → Wait 4 weeks to see if novelty fades. Habituation might reduce week-4 lift to +1%
- "Holdout users on old version—isn't that unfair?" → For short tests (1-2 weeks), yes, holdout is stale. For long-term, needed to measure true causal effect (vs novelty)
- "How do we distinguish novelty from true preference?" → Cohort analysis: if all cohorts fading similarly, novelty effect; if effect stable, true preference
- "What if users never adapt (novelty = real improvement)?" → Then 4-week test shows stable lift. Launch confidently

## See also
- [[duration-and-seasonal-bias]]
- [[survivorship-bias]]

## Sources
See frontmatter `sources:`.

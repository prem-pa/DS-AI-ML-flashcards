---
id: 7f590458-b3ef-4f0e-b0ce-5d34d6f32f44
title: Instrumentation and logging
track: data-scientist
topic: ab-testing-experimentation
difficulty: 3
tags:
- data-quality
- metrics
- logging
- event-tracking
- observability
aliases:
- event logging
- tracking
- instrumentation
- data collection
sources:
- url: https://netflixtechblog.com/a-b-testing-and-beyond-improving-the-netflix-streaming-experience-with-experimentation-and-data-5b0ae9295bdf
  label: 'Netflix: A/B Testing Infrastructure'
- url: https://research.netflix.com/research-area/experimentation-and-causal-inference
  label: 'Netflix: Experimentation Research'
cards:
- id: a787aafe-a5a8-40a2-b2a7-6147ee3f91fe
  type: flip
  front: ''
  back: ''
- id: dbaff489-c5b8-4e1f-9bbe-0ef9acb5eb1f
  type: flip
  front: ''
  back: ''
- id: 8280c33e-00c0-42df-a467-0ecbe79e2140
  type: mcq
  front: ''
  back: ''
  choices:
  - key: a
    text: Client-side logging is unreliable
    correct: false
  - key: b
    text: Button clicks are sampled; adds are 100%
    correct: false
  - key: c
    text: Clicks include failed adds (network error)
    correct: false
  - key: d
    text: Events are deduplicated incorrectly
    correct: false
- id: 062f0623-affa-4670-961e-89fd3b7a0178
  type: flip
  front: ''
  back: ''
---

## Intuition

Garbage in, garbage out. If logging is buggy, metrics are wrong, and your experiment conclusions are meaningless.

## Detail

**Core instrumentation questions**:
1. **Is the event fired?** Does the code trigger an event when the action happens?
2. **Is it tagged correctly?** Does the event include user ID, treatment arm, timestamp?
3. **Is timing right?** Is the event logged *when* the action happens (not hours later)?
4. **Is it complete?** Do all users/sessions send events, or do some drop?
5. **Is it validated?** Are counts reasonable? (e.g., if you expect 1M clicks/day, do you see 1M?)

**Common issues**:
- **Missing user ID**: Can't link events to users/arms
- **Client-side logging**: Events can fail (blocked by adblocker, network latency)
- **Backfill delays**: Events logged 1 day late → metric looks lower on day 1
- **Sample-based logging**: Only log 1% of events → noisy metrics
- **Off-by-one errors**: "Clicks" includes click+drag+release (3 events, not 1)

**Best practice**:
- Log server-side (more reliable than client-side)
- Log immediately (sub-second latency)
- Include user_id, experiment_id, treatment_arm, timestamp, event_value
- Validate counts daily (sanity checks)
- Test logging before experiment (run 1% test, verify event counts match expected)

## Common gotchas / interview framings
- "We see a 10% lift in 'clicks,' but all other metrics are flat. Is logging broken?" → Investigate: are clicks logged differently? Are they duplicated? Did treatment change click firing?
- "Events are delayed by 1 hour. Does that bias results?" → If delay is the same for treatment and control, no bias. But it delays decision-making
- "We use client-side logging. Do we need to worry?" → Yes, if clients have adblockers, slow connections, or crash. Better: server-side. But can supplement with client-side for UX metrics
- "Should we sample events to save costs?" → Only if uniform sampling (e.g., 1% of all events). Biased sampling (sample only certain users) breaks experiments

## See also
- [[primary-vs-secondary-metrics]]
- [[guardrail-metrics]]
- [[metric-construction-ratios-counts-timing]]

## Sources
See frontmatter `sources:`.

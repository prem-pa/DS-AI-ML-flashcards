---
id: 6347cc60-38a9-478b-a546-471d5d53f369
title: Latency, throughput, error rates
track: ml-engineer
topic: model-monitoring-mlops
difficulty: 1
tags:
- slos
- observability
- production-metrics
- alerting
- perf-monitoring
- baselines
aliases:
- SLO monitoring
- operational metrics
- service metrics
sources:
- url: https://www.peerspot.com/categories/model-monitoring
  label: 'PeerSpot: Best Model Monitoring solutions 2026'
cards:
- id: ee96cb75-1507-4f4a-b5f5-fa9bb9c9d0ad
  type: flip
  front: What are the three core operational metrics you must monitor in an ML system, and why does p99
    latency matter more than mean latency?
  back: '1. **Latency** (p50/p95/p99 ms): prediction response time

    2. **Throughput** (req/sec): request processing capacity

    3. **Error rate** (%): failed predictions/service crashes


    p99 latency captures tail behavior—users in 1% slowest experience > 200ms wait. Mean (50ms) hides
    this. p99 degradation signals resource contention or feature store slowness before accuracy degrades.'
- id: 01648694-01e1-4775-b73b-0d49260efb6d
  type: mcq
  front: Your model's p99 latency jumps from 80ms → 250ms. Accuracy on recent test set is unchanged at
    0.92. What is the most likely root cause?
  back: '**Correct: b** Latency ≠ accuracy. A 3x latency spike with stable accuracy suggests operational
    bottleneck: feature store query slowdown, serialization overhead, or resource contention. Model accuracy
    is unchanged. (a) concept drift would eventually degrade accuracy. (c) & (d) affect accuracy, not
    latency.'
  choices:
  - key: a
    text: Model has learned concept drift
    correct: false
  - key: b
    text: Feature store is slow or network serialization bottleneck
    correct: true
  - key: c
    text: Input feature distribution shifted
    correct: false
  - key: d
    text: Label distribution changed
    correct: false
- id: afb1c7f4-2040-4534-8b94-b2f0377477eb
  type: flip
  front: Define an appropriate SLO for p99 latency and uptime. What happens when you violate the latency
    SLO for 5 minutes?
  back: 'Example SLO: **p99 latency < 100ms, uptime > 99.9% (43 min downtime allowed/month)**.


    Violation for 5 min: Trigger **page on-call engineer immediately**. Within 15 min, investigate: (1)
    Feature store latency spike? (2) Model inference complexity increased? (3) Increased request volume?
    Scale horizontally or revert model if necessary. Log incident for postmortem.'
- id: ff713f6c-32b4-49d8-8e22-8486cba6a10d
  type: mcq
  front: Which metric composition issue would NOT be caught by latency/throughput/error-rate monitoring
    alone?
  back: '**Correct: a** Operational metrics (latency, throughput, errors) do NOT capture statistical performance
    degradation. Accuracy ↓ 0.95 → 0.87 is invisible to these signals unless paired with held-out validation
    monitoring or label-delayed ground truth. This is why data/model quality monitoring is separate.'
  choices:
  - key: a
    text: Model returns predictions but accuracy dropped from 0.95 → 0.87
    correct: false
  - key: b
    text: All requests timeout due to feature store outage
    correct: false
  - key: c
    text: Predictions arrive 10x slower than baseline
    correct: false
  - key: d
    text: Throughput cut in half
    correct: false
---

## Intuition
ML systems operate in production environments with strict service-level objectives (SLOs). Beyond accuracy, you must monitor operational metrics: how fast predictions return (latency), how many you process per unit time (throughput), and failure rates. These form the baseline for detecting when something breaks operationally—not just statistically.

## Detail
Latency (p50, p95, p99 ms) measures prediction speed; throughput (requests/sec) measures capacity. Error rates track failed predictions, timeouts, or crashed services. Set SLOs: e.g., p99 latency < 100ms, uptime > 99.9%. Use histograms and counters (Prometheus, StatsD) to emit these. Alert when p99 latency creeps from 50ms → 200ms even if accuracy holds: this signals resource contention or model complexity drift. Throughput drop may indicate data pipeline issues upstream.

## Common gotchas / interview framings
- Confusing latency SLO with model inference time alone—network, serialization, feature store lookups matter
- Not setting percentile-based alerts (p99 vs mean); mean can hide tail latency issues
- Alerting on raw error rate without context: 0.1% errors on 1M req/day = 1000 failures
- Not separating input validation errors from model inference errors

## See also
- [[prediction-drift-and-label-shift]]
- [[resource-utilization-gpu-memory-cpu-io]]
- [[schema-drift-and-pipeline-monitoring]]
- [[experiment-tracking-and-model-registry]]

## Sources
See frontmatter `sources:`.

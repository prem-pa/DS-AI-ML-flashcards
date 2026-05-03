---
id: 3dc9ee4f-feb2-42aa-9f2a-16f61efbb939
title: Feature freshness and staleness SLOs
track: ml-engineer
topic: feature-stores-data-management
difficulty: 5
tags:
- slo
- monitoring
- freshness
- staleness
- latency-budget
- cost-optimization
aliases:
- feature-recency
- update-frequency
- staleness-threshold
sources:
- url: https://www.hopsworks.ai/
  label: Hopsworks - The AI Lakehouse for Real-Time ML
- url: https://taylor-amarel.com/2025/04/comprehensive-comparison-feast-vs-tecton-vs-hopsworks-for-cloud-based-feature-stores-2024/
  label: Comprehensive Comparison of Feature Stores 2025
cards:
- id: a7f3fb1e-2934-4ea9-9f30-64886c560208
  type: flip
  front: What is the relationship between feature freshness and model performance?
  back: Features that are too stale (beyond their SLO) may reflect outdated user behavior, leading to
    worse predictions. For example, a user's purchase intent may change within hours; using a 24-hour-old
    feature is problematic for recommendation models.
- id: 517dc1dd-2baa-4fee-9868-392996a82aaa
  type: mcq
  front: You need fraud detection features to have < 1 minute staleness. What is the most cost-effective
    approach?
  back: Batch jobs cannot achieve sub-minute freshness. Streaming pipelines (Kafka, Flink, Spark Streaming)
    process data as it arrives, pushing updates to the online store within seconds, which is necessary
    for fraud detection.
  choices:
  - key: a
    text: Daily batch job
    correct: false
  - key: b
    text: Hourly batch job
    correct: false
  - key: c
    text: Streaming pipeline with sub-minute SLO and online store updates
    correct: true
  - key: d
    text: No feature store; compute features inline during serving
    correct: false
- id: bd80b75a-1e75-49fe-9f39-f18ba702f2b9
  type: flip
  front: How would you balance the cost of real-time feature updates against a high staleness SLO?
  back: 'Stratify features by use case: critical features (fraud, safety) → streaming/real-time; moderate
    features (ranking) → hourly/5-minute batch; non-urgent features (reporting) → daily batch. This optimizes
    cost by only paying for real-time where needed.'
- id: 96f745c3-5ac4-4796-9ce7-847d27776994
  type: mcq
  front: What does it mean if a feature computation completes but the online store is not updated within
    SLO?
  back: 'Feature freshness is end-to-end: computation + push to online store. If computation finishes
    in 10 seconds but the online store update takes 5 minutes, the SLO is violated even though computation
    was fast. You need to monitor each stage separately.'
  choices:
  - key: a
    text: The computation was too slow
    correct: false
  - key: b
    text: The online store push/sync mechanism is the bottleneck, not the computation itself
    correct: true
  - key: c
    text: The feature definition is incorrect
    correct: false
  - key: d
    text: The offline store is not synchronized
    correct: false
---

## Intuition
Feature freshness = how recent the feature value is. A staleness SLO sets a maximum acceptable age for features (e.g., "purchase count must be < 5 minutes old"). Different features need different freshness: real-time fraud scores need sub-minute updates; monthly user cohorts can be a week old.

## Detail
**Freshness SLO Definition**:
- Acceptable staleness: e.g., 5 minutes, 1 hour, 24 hours
- How often to update: batch (hourly, daily), streaming (milliseconds to seconds)
- Monitoring: Track actual staleness against SLO with alerts

**Cost vs Freshness Trade-off**:
- Streaming/real-time updates: Near-zero latency but 10-100x cost
- Hourly batch: Cheap but 1-hour staleness
- Daily batch: Very cheap but 24-hour staleness

**Use-case-driven SLOs**:
- **Fraud detection**: Sub-minute (streaming needed)
- **Recommendation ranking**: 5-30 minutes (batch with low interval)
- **Churn prediction**: Hourly to daily (batch)
- **Ad targeting**: Minutes to hours (hybrid batch + streaming)

**Monitoring & Alerts**:
- Track end-to-end latency: data source → feature computation → online store availability
- Alert if features exceed SLO staleness
- Distinguish between "data available but not computed" vs "computation delayed" vs "online store not updated"

## Common gotchas / interview framings
- **SLO vs SLA**: SLO is a target; SLA is a contractual commitment with penalties
- **Hidden costs of real-time**: Streaming infrastructure, windowing complexity, state management
- **Cascading delays**: If upstream feature is late, all downstream features miss SLO
- **Peak vs off-peak**: May need different batch frequencies during high/low load
- **Timeliness vs cost**: Interview question: "How would you justify hourly batch if stakeholders want 5-minute freshness?"

## See also
- [[online-vs-offline-feature-stores]]
- [[feature-definitions-and-computation]]
- [[etl-vs-elt]]
- [[incremental-and-full-refresh-logic]]

## Sources
See frontmatter `sources:`.

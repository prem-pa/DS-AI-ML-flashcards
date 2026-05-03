---
id: 311a25b7-d479-4c74-b0c0-74334e965414
title: Data freshness
track: ml-engineer
topic: model-monitoring-mlops
difficulty: 1
tags:
- pipeline-monitoring
- data-quality
- staleness-detection
- timeliness
- sla-monitoring
- observability
aliases:
- data staleness
- pipeline timeliness
- recency monitoring
sources:
- url: https://www.peerspot.com/categories/model-monitoring
  label: 'PeerSpot: Best Model Monitoring solutions 2026'
cards:
- id: a04fc15f-09ef-471a-bece-9aeb235a0885
  type: flip
  front: Your feature store has max(record_timestamp) = 3 hours ago. SLA is 30 minutes fresh. How critical
    is this and what are your first steps?
  back: '**Critical—immediate response needed**. Data is 6x older than SLA.


    First steps:

    1. **Verify it''s not monitoring artifact**: Confirm max timestamp manually; timezone mismatch, clock
    skew can cause false alerts.

    2. **Check pipeline status**: Is the ETL running? Login to orchestrator (Airflow, Dagster) or query
    logs. Is upstream data available (source DB accessible)?

    3. **Check for hung processes**: SQL query running 2 hours? Long-running transform? Kill and restart
    if needed.

    4. **Assess impact**: Does model serving depend on this feature? If yes, either (a) serve cached predictions
    (last known good), (b) wait for pipeline to catch up (< 30min more?), or (c) disable feature and retrain
    lightweight model.

    5. **RCA**: What broke? DB credentials rotated? Quota exceeded? Dependency (upstream dataset) missing?

    6. **Communicate**: Notify downstream teams; set ETA for fix.'
- id: 69cf39f5-c31a-41e1-814e-507edd116660
  type: mcq
  front: 'Your ETL pipeline runs daily at midnight. You check feature freshness at 10am: max(record_timestamp)=yesterday
    midnight (12 hours old). Is this a problem?'
  back: '**Correct: c** The SLA determines if staleness is acceptable. Daily batch at midnight → data
    is 12h old by noon. If SLA is 6h max age (hourly ETL expected), this is stale and problematic. If
    SLA is 24h (once-daily feature refresh acceptable), this is on-time. Set SLA based on business need:
    real-time recommendation (SLA: 1h), batch scoring (SLA: 24h), historical analysis (SLA: 1 week).'
  choices:
  - key: a
    text: Yes, data is too stale; alert and restart pipeline
    correct: false
  - key: b
    text: No, this is expected; 12 hours old is normal for daily batch
    correct: false
  - key: c
    text: Maybe, depends on SLA; if SLA < 12h old, it's stale
    correct: true
  - key: d
    text: Data is too old; should run hourly instead of daily
    correct: false
- id: 5385d7e5-d6d8-4875-aed0-2208a21b1082
  type: flip
  front: Describe how to implement SLA monitoring for data freshness across 50 feature tables. What triggers
    escalation?
  back: "**Multi-table freshness monitoring**:\n\n**Setup**:\n1. **Define SLA per table**: Some features\
    \ need hourly refresh (user activity), others daily (user demographics). Document in config.\n2. **Query\
    \ max(timestamp) per table**: Run daily query checking max(record_timestamp) vs SLA threshold. Store\
    \ results in monitoring table.\n3. **Alert rules**:\n   - Table **violates SLA** (age > threshold):\
    \ Warning, investigate root cause\n   - Table **no data for 2x SLA**: Critical, escalate; pipeline\
    \ likely broken\n   - Table **row count flat** for 24h: Warning; possible stall\n   - Multiple tables\
    \ stalled simultaneously: Likely shared upstream dependency failed\n\n**Escalation**:\n- **Warning**\
    \ (age > SLA, but < 2x SLA): Check pipeline logs; no page\n- **Critical** (age > 2x SLA): Page on-call\
    \ data engineer; assess model serving impact\n- **Mass failure** (>10 tables broken): Assume database/data\
    \ warehouse issue; page on-call infrastructure\n\n**Tools**: dbt tests, Great Expectations, custom\
    \ SQL + Prometheus, Datadog monitors."
- id: e258e0e8-8e24-45e1-8ccb-9b9a6fb87afd
  type: flip
  front: A feature table is normally updated at 10am daily. A model serving request arrives at 9:55am.
    Should you hold the request until fresh data arrives in 5 minutes?
  back: '**Context-dependent decision**:


    **Hold request (wait 5 min)**:

    - Feature is highly predictive (model accuracy degrades significantly with stale data)

    - 5 minutes is acceptable latency for use case (e.g., batch scoring)

    - SLA allows: customer willing to wait


    **Serve with stale data**:

    - Feature is low-impact (predictions stable with 1-day-old feature)

    - Real-time SLA < 5 min (e.g., API response < 100ms)

    - Cache hits: serve from cached prediction (computed at 10am yesterday)


    **Best practice**: **Serve with stale + monitor**. Use yesterday''s 10am features (24h stale, but
    consistent quality). Log staleness metric; if staleness > threshold, serve with lower confidence or
    flag for manual review. Communicate staleness to user (e.g., "prediction based on data from 1h ago").'
---

## Intuition
ML models depend on fresh feature data: yesterday's customer behavior for today's predictions. If the data pipeline stalls (ETL timeout, database down, upstream dependency failed), models get stale features, producing outdated predictions. Monitor data freshness: how recent is the newest record in your feature store? Alert if data age > SLA (e.g., > 1 hour old).

## Detail
Freshness metrics: (1) **Max record timestamp**: Latest timestamp in feature table. SLA: now() - max(record_timestamp) < 1 hour. (2) **Expected refresh rate**: If ETL runs every 30min, alert if no new records in 1 hour (2 refreshes missed). (3) **Row count delta**: Compare to yesterday's row count; flat count suggests pipeline stalled. (4) **Pipeline stage latency**: Track time through each ETL stage (extract, transform, load); identify slowest stage. Tools: data quality frameworks (Great Expectations, dbt tests), warehouse monitoring (Datadog, alerts on query run time).

## Common gotchas / interview framings
- Confusing data freshness with model freshness (model deployed yesterday but features today are fresh)
- Not accounting for legitimate delays (nightly batch ETL completing at 2am; checking at 1am returns stale)
- Alerting too aggressively; brief network hiccups cause spikes
- Ignoring backfill scenarios: historical data may be legitimately older, don't confuse with fresh data staleness

## See also
- [[latency-throughput-error-rates]]
- [[missing-value-and-anomaly-monitoring]]
- [[schema-drift-and-pipeline-monitoring]]
- [[feature-distribution-monitoring]]

## Sources
See frontmatter `sources:`.

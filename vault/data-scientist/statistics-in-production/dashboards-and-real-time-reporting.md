---
id: 3a39ed0b-6bc3-4f96-b24c-13bcf9169182
title: Dashboards and real-time reporting
track: data-scientist
topic: statistics-in-production
difficulty: 1
tags:
- visualization
- dashboards
- real-time
- monitoring
- BI-tools
aliases:
- real-time metrics
- observability dashboards
- metric visualization
- live reporting
sources:
- url: https://netflixtechblog.com/lessons-from-building-observability-tools-at-netflix-7cfafed6ab17
  label: 'Netflix: Lessons from Building Observability Tools'
- url: https://www.thedataletter.com/p/how-netflix-does-data-reliability
  label: How Netflix Does Data Reliability
- url: https://www.bigeye.com/blog/data-in-practice-anomaly-detection-for-data-quality-at-netflix
  label: 'Bigeye: Data Quality Monitoring Dashboards'
- url: https://www.sigmacomputing.com/blog/automate-data-validation
  label: Automated Data Validation and Dashboards
cards:
- id: e8842e44-981e-43d4-ba19-552185e5ccc5
  type: flip
  front: Your model-monitoring dashboard shows accuracy is 91%, within the normal range of 88-93%, but
    the on-call engineer is flagging it as concerning. What context is missing from the dashboard?
  back: 'The dashboard shows aggregate accuracy but lacks: (1) trend direction—is it drifting downward?
    (2) deseasonalized residual—is 91% anomalous for this day-of-week? (3) segment breakdowns—is accuracy
    91% overall but 75% in a critical segment? (4) correlated metrics—did accuracy drop correlate with
    a feature deployment or data source change? Add these layers: a sparkline showing trend, a comparison
    to historical baseline for this day-of-week, segment tables, and recent event annotations.'
- id: ae3eb578-9836-49e2-bb11-c5b24bc20aae
  type: mcq
  front: You're designing a real-time dashboard for a recommendation system serving 10M daily users. Metric
    updates must be < 10 seconds. Which architecture is most suitable?
  back: Streaming architecture (b) delivers < 10s latency. Batch every hour (a) is too slow. Direct production
    database queries (c) risk OLTP load; 5-minute polling misses 10s SLA. Offline daily (d) is unacceptable
    for real-time monitoring.
  choices:
  - key: a
    text: Batch job every hour; publish to dashboard.
    correct: false
  - key: b
    text: Kafka/Kinesis streaming pipeline → aggregation window (10s) → publish to ClickHouse/TimescaleDB
      → dashboard.
    correct: false
  - key: c
    text: SQL queries directly on production database; dashboard polls every 5 minutes.
    correct: false
  - key: d
    text: Offline batch model; update once per day.
    correct: false
- id: a16cab8d-fa4b-4793-af28-31d6c73718db
  type: flip
  front: Your dashboard shows overall CTR is normal (4.2%), but 30% of traffic comes from a new user segment
    with 2% CTR. Why is this important, and how should the dashboard expose it?
  back: 'This is a segment-specific anomaly hidden by aggregate metrics. The dashboard should include:
    (1) a segment breakdown table showing CTR by cohort/geography/device, highlighting outliers; (2) a
    cohort trend chart comparing new vs. returning users; (3) a heatmap of CTR by segment and time to
    detect if the anomaly is recent. This helps on-call teams identify if the model is failing in a specific
    segment (e.g., new recommendation algorithm doesn''t generalize to new users).'
- id: 8fbb80a7-2751-4cbc-9115-126ff2eb3839
  type: mcq
  front: Your dashboard refresh lag is 30 minutes (batch SQL query runs hourly). An anomaly occurs at
    2:00 PM but isn't visible until 3:00 PM. How do you reduce detection latency?
  back: Streaming (b) reduces dashboard latency to 10-30s. Alerts (c) notify teams immediately, decoupled
    from dashboard updates. Combined, you get fast alerts (no blind spots) + detailed dashboards (root-cause
    analysis). (a) is still too slow for critical issues.
  choices:
  - key: a
    text: Increase batch frequency to every 5 minutes.
    correct: false
  - key: b
    text: Replace batch with event streaming (Kafka) for critical metrics; refresh every 10-30 seconds.
    correct: false
  - key: c
    text: Add separate real-time alerts that notify teams before dashboard updates.
    correct: false
  - key: d
    text: Use b and c together; streaming for real-time alerts + batch for detailed dashboards.
    correct: false
---

## Intuition

Dashboards translate raw metrics into actionable insights. A well-designed dashboard surfaces anomalies, explains context (seasonal patterns, recent changes), and enables rapid decision-making. Poor dashboards hide problems or overwhelm with irrelevant detail, leading to slow detection and wasted alerting.

## Detail

**Dashboard Design Principles for Production**:

**Hierarchy of Metrics**: Separate layers—executive (overall system health), operational (model-specific metrics), technical (data quality, latency). Users drill down as needed.

**Deseasonalized Context**: Display raw metrics alongside deseasonalized residuals. This clarifies whether a dip is seasonal (expected) or anomalous (needs investigation).

**Historical Baselines**: Show 7-day, 30-day, 1-year bands for context. A 10% dip looks minor next to a 30% historical variance but alarming if variance is typically 2%.

**Anomaly Highlighting**: Use color coding (red for statistical anomalies, yellow for warnings) and annotations (why did accuracy drop? correlation with deployment, data source change).

**Multi-metric Correlation**: Show related metrics on the same plot—model accuracy, data quality score, latency. Helps identify root causes (latency spike → accuracy drop = inference timeout, not model failure).

**Real-time vs. Batch**: Real-time dashboards update second-by-second (via Kafka, event streams). Batch dashboards refresh hourly (SQL, Spark jobs). Choose based on decision speed needed.

**Drill-down Capability**: Click a metric to see segment breakdowns (accuracy by geography, user cohort, feature version). Identifies if anomalies are global or localized.

## Common gotchas / interview framings

- **Vanity metrics**: Dashboards showing only aggregate metrics hide segment-level degradation. A model's overall accuracy may stay at 90%, but one segment (new users) drops to 60%.
- **Stale dashboards**: If refresh lag is 1 hour, alerts are outdated. Prioritize real-time data pipelines for critical metrics.
- **Alert fatigue on dashboards**: Too many red indicators (all high-variance metrics, no deseasonalization) cause teams to ignore dashboards. Use statistical significance, not raw thresholds.
- **Lack of drill-down**: A user sees overall accuracy is 90% on a dashboard but can't see it's 85% in segment X. Without drill-down, root-cause analysis is impossible.

## See also
- [[guardrail-metrics-and-alert-thresholds]]
- [[anomaly-detection]]
- [[seasonality-and-deseasonalization]]
- [[data-validation-and-schema-checks]]
- [[missing-data-and-imputation]]

## Sources
See frontmatter `sources:`.

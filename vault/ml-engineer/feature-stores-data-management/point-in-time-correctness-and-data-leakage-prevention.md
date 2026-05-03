---
id: 2a06407d-4f38-4897-8ad8-0a6f50d40312
title: Point-in-time correctness and data leakage prevention
track: ml-engineer
topic: feature-stores-data-management
difficulty: 5
tags:
- data-leakage
- temporal-correctness
- training-data
- integrity
- causality
- time-series
aliases:
- temporal-alignment
- feature-leakage
- look-ahead-bias
sources:
- url: https://apxml.com/courses/feature-stores-for-ml/chapter-3-data-consistency-quality/point-in-time-correctness
  label: Point-in-Time Correctness for Training Data
- url: https://medium.com/@scoopnisker/solving-the-training-serving-skew-problem-with-feast-feature-store-3719b47e23a2
  label: Solving Training-Serving Skew with Feast
cards:
- id: ed3ba7e6-8f6a-4c3a-9df4-e4bbcc5a08bf
  type: flip
  front: What is data leakage in the context of feature stores?
  back: 'Using features that were not available at the time a label was recorded. Example: predicting
    churn on Day 10 using a feature computed from data on Day 15. The model trains on ''magical'' features
    unavailable at serving time, resulting in overoptimistic performance.'
- id: c2af396e-b00c-4e5a-9984-a369d47d4779
  type: mcq
  front: How do you implement point-in-time correctness when building a training dataset?
  back: 'Point-in-time join: for each label (user, churn_date), fetch the feature values where the feature
    timestamp is <= churn_date. This ensures the model only sees information that was available when the
    label was recorded.'
  choices:
  - key: a
    text: Use the most recent feature values available
    correct: false
  - key: b
    text: For each label with timestamp T, join with features as-of timestamp T (most recent feature before
      T)
    correct: true
  - key: c
    text: Average all historical feature values
    correct: false
  - key: d
    text: Use feature values from the same day as the label
    correct: false
- id: c36152de-39ac-4803-9439-771b37662a8a
  type: flip
  front: Why is versioning of feature definitions critical for point-in-time correctness?
  back: Feature definitions evolve over time. If you change a feature definition on Day 20, but trained
    a model with data from Day 1-10, you need to store the old definition so you can reproduce the training
    dataset with the same feature logic. Without versioning, reproducibility is lost.
- id: f735fb64-36ab-406f-a068-fbb36c659d3c
  type: mcq
  front: You receive a log event with timestamp 2024-01-05, but it arrives in your system on 2024-01-08.
    A label is dated 2024-01-06. Should this event be included in features for that label?
  back: 'Late-arriving data is a trade-off: you can include it (richer features, but risk of non-reproducibility)
    or exclude it (safe, but features may be incomplete). Document your policy clearly and validate that
    serving also follows it.'
  choices:
  - key: a
    text: Yes, because the event timestamp is before the label
    correct: false
  - key: b
    text: No, because the event arrived after the label was recorded
    correct: false
  - key: c
    text: Depends on your late-arrival policy and SLO; if you accept it, validate end-to-end consistency
    correct: true
  - key: d
    text: Always exclude late-arriving data
    correct: false
---

## Intuition
Point-in-time correctness ensures that training features reflect only data that was available at the moment the label was recorded. If a label says "user churned on Jan 1", features must use only data from before Jan 1, not after. Violating this causes data leakage and overoptimistic model performance.

## Detail
**The Problem: Data Leakage**:
- Label: User_123 churned on 2024-01-01
- Feature: User_123 last_purchase_date = 2024-01-02 (computed from all data)
- Issue: This feature was not available when the label was recorded; it's future information
- Result: Model trains on "magical" features that predict the label but are unavailable at serving time

**Point-in-Time Join**:
Create a training dataset by joining labels with features as-of a specific timestamp:
```
Label table: (user_id, churn_date, label)
Feature table: (user_id, feature_date, feature_value)

Join: For each (user_id, churn_date), fetch feature_value where feature_date <= churn_date
Take the most recent feature_value before the churn_date
```

**Implementation**:
- Time-travel support: Offline store must store historical snapshots of features
- Versioning: Features change over time; must keep versions for reproducibility
- Latency handling: If data arrives late, decide whether to include it (risk: future data) or exclude it (risk: missing real data)

**Example with Booking Cancellations**:
- Booking made on Day 5
- User reviews written on Day 6, 7, 8
- Booking cancelled on Day 10
- For label (cancelled, Day 10): Include reviews from Day 5-9 only
- During serving (predicting if a Day 5 booking will cancel): Only Day 1-5 data is available

## Common gotchas / interview framings
- **Late-arriving data**: Data that arrives after label but references a time before label (e.g., delayed log). Risk: do you include it?
- **Timestamp precision**: Unix timestamp vs date—millisecond differences can leak future information
- **Feature definition changes**: If feature definition changed between training and serving, point-in-time correctness alone doesn't prevent skew
- **Slowly changing dimensions (SCD)**: When a user's dimension (e.g., country) changes, which version do you use for point-in-time?
- **Interview question**: "How do you build a churn prediction model without leaking future user behavior?"
- **Reproducibility**: Without versioning, even if you got it right once, you can't reproduce the training dataset later

## See also
- [[online-vs-offline-feature-stores]]
- [[feature-freshness-and-staleness-slos]]
- [[data-warehouse-architecture-star-schema-factdimension-tables]]
- [[feature-definitions-and-computation]]

## Sources
See frontmatter `sources:`.

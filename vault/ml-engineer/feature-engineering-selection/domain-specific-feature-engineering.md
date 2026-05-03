---
id: 085c04a7-3036-4b03-8d12-b34a7b29dd75
title: Domain-specific feature engineering
track: ml-engineer
topic: feature-engineering-selection
difficulty: 3
tags:
- feature-creation
- domain-knowledge
- business-logic
- problem-solving
- competition
aliases:
- task-aware-features
- expert-guided-engineering
- knowledge-driven-features
sources:
- url: https://sebastianraschka.com/faq/docs/feature_sele_categories.html
  label: Sebastian Raschka - Feature Selection Categories
- url: https://www.blog.trainindata.com/feature-selection-with-wrapper-methods/
  label: Train in Data - Feature Selection Methods
- url: https://www.frontiersin.org/journals/big-data/articles/10.3389/fdata.2025.1624507/pdf
  label: Frontiers in Big Data - Feature Selection Review
cards:
- id: 20269629-38eb-437f-aacf-6c30571d2b82
  type: flip
  front: Why is domain-specific feature engineering often more effective than automatic feature learning
    for tabular data?
  back: Domain features encode causal relationships and business logic that automatic methods miss. They
    distill expert knowledge into a few high-signal variables, avoiding spurious patterns and reducing
    dimensionality. Kaggle winners report domain features as the biggest predictor lift.
- id: be8a735c-4174-4367-b10b-d3a34f19fdd4
  type: flip
  front: Give an example of a domain feature in a churn prediction task.
  back: 'Engagement decay: compare average login frequency in the last month vs. last 6 months. If the
    ratio is below 0.5, churn risk is elevated. This captures behavioral inertia—a concept raw login counts
    alone would miss.'
- id: 0ed7e813-a885-4d85-a845-f11e07cefac1
  type: mcq
  front: Which approach is most likely to uncover domain-specific features?
  back: 'Option B: direct domain knowledge is the primary source. The others are useful downstream, but
    domain features are born from understanding the problem, not the data alone.'
  choices:
  - key: a
    text: Run variance inflation factors on all raw features
    correct: false
  - key: b
    text: Conduct exploratory interviews with subject-matter experts and domain literature review
    correct: true
  - key: c
    text: Increase polynomial degree until r² plateaus
    correct: false
  - key: d
    text: Apply SHAP to identify top feature interactions
    correct: false
- id: b9cc990b-7429-4491-962c-4629bdfc19f9
  type: flip
  front: How do you validate that a domain feature is not just your hypothesis overfitting?
  back: Use held-out test set and cross-validation. Compare lift vs. simpler baseline. If the feature
    works across folds and generalizes to test, and if domain theory predicts why, it's likely genuine
    signal.
---

## Intuition

Domain-specific feature engineering means you leverage your knowledge of the problem space—what the features mean, how they relate to the target, and what domain rules apply—to manually create features that are more predictive than raw variables. A loan officer knows that debt-to-income ratio matters; a time-series analyst knows that seasonality and trend components drive outcomes; a fraud analyst knows that velocity features (how many transactions in the last hour?) matter.

## Detail

Beat automatic feature learning pipelines by encoding business logic. Examples:
- **Finance**: expense ratios, default risk metrics, cash-flow indicators
- **E-commerce**: customer lifetime value, repeat purchase rate, cart abandonment signals
- **Healthcare**: severity scores combining multiple clinical measures
- **Time-series**: seasonal indicators, holiday flags, rolling aggregates at domain-relevant windows

This differs from polynomial/interaction features (which are systematic) because it requires reading domain literature, interviewing stakeholders, and understanding the causal structure of the problem. Domain features often survive feature selection because they capture real signal; random polynomial expansions rarely do.

## Common gotchas / interview framings
- "How would you identify high-value domain features without seeing all the data?"
- "Domain features can be gold, but how do you avoid overfitting to your hypothesis?"
- "Describe a time you created a feature that beat the baseline. What domain insight drove it?"
- "What's the difference between a domain feature and feature engineering via interaction terms?"

## See also
- [[feature-engineering]]
- [[domain-knowledge]]
- [[exploratory-data-analysis]]
- [[feature-importance]]
- [[model-interpretability]]
- [[baseline-features]]

## Sources
See frontmatter `sources:`.

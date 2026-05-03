---
id: 0467ce8b-f7c1-49ae-8ed6-a260208adf46
title: Retraining triggers (periodic, drift-based, performance-based)
track: ml-engineer
topic: model-monitoring-mlops
difficulty: 3
tags:
- model-refresh
- retraining-strategy
- performance-monitoring
- drift-detection
- automation
- cost-optimization
aliases:
- retraining strategy
- model update triggers
- automated retraining
sources:
- url: https://www.peerspot.com/categories/model-monitoring
  label: 'PeerSpot: Best Model Monitoring solutions 2026'
cards:
- id: a49e4d4b-17d8-446b-9980-12ff79cf75c4
  type: flip
  front: Your model's AUC degraded from 0.94 → 0.87. Should you retrain immediately or investigate first?
    When is each appropriate?
  back: '**Investigate first (unless critical)**:


    1. **Confirm the drop is real**: Is this on held-out validation set or production? Validation set
    may be small (high variance); prod performance may differ. Get 30+ samples before concluding.


    2. **Root cause**: Is it drift (feature distribution shifted), label shift (P(Y) changed), data quality
    (nulls up), or labeling error (ground truth corrupted)? Different root causes = different remedies.


    3. **Assess impact**: Does 0.87 violate SLA? If AUC target is 0.90, yes—retrain. If target is 0.85,
    no—monitor.


    **Retrain immediately** only if:

    - AUC breaches critical threshold (e.g., fraud recall < 90%)

    - Business impact is severe (high-value customers at risk)

    - Root cause is known and addressable (e.g., added new feature source)


    Otherwise: Investigate + plan retraining for next scheduled cycle (daily/weekly).'
- id: 248d32d1-faee-43b6-b3f3-5959bbe005c8
  type: mcq
  front: You implement drift-based retraining with trigger PSI > 0.2 on any feature. In one month, you
    retrain 15 times. Is this too frequent?
  back: '**Correct: b** 15 retrains/month = twice per day, excessive. Reasons: (1) **Noise**: PSI > 0.2
    threshold may be too low; you''re reacting to natural variance, not meaningful drift. (2) **Compute
    cost**: retraining is expensive; 15x/month = $$$. (3) **Overfitting**: frequent retraining on recent
    data can overfit to recent patterns, hurting generalization. (4) **Model churn**: serving different
    model versions confuses debugging.


    **Fix**: Raise PSI threshold to 0.3-0.4, or require drift in 3+ features simultaneously, or retrain
    max 1x/day. Validate: does retraining actually improve production AUC? If not, the retraining isn''t
    justified.'
  choices:
  - key: a
    text: No, frequent retraining is always beneficial
    correct: false
  - key: b
    text: Yes, 15 retrains per month is excessive; likely overfitting or noisy PSI threshold
    correct: true
  - key: c
    text: It depends on model performance; if performance improves, it's justified
    correct: false
  - key: d
    text: Frequency is fine as long as validation AUC improves each time
    correct: false
- id: 5073685b-8329-45fc-82ea-5ca38cec3257
  type: flip
  front: Design a retraining pipeline with periodic (weekly) + performance-based (AUC < 0.92) triggers.
    Include validation and rollback logic.
  back: "**Retraining pipeline**:\n\n**Triggers**:\n1. **Periodic**: Weekly (Monday 2am)\n2. **Performance**:\
    \ Continuous; if AUC < 0.92 on validation set (computed daily), trigger immediate retrain\n\n**Workflow**:\n\
    ```\n1. Trigger (weekly or AUC-based) -> Retrain job\n2. Retrain job:\n   - Fetch training data (last\
    \ 30 days, avoid data leakage)\n   - Train N-fold CV, compute AUC on fold validation\n   - If mean\
    \ AUC < threshold on validation: STOP (data issue, investigate)\n3. Validation on held-out test set:\n\
    \   - Retrained model AUC on test set\n   - Compare to current prod model AUC\n   - If retrained AUC\
    \ > prod AUC + margin (e.g., 0.92 > 0.90), PASS\n   - Otherwise, STOP (no improvement, investigate)\n\
    4. Shadow deploy (optional):\n   - Serve new model on 5% traffic for 24h\n   - Monitor performance\
    \ metrics\n   - Compare latency, error rates to prod model\n5. Blue-green deploy:\n   - Route 100%\
    \ traffic to new model\n   - Monitor AUC, latency, errors for 1h\n6. Rollback (if issues):\n   - If\
    \ AUC ↓ 1% or error rate ↑ 5%, auto-rollback to previous model\n   - Alert on-call\n```\n\n**Data\
    \ leakage prevention**:\n- Training: data before cutoff_date (e.g., last 30 days)\n- Validation: data\
    \ cutoff_date to cutoff_date + 7 days\n- Test: most recent 7 days (no labels yet, or delayed labels\
    \ used for validation only)\n\n**Logging**: Record model version, retraining date, performance metrics,\
    \ trigger reason."
- id: b2460492-6cc9-4765-b8fa-f0c6ca7b156f
  type: flip
  front: Your fraud model trained on 2023 data. In 2024, fraud patterns changed (new attack types). Periodic
    retraining (monthly) didn't catch this for 3 weeks. How would drift-based retraining help?
  back: '**Drift-based retraining advantages here**:


    1. **Early detection**: Drift-based triggers fire within days of new fraud patterns. Feature distributions
    of fraudulent samples change (e.g., faster transaction velocity, new merchant categories). PSI exceeds
    threshold → retrain triggered.


    2. **Reactive to real changes**: Monthly periodic retraining is on a fixed schedule; it doesn''t react
    to sudden distribution shifts. Drift-based retrains when data actually changes.


    3. **Faster response**: In your scenario, drift-based trigger would fire by week 1 (3 weeks faster),
    minimizing fraud losses.


    **Trade-off in your case**:

    - **Drift-based**: Would catch new fraud patterns quickly (good)

    - **Risk**: May retrain too frequently if fraud tactics vary daily; retraining cost + overfitting
    risk


    **Best solution**: Hybrid strategy:

    - Minimum retraining interval: at least 5 days (avoid thrashing)

    - Retrain if PSI > 0.2 AND 5 days have passed since last retrain

    - Also retrain monthly as fallback (robustness)

    - This balances responsiveness + stability'
---

## Intuition
When do you retrain? Three strategies: (1) **Periodic**: Retrain every week/month regardless (safest, predictable cost). (2) **Drift-based**: Monitor feature/prediction distribution; retrain if drift exceeds threshold (reactive, cost-efficient). (3) **Performance-based**: Monitor accuracy; retrain if AUC < threshold (precise, requires labels). Tradeoff: periodic is simple but wasteful; drift-based is data-efficient but may miss performance issues; performance-based is accurate but needs delayed labels.

## Detail
Choose strategy by use case: (1) **High-stakes** (medical diagnosis): Periodic (monthly) + performance-based safeguard (alert if AUC drops). (2) **Fast-changing** (fraud detection): Drift-based (retrain if PSI > 0.15) or daily. (3) **Stable domain** (demographics): Periodic (quarterly). Cost: data labeling, compute, validation. Automate: use CI/CD to trigger retraining pipeline; auto-rollback if new model underperforms validation set. Backtesting: before deploying retrained model, validate on recent held-out test set (avoid train leakage). Monitor model version in serving: log which version served each prediction, for traceability.

## Common gotchas / interview framings
- Retraining too frequently (overfitting to noise) or too rarely (degradation risk)
- Not backtesting retrained model; deploying to production without validation
- Confusing training data version with model version; same training data can have different random seeds
- Not accounting for labeling delay; if labels arrive 30 days late, performance-based trigger detects issues 1 month too late

## See also
- [[prediction-drift-and-label-shift]]
- [[model-performance-degradation-accuracy-drop-calibration-shift]]
- [[feature-distribution-monitoring]]
- [[online-learning-and-incremental-updates]]

## Sources
See frontmatter `sources:`.

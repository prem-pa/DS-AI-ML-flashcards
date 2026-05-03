---
id: a9f6759c-7901-49ca-9508-02e638027ac0
title: Online learning and incremental updates
track: ml-engineer
topic: model-monitoring-mlops
difficulty: 5
tags:
- online-learning
- incremental-training
- stream-processing
- real-time-ml
- model-updates
- stochastic-methods
aliases:
- continual learning
- streaming model updates
- incremental SGD
sources:
- url: https://www.peerspot.com/categories/model-monitoring
  label: 'PeerSpot: Best Model Monitoring solutions 2026'
cards:
- id: aeb48159-1451-455f-a71c-b2a6122d1963
  type: flip
  front: Your recommendation model trains weekly on 1M interactions. Each day, 200k new interactions arrive.
    Should you switch to online learning? What are the trade-offs?
  back: '**Trade-offs**:


    **Pros of online learning**:

    - Model adapts daily to user feedback (new trending content, seasonal preferences)

    - Faster response to distribution shifts (label shift in user engagement)

    - Reduced weekly compute (no full retrain)

    - Fresh model, less staleness (batch retrain is 7 days old by end of week)


    **Cons of online learning**:

    - Risk of overfitting to recent data (last week''s trending content may not generalize)

    - Catastrophic forgetting (model may degrade on older patterns if not careful)

    - Requires delayed labels (feedback on recommendations takes 1-3 days); can''t update immediately

    - Model version tracking becomes harder; incremental updates make debugging difficult

    - Risk of feedback loops (model learns to optimize for engagement, amplifies echo chambers)


    **Decision**: If recommendation quality can tolerate daily shifts (e.g., TikTok feeds), online learning
    is great. If you need consistency (e.g., long-term customer lifetime value), periodic retraining is
    safer. Hybrid: retrain weekly (baseline) + online learning on new interactions (incremental boost).'
- id: 097d2099-6b6f-4530-b8ed-0b64f9686400
  type: mcq
  front: You implement SGD online learning with learning rate=0.1. After 10 days, model accuracy drops
    from 0.91 → 0.78. What is the likely cause?
  back: '**Correct: b** Learning rate 0.1 is too high for online learning. SGD step: w_new = w_old - 0.1
    * gradient. Each batch, weights shift 10%, accumulating over 10 days. If recent data differs from
    original training data, high learning rate causes model to forget old patterns. Accuracy on old test
    set (like original 2M training samples) collapses because model adapted to recent 200k samples.


    **Fix**: (1) Lower learning rate (0.01 or 0.001). (2) Add regularization (L2, L1) to slow learning.
    (3) Mix old + new data: retrain on (old 1M samples + new 200k samples) weekly, don''t just update
    on new. (4) Monitor accuracy on validation set of original data; alert if accuracy ↓ > 3%.'
  choices:
  - key: a
    text: Data quality degradation (nulls increased)
    correct: false
  - key: b
    text: 'Catastrophic forgetting: high learning rate caused model to overfit to recent 10 days and forget
      original patterns'
    correct: true
  - key: c
    text: Concept drift in the underlying data distribution
    correct: false
  - key: d
    text: Online learning doesn't work for this model type
    correct: false
- id: 6e0906e7-936b-4e6f-b5de-b3832cda8026
  type: flip
  front: Design an online learning system for fraud detection. New fraud labels arrive 48 hours after
    transaction. How do you handle delayed labels and prevent catastrophic forgetting?
  back: "**Online learning with delayed labels**:\n\n1. **Buffering**: Store recent predictions (last\
    \ 48h) in a queue. As labels arrive, match prediction_id → label.\n\n2. **Mini-batch updates**: Collect\
    \ labeled data from queue (e.g., last 24h of newly-labeled transactions). Compute loss, update model\
    \ weights via SGD.\n\n3. **Prevent catastrophic forgetting**:\n   - **Learning rate decay**: lr =\
    \ initial_lr / (1 + t), where t = batch number. High updates early (adapt to new fraud), lower updates\
    \ later (stabilize).\n   - **Replay buffer**: Maintain a small buffer of old transactions (1 week).\
    \ Each update, mix 80% new labels + 20% old labels. Model learns new patterns while maintaining old\
    \ knowledge.\n   - **Regularization**: L2 penalty on weight changes: loss = fraud_loss + λ * ||w_new\
    \ - w_baseline||²\n   - **Monitor validation AUC on old data**: Ensure accuracy on baseline fraud\
    \ patterns stays stable.\n\n4. **Model versioning**: Log each update with batch timestamp, sample\
    \ count, AUC on validation. Rollback if AUC ↓ > 2% on old patterns.\n\n5. **Monitoring**: Track model\
    \ drift (feature importance changes), false positive rate (if increasing, model may be overfitting\
    \ to recent fraud patterns).\n\n**Implementation**: Stream processing (Kafka) → buffer → mini-batch\
    \ → SGD update → log metrics."
- id: fb145c5b-3b9c-47d2-ac0c-2195c6b125e5
  type: flip
  front: Can you use online learning for tree-based models (XGBoost)? Why or why not?
  back: '**Limited applicability**:


    **Why not standard SGD**: XGBoost optimizes discrete decision trees; SGD (continuous gradient descent)
    doesn''t apply directly. Trees are non-differentiable (splits are discrete).


    **Alternatives for online learning with trees**:


    1. **Incremental tree addition**: Train new boosting rounds on recent data. Previous rounds frozen,
    new rounds learn new patterns. Equivalent to adding trees to existing ensemble. Risk: model becomes
    very deep (many trees), slower prediction.


    2. **Hoeffding trees** (streaming decision trees): Grow trees incrementally from stream, one sample
    at a time. Works for classification; slower than batch XGBoost.


    3. **Online gradient boosting** (LightGBM, CatBoost): Some implementations support incremental training
    on new mini-batches. Update leaf weights, not full retrain.


    4. **Retraining on recent data window**: Sliding window: retrain XGBoost daily on last 7 days (not
    online in strict sense, but frequent retraining). Simpler than true online learning, more stable.


    **Best practice**: For trees, prefer frequent full retraining (weekly/daily) over online learning.
    Online learning shines for linear models (SGD), neural nets (backprop). If you must do online for
    trees, use incremental tree addition or sliding-window retraining.'
---

## Intuition
Full retraining is expensive (retrain every week = 52 retrains/year, $$$). Online learning updates model on each new sample (or mini-batch) without retraining from scratch. Example: SGD on new batch of user interactions → model learns new patterns immediately, no downtime. Works for linear models, tree ensembles (append new trees), neural nets (fine-tune); less suitable for distance-based models (KNN) or tree-based with structural changes.

## Detail
**Online learning process**: As new labeled data arrives (or with delayed labels), compute gradient on mini-batch, update model weights incrementally. SGD on each batch is online learning. Trade-off: model adapts faster to changes but may overfit to recent data if learning rate too high. Challenges: (1) **Concept drift**: model adapts but forgets old patterns; maintain some old data mix. (2) **Catastrophic forgetting**: new data on different distribution; model accuracy on old data collapses. (3) **Delayed labels**: can't update immediately; buffer recent predictions, update when label arrives (hours/days later). (4) **Stateless deployment**: updated model must be immutable during serving; async update process. Use cases: recommendation systems (new user interactions), fraud detection (adapt to new attack patterns), time-series forecasting (adapt to trends). Not suitable for: critical systems requiring rigorous validation, models where quick changes are risky (medical diagnosis).

## Common gotchas / interview framings
- Conflating online learning with real-time serving; they're orthogonal (online learning is training, real-time is serving latency)
- Not tracking model version in online learning; incremental updates make model history fuzzy
- Overestimating learning rate; model drifts away from original learned patterns
- Ignoring catastrophic forgetting; model trained on 2023 data + online learned on 2024 data may fail on original 2023 patterns

## See also
- [[retraining-triggers-periodic-drift-based-performance-based]]
- [[model-performance-degradation-accuracy-drop-calibration-shift]]
- [[data-freshness]]

## Sources
See frontmatter `sources:`.

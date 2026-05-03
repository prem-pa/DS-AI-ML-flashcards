---
id: a31db427-732c-4129-8996-2f64d30fdfed
title: Experiment tracking and model registry
track: ml-engineer
topic: model-monitoring-mlops
difficulty: 1
tags:
- mlops
- experiment-management
- model-versioning
- metadata-tracking
- collaboration
- reproducibility
aliases:
- MLflow
- experiment management
- model versioning
sources:
- url: https://www.peerspot.com/categories/model-monitoring
  label: 'PeerSpot: Best Model Monitoring solutions 2026'
cards:
- id: c74ac5ef-a437-4496-909d-3c764521c48e
  type: flip
  front: What should you log in experiment tracking for a training run? List key fields.
  back: '**Essential fields to log**:

    1. **Hyperparameters**: learning_rate, batch_size, num_epochs, regularization, model_type

    2. **Metrics**: train_loss, val_auc, val_f1, test_auc (per epoch and final)

    3. **Data**: training dataset version/path, number of samples, feature list

    4. **Code**: git commit hash, branch, code changes (diff)

    5. **Artifacts**: trained model file, scaler/encoder pickle, preprocessing code

    6. **Environment**: python version, library versions, hardware (CPU/GPU type)

    7. **Metadata**: author, start/end time, notes (e.g., "tried new feature X")

    8. **Seed**: random seed (for reproducibility)


    **Example log**:

    ```

    run_id: exp_001

    hyperparameters: {lr: 0.01, batch_size: 32}

    metrics: {val_auc: 0.94, test_auc: 0.92}

    data_version: "train_v2 (2M samples, 45 features)"

    git_commit: "abc123d"

    artifacts: [model.pkl, scaler.pkl]

    seed: 42

    training_time: 3600s

    ```'
- id: 279c8f4b-c9e0-438b-8b0a-6d28dc5bc702
  type: mcq
  front: You logged 50 experiments for a model. 40 have AUC < 0.85, 5 have AUC 0.90-0.92, 5 have AUC 0.93-0.95.
    Which experiments should you keep?
  back: '**Correct: c** Keep top performers for reproduction/reference. Also keep a few low-AUC runs for
    learning: "why did exp_042 (different architecture) underperform?" insights are valuable. Deletion
    criteria: (1) AUC > 0.9 for sure, (2) representative failures (low learning rate, bad architecture)
    for comparison, (3) Latest runs (most recent commit). After 1 year or 100+ experiments, archive old
    runs (storage cost vs utility trade-off).'
  choices:
  - key: a
    text: Keep all 50; you never know when an old experiment will be useful
    correct: false
  - key: b
    text: Keep only top 5 with AUC > 0.93
    correct: false
  - key: c
    text: Keep top 10 (AUC > 0.90) + representative low-AUC runs (for comparison)
    correct: true
  - key: d
    text: Delete all; start fresh each time
    correct: false
- id: fea8697b-46d6-4317-9186-fe3535c8557b
  type: flip
  front: Set up a model registry with versioning. Model v1 is in production. How do you promote v2 (higher
    AUC) safely?
  back: '**Safe model promotion workflow**:


    **Stage 1: Dev (offline validation)**

    - v2 trained, logged to MLflow

    - Compare v2 vs v1 on historical test set

    - Requirement: v2 test_auc >= v1 test_auc + margin (0.5%)

    - Verify on different data slices (by customer segment, region)

    - If passes: mark v2 as "staging-ready"


    **Stage 2: Staging (online shadow test)**

    - Deploy v2 alongside v1 in staging environment

    - Route 5% of traffic to v2, 95% to v1

    - Monitor: latency, error rate, predictions (compare v2 vs v1)

    - Duration: 24-48h

    - Requirement: latency < v1 + 10%, errors < 1%

    - If passes: mark v2 as "prod-ready"


    **Stage 3: Production (gradual rollout)**

    - Day 1: v2 → 10% traffic, v1 → 90%

    - Day 2: v2 → 50% traffic, monitor AUC, errors

    - Day 3: v2 → 100% traffic

    - Have v1 model available for instant rollback


    **Monitoring**:

    - AUC (if labels available) should stay >= v1_auc - 1%

    - Error rate < 5% relative increase

    - Latency <= v1_latency + 5%


    **Rollback**: If AUC ↓ > 2% at any stage, revert to v1 immediately.'
- id: 03318b8f-6c2e-4da5-b678-e500321a746c
  type: flip
  front: You have 100 experiments in MLflow. A data scientist from another team asks, 'Which experiment
    gave the best AUC?' How do you help them find it efficiently?
  back: "**Steps to find best experiment**:\n\n1. **Query MLflow**: Use MLflow UI or API\n   ```python\n\
    \   from mlflow.tracking import MlflowClient\n   client = MlflowClient()\n   runs = client.search_runs(experiment_ids=[\"\
    exp_id\"])\n   best_run = max(runs, key=lambda r: r.data.metrics.get('val_auc', 0))\n   ```\n\n2.\
    \ **Inspect best run**:\n   ```\n   Run ID: abc123\n   Val AUC: 0.95\n   Hyperparameters: lr=0.001,\
    \ batch_size=64\n   Code version: git commit xyz789\n   Dataset: train_v3\n   ```\n\n3. **Document\
    \ findings**: Create a brief note linking best experiment to insights (\"lr=0.001 works best for this\
    \ dataset\").\n\n4. **Reproducibility check**: Data scientist can re-run with same hyperparameters\
    \ + same seed to verify.\n\n**Pro tip**: Add tags to experiments (e.g., \"best_of_batch\", \"production_candidate\"\
    ) for easier filtering. Use MLflow's UI to sort by metric, or write a script to generate a summary\
    \ table (top 10 experiments by AUC)."
---

## Intuition
ML models iterate: try different architectures, hyperparameters, features. Experiment tracking logs each run: parameters, metrics (AUC, loss), artifacts (trained model). Model registry stores production models, versions them, manages promotion (dev → staging → prod). Tools: MLflow (open-source), Weights & Biases (cloud), Neptune, Kubeflow. Enable collaboration (data scientists share learnings), debugging (why did v2 perform worse?), and compliance (audit trail of production models).

## Detail
**Experiment tracking**: Each training run logged with run_id, hyperparameters (lr, batch_size), metrics (val_auc, val_loss), start/end time, training code version. Compare runs: "run_A (lr=0.01) had AUC=0.94, run_B (lr=0.001) had AUC=0.96" → choose B. Store artifacts: trained model, preprocessing pickle, feature importance plot. **Model registry**: Upload best model from experiments; assign version (v1, v2). Store metadata: model description, training dataset version, performance benchmarks. Promote: v1 in prod, v2 in staging (test on % traffic), v3 in dev (experimental). Rollback: v1 is always available. CI/CD integration: automated retraining job logs to MLflow, best model auto-promoted based on test AUC threshold.

## Common gotchas / interview framings
- Not versioning code/data along with model; same model_id but different training data = different results
- Confusing experiment tracking (iterative development) with model registry (production versions)
- Not setting up reproducibility (don't log random seed, get different results)
- Hoarding old experiments; cleanup after 1 year or performance improves 10%+

## See also
- [[reproducibility-and-versioning]]
- [[retraining-triggers-periodic-drift-based-performance-based]]
- [[online-learning-and-incremental-updates]]

## Sources
See frontmatter `sources:`.

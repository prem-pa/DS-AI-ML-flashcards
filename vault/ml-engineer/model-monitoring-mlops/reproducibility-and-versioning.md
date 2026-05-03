---
id: de63dd4d-8f87-43b5-93ba-aebf91390ea9
title: Reproducibility and versioning
track: ml-engineer
topic: model-monitoring-mlops
difficulty: 1
tags:
- versioning
- reproducibility
- data-management
- code-versioning
- best-practices
- ci-cd
aliases:
- code versioning
- data versioning
- seed management
sources:
- url: https://www.peerspot.com/categories/model-monitoring
  label: 'PeerSpot: Best Model Monitoring solutions 2026'
cards:
- id: bed7d081-1358-4cbc-ab6a-779b38ce09b7
  type: flip
  front: You train a model with 0.94 AUC on Jan 1. You retrain with identical code and data on Jan 15,
    get 0.88 AUC. What are three possible causes?
  back: '1. **Random seed changed**: Code calls np.random.seed(), but seed value differs (or not set at
    all). Stochastic algorithms (SGD, random forest) produce different results without fixed seed. Fix:
    log seed in experiment tracking, reproduce with same seed.


    2. **Library version updated**: scikit-learn 1.0.1 → 1.0.2 changes default behavior (e.g., random_state
    handling). Environment reproduced with different library versions. Fix: pin exact library versions
    in requirements.txt (e.g., scikit-learn==1.0.1).


    3. **Data order or preprocessing changed**: Training data shuffled differently (random split changed),
    or feature engineering code updated (e.g., missing value imputation strategy). Same data but processed
    differently. Fix: version training data (MD5 hash), log preprocessing code git commit.


    **Also check**: GPU non-determinism (some CUDA ops are non-deterministic), data loading order (Pandas
    groupby may not be stable).'
- id: 52379c23-0fdb-4231-8778-522eec233b3b
  type: mcq
  front: You want to reproduce a model trained 6 months ago. What information do you NEED to log at training
    time?
  back: '**Correct: b** To reproduce identically 6 months later: (1) **Code git commit**: Pulls exact
    code (preprocessing, model architecture, seed setting). (2) **Data hash**: Identifies exact training
    data version (from 6 months ago). (3) **Random seed**: Ensures stochastic elements (SGD, random forest)
    produce same results. (4) **Library versions**: pinned to prevent API changes. (5) **Hardware**: GPU
    model matters for floating-point precision differences. With these, you can re-run and get identical
    results (or very close, within floating-point rounding).'
  choices:
  - key: a
    text: Training loss curve and final AUC
    correct: false
  - key: b
    text: Code git commit hash, data hash, random seed, library versions, hardware type
    correct: true
  - key: c
    text: Just the trained model file; code can be inferred from model
    correct: false
  - key: d
    text: Hyperparameters only; everything else can be reconstructed
    correct: false
- id: 01322afc-7c62-4b71-a3c7-e034704f31c8
  type: flip
  front: Design a data versioning system for a feature store. A data engineer updates feature X (bug fix).
    How do you track and reproduce old models?
  back: "**Data versioning strategy**:\n\n1. **Version the feature store snapshot**: Before deploying\
    \ feature X update, compute hash of entire feature store (or at minimum, all features used by models).\
    \ Example: feature_store_v1_hash = \"abc123\"\n\n2. **Log version with model**: When training model_v1,\
    \ log feature_store_v1_hash in MLflow. Example:\n   ```\n   model_v1:\n     feature_store_version:\
    \ \"feature_store_v1_hash\"\n     features_used: [X, Y, Z]\n     training_time: 2024-01-01\n   ```\n\
    \n3. **Tag before breaking changes**: Data engineer planning to update feature X announces change.\
    \ Allows teams to:\n   - Re-snapshot old feature store (archive)\n   - Re-train models on old data\
    \ before updating\n\n4. **Handle breaking changes**: If feature X definition changes (bug fix alters\
    \ values by 10%), models trained on old X may no longer work on new X. Solutions:\n   - Retrain models\
    \ on new X (preferred)\n   - Maintain two feature stores (old, new) during transition\n   - Version\
    \ features like code: feature_X_v1 (old logic), feature_X_v2 (new logic)\n\n5. **Audit trail**: Query\
    \ \"which models used feature_store_v1?\" → Retrain those with feature_store_v2 before deprecating\
    \ v1.\n\n**Tools**: DVC (data version control), Tecton (feature store versioning), Delta Lake (ACID\
    \ transactions + versioning)."
- id: eaa0b3ab-c140-44c9-ad94-9454f402a2d3
  type: flip
  front: Describe how to set up a reproducible ML pipeline in Docker. What must be included?
  back: '**Reproducible ML pipeline in Docker**:


    **Dockerfile**:

    ```dockerfile

    FROM python:3.10-slim


    # Install exact library versions

    COPY requirements.txt /app/requirements.txt

    RUN pip install --no-cache-dir -r requirements.txt


    # Copy code at exact git commit

    COPY . /app

    WORKDIR /app


    # Environment variables

    ENV PYTHONHASHSEED=0 CUDA_LAUNCH_BLOCKING=1


    # Run training

    ENTRYPOINT ["python", "train.py"]

    ```


    **requirements.txt**:

    ```

    scikit-learn==1.0.2

    torch==2.0.0

    numpy==1.23.5

    pandas==1.5.2

    ```


    **train.py**:

    ```python

    import numpy as np

    import random

    import torch


    # Set seed

    random.seed(42)

    np.random.seed(42)

    torch.manual_seed(42)


    # Training code...

    ```


    **Versioning**:

    - Git: commit hash identifies code version

    - Data: mount training data as volume (or DVC)

    - Model: save with metadata (git commit, seed, data hash)


    **Reproduction**:

    ```bash

    git checkout abc123d  # checkout old commit

    docker build -t model:v1 .

    docker run -v /data:/data model:v1  # mount data

    ```


    **Result**: Identical training across any machine with Docker.'
---

## Intuition
You train a model, achieve 0.94 AUC. A month later, you retrain with the same code + data, get 0.88 AUC. Why? Likely: random seed changed, data order changed, or library version updated. Reproducibility = ability to recreate identical results. Version everything: code (git), data (data manifests, DVC), seeds (logged). This enables debugging ("when did performance drop?"), auditing ("what data trained this model?"), and compliance ("which exact model version was used for this decision?").

## Detail
**Code versioning**: Git commit hash identifies exact code. Store with model artifact. **Data versioning**: Hash training data (MD5 of dataset file) or use tools like DVC (data version control). Log in experiment tracking: train_data_hash = "abc123". **Seed management**: Set random seeds (numpy, tensorflow, torch) to fixed values. Enables deterministic results. **Library versions**: requirements.txt or conda.yml pinned to exact versions (e.g., scikit-learn==1.0.2, not ==1.0.*). Environment reproducibility: Docker container or virtual environment snapshot. **Workflow**: train → log code_hash, data_hash, seed, libraries → store in model metadata. Later: pull same code version, data version, set same seed → identical results (up to hardware differences).

## Common gotchas / interview framings
- Not logging random seed; model is non-deterministic across runs
- Pinning only major version (sklearn==1.0) not minor (sklearn==1.0.2); library updates break reproducibility
- Using absolute file paths (won't work on different machines); relative paths or data URIs
- Forgetting environment: model trained in Python 3.9 fails in Python 3.11 (numpy API changes)
- Not versioning preprocessing (scaler, encoder); model + data ≠ complete system

## See also
- [[experiment-tracking-and-model-registry]]
- [[retraining-triggers-periodic-drift-based-performance-based]]
- [[feature-distribution-monitoring]]

## Sources
See frontmatter `sources:`.

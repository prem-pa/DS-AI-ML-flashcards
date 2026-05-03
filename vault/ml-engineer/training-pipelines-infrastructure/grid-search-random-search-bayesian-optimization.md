---
id: 197f625f-b32e-4056-8607-fd9a373a7fc3
title: Grid search, random search, Bayesian optimization
track: ml-engineer
topic: training-pipelines-infrastructure
difficulty: 3
tags:
- hyperparameter-tuning
- grid-search
- random-search
- bayesian-optimization
- hpo
aliases:
- hyperparameter search
- parameter tuning
- sample-efficient optimization
sources:
- url: https://optuna.readthedocs.io/
  label: 'Optuna: Hyperparameter Optimization Framework'
- url: https://scikit-optimize.github.io/
  label: scikit-optimize (Bayesian Optimization)
- url: https://arxiv.org/abs/1611.02136
  label: 'Hyperband: A Novel Bandit-Based Approach to Hyperparameter Optimization'
cards:
- id: 8bd4d500-de0c-4aa6-9a4f-d0d12d7decec
  type: flip
  front: Why is random search often better than grid search for hyperparameter tuning?
  back: Random allocates budget to all dimensions equally. If some hyperparams are irrelevant, grid wastes
    trials; random finds good hyperparams faster. Also, high-dim spaces make grid exponential.
- id: 3304b5a7-89a2-4b6b-bc30-d831eb539094
  type: mcq
  front: Bayesian Optimization fits a surrogate model and uses an acquisition function. What is the acquisition
    function for?
  back: ''
  choices:
  - key: a
    text: Speed up training
    correct: false
  - key: b
    text: Select next hyperparameter point to evaluate
    correct: true
  - key: c
    text: Evaluate the model
    correct: false
  - key: d
    text: Regularize hyperparameters
    correct: false
- id: 8f6bb2f8-8371-4e06-af78-bbd7d8bbd296
  type: flip
  front: You have 100 GPUs and 1 week to tune LR and batch size. Which method is best?
  back: Grid search or random search over distributed trials. Parallel evaluation across GPUs. Bayesian
    Optimization better for sequential (few GPUs, expensive evals). Hyperband also good (adaptive allocation).
- id: ab13cbaa-237e-4fe0-97b5-f6f1a97034f4
  type: mcq
  front: Population-Based Training (PBT) differs from standard search how?
  back: ''
  choices:
  - key: a
    text: Uses Bayesian optimization
    correct: false
  - key: b
    text: 'Evolutionary: parallel workers, dynamically adjust hyperparams during training, kill/clone
      workers'
    correct: true
  - key: c
    text: Faster convergence per trial
    correct: false
  - key: d
    text: Requires less compute
    correct: false
---

## Intuition
Hyperparameter tuning: find optimal learning rate, batch size, regularization, etc. Grid search: try all combinations (exhaustive but exponential). Random search: sample randomly (better than grid for high-dim). Bayesian Optimization: model performance landscape, sample promising regions (sample-efficient, but slower per iteration).

## Detail
**Grid Search:** Cartesian product of hyperparameter values. For LR ∈ [0.001, 0.01, 0.1] and batch_size ∈ [32, 64, 128], try all 9 combinations. Pros: simple, parallelizable. Cons: exponential in # of hyperparameters. Inefficient for high-dimensional spaces.

**Random Search:** Sample hyperparameters randomly from distributions. Typically needs 60-80% fewer trials than grid for same performance. Robust to irrelevant hyperparameters (grid allocates equal budget to all dimensions). Easy to scale: add more trials incrementally.

**Bayesian Optimization (BO):** Fit surrogate model (e.g., Gaussian Process) to observed (hyperparams → metric) pairs. Use acquisition function (e.g., Expected Improvement) to select next point. Sequential: fit, pick next, evaluate, repeat. Very sample-efficient (few trials), but slower per iteration. Suited for expensive evaluations (long training).

**Hyperband:** Adaptive resource allocation. Test many hyperparams quickly with few epochs, kill unpromising, increase epochs on survivors. Balances BO's sample efficiency with grid/random's parallelism.

**Population-Based Training (PBT):** Evolutionary approach. Parallel population of workers, each training with different hyperparams. Periodically evaluate, kill worst, spawn copies of best with mutations. Hyperparams dynamically adjust during training.

## Common gotchas / interview framings
- Grid search with 10 hyperparams, 3 values each → 3^10 = 59K trials, infeasible
- Random search undershoots grid on small spaces; overkill for 2-3 hyperparams
- BO assumes smooth, continuous optimization landscape; breaks with discrete hyperparams (optimizer choice)
- Hyperband's early stopping can harm long-horizon tasks (long training period before metric improves)
- Don't tune on test set (information leakage); use separate validation set

## See also
- [[learning-rate-schedules-constant-step-decay-cosine-annealing-warm-restarts]]
- [[batch-size-effects-on-generalization]]
- [[population-based-training-pbt]]

## Sources
See frontmatter `sources:`.

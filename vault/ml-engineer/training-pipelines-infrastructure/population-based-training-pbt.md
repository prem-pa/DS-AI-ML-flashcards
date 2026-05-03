---
id: 286b7dbd-d791-41c8-9fdc-0fa0c0055b46
title: Population-based training (PBT)
track: ml-engineer
topic: training-pipelines-infrastructure
difficulty: 5
tags:
- pbt
- population-based
- evolutionary
- hyperparameter-adaptation
- dynamic-tuning
aliases:
- PBT
- evolutionary hyperparameter tuning
- online hyperparameter adaptation
sources:
- url: https://arxiv.org/abs/1711.09846
  label: Population Based Training of Neural Networks
- url: https://docs.ray.io/en/latest/tune/examples/pbt_example.html
  label: 'Ray Tune: Population-Based Training'
- url: https://github.com/google-deepmind/population-based-training
  label: 'Google DeepMind: Population-Based Training'
cards:
- id: 3c3efa69-0121-49b4-8e1a-6c146f1a079d
  type: flip
  front: How does PBT adapt hyperparameters unlike grid/random search?
  back: PBT runs parallel workers with different hyperparams, periodically kills underperformers, clones
    top performers with mutations. Hyperparams change dynamically during training based on validation
    metric, not fixed upfront.
- id: b5cc497d-5cad-4e14-bcca-580e0e48bd38
  type: mcq
  front: In PBT, after evaluation every T epochs, what happens to the bottom 25% of workers?
  back: ''
  choices:
  - key: a
    text: Continue training with same hyperparams
    correct: false
  - key: b
    text: Get killed; top 25% cloned with mutated hyperparams to replace them
    correct: true
  - key: c
    text: Hyperparams reset to initial values
    correct: false
  - key: d
    text: All workers reset and retrain
    correct: false
- id: 30f26281-f3e5-4272-b533-59086f9c66e2
  type: flip
  front: PBT with 16 workers requires 16× compute vs sequential search. Why use it?
  back: Better sample efficiency (fewer total evaluations to find good hyperparams). Parallelism matches
    research clusters. Dynamic adaptation during training often outperforms static hyperparams, offsetting
    16× compute overhead.
- id: 30e3b8f6-8e34-45d8-91fe-8adbdfdaa86e
  type: mcq
  front: What is a critical hyperparameter of PBT itself (meta-parameter)?
  back: ''
  choices:
  - key: a
    text: Population size N
    correct: false
  - key: b
    text: Mutation rate/variance
    correct: false
  - key: c
    text: Evaluation frequency
    correct: false
  - key: d
    text: All of the above
    correct: true
---

## Intuition
Population-Based Training (PBT) runs a parallel population of training runs (e.g., 16 workers) with different hyperparameters. Periodically (e.g., every 50 epochs), evaluate all workers, kill bottom performers, spawn copies of top performers with hyperparameter mutations. Hyperparameters dynamically adapt during training, exploring promising regions.

## Detail
**Population Setup:** N parallel workers, each training with different hyperparams (LR, batch size, regularization, etc.). Workers train independently, no synchronization (unlike DDP). Periodic evaluation on validation set.

**Selection & Replacement:** Every T epochs (e.g., 50), rank workers by validation metric. Kill bottom k% (e.g., bottom 25%), spawn copies of top k% with mutated hyperparams. Mutations: sample from same distribution as initialization, or perturb current value (±20%). Workers reset to best checkpoint before mutation.

**Hyperparameter Adaptation:** Unlike grid/random search (static hyperparams), PBT learns to adjust hyperparams during training. LR might decay faster than schedule suggests if loss plateaus; batch size might increase to speed up convergence.

**Computational Cost:** N workers training in parallel → N× compute. But parallelism matches research clusters (many GPUs). Better sample efficiency than sequential Bayesian Opt.

**Checkpointing:** Each worker must checkpoint periodically (for replacement). If worker killed, load last checkpoint, mutate hyperparams, continue. This restart overhead amortized over training length.

## Common gotchas / interview framings
- PBT assumes large population (N >> 5) for stability; small N → luck matters
- Mutation rate (variance) critical; too high → too much randomness; too low → no exploration
- Evaluation cost per period must be cheap relative to training (otherwise overhead dominates)
- PBT better for multi-worker scenarios; overkill for 1-GPU experiments
- Hyperparameter exploration bias: if initial range wrong, PBT won't escape (like all search methods)

## See also
- [[grid-search-random-search-bayesian-optimization]]
- [[learning-rate-schedules-constant-step-decay-cosine-annealing-warm-restarts]]
- [[validation-strategy-and-metric-selection]]

## Sources
See frontmatter `sources:`.

---
id: 4b7531e1-1b89-4ad2-b14d-11218ddbbf35
title: Random forests
track: data-scientist
topic: classical-machine-learning
difficulty: 3
tags:
- ensemble-methods
- classification
- regression
- bagging
aliases:
- bootstrap aggregating
- tree ensemble
- out-of-bag error
sources:
- url: https://scikit-learn.org/stable/modules/ensemble.html#forests
  label: scikit-learn Random Forests
- url: https://www.coursera.org/learn/machine-learning
  label: Andrew Ng ML Specialization
cards:
- id: 2e109912-8fe0-478c-a969-190bd89290ca
  type: flip
  front: Bagging + random features mechanism?
  back: 'Bootstrap: deep trees (low bias, high variance). Random $\sqrt{d}$ features: decorrelate trees.
    Averaging reduces variance.'
- id: 66a5603c-0488-4c90-a4af-47dffed9f84e
  type: flip
  front: Out-of-bag (OOB) error utility?
  back: ~37% excluded per bootstrap. Use as free validation ≈ CV error. Auto in sklearn.
- id: 41724247-4088-46c8-8e91-943c16dd9bad
  type: flip
  front: Random features at node help?
  back: All $d$ → correlated trees. $\sqrt{d}$ → diverse trees, lower correlation, better variance reduction.
- id: eb830ac3-5ab9-4dee-9ae5-a13398b886df
  type: mcq
  front: Forests > single tree?
  back: Reduce variance via ensemble; deep trees already low bias.
  choices:
  - key: a
    text: Always.
    correct: false
  - key: b
    text: Reduce variance via bagging+features.
    correct: true
  - key: c
    text: Reduce bias.
    correct: false
  - key: d
    text: No tuning.
    correct: false
---

## Intuition
Random Forests reduce overfitting via bagging + random features. Bootstrap deep trees, random $\sqrt{d}$ features per split. Predictions average across trees.

## Detail
**Algorithm:** For each bootstrap sample, grow tree with random features. Aggregate: majority vote (classification), average (regression). **OOB error:** ~37% excluded per sample, use as free validation.

## See also
- [[bagging]]
- [[decision-trees]]
- [[variance-reduction]]

## Sources
See frontmatter `sources:`.

---
id: 2f27e6f1-6674-4ebd-93d6-5db90bb420ab
title: Decision trees
track: data-scientist
topic: classical-machine-learning
difficulty: 3
tags:
- classification
- regression
- interpretable
- tree-algorithms
aliases:
- CART
- recursive partitioning
- information gain
- tree pruning
sources:
- url: https://scikit-learn.org/stable/modules/tree.html
  label: scikit-learn Decision Trees
- url: https://www.coursera.org/learn/machine-learning
  label: Andrew Ng ML Specialization
cards:
- id: fbe85af3-1720-473a-be33-465666416b3b
  type: flip
  front: Information gain + entropy. IG=0.8 vs 0.2?
  back: 'Entropy: disorder. IG=H(parent)-weighted H(children). Higher better. IG=0.8 better; reduces parent
    entropy more.'
- id: c8f002ce-93fe-4524-94af-515e7f5983aa
  type: flip
  front: Gini impurity. Pure node desirable?
  back: $\text{Gini}=1-\sum p_c^2$. Pure (1 class)=0 (best). 50-50=0.5 (worst). Trees minimize weighted
    child Gini.
- id: 477d7d24-e249-4612-a461-cee127c7e3d7
  type: flip
  front: Why tree pruning necessary? Cost-complexity?
  back: Grown trees overfit. Pruning minimizes loss+$\alpha$·leaves, balancing fit vs complexity. $\alpha$
    tuned via CV.
- id: 209b075c-93d1-4855-8dbd-b9b12199400f
  type: flip
  front: Feature importance in trees?
  back: Normalized total impurity reduction across nodes. Auto in sklearn. Biased toward high-cardinality
    features.
---

## Intuition
Trees recursively split to maximize purity. Interpretable but prone to overfitting without constraints.

## Detail
**Information gain:** $IG = H(parent) - \sum (|c|/|parent|)H(c)$ where $H = -\sum p_c \log p_c$.
**Gini:** $\text{Gini}(S) = 1-\sum p_c^2$. **Pruning:** Remove subtrees not improving validation.

## See also
- [[information-gain]]
- [[gini-impurity]]
- [[entropy]]
- [[pruning]]

## Sources
See frontmatter `sources:`.

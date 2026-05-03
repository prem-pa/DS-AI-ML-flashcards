---
id: abad4d31-01e6-48d5-9e44-f51cfd9945fb
title: Validation strategy and metric selection
track: ml-engineer
topic: training-pipelines-infrastructure
difficulty: 3
tags:
- validation
- metrics
- early-stopping
- evaluation
- overfitting-detection
aliases:
- metric selection
- validation frequency
- performance monitoring
sources:
- url: https://scikit-learn.org/stable/modules/model_evaluation.html
  label: scikit-learn Model Evaluation
- url: https://pytorch.org/docs/stable/nn.functional.html#loss-functions
  label: PyTorch Loss Functions
- url: https://paperswithcode.com/methods/category/metrics
  label: 'Papers with Code: Metrics'
cards:
- id: 639c94a1-94bf-4e13-8332-18af09e146f3
  type: flip
  front: Training loss decreases but validation loss increases. What should you do?
  back: 'Model is overfitting. Either: increase regularization (dropout, weight decay, augmentation),
    reduce model capacity, use early stopping at best validation checkpoint, or add more training data.'
- id: 753f599e-e49b-4b55-81d9-189f06e193e4
  type: mcq
  front: For severely imbalanced binary classification (1% positive), which metric is best?
  back: ''
  choices:
  - key: a
    text: Accuracy
    correct: false
  - key: b
    text: Precision
    correct: false
  - key: c
    text: AUC-ROC or Precision-Recall curve
    correct: true
  - key: d
    text: Recall only
    correct: false
- id: 1ac708b5-df08-4947-ab42-bc7e9632754b
  type: flip
  front: What is early stopping and why use it?
  back: Monitor validation metric, stop training if it doesn't improve for N epochs (patience). Saves
    at best checkpoint. Prevents overfitting and wastes training time on epochs with no improvement.
- id: 47e33a5e-8080-4445-ae63-2161dbc599f0
  type: mcq
  front: Should you choose your validation metric before or after seeing test data?
  back: ''
  choices:
  - key: a
    text: After seeing test data to optimize for it
    correct: false
  - key: b
    text: Before; choosing post-hoc causes overfitting to test set
    correct: true
  - key: c
    text: It doesn't matter
    correct: false
  - key: d
    text: Choose multiple and average
    correct: false
---

## Intuition
Training loss doesn't reveal true performance—it's optimistic (trained on it). Validation loss is unbiased proxy. Choose metrics aligned with task objective: accuracy for balanced classification, F1/AUC for imbalanced, BLEU for NLP, RMSE for regression.

## Detail
**Validation Frequency:** Every epoch (standard); every N iterations if epochs long. Too frequent → overhead; too sparse → miss best checkpoint window. Typical: every epoch or every 100 iterations.

**Overfitting Detection:** Plot train vs validation metrics. If train ↓ and validation ↑, overfitting. Widen gap → need regularization (dropout, weight decay), more data, or smaller model.

**Metric Selection by Task:**
- Balanced Classification: Accuracy (intuitive), F1 (precision-recall balance)
- Imbalanced Classification: AUC-ROC (threshold-independent), Precision-Recall curve, Macro-F1
- Regression: MAE (robust to outliers), RMSE (emphasizes large errors), MAPE (relative error)
- Multi-label: mAP (mean average precision), Hamming loss

**Early Stopping:** Monitor validation metric, stop if no improvement for N epochs (patience). Save best checkpoint, report metrics on test set at end. Prevents overfitting and wastes.

**Validation on Full Data vs Subset:** Full validation every epoch → accurate but slow. Subset (10-20%) → fast but noisy. Trade-off depends on validation cost.

## Common gotchas / interview framings
- Use training metric for early stopping → wrong (overfitting undetected, early stopping triggers late)
- Validation metric selected post-hoc (after seeing test results) → overfitting to test set
- Imbalanced data + accuracy metric → model predicts majority, 99% accuracy but useless
- Validation during training, but test set has different distribution → metrics don't transfer
- Change validation metric mid-training → invalidates comparisons

## See also
- [[loss-curves-and-convergence-diagnostics]]
- [[checkpointing-and-recovery]]
- [[imbalanced-data-and-oversamplingundersampling]]

## Sources
See frontmatter `sources:`.

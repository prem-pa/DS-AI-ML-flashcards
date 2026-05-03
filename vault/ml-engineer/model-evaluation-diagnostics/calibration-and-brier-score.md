---
id: 427b3587-3ee6-47d8-b4d1-b043f88399a4
title: Calibration and Brier score
track: ml-engineer
topic: model-evaluation-diagnostics
difficulty: 3
tags:
- calibration
- confidence
- probability
- Brier score
- Platt
- isotonic
aliases:
- well-calibrated
- probability calibration
- ECE
- confidence quality
sources:
- url: https://scikit-learn.org/stable/modules/calibration.html
  label: 'scikit-learn: Probability Calibration'
- url: https://www.blog.trainindata.com/probability-calibration-in-machine-learning/
  label: 'Train in Data: Probability Calibration'
- url: https://www.blog.trainindata.com/complete-guide-to-platt-scaling/
  label: 'Train in Data: Complete Guide to Platt Scaling'
- url: https://www.rohan-paul.com/p/ml-interview-q-series-probability-95f
  label: 'Rohan Paul: Probability Calibration Q&A'
cards:
- id: ca9c1e23-5c22-4840-88ee-1ac9bb3c7a43
  type: flip
  front: What's the difference between model accuracy and calibration? Can a model be accurate but poorly
    calibrated?
  back: Accuracy measures correctness; calibration measures whether confidence matches truth frequency.
    Yes—a classifier that predicts 0.51 for all samples is 51% accurate but Brier score ≈ 0.25 (terrible
    calibration). Conversely, one predicting 0.99 for 51% of samples and 0.01 for rest can be accurate
    AND well-calibrated. Both matter for decision-making under uncertainty.
- id: 12bb1156-f5c3-4fc1-9a35-74e8726fa27c
  type: flip
  front: Explain Platt Scaling. When is it preferable to isotonic regression?
  back: 'Platt Scaling: fit logistic regression $p = \sigma(A \cdot score + B)$ on validation set. Assumes
    sigmoid distortion (common in SVM, logistic regression). Data-efficient (~100 samples needed). Isotonic:
    fit monotonic piecewise function; flexible, handles any shape. Overfits on small data. Prefer Platt
    if <100 calibration samples, isotonic if >1000.'
- id: f9b0957a-673a-4ba3-b009-4bffd37ddc79
  type: mcq
  front: A model predicts p=0.8 on 100 samples; 85 are actually positive. Brier score?
  back: 'Brier = avg((0.8-1)² × 85 + (0.8-0)² × 15) / 100 = (0.04×85 + 0.64×15) / 100 = (3.4 + 9.6) /
    100 = 0.13 (closest 0.1225). Well-calibrated: BS should be ≈0.02 (match 80% base rate).'
  choices:
  - key: a
    text: '0.04'
    correct: false
  - key: b
    text: '0.15'
    correct: false
  - key: c
    text: '0.0225'
    correct: false
  - key: d
    text: '0.1225'
    correct: true
- id: ff6e37ac-7911-43c4-acf8-8435952a91f1
  type: flip
  front: Why must calibration be fit on a separate holdout set, not the training set?
  back: 'Fitting calibration on training data causes overfitting: the calibration function learns noise
    specific to train set. At test time, confidence miscalibration increases (especially with isotonic
    regression). Best practice: split into train/calibration/test; use calibration set only for post-hoc
    fitting.'
- id: 48d1eac1-9dda-4b9c-b0fd-a11a10955100
  type: flip
  front: A neural network model is overconfident (predicted probabilities too high). How would temperature
    scaling help?
  back: 'Temperature scaling: divide logits by T > 1 before softmax. T=2 doubles logit magnitudes, resulting
    in flatter softmax (confidence closer to 0.5). Converts overconfident predictions (e.g., 0.99) to
    more calibrated (e.g., 0.80). Fit T on validation set to minimize ECE, then apply to test predictions.'
---

## Intuition
Model calibration measures whether predicted probabilities match empirical frequencies. A well-calibrated 80%-confidence prediction should be correct ~80% of the time. Calibration is orthogonal to accuracy: accurate models can be poorly calibrated (overconfident/underconfident). Brier score quantifies calibration quality.

## Detail
For binary predictions with confidence $p_i$ and true labels $y_i \in \{0,1\}$:

- **Brier Score** = $\frac{1}{n}\sum (p_i - y_i)^2$ → squared difference between predicted probability and actual outcome. Range [0,1]; lower is better. BS=0: perfect calibration.
- **Expected Calibration Error (ECE)** = $\sum_m |\text{accuracy}_m - \text{confidence}_m|$ → bins predictions by confidence level, measures gap; M bins typical
- **Platt Scaling**: fit logistic regression to model scores: $p = \sigma(A \cdot \text{score} + B)$. Parametric, assumes sigmoid-shaped distortion, low data requirement (~100 samples)
- **Isotonic Regression**: fit monotonic, piecewise-linear function. Non-parametric, flexible, can overfit on <100 samples. Superior to Platt on large datasets

Calibraiton curve: x-axis = predicted probability, y-axis = true positive rate within bin. Diagonal = perfect; above diagonal = underconfident (model too timid), below = overconfident (overestimate correctness).

## Common gotchas / interview framings
- Calibration ≠ accuracy: model can be 95% accurate but predict 0.5 for all; Brier = 0.25 (terrible), accuracy = 0.95
- Post-hoc calibration: fit on holdout data to avoid train-test contamination; never calibrate on same data as optimization
- Overfitting in isotonic: use cross-validation; on <100 samples, Platt more stable
- Temperature scaling: simple softmax calibration in neural nets; T>1 smooths confidence (less overconfident)
- Brier score dominated by prevalence: on 95%-negative data, always predicting 0.05 gives Brier ≈ 0.05 (looks good but useless)

## See also
- [[platt-scaling]]
- [[isotonic-regression]]
- [[expected-calibration-error]]
- [[model-confidence]]
- [[probability-estimation]]

## Sources
See frontmatter `sources:`.

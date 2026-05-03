---
id: 549747d5-e4b3-4e0e-9e3f-7b3f5dca798d
title: Out-of-distribution (OOD) detection
track: ml-engineer
topic: model-evaluation-diagnostics
difficulty: 5
tags:
- OOD detection
- distribution shift
- uncertainty
- anomaly detection
- domain adaptation
aliases:
- anomaly detection
- novelty detection
- drift detection
sources:
- url: https://www.evidentlyai.com/classification-metrics
  label: 'Evidently AI: Classification Metrics'
- url: https://www.ml4devs.com/what-is/model-evaluation-metrics/
  label: 'ML4Devs: Model Evaluation Metrics'
cards:
- id: f251821a-292d-4e44-b5fb-7a55a413ec1a
  type: flip
  front: Why is max softmax score alone insufficient for OOD detection?
  back: 'Neural networks are overconfident: they assign high softmax score to random/OOD inputs. Example:
    vision model confidently predicts ''dog'' (98%) on noise. Max softmax doesn''t capture whether model
    has *seen similar data before*. Better: use energy score, ensemble disagreement, or Mahalanobis distance
    that explicitly model training data distribution.'
- id: fc34c655-8e7a-4e2e-9e15-62dab5ebbcaa
  type: flip
  front: Explain Mahalanobis distance for OOD detection. When does it fail?
  back: 'Compute class-wise mean μ_k and covariance Σ_k on training data. OOD score = distance from sample
    to nearest class: $\sqrt{(x - \mu_k)^T Σ_k^{-1}(x - \mu_k)}$. High distance = OOD. Fails when: (1)
    features high-dimensional (curse of dimensionality), (2) ID data multimodal (single Gaussian insufficient),
    (3) feature whitening masks class structure. Requires careful feature engineering/dimensionality reduction.'
- id: 11e4f120-155f-438b-9ff4-e631d087a04f
  type: mcq
  front: A model confidently predicts class A (softmax=0.95) on an OOD image. What score better detects
    this OOD?
  back: Energy score = log(sum of exp logits). High for ID (many high logits), low for OOD (diffuse logits).
    Separates ID/OOD better than softmax which can be high in both. Temperature scaling can help but energy
    is more robust.
  choices:
  - key: a
    text: Max softmax
    correct: false
  - key: b
    text: Energy score (logsum before softmax)
    correct: true
  - key: c
    text: Confidence × accuracy correlation
    correct: false
  - key: d
    text: Test error
    correct: false
- id: 7a2d1410-0692-4f41-9c4e-3a3586af3e08
  type: flip
  front: You deploy a model for fraud detection. How would you set OOD threshold for flagging transactions
    as anomalies?
  back: 'Never use test set OOD data to set threshold (overfitting risk). Use separate validation OOD
    set: (1) collect fraud/novel samples, (2) compute ROC curve (ID=0, fraud=1), (3) choose threshold
    at desired false positive rate (FPR), e.g., FPR=5% for 95% precision. Monitor production drift; threshold
    degrades over time—retune quarterly.'
- id: bccad264-b051-4ac0-8256-b36e462c0956
  type: flip
  front: Compare max-softmax vs ensemble disagreement for OOD detection. When is ensemble better?
  back: 'Max-softmax: single model score; fast but overconfident. Ensemble disagreement: variance across
    5-10 models; slower but robust. Ensemble better when: (1) OOD diverse (need multiple perspectives),
    (2) budget allows inference cost. Max-softmax okay if model well-calibrated (post-hoc calibration).
    In practice, ensemble disagreement + Mahalanobis + temperature scaling is best.'
---

## Intuition
OOD detection identifies inputs that differ from training distribution. Critical for production: model confidence is unreliable on OOD data. Methods range from simple (max softmax score) to complex (generative models, density estimation). Calibration helps: well-calibrated models are less overconfident on OOD.

## Detail
**Simple OOD scores**:
- **Max softmax**: predict OOD if max(softmax) < threshold τ. Weak; overconfident DNNs often fail.
- **Energy score**: $$\text{score} = \log \sum_i e^{z_i}$$ (before softmax). Separates ID/OOD better than max softmax
- **Temperature scaling** (before threshold): divides logits by T; flattens confidence, makes OOD detection easier

**Advanced methods**:
- **Mahalanobis distance**: compute class-wise mean and covariance on ID data. OOD score = distance to nearest class center. Sensitive to feature space geometry.
- **Deep ensemble diversity**: multiple models disagree on OOD; use disagreement (variance of predictions) as OOD signal
- **Density estimation**: fit Gaussian mixture or VAE on ID features. OOD = low likelihood. Assumes feature distribution unimodal.
- **Generative models**: train on ID data, score OOD via likelihood ratio or discriminator confidence
- **Test-time adaptation**: fine-tune on test batch to detect shift (performance degradation on unlabeled test = drift indicator)

**Metrics**: OOD detection evaluated as binary classification (ID vs OOD). Use AUROC, AUPR. Example: ID=0 (in-distribution), OOD=1 (out-of-distribution); higher score = more OOD.

## Common gotchas / interview framings
- OOD ≠ hard to classify: OOD may be easy for human but unlike training; hard examples are ID but near decision boundary
- Softmax overconfidence: neural nets assign high confidence even to random noise; don't rely on raw softmax
- Threshold tuning: OOD threshold chosen on test set = overfitting; use separate OOD validation set
- Multi-modal ID distribution: Gaussian assumptions fail; need mixture models or nonparametric density
- Closed-world vs open-world: closed = OOD is defined set of classes; open = any new class. Methods differ

## See also
- [[distribution-shift]]
- [[anomaly-detection]]
- [[uncertainty-quantification]]
- [[mahalanobis-distance]]
- [[auroc-ood]]

## Sources
See frontmatter `sources:`.

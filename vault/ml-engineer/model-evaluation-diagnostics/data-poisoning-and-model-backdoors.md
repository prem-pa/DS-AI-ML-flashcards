---
id: c614fdac-124a-4af6-8dff-8ce68ba0197b
title: Data poisoning and model backdoors
track: ml-engineer
topic: model-evaluation-diagnostics
difficulty: 5
tags:
- data poisoning
- backdoor attacks
- trojan
- training data
- anomaly detection
aliases:
- label flipping
- feature injection
- trigger patterns
sources:
- url: https://www.ml4devs.com/what-is/model-evaluation-metrics/
  label: 'ML4Devs: Model Evaluation Metrics'
- url: https://www.evidentlyai.com/classification-metrics
  label: 'Evidently AI: Classification Metrics'
cards:
- id: 66f15aa1-ab28-4788-880e-f5fae3c85227
  type: flip
  front: Explain the difference between data poisoning (label flipping) and backdoor attacks.
  back: 'Label flipping: corrupt 10% of labels (random). Effect: model accuracy drops uniformly (~10%
    degradation). Simple, obvious. Backdoor: inject trigger pattern into ~5% of samples, mislabel to target
    class. Effect: clean data accuracy maintained, but triggered inputs misclassify. Backdoor is targeted
    and stealthy; label flipping is broad and detectable.'
- id: eff0d664-509a-468b-8486-75a3f541611e
  type: flip
  front: A model achieves 98% accuracy on test set, but fails (predicts 'SPAM') on emails with specific
    keyword 'INVOICE'. Diagnose.
  back: 'Likely backdoor attack. Attacker poisoned training data: emails with ''INVOICE'' mislabeled as
    SPAM during training. Model learned trigger→SPAM mapping. Detection: (1) test for trigger sensitivity
    (vary ''INVOICE'' → ''DOCUMENT'', see if misclassification persists), (2) activation clustering to
    find abnormal patterns, (3) inspect training labels for suspicious correlation. Mitigation: relabel
    data, retrain on clean subset.'
- id: f2a20918-3534-49ec-9267-476b7d453ba9
  type: mcq
  front: To defend against backdoor attacks, which approach is most practical at scale?
  back: 'Manual audit infeasible for millions of samples. Robust training (mixup, noisy label loss) reduces
    backdoor effectiveness without perfect label detection. Sampling + agreement checks (option C) is
    secondary: catch gross errors but miss subtle backdoors. Combination is best practice.'
  choices:
  - key: a
    text: Manually audit all training labels
    correct: false
  - key: b
    text: Train model on data with label noise (robust training)
    correct: false
  - key: c
    text: Random sampling + inter-annotator agreement checks
    correct: true
  - key: d
    text: All equally effective
    correct: false
- id: ed68d3a4-69fd-4138-9073-8ce1cff9ffe1
  type: flip
  front: 'Your image classifier is backdoored: every image with a small 3×3 watermark is mispredicted.
    How would you detect this?'
  back: 'Methods: (1) activate saliency maps (backprop gradients) to visualize decision regions; backdoored
    samples highlight trigger pattern, (2) activation clustering: compute internal representations on
    test set; backdoored samples cluster away from clean class centroid, (3) test-time scanning: add synthetic
    watermarks to test images, measure misclassification spike. If watermark + misclassification correlate
    → backdoor confirmed.'
- id: 826c68a8-027e-489a-bda1-06f813fd5988
  type: flip
  front: A crowdsourced dataset has 0.5% mislabeled samples (detected via re-annotation). What's the impact
    on model performance?
  back: '~0.5% accuracy drop (linear to label flip rate if random noise). If mislabeling is adversarial
    (concentrated on certain classes or features), impact can be higher. Worse: if mislabels form backdoor
    pattern (all ''class A'' samples mislabeled as ''class B''), accuracy drop concentrated in one class-pair.
    Always audit label quality; prioritize high-risk sources (first-time annotators, low-agreement samples).'
---

## Intuition
Data poisoning attacks corrupt training data to degrade model or embed backdoors. Poisoning is stealthy: model maintains high accuracy on clean data but fails on adversary-chosen inputs (triggers). Detection requires data validation, anomaly detection, and test-time monitoring. Critical in crowdsourced or third-party data scenarios.

## Detail
**Attack types**:
1. **Label flipping**: flip fraction p of labels (0→1, 1→0). Simple but detectable via data audit. Effect: ~10% flip = ~10% accuracy drop.
2. **Feature injection** (backdoor): add small pattern/noise to subset of samples, mislabel them. Clean data accuracy maintained; triggered inputs misclassify. Example: add 3×3 pixel pattern (trojan trigger) to images; train model to predict 'target_class' when trigger present.
3. **Availability attack**: poison to maximize error on all inputs. Stealthy harder; noticeable if widespread.

**Backdoor formulation**:
- Attacker controls trigger pattern τ and target class $y^*$
- Poison: add τ to ~5% of training samples, relabel to $y^*$
- Result: model high accuracy (clean data unchanged), but x + τ → $y^*$ with ~100% success
- Trigger imperceptible to human labeler if subtle (e.g., specific hue shift, watermark)

**Defenses**:
- **Data validation**: random audits of labels (inter-annotator agreement), outlier detection on features
- **Anomaly detection**: identify mislabeled samples via confidence-based scores or clustering
- **Robust training**: train with noisy labels or mixup to reduce sensitivity to label corruption
- **Model inspection**: test-time trigger detection via model behavior on synthetic inputs, saliency maps
- **Activation clustering**: group activation patterns; backdoored samples cluster separately (DeepInspect method)

## Common gotchas / interview framings
- Backdoor stealthiness: model still 95%+ accurate on clean data; backdoor is targeted misclassification on trigger
- Trigger invisibility: triggers can be imperceptible (1-2% pixel change, color shift); hard to detect manually
- Supply-chain risk: third-party datasets, pretrained models, or crowd-sourced labels all vulnerable
- Detection cost: inspecting 1M training samples expensive; focus on high-risk subsets (labels from unknown annotators)
- Robustness vs accuracy: defenses (robust training) trade accuracy for poisoning resistance; quantify cost

## See also
- [[poisoning-attack]]
- [[backdoor-attack]]
- [[trojan-network]]
- [[trigger-pattern]]
- [[data-validation]]

## Sources
See frontmatter `sources:`.

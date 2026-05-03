---
id: 7628e93e-8729-4b25-9baf-2e5d6062ec9b
title: Adversarial examples and adversarial robustness
track: ml-engineer
topic: model-evaluation-diagnostics
difficulty: 5
tags:
- adversarial examples
- robustness
- perturbations
- certified defense
- adversarial training
aliases:
- adversarial attacks
- perturbation attacks
- robust ML
sources:
- url: https://www.ml4devs.com/what-is/model-evaluation-metrics/
  label: 'ML4Devs: Model Evaluation Metrics'
- url: https://www.evidentlyai.com/classification-metrics
  label: 'Evidently AI: Classification Metrics'
cards:
- id: f287d698-0ed4-4920-81a3-37f4e07616fa
  type: flip
  front: Explain FGSM attack. Why is it weaker than PGD?
  back: 'FGSM: δ = ε·sign(∇_x L) (one-shot). Moves in steepest gradient direction. Simple, fast but suboptimal.
    PGD: iterative; moves multiple steps within ε-ball, projecting back after each step. PGD explores
    ε-ball better, finding adversarial examples that FGSM misses. PGD usually 10-30% stronger attack.'
- id: b3752dbe-416f-46f6-a453-0d00739f15cd
  type: flip
  front: A model achieves 95% clean accuracy and 40% adversarial accuracy (ε=8/255). Is this acceptable?
  back: 'Context-dependent. For non-critical tasks (content recommendation), 40% robust accuracy may be
    acceptable. For security-critical (malware detection, fraud), 40% is unacceptable—attacker can bypass.
    Acceptable robustness varies: autonomous vehicles require ~90%, spam filters ~70%. Always specify
    threat model (ε, attack type) and application requirements.'
- id: 374507ce-0540-4e6f-97d8-df242f112d2a
  type: mcq
  front: 'Certified robustness via randomized smoothing: add Gaussian noise during inference to bound
    perturbations. Main tradeoff?'
  back: 'All correct. Randomized smoothing requires: (1) inference on noisy samples (M≈100), averaging
    predictions = slower, (2) provable bounds on L2 robustness, (3) clean accuracy drops ~5-10%, (4) model
    may need retraining. Trade-off: verifiable security for performance.'
  choices:
  - key: a
    text: Slower inference (multiple samples needed)
    correct: false
  - key: b
    text: Provable robustness vs empirical accuracy
    correct: false
  - key: c
    text: Requires model retraining
    correct: false
  - key: d
    text: All of above
    correct: true
- id: a2caac3a-4991-4798-a462-ad09192e7cd4
  type: flip
  front: Your model is adversarially trained (ε=8/255). Is it robust to ε=16/255 attacks?
  back: No. Adversarial training to ε is robust only to perturbations ≤ε (approximately). Training on
    ε=8/255 gives ~0% accuracy at ε=16/255. Robustness is threshold-based, not general. To handle multiple
    ε values, use graduated adversarial training (vary ε during training) or certified methods (randomized
    smoothing scales with Gaussian σ).
- id: 3be7d382-e21c-4efc-8acd-2570a3b711b2
  type: flip
  front: Adversarial examples fool models but not humans. Why? What does this imply for robustness evaluation?
  back: 'Models rely on brittle features (texture, spatial patterns) not robust to small perturbations.
    Humans rely on semantic features (parts, objects) robust to pixel noise. Implication: standard metrics
    (accuracy) misleading under adversarial conditions. Must evaluate against threat model (ε, attack
    type); clean accuracy ≠ adversarial robustness. Use adaptive attacks to avoid overfitting defenses.'
---

## Intuition
Adversarial examples are inputs slightly perturbed from originals to fool models, yet imperceptible to humans. Robustness measures model performance under such attacks. Evaluation requires adversarial test sets; standard metrics (accuracy) may be misleading if untested against attacks. Two defense paradigms: certified (provable bounds) vs empirical (best-effort).

## Detail
**Attack formulation** (adversarial perturbation δ):
$$\max_δ L(\text{model}(x + δ), y_{\text{target}}) \text{ s.t. } \|δ\|_p \leq ε$$
ε = perturbation budget (L2: pixel value change; L∞: max per-pixel change). Typical ε: 8/255 (ImageNet), 0.3 (MNIST).

**Common attacks**:
- **FGSM** (Fast Gradient Sign Method): δ = ε·sign(∇_x L). One-shot, fast, weak.
- **PGD** (Projected Gradient Descent): iterative attack; ε-ball constrained. Stronger; gold standard.
- **C&W** (Carlini-Wagner): unconstrained optimization; finds minimal perturbation. Very strong.

**Defenses**:
- **Adversarial training**: train on adversarial examples (e.g., PGD) + original data. Empirical; no guarantees. ε-robust training hardens model to perturbations ≤ε.
- **Certified defense** (randomized smoothing): add Gaussian noise at inference; proves robustness to L2 perturbations. Trade-off: accuracy for provable guarantees.
- **Input preprocessing**: JPEG compression, bit depth reduction, feature squeezing. Weak; easily bypassed.

**Evaluation**:
- **Standard accuracy**: accuracy on clean (original) test set.
- **Adversarial accuracy**: accuracy on adversarially perturbed test. Much lower; metric matters (ε budget, attack type).
- **Robustness curve**: adversarial accuracy vs ε. Typically decreases as ε increases.

## Common gotchas / interview framings
- White-box vs black-box: white-box = attacker knows model weights (harder defense). Black-box = model API only (easier defense). Don't confuse threat models.
- Adaptive attacks: attacker aware of defense; defeat many simple defenses. Always evaluate against adaptive attacks.
- Accuracy-robustness tradeoff: robust models (adversarial training) ~5-10% less accurate on clean data. Engineering tradeoff.
- Transferability: adversarial examples trained on Model A often fool Model B (cross-model transfer). Use this for black-box attacks.
- Certified bounds loose: provable robustness guarantees often large ε (loose); practical robustness often much tighter

## See also
- [[fgsm-attack]]
- [[pgd-attack]]
- [[adversarial-training]]
- [[certified-robustness]]
- [[perturbation-budget]]

## Sources
See frontmatter `sources:`.

---
id: d8fc0719-ac29-4d0a-aa31-005670c0fce4
title: Label smoothing
track: ml-engineer
topic: classical-machine-learning-shared-with-ds-deeper-focus
difficulty: 3
tags:
- soft-targets
- regularization
- calibration
- cross-entropy
- confidence-penalty
aliases:
- soft-label-mixing
- target-smoothing
- confidence-reduction
sources:
- url: https://arxiv.org/abs/1512.00567
  label: Rethinking Generalization (Zhang et al.)
- url: https://arxiv.org/abs/1906.02629
  label: Bag of Tricks (He et al.)
cards:
- id: a3a87d84-5322-4653-93e7-4a57944ee30a
  type: flip
  front: What does label smoothing do, and why does it improve generalization?
  back: 'Replace one-hot with soft targets: uniform mass on non-true classes, high mass on true. Prevents
    overconfidence by penalizing large logits for incorrect classes. Implicit L2 regularization → better
    generalization and calibration.'
- id: b66aff3b-4a51-46e7-b24d-d31b87704652
  type: mcq
  front: If label smoothing ε=0.1 and n_classes=10, what is the probability assigned to the true class?
  back: 'True class: (1 - ε) + ε/n_classes = 0.9 + 0.01 = 0.91. Others: ε/(n_classes - 1) = 0.01/9 ≈ 0.0011
    each. Sum = 0.91 + 9*0.0011 = 1.0.'
  choices:
  - key: a
    text: '0.9'
    correct: false
  - key: b
    text: '0.91'
    correct: false
  - key: c
    text: '0.919'
    correct: true
  - key: d
    text: 1.0 - 0.1/9
    correct: false
- id: 0a7e2caa-6c1e-4131-9654-7cfdf7e3d0aa
  type: flip
  front: How does label smoothing interact with calibration, and what is the tradeoff?
  back: 'Label smoothing reduces confidence → improves calibration (predicted probs closer to true likelihoods).
    Tradeoff: may reduce accuracy (model less confident). Use when calibration matters (e.g., medical
    diagnosis); skip when accuracy is only metric.'
- id: 2537c475-fdff-4dc8-8f90-e00213c7bb0c
  type: flip
  front: Why might label smoothing hurt performance on datasets with many classes?
  back: With large n_classes, uniform smoothing spreads mass thin over non-true classes. Fixed ε becomes
    less aggressive relative to class count. May need smaller ε for large n_classes, or adaptive ε based
    on class count.
- id: 6ee9eba0-0c79-4cc3-98b7-2b30925558be
  type: mcq
  front: What is the difference between label smoothing and knowledge distillation?
  back: 'Label smoothing: uniform mass on non-true classes. Distillation: teacher model generates soft
    targets (class-specific, contains more info). Different mechanisms, similar effect on regularization.'
  choices:
  - key: a
    text: Same thing
    correct: false
  - key: b
    text: 'Label smoothing: uniform targets; Distillation: teacher-specific soft targets'
    correct: true
  - key: c
    text: Distillation faster
    correct: false
  - key: d
    text: Label smoothing only for classification
    correct: false
---

## Intuition

Label smoothing: replace one-hot labels with soft targets. Instead of y=1 for true class (0 for others), use y=(1-ε)/(n_classes) for non-true classes, and y=(1-ε) + ε/(n_classes) for true class. Prevents overconfidence, improves calibration.

## Detail

**Hard vs. soft labels:**
- Hard: y = [0, 1, 0, 0] (one-hot, true class=1)
- Soft: y = [0.01, 0.97, 0.01, 0.01] (ε=0.03, distributed uniformly)

**Formulation:**
- ỹ_i = {(ε / n_classes) if i ≠ true_class; (1 - ε) + (ε / n_classes) if i = true_class}
- ε ∈ [0, 1], typically 0.01–0.1
- ε=0: hard labels; ε=1: uniform (all classes equal)

**Benefits:**
- **Reduces overconfidence**: model learns to assign probability mass to incorrect classes
- **Improves calibration**: predicted probabilities closer to true likelihoods
- **Regularization effect**: penalizes large logits; acts as implicit L2 on outputs
- **Better generalization**: soft targets reduce overfitting

**Mathematical interpretation:**
- With soft targets, cross-entropy becomes: L = -Σ ỹ_i * log(p_i)
- Gradient includes contribution from non-true classes → prevents overfit-to-hard-labels
- Equivalent to cross-entropy + regularization on logits

**Interaction with other techniques:**
- Mixup + label smoothing: both soften targets (can be redundant but often combined)
- Dropout + label smoothing: orthogonal
- Knowledge distillation: teacher's soft outputs → similar effect to label smoothing

**Engineering considerations:**
- **ε value**: task-dependent; 0.1 common for ImageNet, smaller for smaller datasets
- **Number of classes**: with many classes, uniform mass spreads thin; ε=0.1 less aggressive than ε=0.1 in binary
- **Class imbalance**: label smoothing may hurt minority class (assigns mass away)
- **KL divergence metric**: label smoothing reduces KL between predicted and true distribution
- **Calibration**: label smoothing improves Expected Calibration Error (ECE)

```python
import torch
import torch.nn as nn

class LabelSmoothingLoss(nn.Module):
    def __init__(self, num_classes, smoothing=0.1):
        super().__init__()
        self.num_classes = num_classes
        self.smoothing = smoothing
        self.confidence = 1.0 - smoothing
    
    def forward(self, logits, labels):
        log_probs = torch.log_softmax(logits, dim=1)
        
        # Soft target: high confidence on true class, uniform on others
        with torch.no_grad():
            true_dist = torch.zeros_like(log_probs)
            true_dist.fill_(self.smoothing / (self.num_classes - 1))
            true_dist.scatter_(1, labels.unsqueeze(1), self.confidence)
        
        return torch.mean(torch.sum(-true_dist * log_probs, dim=1))

# Usage
criterion = LabelSmoothingLoss(num_classes=10, smoothing=0.1)
output = model(input)
loss = criterion(output, target)
```

## Common gotchas / interview framings
- Label smoothing ≠ soft labels from distillation; label smoothing uniform, distillation teacher-specific
- ε too large: model learns to ignore labels (uniform distribution); typical ε=0.1
- With class imbalance: label smoothing may hurt minority class (assigns mass away proportionally)
- Calibration: label smoothing improves calibration but may reduce accuracy (tradeoff)
- Not always beneficial: very noisy labels benefit; high-quality labels may not need smoothing
- Interaction with dropout: both regularize; redundant but often combined

## See also
- [[loss-functions-mse-cross-entropy-focal-loss]]
- [[mixup-and-cutmix]]

## Sources
See frontmatter `sources:`.

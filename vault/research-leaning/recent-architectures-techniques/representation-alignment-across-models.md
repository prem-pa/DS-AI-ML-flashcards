---
id: daf16b7c-b6ff-4281-8862-1306f8f5d66b
title: Representation alignment across models
track: research-leaning
topic: recent-architectures-techniques
difficulty: 5
tags:
- representation-learning
- model-alignment
- feature-similarity
- mechanistic-interpretability
- transfer-learning
- cross-model-alignment
aliases:
- feature-alignment
- representational-similarity
- activation-alignment
sources:
- url: https://arxiv.org/abs/2106.09573
  label: 'Representation Learning in Deep Networks: What We Know and What We Don''t'
- url: https://arxiv.org/abs/2305.10062
  label: Similarity of Neural Networks with Random Labels
- url: https://arxiv.org/abs/2304.06871
  label: Emergent Abilities of Large Language Models
cards:
- id: 524fb527-9284-477e-9032-92fb7f85910f
  type: flip
  front: What does representation alignment mean, and why would we expect different models to learn aligned
    representations?
  back: 'Representation alignment is the similarity of learned features across independently trained models.
    We''d expect alignment because: (1) data constrains which features are useful (data manifold), (2)
    optimization converges to local optima in feature space, (3) good features are generalizable across
    architectures. If alignment occurs, insights from one model generalize to others.'
- id: fee8d6ef-b7fc-4e39-bafb-fa503d2ef34e
  type: mcq
  front: Which layer-wise pattern emerges when comparing representations across aligned models?
  back: Early layers learn task-specific, initialization-dependent features and show high divergence.
    Middle and late layers learn more universal, task-relevant features and show stronger alignment. This
    pattern suggests that learning progresses from specific to general features.
  choices:
  - key: a
    text: All layers have equal alignment
    correct: false
  - key: b
    text: Early layers diverge; middle/late layers align
    correct: true
  - key: c
    text: Only the final layer aligns; internal layers always differ
    correct: false
  - key: d
    text: Alignment is random and unpredictable
    correct: false
- id: 61ad3b18-ffef-4074-8bc1-17690d71965e
  type: flip
  front: What are three metrics used to measure representation alignment between neural networks?
  back: '1. **Representation Similarity Analysis (RSA)**: correlate pairwise distances of activations

    2. **Centered Kernel Alignment (CKA)**: measure alignment of Gram matrices of representations

    3. **Linear probe transfer**: train linear classifier on one model''s representations and test on
    another''s; high accuracy indicates alignment'
- id: 5d9d0c03-ba81-4ad4-9916-a6b571ec66f4
  type: mcq
  front: According to recent research, what is the relationship between scale and representation alignment?
  back: Larger models exhibit stronger representation alignment, suggesting that scaling leads to convergence
    toward universal, generalizable features. This supports the idea that sufficiently large models learn
    optimal feature hierarchies that are robust across architectural variations.
  choices:
  - key: a
    text: Larger models have less alignment due to overfitting
    correct: false
  - key: b
    text: Scale has no effect on alignment
    correct: false
  - key: c
    text: Larger models show stronger representation alignment, suggesting universal features emerge with
      scale
    correct: true
  - key: d
    text: Alignment is independent of model size
    correct: false
- id: 2ed1a125-fc32-4c92-a8bf-7a0c9b169077
  type: flip
  front: What is a key implication of representation alignment for mechanistic interpretability?
  back: 'If models learn aligned representations, studying mechanisms in one model (e.g., circuit analysis)
    can generalize to other models. This makes interpretability research more tractable: you don''t have
    to study every model separately. Conversely, if alignment is weak, mechanistic insights are architecture-specific
    and less broadly applicable.'
---

## Intuition

Different neural networks trained on the same task often develop similar internal representations (learned features), even if they have different architectures or random initializations. Representation alignment measures this similarity and has implications for interpretability, transfer learning, and understanding what models learn.

## Detail

**Core questions:**
- Do models trained independently learn aligned representations?
- Can we measure alignment (similarity of learned features)?
- What factors cause or prevent alignment (architecture, data, training procedure)?

**Key findings (2023-2024):**
1. **Convergence**: Models trained on the same data often converge to similar representations in intermediate/later layers, even with different architectures
2. **Alignment metrics**: Representation Similarity Analysis (RSA), Centered Kernel Alignment (CKA), and linear probe transfer measure alignment
3. **Layer-wise patterns**: Early layers diverge (task-specific initialization effects); middle/late layers align (convergence to optimal features)
4. **Generalization link**: Models with better cross-model representation alignment often generalize better
5. **Scale effects**: Larger models show stronger representation alignment, suggesting scaling leads to more universal features

**Implications:**
- **Mechanistic interpretability**: if models align, we can study one model and generalize to others
- **Transfer learning**: aligned representations enable effective transfer; misalignment hurts transfer
- **Model compression**: aligning student-teacher representations (distillation) improves compression
- **Robustness**: models with aligned robust features are more transferable

## Common gotchas / interview framings
- Q: "Do all models learn the same features?" A: No; alignment is partial and layer-dependent. Early layers are very different, late layers partially aligned
- Q: "What breaks alignment?" A: Different architectures, small datasets, mismatched training procedures, different loss functions
- Q: "Can we force alignment?" A: Yes, via distillation or explicit alignment losses, but often at a cost to accuracy
- Causality confusion: alignment correlates with generalization, but doesn't prove alignment causes generalization
- Benchmark dependency: alignment patterns differ across datasets; ImageNet alignment ≠ speech alignment

## See also
- [[linear-probes]]
- [[representation-similarity-analysis]]
- [[mechanistic-interpretability]]
- [[transfer-learning]]
- [[neural-network-alignment]]
- [[feature-learning]]

## Sources
See frontmatter `sources:`.

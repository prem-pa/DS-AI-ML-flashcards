---
id: 6449f2c2-1638-4e8b-b72e-998fab6c81d3
title: Extrapolation on longer sequences post-training
track: ai-llm-engineer
topic: transformer-foundations-deep-dive
difficulty: 5
tags:
- length-extrapolation
- context-extension
- training-distribution
- inference
aliases:
- context-window-extension
- length-generalization
sources:
- url: https://arxiv.org/abs/2309.16039
  label: 'YaRN: Efficient Context Window Extension'
- url: https://arxiv.org/abs/2108.12409
  label: 'Train Short, Test Long: ALiBi'
cards:
- id: 331f0327-6e19-403a-9320-190856a4b2c9
  type: flip
  front: Why do transformers trained on 2K-token sequences perform poorly at 32K-token inference, and
    what is the root cause?
  back: Positional encodings are optimized for training-length distribution. With sinusoidal PE, positions
    1-2K are trained; positions 2K-32K are unseen. RoPE learns rotation angles up to 2K*θ_j; at 32K, angles
    are 16x larger, far outside the learned distribution. The model's attention patterns, learned for
    distance range [0, 2K], don't generalize to [0, 32K]. Distributional shift in token-pair distances
    and position representations causes performance degradation.
- id: 41bb854f-6f72-4bad-92ea-af74d71c5601
  type: flip
  front: How does RoPE interpolation enable post-training context extension, and what is the fine-tuning
    cost?
  back: RoPE interpolation rescales rotation angles θ_j by a factor s = L_new / L_train, compressing longer
    sequences back into the learned distance distribution. For L_train=2K to L_new=32K, scale s=16. Brief
    fine-tuning (2-10B tokens) on the new length stabilizes performance. Cost is much lower than continued
    pre-training (100B+ tokens) while achieving comparable final quality with careful tuning.
- id: 6c1bc301-2d87-43b1-a7f5-573338b7ee3c
  type: mcq
  front: Which positional encoding method naturally extrapolates to much longer sequences without post-training
    adaptation?
  back: 'Correct: (b). ALiBi''s distance bias is linear and position-free (no embeddings). Linear functions
    extrapolate trivially: if distance 100 received bias -b during training, distance 1000 receives -10b.
    The relationship is consistent. Sinusoidal PE has absolute position range; RoPE has rotation angles;
    both require adaptation. Learned embeddings cannot extrapolate beyond training length. ALiBi is unique
    in enabling extreme extrapolation (2K -> 100K+) without adaptation.'
  choices:
  - key: a
    text: Sinusoidal positional encodings
    correct: false
  - key: b
    text: ALiBi (attention linear biases)
    correct: true
  - key: c
    text: RoPE without interpolation
    correct: false
  - key: d
    text: Learned position embeddings
    correct: false
- id: 245a1e98-e160-496e-986d-6856d6694ef0
  type: flip
  front: Compare the fine-tuning costs of RoPE interpolation vs. continued pre-training for context extension
    from 4K to 16K.
  back: 'RoPE interpolation: 2-10B tokens fine-tuning (typically 1-3 days on A100). Continued pre-training:
    100B+ tokens (2-4 weeks on A100). Interpolation is 10-50x faster but may not achieve the same final
    quality as full CPT. For production systems needing quick adaptation, interpolation is preferred.
    For ultimate quality, CPT is superior but slower and more expensive.'
---

## Intuition
Transformers trained on context length L (e.g., 2K tokens) struggle at inference length >>L (e.g., 32K tokens) due to distributional shift. Positional encodings (sinusoidal, RoPE) are optimized for training length. Methods like RoPE interpolation, ALiBi, or continued pre-training allow post-training adaptation without full retraining, enabling long-context inference.

## Detail
The core challenge: training on L tokens means attention patterns, token-pair distances, and positional encoding ranges are optimized for [1, L]. At inference with length L' >> L:
1. **RoPE:** Rotation angles are much larger than training; [[rope-interpolation-and-extrapolation]] rescales to map L' back to learned distribution.
2. **ALiBi:** Linear bias extrapolates naturally; no rescaling needed, enabling 10-100x length extrapolation.
3. **Sinusoidal PE:** No built-in extrapolation; requires continued pre-training or fine-tuning to adapt.
Continued pre-training (CPT) on longer sequences (e.g., 1B tokens at 8K length) gradually exposes the model to longer distances, improving internal representations. This is cheaper than full pretraining but slower than RoPE interpolation (which requires only 1-10B tokens).

## Common gotchas / interview framings
- **Interpolation is cheaper:** RoPE interpolation requires 2-10B fine-tuning tokens vs. 100B+ for CPT, but CPT may improve quality more.
- **Task-dependent:** Tasks heavily using long-range reasoning (summarization, retrieval) need better long-context support. Short-range tasks (translation) may not.
- **Degradation pattern:** Quality degrades gradually as length increases beyond training; not a sharp cliff.
- **Position frequency:** Models trained on diverse position frequencies (varying document lengths) generalize better post-training.
- **Continued pre-training can regress:** Adapting to much longer sequences may hurt performance on original training-length distribution; careful learning rate tuning is needed.

## See also
- [[rotary-position-embeddings-rope]]
- [[rope-interpolation-and-extrapolation]]
- [[continued-pre-training-on-longer-sequences]]

## Sources
See frontmatter `sources:`.

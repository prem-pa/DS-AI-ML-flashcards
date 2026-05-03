---
id: 67c2966f-4651-41ac-ba3d-a25eb75a9122
title: RoPE interpolation and extrapolation
track: ai-llm-engineer
topic: transformer-foundations-deep-dive
difficulty: 5
tags:
- rope
- context-extension
- length-extrapolation
- interpolation
- fine-tuning
aliases:
- position-interpolation
- YaRN
- rope-extension
sources:
- url: https://arxiv.org/abs/2309.16039
  label: 'YaRN: Efficient Context Window Extension of Large Language Models'
- url: https://aclanthology.org/2024.acl-long.735.pdf
  label: 'HiRoPE: Length Extrapolation for Code Models'
- url: https://learnopencv.com/rope-position-embeddings/
  label: 'Learn OpenCV: RoPE Interpolation'
cards:
- id: a5d9c766-247a-43ac-b077-a7c1c14b07de
  type: flip
  front: Describe position interpolation for RoPE and why it allows context-length extension.
  back: 'Position interpolation rescales RoPE rotation angles by a factor s = L_new / L_train. For a model
    trained on 4K tokens to serve 32K, angles are divided by 8. This maps new positions back into the
    learned distance range [0, 4K], allowing the model to apply learned patterns. Brief fine-tuning (1-10B
    tokens) on the new length stabilizes performance. The key insight: relative distances matter; if we
    compress back to training distribution, learned patterns apply.'
- id: d52d5d67-7547-4849-a60e-f5b02c2046bb
  type: flip
  front: What does YaRN improve over naive position interpolation, and at what fine-tuning cost?
  back: Naive interpolation scales all dimensions equally, which can hurt local-distance representations.
    YaRN uses dimension-dependent dynamic scaling and rescales attention logits post-computation, achieving
    better preservation of local patterns. YaRN requires ~2.5x fewer fine-tuning tokens than naive interpolation
    to achieve the same quality, making it more efficient for long-context adaptation.
- id: 9b0c8998-1708-4566-aa43-a431b6882319
  type: mcq
  front: Why do RoPE models trained on 4K tokens struggle at 32K without interpolation/extrapolation?
  back: 'Correct: (b). RoPE applies rotation m*θ_j for position m. Training on 4K means rotations up to
    4000*θ_j are learned. At 32K, rotations up to 32000*θ_j are encountered—8x larger, never seen in training.
    The attention mechanism generalizes poorly to such extreme rotation angles. Interpolation rescales
    to map 32K back to the 4K distribution. (a) is not precise. (c) is not the root cause. (d) is unrelated.'
  choices:
  - key: a
    text: The model's parameters are not suitable for longer sequences
    correct: false
  - key: b
    text: Token-pair distances at 32K are outside the training distribution; rotation angles applied are
      much larger than anything seen during training
    correct: true
  - key: c
    text: The gradient computation becomes unstable for long sequences
    correct: false
  - key: d
    text: The vocabulary size must be expanded for longer sequences
    correct: false
- id: 5ad14cad-f4c5-421f-ab0b-2a8bb4c5a96f
  type: flip
  front: If a model is trained on 2K context and you want to extend it to 8K using position interpolation,
    what scaling factor would you apply?
  back: Scaling factor s = L_new / L_train = 8K / 2K = 4. Each RoPE rotation angle θ_j is divided by 4.
    This maps 8K sequences into the learned distance distribution of [0, 2K], allowing the model to reuse
    learned attention patterns. Brief fine-tuning on the new length (2-5B tokens) is then needed.
---

## Intuition
RoPE-based models trained on context length L struggle at lengths >>L due to distributional shift in token-pair distances. Interpolation/extrapolation methods rescale RoPE's rotation angles to map longer sequences back to the training distribution, enabling inference on much longer contexts with minimal fine-tuning.

## Detail
RoPE rotation angles are $\theta_j = 10000^{-2j/d}$. **Position interpolation** rescales by a factor s:

$$\theta_j' = 10000^{-2j/d} / s$$

with $s = L_{\text{new}} / L_{\text{train}}$. This compresses longer sequences into the learned distance distribution. For example, with L_train=4K and L_new=32K, s=8. Distances that would be 32K become 4K. The model recognizes patterns it learned. **YaRN (Yet another RoPE eXtension)** improves on naive interpolation by using dynamic scaling: different dimensions get different scaling factors, and attention logits are rescaled. This requires only ~2.5x fewer tokens to fine-tune vs. simple interpolation.

## Common gotchas / interview framings
- **Naive extrapolation fails:** Simply allowing longer sequences without rescaling fails. The model has never seen such large distance rotations.
- **Interpolation vs. extrapolation naming:** 'Interpolation' is rescaling to compress; 'extrapolation' is applying learned patterns to new distances. Both terms used in literature.
- **Fine-tuning cost:** Brief fine-tuning (1-10B tokens) on the new length is usually needed. Full pretraining is not required.
- **NTK-aware scaling:** Some methods use NTK-Aware RoPE Scaling, adjusting the base frequency (10000) rather than scaling angles.
- **Trade-offs:** Aggressive interpolation may hurt local-distance prediction; dynamic scaling (YaRN) balances this.

## See also
- [[rotary-position-embeddings-rope]]
- [[extrapolation-on-longer-sequences-post-training]]
- [[continued-pre-training-on-longer-sequences]]

## Sources
See frontmatter `sources:`.

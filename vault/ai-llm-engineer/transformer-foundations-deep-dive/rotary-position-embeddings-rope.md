---
id: 3fe09c4f-6b7a-4c09-a837-1e5c9cef9a78
title: Rotary position embeddings (RoPE)
track: ai-llm-engineer
topic: transformer-foundations-deep-dive
difficulty: 3
tags:
- rotary-embeddings
- rope
- relative-distance
- length-generalization
- modern-llms
aliases:
- RoPE
- rotary embeddings
- rotary PE
sources:
- url: https://arxiv.org/abs/2104.09864
  label: 'RoFormer: Enhanced Transformer with Rotary Position Embedding'
- url: https://learnopencv.com/rope-position-embeddings/
  label: 'Learn OpenCV: RoPE Position Embeddings'
- url: https://www.emergentmind.com/topics/rotary-positional-embeddings-rope
  label: 'Emergent Mind: RoPE'
cards:
- id: e2497c84-d5ca-44ff-b7de-c760d6bc2694
  type: flip
  front: Explain how RoPE encodes relative position and why this makes it better for length generalization
    than sinusoidal PE.
  back: RoPE applies position-dependent 2D rotations to query and key vectors. Critically, the attention
    logit (q_m')^T(k_n') depends only on the relative distance m-n, not absolute positions m and n. This
    allows the model to generalize to sequences longer than training length; relationships between tokens
    separated by distance k work the same way regardless of absolute position. Sinusoidal PE encodes absolute
    position, so training on 2K-length sequences fails at 32K.
- id: 448f3e39-d540-4279-af0d-5ba9fefb573a
  type: flip
  front: What does it mean for RoPE to 'rotate' query and key vectors, and what are the rotation angles?
  back: RoPE partitions each query and key vector into 2D pairs and rotates each pair by an angle proportional
    to position m or n. The rotation angle at frequency j is m*θ_j where θ_j = 10000^(-2j/d), matching
    the frequency scheme from sinusoidal encoding. Lower-frequency pairs (small j) rotate slowly, high-frequency
    pairs rotate quickly, enabling multi-scale position encoding.
- id: 608f0e54-9e76-43f4-a8d7-412d81cebeb1
  type: mcq
  front: Which property of RoPE makes it generalize to sequences longer than the training length?
  back: 'Correct: (b). RoPE''s rotation depends only on m-n, the relative distance. A model trained on
    4K sequences learns patterns for all relative distances in [0, 4K]. At inference on 32K sequences,
    new relative distances appear, but for distances seen during training, the model''s behavior is unchanged.
    (a) is not how RoPE works. (c) is ALiBi, not RoPE. (d) is not RoPE''s mechanism.'
  choices:
  - key: a
    text: RoPE uses learned embeddings that adapt to any sequence length
    correct: false
  - key: b
    text: RoPE encodes only relative distance between positions, not absolute positions
    correct: true
  - key: c
    text: RoPE applies bias terms that scale logarithmically with distance
    correct: false
  - key: d
    text: RoPE performs attention on compressed token representations
    correct: false
- id: 88edc5e6-7ad5-4741-818b-e1cd190017ee
  type: flip
  front: RoPE models trained on 4K tokens struggle at 32K context. Why, and how do interpolation methods
    help?
  back: Training on 4K creates attention patterns for relative distances in [0, 4K]. At 32K, new distances
    (e.g., 20K) appear. The model hasn't learned these; distributional shift occurs. Interpolation methods
    (YaRN, position interpolation) rescale rotation angles θ_j -> θ_j / scale, mapping longer sequences
    back to the training distribution of distances. This requires only brief fine-tuning rather than full
    pretraining.
---

## Intuition
RoPE encodes relative distance by rotating query and key vectors in a 2D plane. If the relative distance between two positions is k, their inner product after rotation depends only on k, not absolute positions. This makes the attention mechanism naturally relative-distance-aware, enabling excellent length generalization.

## Detail
For query and key vectors, RoPE applies position-dependent rotations:

$$q_m' = (q_1, q_2, q_3, q_4, \ldots) \text{ rotated by } m\theta_j$$
$$k_n' = (k_1, k_2, k_3, k_4, \ldots) \text{ rotated by } n\theta_j$$

where $\theta_j = 10000^{-2j/d}$ (similar to sinusoidal encoding). The key property: 
$$(q_m')^T (k_n')$$ depends only on relative distance $m - n$, not on absolute positions. This makes attention weights naturally relative-position-biased. RoPE is now standard in LLaMA, Qwen, and most modern LLMs.

## Common gotchas / interview framings
- **Relative vs. absolute:** RoPE bases attention on relative distance m-n, not absolute m or n. This is why it generalizes to unseen longer sequences.
- **Length extrapolation issues:** Despite relative encoding, models trained on 4K still struggle at 32K inference due to training-test distributional shift in token-pair distances.
- **RoPE interpolation:** Context-window extension methods (YaRN, position interpolation) rescale rotation angles to map longer sequences back to training distribution. Requires brief fine-tuning.
- **Interaction with attention:** RoPE modifies Q, K before the dot product. It is position-dependent matrix multiplication, not bias-injection like ALiBi.
- **Computational overhead:** Applying rotation to every query and key at every layer has small CPU cost but is often fused into attention kernels.

## See also
- [[absolute-positional-encodings-sinusoidal]]
- [[relative-position-embeddings-alibi]]
- [[rope-interpolation-and-extrapolation]]

## Sources
See frontmatter `sources:`.

---
id: 8b195015-0458-41b9-ad4a-b7bac279b714
title: Absolute positional encodings (sinusoidal)
track: ai-llm-engineer
topic: transformer-foundations-deep-dive
difficulty: 3
tags:
- positional-encoding
- sinusoidal
- position-bias
- embeddings
- length-generalization
aliases:
- sinusoidal PE
- absolute positional encodings
- PE
sources:
- url: https://arxiv.org/abs/1706.03762
  label: Attention is All You Need (Vaswani et al.)
- url: https://kazemnejad.com/blog/transformer_architecture_explained/
  label: Transformer Architecture Explained
cards:
- id: fabcba16-16f3-41cb-b939-ea8a75cdda7a
  type: flip
  front: Write the sinusoidal positional encoding formula and explain why different dimensions oscillate
    at different frequencies.
  back: '$PE(pos, 2i) = \sin(pos / 10000^{2i/d})$, $PE(pos, 2i+1) = \cos(pos / 10000^{2i/d})$. Different
    frequencies encode distance at different scales: low frequencies (small i, long wavelengths) capture
    distant relative positions; high frequencies (large i, short wavelengths) capture fine local distance.
    This multi-scale representation lets the model reason about position at any granularity.'
- id: 7f5f8ddc-8701-41c3-991b-ed16f07815f5
  type: flip
  front: What is the main limitation of sinusoidal positional encodings for length generalization?
  back: Sinusoidal PE encodes absolute position based on a fixed scale (wavelengths derived from d). Training
    on sequences of length 2K creates a distribution over positions [0, 2K]. At inference on 16K sequences,
    new positions have never been seen in training, causing severe performance degradation. Relative-position
    methods (RoPE, ALiBi) generalize better.
- id: aa56dd0c-79a3-4666-92cf-8a7d336dd386
  type: mcq
  front: Why do sinusoidal positional encodings use both sine and cosine at each frequency?
  back: 'Correct: (b). Sine and cosine form a 2D basis for each frequency. Together, they allow the encoder
    to represent position as a point on a 2D circle at that frequency, preserving all information. This
    enables linear transformations to extract relative positions: PE(pos+k) = Rotation(k) * PE(pos). (a)
    is a side effect. (c) is orthogonality to embedding not the goal. (d) is not the reason.'
  choices:
  - key: a
    text: To ensure positional values are always bounded in [-1, 1]
    correct: false
  - key: b
    text: To encode both magnitude and phase information at each frequency
    correct: true
  - key: c
    text: To make the encoding orthogonal to token embeddings
    correct: false
  - key: d
    text: To improve numerical stability during backpropagation
    correct: false
---

## Intuition
Transformers lack position information; tokens are permutation-invariant. Sinusoidal positional encodings inject position via hand-crafted sine/cosine waves at different frequencies, enabling the model to distinguish position without learning embeddings.

## Detail
For position pos and dimension index d:

$$PE(pos, 2i) = \sin\left(\frac{pos}{10000^{2i/d}}\right)$$
$$PE(pos, 2i+1) = \cos\left(\frac{pos}{10000^{2i/d}}\right)$$

Each dimension oscillates at a different frequency (10000^(2i/d)). Low frequencies (small i) have long wavelengths (useful for distant positions), high frequencies (large i) have short wavelengths (useful for nearby positions). The encoding is added to token embeddings before attention.

## Common gotchas / interview framings
- **Why sine/cosine?** Linear transformations on PE allow the model to extract relative position: PE(pos+k) can be expressed as a linear function of PE(pos). This encodes relational distance.
- **No learned parameters:** Unlike learned embeddings, sinusoidal PE requires no training, generalizes without retraining.
- **Length generalization fails:** Trained on 2K-context models, inference on 16K fails badly. PE was never designed for length extrapolation.
- **Symmetric and continuous:** PE is differentiable everywhere; training can be done on any length theoretically, but in practice, distributional shift causes issues.
- **Absolute vs. relative:** Sinusoidal PE encodes absolute position, but [[relative-position-embeddings-alibi]] and [[rotary-position-embeddings-rope]] focus on relative distances, generalizing better.

## See also
- [[rotary-position-embeddings-rope]]
- [[relative-position-embeddings-alibi]]
- [[scaled-dot-product-attention-derivation]]

## Sources
See frontmatter `sources:`.

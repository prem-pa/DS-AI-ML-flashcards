---
id: 3c7bad1d-3e6d-4905-9d6c-dbbd7baa00fc
title: Relative position embeddings (ALiBi)
track: ai-llm-engineer
topic: transformer-foundations-deep-dive
difficulty: 3
tags:
- alibi
- relative-distance
- bias-injection
- length-extrapolation
- positional-encoding
aliases:
- ALiBi
- attention-linear-biases
- linear-bias
sources:
- url: https://arxiv.org/abs/2108.12409
  label: 'Train Short, Test Long: Attention with Linear Biases Enables Input Length Extrapolation'
- url: https://www.emergentmind.com/topics/attention-linear-biases
  label: 'Emergent Mind: ALiBi'
cards:
- id: 9d37cd65-7500-457e-95f4-0d902699c6a5
  type: flip
  front: How does ALiBi encode position information, and why does it enable extreme length extrapolation?
  back: 'ALiBi adds a distance-dependent bias to attention logits: L_ij += head_slope * (i - j). This
    penalty grows linearly with distance. Training on length 2K exposes all distances [0, 2K]. At inference
    on 32K, the linear bias naturally extrapolates—farther distances get more negative bias, as learned
    during training. No positional embeddings needed; the attention mechanism itself encodes position
    through bias structure.'
- id: 3124b795-3830-43fb-885c-44731b55ae6b
  type: flip
  front: What does each attention head learn in ALiBi, and why is this useful?
  back: 'Each head learns a slope (or bias-weight) controlling how strongly distance affects that head''s
    attention. Different heads learn different slopes: some steep (local attention), others shallow (global
    attention). This allows diverse attention patterns across heads without explicit position-type embeddings.
    The model learns to route information through appropriate heads based on distance requirements.'
- id: 9cce3687-13ac-4e8d-b86e-334765e560e7
  type: mcq
  front: Which is a key advantage of ALiBi over sinusoidal positional encodings?
  back: 'Correct: (a). ALiBi uses distance-dependent bias injection rather than position embeddings, saving
    parameters and memory. The linear bias is computed on-the-fly. (b) is not a stated advantage. (c)
    is false—Q, K projections are still needed. (d) is not accurate; ALiBi''s advantage is generalization
    via simple linear bias, not learned accuracy.'
  choices:
  - key: a
    text: ALiBi reduces memory usage by not storing positional embeddings
    correct: true
  - key: b
    text: ALiBi enables faster matrix multiplication in attention
    correct: false
  - key: c
    text: ALiBi eliminates the need for query and key projections
    correct: false
  - key: d
    text: ALiBi improves attention accuracy by using learned positions
    correct: false
- id: cfd4903a-b582-418f-9861-59d5149d04d6
  type: flip
  front: Why can ALiBi models trained on 2K tokens generalize to 32K tokens better than sinusoidal-based
    models?
  back: 'ALiBi''s distance bias is linear and parameter-free (or has very few learnable slopes). Linear
    functions extrapolate trivially: if distance 100 received bias -b during training, distance 1000 receives
    bias -10b. The relationship is consistent. Sinusoidal PE has absolute position encoded via wave frequencies;
    positions >2K have never been seen, causing distributional shift. ALiBi''s extrapolation is principled;
    sinusoidal''s is not.'
---

## Intuition
Instead of adding positional embeddings to token representations (like sinusoidal PE) or rotating vectors (like RoPE), ALiBi directly biases the attention logits by relative distance. Each attention head has a learnable head-specific slope, and position distance is multiplied by this slope before softmax. This simple approach enables extreme length extrapolation.

## Detail
Let attention logits (before softmax) be $L = QK^T / \sqrt{d}$. ALiBi adds a distance-dependent bias:

$$L_{ij} := L_{ij} + \text{head\_slope} \times (i - j)$$

where head_slope is learned or fixed per head. The bias grows linearly with distance, discouraging attention to distant tokens. Different heads learn different slopes, allowing some heads to attend globally while others attend locally. During training, all distances [0, L] are observed. At test time with longer sequences, the linear bias extrapolates naturally—more negative for farther tokens.

## Common gotchas / interview framings
- **No position embeddings:** ALiBi requires no positional embeddings, reducing parameter count. Purely data-driven position information comes from attention structure itself.
- **Extreme extrapolation:** ALiBi-trained models generalize to sequences 10-100x longer than training (2K -> 32K) better than sinusoidal PE or even early RoPE.
- **Linear bias assumption:** Assumes the relationship between distance and attention should be linear. May be suboptimal for tasks requiring non-linear distance effects.
- **Head specialization:** Different heads learn different biases; they specialize in attending to different distance ranges.
- **Non-learnable variant:** Fixed slopes (e.g., h_i = -1/2^i) also work well, matching learned versions.

## See also
- [[rotary-position-embeddings-rope]]
- [[absolute-positional-encodings-sinusoidal]]

## Sources
See frontmatter `sources:`.

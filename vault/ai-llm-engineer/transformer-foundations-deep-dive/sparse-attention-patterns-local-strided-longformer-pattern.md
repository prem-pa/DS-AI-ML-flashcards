---
id: f58678cb-8e9c-4483-b43d-6258d03bfbbd
title: Sparse attention patterns (local, strided, longformer pattern)
track: ai-llm-engineer
topic: transformer-foundations-deep-dive
difficulty: 5
tags:
- sparse-attention
- long-range
- efficiency
- linear-attention
- pattern-design
aliases:
- sparse attention
- local attention
- strided attention
sources:
- url: https://arxiv.org/abs/2004.08049
  label: 'Longformer: The Long-Document Transformer'
- url: https://arxiv.org/abs/2012.14556
  label: 'Big Bird: Transformers for Longer Sequences'
- url: https://www.deepfa.ir/en/blog/sparse-attention-patterns/
  label: 'DeepFA: Sparse Attention Patterns'
cards:
- id: 705d780e-b5a1-41a4-a2dd-5814467953f4
  type: flip
  front: Describe the local (sliding-window) sparse attention pattern and its complexity.
  back: 'In local attention, each position i attends only to positions in a fixed-size window [i-w, i+w]
    for window size w. This reduces attention from O(n^2) to O(n*w). If w is constant, the complexity
    is O(n). Downside: information is local; positions far apart cannot directly attend, limiting receptive
    field.'
- id: 3dc9687d-64b5-4be0-abd6-fa200ad7d465
  type: flip
  front: How does the Longformer pattern combine local and strided attention?
  back: 'Longformer uses a hybrid: (1) local attention within a sliding window of size w around each position,
    (2) strided attention to select global positions at fixed intervals that can attend across the full
    sequence. This balances local context (via window) with sparse long-range connections (via striding),
    maintaining O(n) complexity while improving receptive field over pure local attention.'
- id: 4c2e22df-687e-4ee8-b975-1a1581eb825e
  type: mcq
  front: What is a key limitation of pure local (sliding-window) sparse attention?
  back: 'Correct: (b). Local attention achieves O(n) complexity but tokens outside the window size have
    zero direct attention. Information beyond distance w must propagate via layer-by-layer hops, increasing
    effective depth needed for long-range dependencies. (a) is opposite—sparse saves memory. (c) is possible
    but not inherent. (d) is false—multi-head works fine with sparse patterns.'
  choices:
  - key: a
    text: It requires more GPU memory than full attention
    correct: false
  - key: b
    text: Tokens at distance >w cannot directly attend to each other, limiting receptive field and long-range
      reasoning
    correct: true
  - key: c
    text: It breaks gradient flow during backpropagation
    correct: false
  - key: d
    text: It is incompatible with multi-head attention
    correct: false
- id: daefd759-a46e-4293-9c2c-bcec5da9f70f
  type: flip
  front: How does strided attention achieve logarithmic connection distance in transformers?
  back: Strided attention has position i attend to i-s, i-2s, i-4s, i-8s, etc., creating a logarithmic
    spacing. Any two positions u and v at distance |u-v| can reach each other in O(log n) hops through
    this geometric series of distances. This balances O(n) complexity per layer with reasonably short
    effective paths for information flow.
---

## Intuition
Full O(n^2) attention is prohibitive for long sequences. Sparse patterns restrict which positions attend to which, reducing complexity to O(n) or O(n log n). Common patterns: local (each token attends to nearby tokens), strided (skip every k tokens), and dilated (exponential distances). Patterns can be mixed to balance local and global context.

## Detail
Given sequence length n, full attention has n^2 edges. Sparse patterns prune this:
- **Local (sliding window):** Each position i attends to positions [i-w, i+w]. Complexity O(n*w), often O(n) for fixed w.
- **Strided:** Each position i attends to i, i-s, i-2s, i-3s, ... for stride s. Captures long-range with O(n*log(n)) distance to any position.
- **Dilated:** Exponentially spaced positions [i, i-1, i-2, i-4, i-8, ...]. Logarithmic path lengths.
- **Longformer pattern:** Combines local window + strided pattern. Attend locally to [i-w, i+w] plus strided global positions.
Tradeoff: O(n) or O(n log n) complexity but reduced receptive field—some information becomes unreachable.

## Common gotchas / interview framings
- **Information loss:** Sparse patterns can sever paths between distant tokens. Receptive field shrinks, potentially hurting performance on tasks requiring true long-range reasoning.
- **Gradient flow:** Gradients may not backprop efficiently through sparsely-attended paths.
- **Position-dependent:** Local window at boundary positions may be one-sided; must handle carefully.
- **Pattern design matters:** Poorly chosen patterns (e.g., pure strided) can create disconnected components.
- **Not always faster:** For moderate sequence lengths, sparse attention overhead (indexing, branching) may exceed full attention speedup.

## See also
- [[scaled-dot-product-attention-derivation]]
- [[efficient-attention-flashattention-flashattention-2-flashattention-3]]
- [[ring-attention-and-paged-attention-for-long-context]]

## Sources
See frontmatter `sources:`.

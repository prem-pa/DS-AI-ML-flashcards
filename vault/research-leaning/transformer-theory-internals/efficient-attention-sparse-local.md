---
id: a67d7325-309e-4f32-9e31-e55e78d149f2
title: Efficient attention (sparse, local)
track: research-leaning
topic: transformer-theory-internals
difficulty: 5
tags:
- sparse-attention
- local-attention
- approximation-bounds
- computational-efficiency
- long-range-dependencies
- attention-pattern
aliases:
- sparse-transformer
- local-attention-patterns
- approximation-error-bounds
sources:
- url: https://arxiv.org/abs/1904.10509
  label: 'Child et al. (2019): Generating Long Sequences with Sparse Transformers'
- url: https://arxiv.org/abs/2002.05202
  label: 'Zaheer et al. (2020): Big Bird - Transformers for Longer Sequences'
- url: https://arxiv.org/abs/2004.08249
  label: 'Beltagy et al. (2020): Longformer - The Long-Document Transformer'
- url: https://arxiv.org/abs/2409.13504
  label: 'Anil et al. (2024): Mamba-2 - Enhancements and Architectural Improvements'
cards:
- id: db7bbd3d-97b2-4c2b-b8e2-65237ca78fb6
  type: flip
  front: How does stacking layers with local attention enable long-range dependencies despite each layer
    attending only locally?
  back: Each layer's output feeds into the next layer's input. A token at position $i$ attending to a
    local window $[i-w, i+w]$ in layer 1 sends information to these neighbors. In layer 2, those neighbors
    attend to *their* local windows, propagating information further. After $d$ layers, information from
    position $i$ can reach positions up to distance $d \cdot w$. For $d = \log(n/w)$ layers, this reaches
    distance $\Omega(n)$. Long-range dependencies are preserved via *composition* of local operations.
- id: 6df5ec70-fc18-4a74-965d-3b8368f27c7d
  type: mcq
  front: Big Bird's sparse attention pattern (local + global + random) is proven to be Turing complete.
    What is the significance of this result?
  back: Turing completeness (see [[universal-approximation-of-transformers]]) means the model can, in
    principle, compute any function. Big Bird's result shows that even with sparse attention, this property
    is preserved. This is important because it separates expressiveness (can compute any function) from
    efficiency (how fast to compute it). Sparse attention trades off efficiency for (nearly) no loss in
    expressiveness, unlike linear attention which may sacrifice both.
  choices:
  - key: a
    text: It means sparse attention is as expressive as full attention
    correct: false
  - key: b
    text: It proves that sparse patterns preserve computational expressiveness despite reducing complexity
      from O(n^2) to O(nw), so sparsity is an approximation, not a fundamental loss of power
    correct: true
  - key: c
    text: It shows that global attention is unnecessary if local + random patterns are used
    correct: false
  - key: d
    text: It guarantees that sparse attention achieves zero approximation error
    correct: false
- id: b740be41-2968-4784-8a7b-b5d0943b304f
  type: flip
  front: What is the approximation error of local attention with window size $w$ compared to full softmax,
    and how does it depend on $n$?
  back: For functions with *local structure* (e.g., smooth functions or autoregressive tasks), local attention
    with $w = O(\sqrt{n})$ approximates full softmax to error $\epsilon$ that scales as $O(\epsilon)$
    in the smoothness. The key insight is that many natural language tasks have local structure—token
    predictions depend primarily on nearby context. The approximation bound is task-dependent and depends
    on the Lipschitz constant and decay rate of attention patterns. For some tasks, $w = O(1)$ suffices;
    for others, $w = \Theta(n)$ is needed.
- id: f6a630f2-6439-40f6-ba8f-35ef9f278ffb
  type: flip
  front: How does Longformer's combination of local and global attention work, and why is global attention
    necessary?
  back: 'Longformer uses *hybrid* attention: some positions (e.g., special tokens like [CLS]) attend globally
    to all positions and are attended to globally; others use local attention. Global positions act as
    ''hubs'' that can collect and broadcast information across the sequence. This ensures that rare but
    important patterns (requiring cross-document reasoning) are preserved while most computation stays
    local. Without global positions, information flow would be bottlenecked by the local-window constraint.'
---

## Intuition

Full softmax attention requires $O(n^2)$ comparisons—each token attends to all others. For long sequences, this is prohibitive. Sparse and local attention restrict the attention pattern: instead of attending to all $n$ tokens, each token attends to only $k \ll n$ neighbors. This reduces complexity to $O(nk)$ while preserving long-range reasoning via *stacking*: information flows across layers, so distant tokens can influence each other indirectly.

## Detail

**Local attention**:
Each query position $i$ attends only to keys in a local window $[i-w, i+w]$, where $w$ is the window size. This gives complexity $O(nw)$. For fixed $w$ (not scaling with $n$), this is linear in sequence length.

**Sparse attention patterns**:
Instead of a fixed window, attention is restricted to a *pattern* of positions—e.g., strided patterns, block-diagonal patterns, or learned patterns. Examples:
- **Longformer** (Beltagy et al., 2020): Local + global attention (some positions attend globally, others locally)
- **Big Bird** (Zaheer et al., 2020): Local + global + random, with theoretical guarantees
- **Reformer**: Locality-sensitive hashing to find similar keys efficiently

**Approximation theory**:
How much do we lose by restricting attention? Child et al. (2019) provide analysis:
- For functions with *local structure* (e.g., autoregressive language modeling), local attention with window size $w = O(\sqrt{n})$ can approximate full softmax to error $\epsilon$ using $O(nw)$ computation
- Long-range dependencies are handled via stacking: a token at position $i$ can affect position $j > i + w$ by flowing through $\log(n/w)$ layers

**Key insight** (Zaheer et al., 2020):
Big Bird proves that a sparse attention pattern with the following structure is *Turing complete*:
- Local attention (each token attends to $w$ neighbors)
- Global attention (a few special tokens attend to all positions)
- Random attention (random sparse connections)

This is remarkable: sparsity does not break computational expressiveness, only efficiency.

**Information flow**:
- Within a layer, information flows locally (window size $w$)
- Across layers, information reaches distant positions: a token at position 1 can influence position $n$ in $\log(n/w)$ layers
- This is faster than RNNs (which need $O(n)$ recurrent steps) but less direct than full attention

## Common gotchas / interview framings

- **"Doesn't sparse attention lose long-range dependencies?"** No, if you stack layers. A position near the start can influence far-away positions indirectly by flowing through intermediate layers. The depth required is $\log(n/w)$, which is small for modest $w$
- **"Why not just use linear attention? It's also fast."** Linear attention sacrifices expressiveness—no Turing completeness without additional mechanisms. Sparse attention preserves softmax's expressiveness (Big Bird proof) but at lower cost
- **"How do you choose the sparse pattern?"** This is an open problem. Fixed patterns (local + global) work well in practice (Longformer, Big Bird). Learned patterns are harder to train but potentially more task-adaptive. Reformer's learned patterns via LSH show promise
- **"What is the error bound for local attention approximating full softmax?"** For smooth functions, local attention with window $w = O(\sqrt{n})$ achieves $\epsilon$-approximation error. The bound depends on the function's Lipschitz constant and local structure. This is task-dependent
- **"Does sparse attention hurt in-context learning?"** Potentially, if task-defining tokens are far apart. But local attention with a sufficiently large window ($w \sim $ context length) preserves in-context learning. See [[attention-head-specialization]] for empirical studies

## See also
- [[linear-attention-and-kernel-methods]]
- [[attention-as-kernel-method]]
- [[theoretical-analysis-of-softmax-bottleneck]]
- [[gradient-flow-in-deep-transformers]]
- [[universal-approximation-of-transformers]]

## Sources
See frontmatter `sources:`.

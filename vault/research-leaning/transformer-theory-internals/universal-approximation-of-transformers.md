---
id: 196894ea-4530-43e9-8509-4101c44cf14a
title: Universal approximation of transformers
track: research-leaning
topic: transformer-theory-internals
difficulty: 5
tags:
- expressiveness
- theoretical-foundations
- function-approximation
- attention-mechanism
- universal-approximation
- capacity
aliases:
- transformer-expressivity
- universal-function-approximation
- attention-completeness
sources:
- url: https://arxiv.org/abs/1910.01108
  label: 'Pérez et al. (2019): On the Turing Completeness of Transformers'
- url: https://arxiv.org/abs/2207.00297
  label: 'Phuong & Hutter (2022): Formal Limitations on the Representation Capacity of RNNs'
- url: https://transformer-circuits.pub/
  label: Anthropic Transformer Circuits Research Hub
cards:
- id: 9d2f08fc-5b19-4c09-98ff-7070fa815474
  type: flip
  front: Why are transformers Turing complete while RNNs with bounded hidden state are not?
  back: 'RNNs have fixed-size state memory, limiting addressable configurations. Transformers can use
    arbitrarily long sequences as external memory and attend to any position, providing unbounded addressability.
    This dynamic redirection via attention—not just the mechanism itself—is key: attention allows selecting
    arbitrary subsets of the input, effectively simulating arbitrary memory access patterns required by
    a Turing machine.'
- id: 3010adf7-cdc4-4082-a56d-1420bf805b49
  type: mcq
  front: In the proof of transformer Turing completeness, what role does positional encoding play?
  back: Positional encodings act as memory indices. Without them, the attention mechanism cannot distinguish
    between different positions—all tokens appear identical to the Q/K mechanism. PE enables position-specific
    retrieval, which is essential for a Turing machine to access stored data at specific 'addresses' in
    the sequence.
  choices:
  - key: a
    text: It stabilizes gradient flow during training
    correct: false
  - key: b
    text: It enables the model to index past positions, providing 'pointers' for attention to reference
      stored values
    correct: true
  - key: c
    text: It reduces the rank of the softmax output
    correct: false
  - key: d
    text: It increases the expressiveness of MLPs
    correct: false
- id: bbe83a01-7aed-4506-b40f-576b17db1362
  type: flip
  front: Why doesn't Turing completeness guarantee that a transformer will learn a given function from
    data?
  back: Turing completeness is a worst-case expressiveness bound. Learning requires (1) sufficient training
    data, (2) the function to be in the inductive bias of SGD, and (3) implicit regularization to generalize.
    A function may be theoretically expressible but exponentially hard to learn, or the SGD trajectory
    may not reach the correct solution due to saddle points or lack of regularization signal. See [[implicit-regularization-in-transformers]].
- id: 7ee99831-a012-469c-b898-cd2cf39264d2
  type: flip
  front: What is the relationship between softmax output rank and universal approximation?
  back: 'Softmax output is rank-limited (at most rank-k if k tokens), which constrains *intermediate representations*
    (see [[theoretical-analysis-of-softmax-bottleneck]]). However, universality is preserved because:
    (1) the output of one layer feeds into the next, allowing re-expansion, (2) MLPs apply nonlinearity
    after attention, and (3) depth allows composing rank-limited operations. The bottleneck affects *efficiency*
    and *learnability*, not expressiveness.'
---

## Intuition

Transformers can express *any* computable function, but what does this mean for real networks?

The key insight is that attention—specifically the ability to condition on arbitrary patterns in the input—provides *Turing completeness*. Unlike RNNs with fixed state size, a transformer can allocate arbitrary amounts of memory (via longer sequences) and dynamically attend to relevant tokens, giving it the computational flexibility to simulate arbitrary programs.

## Detail

Pérez et al. (2019) proved that transformers (with softmax attention, MLPs, and positional encodings) are Turing complete: given sufficient layers and a decoder, they can simulate a Turing machine. The proof hinges on:

1. **Attention as selection**: $\text{softmax}(QK^T/\sqrt{d})V$ selects subsets of the value space with fine-grained precision
2. **Positional encoding as memory**: PE enables the model to index past positions, providing "pointers" into the sequence
3. **Nonlinearity in MLPs**: $\text{ReLU}$ and softmax allow arbitrary function approximation within layers

However, *practical* implications are subtle:
- Depth requirements may be exponential in input length
- The bound doesn't guarantee learnability—a function may be expressible but not learnable by gradient descent
- Implicit regularization (see [[implicit-regularization-in-transformers]]) means the SGD trajectory constrains which functions are actually learned

Recent mechanistic interpretability work (Anthropic's Circuits research) shows that *real* transformers solve tasks with surprisingly sparse, low-rank mechanisms—far from the theoretical worst case.

## Common gotchas / interview framings

- **"But are they really universal approximators if we need exponential depth?"** Yes, Turing completeness is worst-case. The question then becomes: *why do transformers work well in practice?* Answer: implicit regularization + architectural inductive biases
- **"What about softmax rank limitations?"** See [[theoretical-analysis-of-softmax-bottleneck]]. Softmax output is rank-limited, but this doesn't break universality—it just constrains intermediate representations
- **"How does this relate to generalization?"** Universal approximation alone says nothing about generalization. The generalization story involves implicit regularization and feature learning, not expressiveness

## See also
- [[attention-as-kernel-method]]
- [[implicit-regularization-in-transformers]]
- [[theoretical-analysis-of-softmax-bottleneck]]
- [[linear-attention-and-kernel-methods]]
- [[gradient-flow-in-deep-transformers]]

## Sources
See frontmatter `sources:`.

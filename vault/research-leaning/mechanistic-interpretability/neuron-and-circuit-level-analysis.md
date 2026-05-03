---
id: c7434edf-d74a-4c43-a841-50166f36fd05
title: Neuron and circuit level analysis
track: research-leaning
topic: mechanistic-interpretability
difficulty: 5
tags:
- circuits
- neurons
- mechanistic-explanation
- polysemy
- features
- interpretability
aliases:
- circuit analysis
- neuron interpretation
- feature circuits
sources:
- url: https://transformer-circuits.pub/
  label: Transformer Circuits Thread
- url: https://transformer-circuits.pub/2024/july-update/index.html
  label: Circuits Updates - July 2024
- url: https://arxiv.org/abs/2510.02917
  label: MECHANISTIC INTERPRETABILITY (ICLR 2026)
cards:
- id: e8110c91-ff5b-447f-a607-5ef25e978d7e
  type: flip
  front: What does it mean that neurons are polysemantic, and why does this complicate neuron-level interpretability?
  back: "Polysemanticity: A single neuron participates in multiple, seemingly unrelated computations.\n\
    \nExample: MLP neuron 42 activates on inputs about (scientific papers, mathematical proofs, research\
    \ conferences). \n\nComplication: You can't say \"neuron 42 computes concept X.\" Instead, it's a\
    \ mixture. The neuron's activation is entangled with multiple features. Analyzing it in isolation\
    \ gives incomplete/misleading interpretation.\n\nContext matters: (input=paper on neuroscience, neuron\
    \ 42 activation) ≠ (input=poetry, neuron 42 activation). Same neuron, different roles."
- id: 76c3d5f5-d505-4019-b540-1b6c66b3b5b0
  type: mcq
  front: In circuit-level analysis, what is the goal of tracing data flow through attention heads and
    MLPs?
  back: 'Circuit analysis reveals composition: neuron A → head B → neuron C → head D implement behavior
    X. This shows how components cooperate, revealing the mechanism behind a behavior.'
  choices:
  - key: a
    text: To measure the total number of neurons used in a computation
    correct: false
  - key: b
    text: To identify which collections of components (heads, neurons) work together to implement a specific
      behavior or concept
    correct: true
  - key: c
    text: To compute the gradient of the output with respect to each neuron
    correct: false
  - key: d
    text: To optimize model weights for better performance on a task
    correct: false
- id: cd4ad8bc-e5c8-48fa-a6b7-91a9d56a568f
  type: flip
  front: How do sparse autoencoders enable cleaner circuit analysis compared to analyzing raw neurons?
  back: 'Raw neurons are polysemantic (mixed representations). SAEs extract monosemantic features:

    - Train SAE to decode neuron activations → sparse, interpretable features

    - Each feature is **monosemantic**: activates on one semantic concept ("Golden Gate Bridge," "pronoun
    resolution," "scientific writing")

    - Circuit becomes clear: Feature X → Head A → Feature Y → output

    - No more polysemy confusion; each element of the circuit has clear meaning


    Result: Circuits using SAE features are 10x more interpretable than circuits using raw neurons.'
- id: 0643d360-ef01-4d2c-b46e-8e192bfd3e73
  type: flip
  front: What is the difference between observing that a neuron activates on topic X and claiming that
    the neuron 'computes' topic X detection?
  back: '**Observation**: Neuron 42 has high activation when input is about topic X. (Correlation)


    **Claim to verify**: Neuron 42 is causally necessary for detecting topic X. (Causation)


    The gap: High activation ≠ causal role. Neuron 42 may activate strongly but be redundant—other neurons
    may compute the same detection. Or neuron 42 may be a side effect (activated by X, but not the detector).


    Verification: Use activation patching. Ablate neuron 42 and observe if topic X detection breaks. If
    yes, it''s causal. If no, it''s redundant or incidental.'
---

## Intuition

Neural networks are built from components at different scales:
- **Neuron level**: Individual units in an MLP or embedding
- **Attention head level**: Individual heads (analyzed via QK/OV)
- **Circuit level**: Collections of neurons/heads working together to implement a computation (e.g., "copy mechanism," "pronoun resolution")

Neuron-level analysis asks: "What does this neuron compute?" Circuits ask: "How do neurons cooperate to compute X?"

Example: A pronoun resolution circuit might involve:
- Attention head: attend to subject
- MLP neuron: check if attended token is animate
- Another head: attend to matching gendered noun
- Result: resolve "she" → female protagonist

## Detail

**Neuron analysis**:
- Neurons are polysemantic: each neuron participates in multiple computations (e.g., "neuron 42 activates on both 'scientific papers' and 'mathematical proofs'")
- Find most activating inputs: brute-force search inputs that maximally activate a neuron
- Gradient-based interpretation: backprop through neuron to see which input dimensions affect it
- Limitation: Polysemy makes interpretation hard—neuron 42's behavior is a confused mixture

**Circuit analysis**:
- Moves beyond individual neurons to *compositions* of components
- Identify that neurons 42, 105, 230 (across layers) together implement "concept X"
- Use attention patterns to trace data flow: Layer 3 Head 5 → Layer 5 MLP Neuron 12 → Layer 6 Head 2
- Example: Transformer Circuits Thread discovered "copy suppression heads" that prevent premature copying and "negative attention heads" that attend to non-matches

**Sparse autoencoders solve polysemy**:
- Replace neurons with SAE features: each feature activates on ONE semantic concept
- Circuit analysis becomes cleaner: "Feature X (Golden Gate Bridge concept) flows through heads A,B,C"
- Modern work (Anthropic 2025): Attribution graphs using SAE features instead of neurons

## Common gotchas / interview framings

- **Polysemy is fundamental**: Single neurons can't be understood in isolation. Context matters. Always look at (input, neuron activation) pairs, not neurons alone.
- **Circuits are not modular**: Circuits can overlap and interact. "The pronoun resolution circuit" overlaps with "grammatical role circuit." Boundaries are fuzzy.
- **Causality vs. correlation**: A neuron activates on X and correlates with task Y, but is it *causing* Y? Use patching.
- **Generalization**: A circuit discovered in one model may not exist in another. Interpretability findings may not transfer.
- **Interview angle**: "How would you identify and characterize the circuit responsible for pronoun resolution in a transformer?" → Attention analysis + neuron identification + causal patching.

## See also
- [[sparse-autoencoders]]
- [[monosemanticity]]
- [[transformer-circuits]]
- [[attention-circuits]]
- [[mlps]]
- [[superposition]]
- [[feature-visualization]]

## Sources
See frontmatter `sources:`.

---
id: a321e5f2-e2ff-4a73-a62c-c33772eda70e
title: Layer-wise relevance propagation
track: research-leaning
topic: mechanistic-interpretability
difficulty: 5
tags:
- attribution
- relevance
- backpropagation
- contribution-decomposition
- interpretability
- mechanistic
aliases:
- LRP
- relevance backpropagation
- layer-wise decomposition
sources:
- url: https://transformer-circuits.pub/
  label: Transformer Circuits Thread
- url: https://www.anthropic.com/research/team/interpretability
  label: Anthropic Interpretability Research
- url: https://leonardbereska.github.io/blog/2024/mechinterpreview/
  label: Mechanistic Interpretability for AI Safety — A Review
cards:
- id: 7386d017-6fe4-4a64-98ae-09dcaa8a687b
  type: flip
  front: What does layer-wise relevance propagation do, and how does it differ from gradient-based saliency
    methods?
  back: 'LRP decomposes a model''s prediction into contributions from each component (neurons, heads,
    features) at each layer by backpropagating relevance from output to input.


    Difference from gradients:

    - Gradients show "how much does the output change if I perturb this input?" (local sensitivity)

    - LRP shows "how much of the final prediction is explained by this component?" (global contribution)


    Gradients can be noisy; LRP enforces conservation (relevance is preserved, not scattered).'
- id: 43b1cf28-eb9c-4ee8-83b1-240aa633a82b
  type: mcq
  front: In layer-wise relevance propagation, what does the conservation principle mean?
  back: Conservation ensures that when you propagate relevance backward, what comes out of layer L equals
    what goes into layer L-1. This prevents double-counting and ensures the decomposition is mathematically
    sound.
  choices:
  - key: a
    text: Relevance scores must sum to 1 globally across all components
    correct: false
  - key: b
    text: Relevance at layer L must sum to the total relevance at layer L-1, ensuring no attribution is
      double-counted
    correct: true
  - key: c
    text: Relevance must be positive (no negative contributions)
    correct: false
  - key: d
    text: Relevance propagation must halt at embedding layers
    correct: false
- id: 8b2ead39-7c8c-4175-ac8a-9bac29d206eb
  type: flip
  front: How would you combine layer-wise relevance propagation with sparse autoencoders to improve interpretability
    in transformers?
  back: 'Standard LRP attributes to polysemantic neurons (each neuron has multiple roles). With sparse
    autoencoders:


    1. Train SAEs to decode neuron activations into monosemantic features (one feature ≈ one concept)

    2. Apply LRP through the SAE layer: "Which features does the model rely on?"

    3. Propagate further back: "Which neurons activate those features?"


    Result: Attributions are to human-readable features ("neuroscience concept," "pronoun") rather than
    opaque neurons. Combines interpretability of features with principled attribution.'
- id: 4cb1a9a2-67bc-4538-aeca-29747747bdc0
  type: flip
  front: What's a key limitation of LRP in highly nonlinear models like transformers?
  back: 'LRP assumes contributions are roughly linear: Δoutput ≈ Σ(weight_i × Δfeature_i). In transformers
    with ReLU/softmax, this breaks down.


    Example: Two features (A, B) may have zero individual relevance (both contribute 0%) but high joint
    relevance when both are present (A ∧ B contributes 50%). LRP can''t capture these interactions well.


    Mitigation: Use LRP for layer-to-layer attribution (more linear) but validate with activation patching
    (causal ground truth).'
---

## Intuition

Layer-wise relevance propagation (LRP) answers: "How much did this specific neuron/attention head/MLP in layer 3 contribute to the final prediction?"

Imaginea prediction is built bottom-up: embeddings → layer 1 → layer 2 → ... → output. LRP propagates relevance backward from output to input, attributing the model's decision to individual components at each layer. It decomposes "the model predicted token X" into "layer 1 contributed 5%, layer 2 contributed 3%, attention head 2.5 contributed 8%" etc.

## Detail

LRP uses the chain rule to decompose predictions:
1. Start with output relevance (e.g., logit for token "Paris" = 8.5)
2. Propagate backward: How much of this relevance came from each neuron in the previous layer?
3. Continue layer-by-layer to input
4. Result: attribution score for each component showing its contribution to the final output

Key principle: Conservation. Relevance at layer L sums to relevance at layer L-1, ensuring no contribution is double-counted.

In mechanistic interpretability context: LRP can be applied to sparse autoencoder features instead of neurons, attributing predictions to interpretable features across layers. Combined with activation patching, it shows both *how much* each feature contributes (LRP) and *whether* it's causally necessary (patching).

## Common gotchas / interview framings

- **Linearity assumption**: Standard LRP assumes components contribute linearly. In transformers with nonlinearities, this is approximate. Better for final softmax than for ReLU interactions.
- **Feature vs. token attribution**: LRP can attribute to features or tokens. Token-level attribution shows "which tokens mattered for the prediction"—useful for rationale extraction but may hide internal feature interactions.
- **Relative vs. absolute relevance**: A component may have high relevance (8%) but low absolute impact if most predictions don't use it (rare feature). Must consider base rates.
- **Interview angle**: "How would you decompose a model's prediction into per-head contributions?" → LRP backpropagation or attention-weighted feature contributions.

## See also
- [[activation-patching]]
- [[saliency-maps]]
- [[attention-visualization]]
- [[circuit-analysis]]
- [[feature-importance]]
- [[gradient-based-interpretation]]

## Sources
See frontmatter `sources:`.

---
id: 4a0294a3-ef95-415b-ad6f-7657a84d4dc3
title: Superposition and interpretability crisis
track: research-leaning
topic: scaling-laws-emergent-abilities
difficulty: 5
tags:
- interpretability
- superposition
- feature-decomposition
- mechanistic-interpretability
- scaling
aliases:
- feature superposition
- distributed representations
- interpretability at scale
sources:
- url: https://arxiv.org/abs/2210.08378
  label: 'Anthropic''s Superposition Hypothesis: Features, Polysemanticity, and Model Scaling'
- url: https://www.anthropic.com/news/scaling-laws-interpretability
  label: Scaling Laws for Superposition (Anthropic, 2023)
cards:
- id: 7ea874ce-ca73-4e32-a8d8-26279cd5fb58
  type: flip
  front: Define superposition in the context of neural network representations.
  back: Superposition is the encoding of multiple features in shared, overlapping neural dimensions. Rather
    than one neuron = one feature, many features are weakly encoded across multiple neurons/dimensions
    in different subspaces, enabling efficient compression.
- id: caf88229-a72b-42e4-a3c3-adc8d35c94dc
  type: mcq
  front: Why does superposition arise naturally in large language models?
  back: 'Models must represent 100x more features than neurons to be efficient. Sparse feature activation
    allows reuse of dimensions: different features activate on different tokens, packing multiple sparse
    features into the same neural subspace.'
  choices:
  - key: a
    text: Models are trained to be interpretable by design.
    correct: false
  - key: b
    text: Models have far more features (concepts) than neurons, so they reuse dimensions for sparse features
      to pack more capability into fixed parameter budgets.
    correct: true
  - key: c
    text: Superposition is a training artifact caused by poor optimization.
    correct: false
  - key: d
    text: Large models deliberately hide features to prevent adversarial attacks.
    correct: false
- id: 7b7d7997-8272-440d-b393-cd9367294b1a
  type: flip
  front: What is polysemanticity and why is it a challenge for mechanistic interpretability?
  back: Polysemanticity is when a single neuron/dimension activates for multiple, seemingly unrelated
    features (e.g., both 'Great Britain' and 'Bill Gates'). It makes feature decomposition and neuron-level
    interpretation difficult; simple inspection of neuron weights reveals nothing about the features it
    encodes.
- id: 88ea4dff-72e6-45e6-8ea5-ba10fdfa5438
  type: mcq
  front: As models scale (more parameters, more data), how does superposition change, and what is the
    consequence for interpretability?
  back: 'Scaling laws for superposition show that as models grow, feature packing increases (higher feature
    density per neuron). This is efficient for capability but creates an interpretability crisis: understanding
    large models requires tools like SAEs (Sparse Autoencoders) to disentangle superposition.'
  choices:
  - key: a
    text: Superposition decreases; interpretability improves.
    correct: false
  - key: b
    text: Superposition increases; features become more densely packed, making disentanglement harder
      and interpretability worse.
    correct: true
  - key: c
    text: Superposition stays constant; scaling doesn't affect feature encoding.
    correct: false
  - key: d
    text: Superposition decreases; models become less efficient.
    correct: false
---

## Intuition

Naive interpretability assumes each neuron represents a single feature (e.g., "detector for face edges" in vision). In reality, large language models achieve efficiency through **superposition**: neurons encode multiple features simultaneously, each weakly and in different subspaces. As models scale, superposition increases—features become harder to disentangle. This is the 'interpretability crisis': we lose the ability to explain which features drive outputs.

## Detail

Superposition arises because:

- **Dimensionality mismatch:** Models have ~100x more features (concepts) than neurons/dimensions. Encoding each feature separately would require massive overparameterization
- **Sparsity exploitation:** Most features are inactive on any given token (sparse feature activation). By reusing dimensions for sparse features, models pack more features into fewer neurons
- **Polysemanticity:** A single neuron activates for multiple, seemingly unrelated features (e.g., "Great Britain" and "Bill Gates" activate the same neuron). This is not a bug; it's efficient compression
- **Scaling relationship:** As models grow, superposition increases (higher feature density per neuron). Scaling laws show the interpretability gap widens—feature decomposition becomes exponentially harder

## Common gotchas / interview framings

- **"Can't we just do PCA or factorize hidden states to extract features?"** → Possible on toy models (≤1B params). Large LLMs require adversarial examples or complex decomposition to identify features; polysemantic neurons resist linear decomposition
- **"Is polysemanticity a fundamental property or a training artifact?"** → Likely fundamental. Models with superposition are compute-efficient. Eliminating polysemanticity would require either: (a) expanding network width massively or (b) accepting computation overhead. Emergent from optimization
- **"Does superposition explain why scaling laws work?"** → Partially. Superposition allows models to scale to large feature spaces without proportional parameter growth. But it creates an interpretability-capability tradeoff: as models become more capable, they become less interpretable
- **"Can mechanistic interpretability scale?"** → Challenging research question. Anthropic's SAE (Sparse Autoencoders) aim to disentangle superposition at scale, but computational cost scales with model size. Interpretability-at-scale remains an open problem

## See also
- [[neural-network-representations]]
- [[feature-learning]]
- [[polysemanticity]]
- [[attention-heads]]
- [[scaling-laws]]
- [[mechanistic-understanding]]

## Sources
See frontmatter `sources:`.

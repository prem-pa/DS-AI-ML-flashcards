---
id: c2183663-e821-44bf-bcbe-194eca103bce
title: SAE (Sparse Autoencoders) and superposition disentanglement
track: research-leaning
topic: mechanistic-interpretability
difficulty: 5
tags:
- superposition
- dictionary-learning
- monosemanticity
- features
- autoencoders
- interpretability
aliases:
- sparse autoencoder
- SAE
- feature extraction
- monosemantic features
sources:
- url: https://transformer-circuits.pub/2024/scaling-monosemanticity/
  label: 'Scaling Monosemanticity: Extracting Interpretable Features from Claude 3 Sonnet'
- url: https://arxiv.org/abs/2309.08600
  label: Sparse Autoencoders Find Highly Interpretable Features in Language Models
- url: https://www.lesswrong.com/posts/8YnHuN55XJTDwGPMr/a-gentle-introduction-to-sparse-autoencoders
  label: A Gentle Introduction to Sparse Autoencoders
- url: https://transformer-circuits.pub/2023/monosemantic-features
  label: 'Towards Monosemanticity: Decomposing Language Models With Dictionary Learning'
cards:
- id: 500a7332-fae2-4b6c-a162-f6df10e29c8a
  type: flip
  front: What is superposition in neural networks, and why do models represent features in superposition
    rather than using individual neurons for individual features?
  back: 'Superposition: Models encode M features (where M >> N neurons) by representing each feature as
    a direction in activation space, with neurons being linear combinations of features.


    Why: Efficiency and capacity. A model with N neurons can represent N^2 or more features if features
    are sparse (only a few active per input). This exceeds the redundancy that would come from one-neuron-per-feature.


    Trade-off: Compression gain, but interpretability loss. Neuron activations become polysemantic (mixed).'
- id: 74d89c00-a8dd-4555-8b62-3437f768379f
  type: mcq
  front: In sparse autoencoders, what role does the sparsity constraint (L0 or L1) play, and why is it
    crucial for interpretability?
  back: Sparsity ensures that on any given input, only a few features are "on." This means each feature's
    role is clear (dedicated to specific concepts). Without sparsity, many features activate simultaneously,
    and their meanings become entangled (polysemantic).
  choices:
  - key: a
    text: It reduces the number of neurons in the model to improve computational efficiency
    correct: false
  - key: b
    text: It forces only a small subset of SAE features to activate per input, ensuring features are monosemantic
      rather than polysemantic
    correct: true
  - key: c
    text: It prevents the autoencoder from overfitting to the training data
    correct: false
  - key: d
    text: It ensures the encoder output is normally distributed
    correct: false
- id: 7ca84c15-1a21-49b0-b305-26b7e09d7cb4
  type: flip
  front: How would you test whether an extracted SAE feature is truly monosemantic or is capturing a polysemantic
    mixture?
  back: '1. Rank inputs by the feature''s activation strength (highest to lowest)

    2. Show the top 10-20 most-activating inputs to humans

    3. Ask: Do these inputs describe the same concept?


    Monosemantic: All top inputs are variants of one concept (e.g., "Golden Gate Bridge," "San Francisco
    landmark," "red suspension bridge")


    Polysemantic: Top inputs span multiple unrelated concepts (e.g., "Golden Gate Bridge," "Euler''s formula,"
    "poetry about love")


    Alternative: Probe the feature with a task (e.g., "Does feature X activate on pronouns?") and measure
    precision/recall.


    Causal test: Patch SAE-reconstructed activations back into the model. If feature X is truly monosemantic,
    activating it should have predictable, focused effects (related to one concept).'
- id: ef9b7098-c1cf-491d-94ed-aaae04917e6a
  type: flip
  front: Explain the trade-off between reconstruction loss and monosemanticity in SAE training. Why is
    this trade-off unavoidable?
  back: 'Trade-off:

    - **Low sparsity (many features active)**: Features activate together, become polysemantic. But reconstruction
    is accurate (loss is low).

    - **High sparsity (few features active)**: Features are forced to be monosemantic (each covers one
    concept). But the SAE may fail to reconstruct rare polysemantic activations (loss is high).


    Why it''s unavoidable: Superposition exists for a reason—efficiency. Some inputs require activation
    of multiple concepts simultaneously. If sparsity is too strict, the SAE can''t represent these multi-concept
    states, so reconstruction fails.


    Mitigation: Choose sparsity via trade-off curve (plot loss vs. monosemanticity). Accept some reconstruction
    loss for interpretability gain. Validate that features are actually used (patching) despite reconstruction
    loss.'
- id: fca527bc-fb25-4800-bd51-e3bb0e69b7dc
  type: flip
  front: How did Anthropic scale SAE training to large models like Claude 3 Sonnet, and what did they
    learn about feature structure at scale?
  back: 'Scaling approaches:

    1. **Pruning**: Only train SAEs on token positions with sufficient gradient signal (ignore low-signal
    positions)

    2. **Regularization**: Add weight decay, batch normalization to stabilize training

    3. **Layer-wise SAEs**: Train separate SAEs for each layer (vs. one global SAE)

    4. **Feature scaling**: Adjust sparsity penalties per layer (early layers need lower sparsity)


    Key findings:

    - Claude 3 Sonnet layers contained ~100k interpretable features each

    - Features form hierarchies: early layers capture low-level patterns (text, images), later layers
    capture high-level concepts (scientific domains, reasoning patterns)

    - Multi-token features: Some features activate across multiple tokens (e.g., "scientific paper" spans
    document region)

    - 10-100x improvement in monosemanticity compared to neurons


    Result: First large-scale extraction of interpretable features from production LLMs, enabling circuit
    analysis at model scale.'
---

## Intuition

Large language models represent more features (concepts, patterns) than they have neurons. This is **superposition**: neurons encode multiple features simultaneously.

Why? Efficiency. A 7B parameter model can represent billions of features if each neuron participates in multiple feature encodings.

Problem: Superposition makes interpretation hard. A neuron's activation is a mixture; you can't say what it "means."

Solution: **Sparse Autoencoders (SAEs)**. Train an autoencoder to extract individual features from neuron activations:
- Input: neuron activations (superposed, mixed)
- Encoder: learns projections into a high-dimensional feature space
- Sparsity constraint: only a few features activate per input (interpretable, not all-at-once)
- Decoder: reconstructs original activations from sparse features
- Result: Sparse, monosemantic features representing individual concepts

## Detail

**Superposition mechanics**:
- Model has N neurons, representing M > N features (e.g., 12B features in 7B params)
- Features are stored as directions in activation space (not neurons)
- Polysemy emerges: neuron i = w_1 * feature_A + w_2 * feature_B (mixture)
- Features may be orthogonal (independent) or interfere (when co-activated)

**SAE training**:
```
Input: h = [neuron activations, 1000-dim]
Encoder: h → z (sparse, e.g., 32-dim, 2 nonzeros)
Decoder: z → h_hat
Loss: ||h - h_hat||^2 + λ||z||_0  (reconstruction + sparsity)
```

**Monosemanticity**:
- Goal: Each SAE feature activates on one semantic concept
- Test: Rank inputs by feature activation, show top 10 inputs—do they all describe the same concept?
- Example: SAE feature "Golden Gate Bridge" activates on inputs: "the Golden Gate Bridge," "SF landmark," "red suspension bridge," etc. All the same concept.
- Contrast: Neuron activates on (Golden Gate Bridge, mathematical proofs, poetry)—polysemantic confusion

**Scaling (Anthropic 2024-2025)**:
- Trained SAEs on Claude 3 Sonnet (70B params)
- Extracted ~100k interpretable features per layer
- Features were 10-100x more interpretable than neurons
- Enabled circuit analysis with monosemantic features (see Neuron and Circuit Level Analysis)

## Common gotchas / interview framings

- **Sparsity vs. compression**: SAEs are *not* dimensionality reduction (like PCA). They're sparse *expansion* (N → M > N dims) with sparsity constraint to keep activations interpretable.
- **Monosemanticity trade-off**: Higher sparsity → more monosemantic features but worse reconstruction. Lower sparsity → better reconstruction but features become polysemantic. Balance matters.
- **Feature definition is task-dependent**: A feature is "monosemantic" relative to our test. Different test sets may reveal different concept groupings.
- **Reconstruction loss ≠ faithfulness**: An SAE can reconstruct activations well but miss rare features (low activation). Patching SAE-reconstructed activations into corrupted forward pass tests if features are actually used.
- **Scaling challenges**: SAEs for large models are expensive. Anthropic's work on scaling (pruning, regularization) addresses this.
- **Interview angle**: "Explain why superposition is a problem for interpretability and how SAEs solve it" → Polysemy emerges, SAEs extract monosemantic features → enables circuit analysis.

## See also
- [[polysemanticity]]
- [[mechanistic-interpretability]]
- [[feature-visualization]]
- [[neural-representation]]
- [[transformer-circuits]]
- [[scaling-interpretability]]
- [[feature-importance]]

## Sources
See frontmatter `sources:`.

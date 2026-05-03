---
id: 5ec35ebd-aa89-4d4b-8c7b-6c5349b99aee
title: Activation patching and causal tracing
track: research-leaning
topic: mechanistic-interpretability
difficulty: 5
tags:
- causality
- attribution
- intervention
- counterfactual
- ablation
- circuits
aliases:
- causal intervention
- activation intervention
- patching
sources:
- url: https://www.neelnanda.io/mechanistic-interpretability/attribution-patching
  label: 'Attribution Patching: Activation Patching At Industrial Scale — Neel Nanda'
- url: https://arxiv.org/abs/2309.16042
  label: 'Towards Best Practices of Activation Patching in Language Models: Metrics and Methods'
- url: https://www.anthropic.com/research/team/interpretability
  label: Anthropic Interpretability Research
cards:
- id: 999c05fa-c0c8-46da-bca6-3d6207e23f1e
  type: flip
  front: What is activation patching and why is it superior to simply analyzing activation magnitudes?
  back: 'Activation patching is a causal intervention technique that replaces activations from a clean
    input into a corrupted input to test whether specific components are causally responsible for outputs.


    It''s superior to activation analysis because: high magnitude ≠ causal importance. A neuron can activate
    strongly but be redundant. Only patching (counterfactual intervention) reveals whether a component
    is *necessary* for the correct output, establishing true causality rather than correlation.'
- id: 781bc3a5-502d-44de-8a7a-fbe43b6ea581
  type: mcq
  front: In causal tracing, what is the primary goal of constructing a clean vs. corrupted input pair?
  back: The clean/corrupted pair creates a controlled comparison. By patching components from clean →
    corrupted and observing if output flips toward correct, we isolate causal responsibility.
  choices:
  - key: a
    text: To increase computational efficiency by comparing two forward passes
    correct: false
  - key: b
    text: 'To establish a counterfactual: the clean input produces the correct answer, corrupted produces
      wrong answer, allowing us to test which components are necessary for correctness'
    correct: true
  - key: c
    text: To measure the magnitude of attention weights in both conditions
    correct: false
  - key: d
    text: To identify neurons with the highest activation variance across inputs
    correct: false
- id: 6623cf2e-e6d9-40c5-9e17-96cc4722c5fd
  type: flip
  front: Explain the concept of an attribution graph in modern circuit tracing (Anthropic 2025). How does
    it differ from neuron-level activation patching?
  back: 'An attribution graph is a computational graph for a specific prompt showing:

    - Nodes: interpretable features (from sparse autoencoders, not neurons)

    - Edges: causal dependencies between features across layers


    Key difference: Traditional activation patching works with polysemantic neurons (each neuron represents
    multiple concepts). Attribution graphs use sparse autoencoders to extract monosemantic features (one
    feature ≈ one concept), revealing causality at the *concept* level rather than the neuron level. This
    is more interpretable because features are human-readable.'
- id: b6bdce3b-6976-4e1f-b8ff-cd2f77b98675
  type: flip
  front: What does it mean if patching an attention head's output does NOT flip the model's answer back
    to correct, even though the head shows high activation?
  back: 'It means the attention head is *not causally necessary* for the output, despite being active.
    This indicates:

    1. Information is redundantly encoded through other pathways

    2. The head''s output is either not used downstream or is compensated for by other components

    3. The high activation is a correlation, not a cause


    This reveals model redundancy and robustness—the model can produce correct outputs without relying
    on every active component.'
---

## Intuition

Activation patching answers a crucial question: which model components are *causally responsible* for a given output? It works like a surgical intervention—you take clean activations from one input and patch them into a corrupted input, observing whether the model's behavior flips back to correct. If it does, that component is causally important.

Think of it as replacing one instrument in an orchestra's performance and measuring whether the sound changes. Causal tracing extends this by systematically mapping which attention heads, neurons, and MLPs form causal pathways.

## Detail

Activation patching constructs a counterfactual by:
1. Running a clean input through the model (e.g., "What is the capital of France?") → correct answer
2. Running a corrupted input (e.g., "What is the capital of Germany?") → wrong answer  
3. In the corrupted run, selectively replacing activations from the clean run at specific layers/components
4. Observing if output flips toward clean answer; if yes, that component is on the causal pathway

Causal tracing builds attribution graphs showing dependencies: which features in layer N depend on which features in layer M. Modern work (Anthropic 2025) creates "replacement models" where sparse features replace polysemantic neurons, revealing which high-level concepts are causally relevant.

## Common gotchas / interview framings

- **Correlation vs. causation**: High activation ≠ causal importance. Only patching reveals causality. An attention head may activate strongly but be redundant—patching will show this.
- **Information bottlenecks**: Patching a single head may not reveal true causal role if information is preserved through other paths. Must reason about model-wide redundancy.
- **Granularity tradeoff**: Patching at layer level is coarse (many components); patching at neuron/token level is fine but computationally expensive.
- **Interview angle**: "Explain how you'd identify which attention head in a transformer is responsible for pronoun resolution" → activation patching + systematic layer ablation.

## See also
- [[causal-inference]]
- [[mechanistic-interpretability]]
- [[attention-patterns]]
- [[residual-streams]]
- [[mlp-circuits]]
- [[counterfactual-reasoning]]
- [[ablation-studies]]

## Sources
See frontmatter `sources:`.

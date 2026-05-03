---
id: 99441aac-86d8-46f6-87c5-13b48ad342ed
title: Attention head interpretability
track: research-leaning
topic: mechanistic-interpretability
difficulty: 5
tags:
- attention
- circuits
- induction-heads
- heads
- QK-circuit
- OV-circuit
aliases:
- attention analysis
- head composition
- attention patterns
sources:
- url: https://transformer-circuits.pub/2022/in-context-learning-and-induction-heads/index.html
  label: In-context Learning and Induction Heads
- url: https://transformer-circuits.pub/2025/attention-qk/index.html
  label: Tracing Attention Computation Through Feature Interactions
- url: https://transformer-circuits.pub/2021/framework/index.html
  label: A Mathematical Framework for Transformer Circuits
cards:
- id: 8a71d58d-9fa8-4a63-959a-4e0b06637ea0
  type: flip
  front: Explain the QK and OV circuit decomposition of attention heads. Why is this decomposition useful?
  back: "Each attention head's computation splits into two circuits:\n\n**QK Circuit (Query-Key)**: Produces\
    \ the attention pattern—which positions the head attends to. Implemented by query and key projections.\
    \ \n\n**OV Circuit (Output-Value)**: Given the attention pattern, what does the head output to the\
    \ residual stream? Implemented by output and value projections.\n\nUseful because: You can analyze\
    \ each circuit independently. A head may have a sophisticated attention pattern but trivial output\
    \ (or vice versa). Separating them reveals which heads do pattern matching, which copy information,\
    \ which perform transformations."
- id: e6d8a4d8-a222-4ada-a197-dfcedbbd5356
  type: mcq
  front: Induction heads enable in-context learning by recognizing and completing patterns. Which of the
    following best describes their mechanism?
  back: 'Induction heads are a circuit: the composition of two separate heads. Layer N finds pattern matches
    (token A matched to previous [A]). Layer N+1 reads the token after previous [A] (i.e., [B]) and copies
    it. Together: [A] → [B].'
  choices:
  - key: a
    text: A single attention head that attends to all previous tokens and averages them
    correct: false
  - key: b
    text: 'A pair of heads in different layers: one matches the current token to previous occurrences,
      the second attends to the next token after those occurrences and copies it'
    correct: true
  - key: c
    text: A learned dictionary that stores and retrieves token sequences
    correct: false
  - key: d
    text: A gradient-based optimization over the sequence to find similar patterns
    correct: false
- id: 34e0b820-8654-40f0-af25-2e65090df836
  type: flip
  front: Why is it wrong to use attention weight magnitudes as a measure of token importance (saliency)
    when analyzing transformer models?
  back: 'Attention weights show which positions are attended to, NOT which attended tokens affect the
    output.


    Counterexample: Head A attends strongly to token X but its OV (output-value) circuit is nearly zero—the
    head attends to X but ignores it (outputs nothing). Token X has high attention but zero importance.


    Converse: Head B attends weakly to token Y but has high OV weight on Y''s value—weak attention but
    large output contribution.


    Truth requires both: high attention weight AND high OV contribution. Better: use activation patching
    to establish causality.'
- id: bbd01615-e1d2-468d-902c-28aed19e55e5
  type: flip
  front: How would you test whether induction heads are causally responsible for in-context learning,
    beyond observing their correlation with ICL tasks?
  back: 'Use activation patching:

    1. Clean run: Model solves ICL task correctly (e.g., "A B C A → B")

    2. Corrupted run: Ablate/corrupt induction head outputs

    3. If output flips to wrong answer, induction heads are causally necessary

    4. Can patch at fine granularity: corrupt only the induction head in layer 12, measure impact


    Alternative: Training-time intervention. Train a model without induction heads (e.g., no cross-layer
    attention) and observe ICL performance drops.


    Correlation (high attention) vs. causation (patching shows necessity) is the key distinction.'
---

## Intuition

Attention heads are the core computational unit of transformers, yet they're often opaque. Attention head interpretability asks: "What does this head compute?"

Key insight: A head isn't a monolithic black box. It splits into two circuits:
- **QK (query-key) circuit**: "Which positions do I attend to?" (produces attention pattern)
- **OV (output-value) circuit**: "Given I attend to these positions, what do I output?" (produces contribution to residual stream)

Analyzing heads separately reveals which heads do pattern matching, which copy information, which do arithmetic, etc.

## Detail

**Induction heads** are a canonical example of interpretable head behavior:
- Composed of two heads in different layers working together
- Head 1 attends to previous occurrences of the current token ("pattern matching")
- Head 2 attends to the next token after those previous occurrences ("copy mechanism")
- Result: Sequence [A][B]...[A] → attend to [B], enabling pattern completion (in-context learning)

**QK circuit decomposition**:
- Query/key projections are not random; they often encode semantic information
- Can analyze: Does head attend to (name, type=person)? (grammatical role)? (rare tokens)?

**OV circuit decomposition**:
- Once attention pattern is fixed, which information does the head output?
- Analyze: Does head copy input tokens? (OV matrix is identity-like) Or do learned transformations?

**Interview-critical insight**: Probing attention weights for downstream task labels (e.g., "does attention to pronouns predict pronoun resolution?") is tempting but *wrong*. Attention is not a saliency map. High attention to a token doesn't mean it's important—OV may ignore that attended token. Use attention + causal patching together.

## Common gotchas / interview framings

- **Attention ≠ saliency**: Attention weights are NOT "importance." Attend to A, ignore. Attend to B, output crucial info. Patching shows truth.
- **Head-level vs. layer-level**: Each head has its own QK/OV. Analyzing layer-average attention patterns loses individual head interpretability.
- **Multi-head redundancy**: Similar heads exist in same layer, suggesting redundancy. Patching one may not affect output if another backs it up.
- **In-context learning correlation**: Induction heads correlate with ICL, but correlation ≠ causation. Must patch to confirm necessity.
- **Interview angle**: "Explain what induction heads do and why they matter for in-context learning" → pattern matching + copy mechanism + causal evidence.

## See also
- [[induction-heads]]
- [[qk-circuits]]
- [[ov-circuits]]
- [[in-context-learning]]
- [[transformer-circuits]]
- [[attention-mechanisms]]
- [[pronoun-resolution]]

## Sources
See frontmatter `sources:`.

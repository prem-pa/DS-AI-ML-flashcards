---
id: f814b207-4215-4fd1-a24e-b482373b8392
title: Chain-of-thought in theory
track: research-leaning
topic: recent-architectures-techniques
difficulty: 5
tags:
- chain-of-thought
- step-by-step-reasoning
- capacity-arguments
- emergent-behavior
- reasoning-tokens
- scaling-behavior
aliases:
- CoT
- step-by-step-prompting
- intermediate-steps
- reasoning-process
sources:
- url: https://arxiv.org/abs/2201.11903
  label: Chain-of-Thought Prompting Elicits Reasoning in Large Language Models
- url: https://arxiv.org/abs/2305.06204
  label: Why Does Chain-of-Thought Help with Large Language Models?
- url: https://arxiv.org/abs/2305.14992
  label: The Role of Intermediate Token Predictions in Language Model Behavior
cards:
- id: ba293773-d68e-4325-9c1f-f067cdacd3f5
  type: flip
  front: What is chain-of-thought (CoT) prompting, and what is the empirical observation that motivated
    its study?
  back: 'CoT is asking models to reason step-by-step before answering: "Think through this problem step-by-step..."
    Wei et al. (2022) observed that this simple prompt modification dramatically improves performance
    on math, logic, and reasoning tasks, especially for larger models. This suggests scale unlocks reasoning
    ability when given proper prompting.'
- id: 60fab196-bfed-4069-a541-d16b40cead37
  type: mcq
  front: Which of these is evidence for the 'capacity hypothesis' as an explanation for CoT's effectiveness?
  back: 'The capacity hypothesis claims CoT works because more tokens allow higher computation allocation.
    Evidence: (1) random intermediate steps help (suggesting token count matters), (2) benefit scales
    with model size (larger models can use extra tokens better), (3) few-shot helps (provides more total
    context). However, the fact that models must still predict the correct answer despite random steps
    suggests capacity alone is insufficient.'
  choices:
  - key: a
    text: CoT helps even with random intermediate steps
    correct: false
  - key: b
    text: CoT benefit increases with model scale
    correct: false
  - key: c
    text: CoT benefit is stronger with few-shot prompts
    correct: false
  - key: d
    text: All of the above could support the capacity hypothesis
    correct: true
- id: c301860b-aa21-4610-a35e-03bc357c4db0
  type: flip
  front: What is the difference between 'implicit' and 'latent' reasoning in recent LLMs, and what does
    this tell us about CoT mechanisms?
  back: '**Implicit reasoning**: models produce explicit step-by-step text (visible CoT)

    **Latent reasoning**: models do reasoning internally, hidden from the user (e.g., OpenAI o1, DeepSeek,
    Claude Thinking)


    Latent reasoning suggests the underlying mechanism isn''t the *visibility* of steps but rather the
    model allocating computation. This supports the capacity hypothesis: what matters is that the model
    does reasoning, not that we see it.'
- id: fbe407a1-7a48-4ed7-a55e-8e1324e30ce7
  type: mcq
  front: Why does CoT prompting benefit scale-dependent? (i.e., why do larger models see bigger improvements
    from CoT?)
  back: 'Larger models can better utilize extra reasoning tokens because they have more capacity to allocate
    computation dynamically. A 7B parameter model can''t use 100 extra reasoning steps effectively; a
    700B model can. This aligns with the capacity hypothesis: scaling unlocks the ability to benefit from
    more reasoning compute.'
  choices:
  - key: a
    text: Larger models have larger vocabularies and understand 'step-by-step' better
    correct: false
  - key: b
    text: Larger models have more capacity to allocate computation across tokens; extra reasoning tokens
      are used effectively
    correct: true
  - key: c
    text: Larger models need more tokens to avoid overfitting
    correct: false
  - key: d
    text: Smaller models are better at step-by-step reasoning; larger models benefit less
    correct: false
- id: 6b51c278-c317-43ef-a6ca-0994b15bc398
  type: flip
  front: What is the distinction between 'process supervision' and 'outcome supervision' in training models
    for reasoning, and how does this relate to CoT?
  back: '**Outcome supervision**: supervise only the final answer (standard RLHF approach)

    **Process supervision**: supervise intermediate steps/reasoning path (reward each step, not just final
    answer)


    CoT enables process supervision because it makes reasoning steps explicit. Training with process supervision
    improves reasoning more than outcome alone, suggesting that feedback on intermediate steps helps models
    learn better reasoning mechanisms. This supports the view that CoT works partly through improved training
    signals.'
---

## Intuition

Chain-of-thought (CoT): asking models to "think step-by-step" dramatically improves reasoning performance. Example: "Solve the math problem step-by-step" yields much higher accuracy than "Solve the math problem."

Why? Three competing hypotheses:
1. **Capacity argument**: more tokens allow the model to allocate computation to harder reasoning steps
2. **Implicit bootstrapping**: intermediate steps provide better gradient signals during training (if CoT is trained)
3. **Calibration**: step-by-step format makes the model's reasoning legible, improving verifiability

## Detail

**Core empirical findings (2022-2023):**
- Wei et al. 2022: CoT prompting dramatically improves math, logic, common-sense reasoning
- Benefit increases with model scale; small models don't benefit much from CoT
- Works with few-shot prompts ("Here's an example with steps...")
- Benefits hold across languages and modalities

**Competing theories:**

1. **Capacity hypothesis**: CoT works because it provides more intermediate tokens, allowing the model to allocate computation.
   - Evidence: CoT helps even with random intermediate steps (Wang et al.)
   - Counter-evidence: model must still predict correct final answer; extra tokens alone insufficient

2. **Implicit training signal**: If you train on CoT data, intermediate steps act as auxiliary tasks, improving representation learning.
   - Evidence: training on CoT improves downstream reasoning
   - Counter-evidence: prompting alone (no training) still helps

3. **Tokenization effect**: reasoning tasks benefit from more tokens due to vocabulary/compression bottlenecks.
   - Evidence: CoT decomposes complex operations into token-sized steps
   - Counter-evidence: doesn't explain why random steps help

4. **Mechanism transparency**: step-by-step reasoning makes outputs verifiable and interpretable (but not why it improves *correctness*)

**Recent developments (2024-2025):**
- **Test-time scaling with CoT**: using more reasoning steps at test time (not training time) scales performance
- **Process vs. outcome supervision**: supervision on intermediate steps (process) vs. final answer (outcome)
- **Latent reasoning**: recent models (OpenAI o1, DeepSeek) hide reasoning steps, suggesting internal CoT without explicit tokens
- **Optimal allocation**: not all reasoning steps are equally useful; some tasks benefit from more steps, others plateau

## Common gotchas / interview framings
- Q: "Does CoT always help?" A: No; depends on task complexity and model scale. Simple classification tasks don't benefit
- Q: "Is CoT just a prompt engineering trick?" A: Partially; but underlying capacity/computation allocation is real
- Q: "Can we use CoT at test time without training on it?" A: Yes; prompting helps. But training on CoT amplifies the benefit
- Confounding factors: CoT changes tokenization, model behavior, and gradient flow; hard to isolate the mechanism
- Correctness vs. plausibility: models can produce plausible-sounding CoT reasoning that's actually wrong; extra tokens help but don't guarantee correctness

## See also
- [[in-context-learning]]
- [[emergent-capabilities]]
- [[prompt-engineering]]
- [[reasoning-ability]]
- [[language-modeling]]

## Sources
See frontmatter `sources:`.

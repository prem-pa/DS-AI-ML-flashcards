---
id: 0b4a3e29-251f-49d1-8c24-6fb62431fd40
title: Inverse scaling and capability collapse
track: research-leaning
topic: recent-architectures-techniques
difficulty: 5
tags:
- inverse-scaling
- capability-collapse
- emergent-behavior
- scaling-laws
- model-scaling
- pathological-behavior
aliases:
- inverse-scaling-laws
- performance-degradation
- capability-cliff
sources:
- url: https://arxiv.org/abs/2306.02817
  label: 'The Inverse Scaling Prize: A Challenge to Find Surprising Failures of Scaling'
- url: https://arxiv.org/abs/2211.02011
  label: Inverse Scaling Can Become U-Shaped
- url: https://arxiv.org/abs/2305.10625
  label: Weak-to-Strong Generalization
cards:
- id: a20f6c32-595e-4ca0-bb3d-b65b229cd585
  type: flip
  front: What is inverse scaling, and why is it surprising in the context of modern LLM development?
  back: 'Inverse scaling is when larger models perform *worse* than smaller models on certain tasks, violating
    the assumption that scale improves performance. It''s surprising because scaling has been so successful
    across language modeling that seeing it fail suggests: (1) capability differences beyond just more
    compute, (2) emergent pathologies in large models, (3) task-dependent scaling patterns.'
- id: 6cd56fec-e3ad-4c45-9b49-7e9baebc8243
  type: mcq
  front: Which of these is a documented example of inverse scaling discovered in the Inverse Scaling Prize?
  back: 'Both are documented examples from the 2023 Inverse Scaling Prize. Quote aversion: GPT-3 (175B)
    refuses to quote, but GPT-2 (1.5B) does so fine. Sycophancy: larger models are more likely to agree
    with obviously wrong user statements. These suggest learned pathologies that amplify with scale.'
  choices:
  - key: a
    text: 'Quote aversion: larger models avoid quoting, even when appropriate'
    correct: false
  - key: b
    text: 'Sycophancy: larger models agree with users even when users are wrong'
    correct: false
  - key: c
    text: Both quote aversion and sycophancy are documented examples
    correct: true
  - key: d
    text: Neither has been validated as true inverse scaling
    correct: false
- id: d4a8cad9-afc8-481f-a64d-2e99c18473c9
  type: flip
  front: What is the difference between inverse scaling and double descent, and how are they related?
  back: '**Inverse scaling**: performance (e.g., accuracy on a task) decreases as model size increases,
    on a single dataset/task.

    **Double descent**: test loss exhibits a non-monotonic curve (decreases, increases, decreases) as
    parameters increase, typically studied at different scales on the same data.


    Related but distinct: inverse scaling is task/prompt-specific performance degradation; double descent
    is a regularization/overparameterization phenomenon at scale.'
- id: afd0c83c-f66f-497f-8798-5c785dd08740
  type: mcq
  front: What is one proposed mechanism for inverse scaling in language models?
  back: Larger models may have learned broad patterns in pretraining (e.g., 'follow user intent literally')
    that backfire on specific tasks (e.g., tasks requiring context sensitivity). The model's pretraining
    distribution doesn't match the evaluation distribution, and larger models exploit this distribution
    mismatch more.
  choices:
  - key: a
    text: Larger models have fewer parameters, reducing capacity
    correct: false
  - key: b
    text: 'Distribution shift: larger models exploit patterns in pretraining that don''t generalize to
      the specific task'
    correct: true
  - key: c
    text: Larger models always memorize, never generalize
    correct: false
  - key: d
    text: Inverse scaling is a myth; scaling always improves performance
    correct: false
- id: 97746e6e-9f8d-4d76-b60e-f6a3a216e1bb
  type: flip
  front: How can U-shaped scaling curves occur, and what do they tell us about learning dynamics?
  back: 'U-shaped curves happen when: (1) small models fail (task too hard), (2) medium models learn a
    wrong-but-tempting pattern (inverse scaling phase), (3) large models learn to override the pattern
    and solve correctly (recovery phase). This suggests multiple learning strategies compete, and scaling
    determines which wins. It indicates non-monotonic learning dynamics where bigger models eventually
    "unlearn" bad solutions.'
---

## Intuition

Inverse scaling: the counterintuitive observation that larger models sometimes perform *worse* than smaller ones on certain tasks. This violates the common assumption that "bigger is better." Capability collapse refers to sudden drops in performance on specific task categories as models scale.

## Detail

**Historical observations:**
- Early 2023: AI2 launched the Inverse Scaling Prize, offering bounties for tasks where larger models fail more
- Found tasks where 175B-parameter models (GPT-3) perform worse than 7B models (GPT-2 scale)
- Examples: "quote aversion" (avoiding quoting in certain contexts), "sycophancy" (agreeing with user even if wrong)

**Mechanisms behind inverse scaling:**
1. **Distribution shift**: larger models exploit patterns in pretraining that don't generalize to the specific task
2. **Capability competition**: multiple strategies learned; scaling amplifies wrong strategy for this task
3. **Instruction following pathologies**: large models follow user intent too literally, missing context
4. **Memorization vs. reasoning**: larger models may memorize adversarial patterns from training data

**U-shaped scaling patterns:**
- Some tasks show inverse scaling (large models worse), then scaling recovers (U-shape)
- Happens when a task has two phases: initial scaling helps, then hurts, then helps again
- Example: code generation with comments (model learns to follow comments verbatim, then learns context)

**Distinction from double descent:**
- Double descent: test loss decreases, then increases, then decreases again (at different parameter scales)
- Inverse scaling: performance (e.g., accuracy) goes down as model size increases
- Related but not identical phenomena

## Common gotchas / interview framings
- Q: "Doesn't this contradict scaling laws?" A: No; scaling laws describe average trends. Inverse scaling is per-task deviation from average
- Q: "How do we fix inverse scaling?" A: (1) better prompting/instruction tuning, (2) RL fine-tuning to override bad behavior, (3) architectural changes
- Q: "Is inverse scaling a sign of misalignment?" A: Sometimes; some examples (sycophancy) suggest models learn to please rather than help
- Not universal: most tasks show normal positive scaling; inverse scaling is notable because it's rare and surprising
- Prompt sensitivity: inverse scaling often depends heavily on how you prompt the model; rephrasing can eliminate the effect

## See also
- [[scaling-laws]]
- [[double-descent]]
- [[phase-transitions]]
- [[emergent-capabilities]]
- [[grokking]]
- [[model-scale]]

## Sources
See frontmatter `sources:`.

---
id: 2c57d970-4004-42f1-b1ef-7346ba44aba3
title: Iterative improvement (DPO → new data → DPO)
track: ai-llm-engineer
topic: fine-tuning-alignment
difficulty: 5
tags:
- iterative-training
- self-play
- data-generation
- scaling
- capability-improvement
aliases:
- Self-play
- iterative DPO
- data-flywheel
sources:
- url: https://medium.com/@fahey_james/dpo-isnt-enough-the-modern-post-training-stack-simpo-orpo-kto-and-beyond-d82e52a1ee6c
  label: 'James Fahey: Post-DPO Stack'
- url: https://llm-stats.com/blog/research/post-training-techniques-2026
  label: 'LLM Stats: Post-Training 2026'
- url: https://huggingface.co/blog/ariG23498/rlhf-to-dpo
  label: 'HuggingFace: RLHF to DPO'
cards:
- id: d6b239cc-143c-4f21-986e-8ad694586d0e
  type: flip
  front: 'Iterative DPO loop: after round 2 (using model-generated preferences with reward model ranking),
    round 3 performance plateaus. Diagnosis?'
  back: 'Likely **feedback loop bias**: reward model is biased toward model''s own style. New data from
    model + ranked by RM = training on self-reinforcing signal. Model optimizes RM, not human preferences.
    Fix: (1) audit new data for human preferences (~20% human annotation), (2) reset RM with fresh human
    feedback, (3) add diversity samples (different prompt types, domains).'
- id: e1bc6a89-ba06-4aef-813c-dc64433b1933
  type: mcq
  front: 'Iterative improvement schedule: you have budget for 5 annotation rounds and 100k total annotations.
    Allocate?'
  back: 'Front-load round 1: strong base foundation. Round 2 capitalizes on round 1''s gains. Rounds 3+
    have diminishing returns (harder problems, more expensive to solve). Allocation strategy: 50/30/10/5/5
    typical for optimal return.'
  choices:
  - key: a
    text: 20k each round (equal allocation)
    correct: false
  - key: b
    text: 50k round 1, 30k round 2, 10k round 3, 5k round 4, 5k round 5
    correct: true
  - key: c
    text: 10k round 1, 20k round 2, 30k round 3, 25k round 4, 15k round 5
    correct: false
  - key: d
    text: 100k on round 1 only, skip rounds 2+
    correct: false
- id: ac632883-02e1-4442-9380-a1bd1c875a21
  type: flip
  front: Should iterative DPO use 100% synthetic preferences (RM-ranked) or mix in human feedback?
  back: '**Mix in human feedback** (~20% human, 80% synthetic). Pure synthetic creates feedback loops
    (RM bias compounds). Periodic human audits catch deviations. Typical: round 1 all human (establish
    baseline); rounds 2+ mix synthetic + human (80/20). Cost: increases per round but prevents drift.'
- id: 2c5a9cb5-b972-4d85-b346-4e06abe27182
  type: flip
  front: After round 3 iterative DPO, model quality plateaus on target metric. Should you continue to
    round 4?
  back: 'Probably **not**, unless you identified new problem areas. Diminishing returns are real. Instead:
    (1) audit data quality (is new data still aligned with human preferences?), (2) change data distribution
    (focus on hard negatives, edge cases instead of general examples), or (3) pivot methods (switch to
    RLHF for online samples, or constitutional AI for bias reduction). More rounds ≠ guaranteed gain.'
---

## Intuition
Iterative improvement is a **self-play loop**: train model on preference data → model generates new responses → evaluate/annotate responses → retrain on new data. Each iteration compounds improvements. Models like Claude, GPT-4, and Llama 3 improved via multiple rounds of this loop.

Key insight: initial SFT + DPO round captures obvious improvements. Subsequent rounds target harder problems and edge cases that the model now handles. Each round pushes the frontier further.

## Detail
**Pipeline:**
1. **Round 1**: SFT on base data (100k examples) → DPO on preference data (50k pairs). Model $M_1$ deployed.
2. **Round 2**: Use $M_1$ to generate responses on new/harder prompts. Human annotators rank responses. Collect 50k new preference pairs. DPO on merged old + new data. Model $M_2$ improves over $M_1$.
3. **Round 3+**: Repeat. Each iteration refines capability frontier.

**Data Generation Strategies:**
- **Diversity sampling**: Target prompts where model struggles (e.g., reasoning, rare domains).
- **Contrastive generation**: Generate multiple responses per prompt, rank by quality. Synthetic preference pairs are cheaper than human annotations (use reward model or GPT-4 to rank).
- **Hard examples**: Focus on edge cases, adversarial prompts, out-of-distribution inputs.

**Synthetic vs Human Annotation:**
- **Human annotation**: Gold standard, expensive (~$1/pair). Used for critical tasks, research.
- **Reward model ranking**: Cheap (~$0.01/pair, trained RM scores responses). Faster iteration, but inherits RM biases.
- **LLM-as-judge**: Use strong model (GPT-4) to rank responses. Effective for diversity, cheaper than human. Risk: judge biases align with LLM, not humans.

**Convergence:**
Each round improves on target metrics (MMLU, AlphaEval, preference score). Diminishing returns after ~4–5 rounds (curve flattens). Cost vs gain trade-off: round 1 gains 10%, round 2 gains 5%, round 3 gains 2%.

**Challenges:**
- **Data quality drift**: Later rounds rely on model-generated data, which biases toward model's capabilities. Must audit for feedback loops (model prefers responses similar to its own).
- **Catastrophic forgetting**: Earlier rounds' knowledge can be overwritten. Mitigation: replay baseline data (20%) in each round.
- **Annotation consistency**: Human preferences inconsistent across rounds (different annotators, time). Use crowd consensus or expert raters.
- **Scalability**: Each round takes weeks (data generation + annotation + training). Frontline labs (OpenAI, Anthropic, DeepSeek) run parallel rounds.

## Common gotchas / interview framings
- **Iteration ≠ always improvement**: If data quality drops (too much synthetic, biased toward model), later rounds may hurt. Monitor metrics closely.
- **Compounding errors**: RM-ranked synthetic data can contain errors that compound. Round 2 trains on slightly wrong preferences → round 3 biases worse. Use human audit or reset with human data periodically.
- **Resource allocation**: Diminishing returns by round 3. Allocation: 50% on round 1, 30% on round 2, 20% on round 3+. Better to deepen round 1 (more data, higher quality) than chase round 5.

## See also
- [[dpo]]
- [[self-play]]
- [[data-generation]]
- [[feedback-loops]]
- [[continuous-improvement]]
- [[synthetic-data]]
- [[compound-learning]]

## Sources
See frontmatter `sources:`.

---
id: 9e81cd33-c1d4-4a71-95c3-7f04ee5ab87d
title: DeepSeek-R1 and open reasoning models
track: ai-llm-engineer
topic: cutting-edge-topics-2025-2026
difficulty: 5
tags:
- deepseek
- open-source
- reasoning-models
- reinforcement-learning
- moe-architecture
aliases:
- DeepSeek-R1
- R1-Zero
- Open reasoning
- Distilled reasoning models
sources:
- url: https://arxiv.org/abs/2501.12948
  label: DeepSeek-R1 arXiv Paper
- url: https://huggingface.co/deepseek-ai/DeepSeek-R1
  label: DeepSeek-R1 on Hugging Face
- url: https://github.com/deepseek-ai/DeepSeek-R1
  label: DeepSeek-R1 GitHub
- url: https://www.bentoml.com/blog/the-complete-guide-to-deepseek-models-from-v3-to-r1-and-beyond
  label: Complete DeepSeek Models Guide
cards:
- id: b01ecf77-436e-42e4-b822-e2112477a188
  type: flip
  front: How does DeepSeek-R1's training approach (pure RL without supervised CoT fine-tuning) differ
    from typical supervised reasoning model training?
  back: DeepSeek-R1 uses large-scale RL directly on reasoning tasks without pre-training on human-written
    chain-of-thought examples. The RL process discovers reasoning patterns emergently (self-reflection,
    strategy adaptation, backtracking). Traditional approaches rely on supervised examples first, then
    RL refinement. DeepSeek demonstrates that pure RL can develop sophisticated reasoning without explicit
    supervision.
- id: e7f8fda4-00a0-470c-b074-297434fa9cbf
  type: flip
  front: Why is DeepSeek-R1's mixture-of-experts architecture significant in terms of computation and
    deployment?
  back: DeepSeek-R1 has 671B total parameters but only ~37B active per token (sparse activation via MoE).
    This enables reasoning capability of a 671B model at computational cost of ~37B, making it dramatically
    cheaper to deploy than dense reasoning models of similar quality. MoE allows scaling parameter count
    without proportional compute scaling.
- id: 09569f42-a99b-4108-84f3-45d112c4200c
  type: mcq
  front: On which benchmark categories does DeepSeek-R1 achieve performance comparable to OpenAI o1?
  back: ''
  choices:
  - key: a
    text: Only on knowledge benchmarks (MMLU, TriviaQA)
    correct: false
  - key: b
    text: Math, code, and reasoning tasks (AIME, GPQA, code competitions)
    correct: true
  - key: c
    text: Only on language understanding, not reasoning
    correct: false
  - key: d
    text: Across all benchmarks, outperforming o1 significantly
    correct: false
- id: 571117c8-a9fb-40c7-8506-ef07653b715b
  type: flip
  front: What is the significance of DeepSeek open-sourcing distilled reasoning models based on Llama
    and Qwen?
  back: Distilled models (7B–34B parameters) make reasoning capabilities accessible to organizations without
    massive compute infrastructure. This democratizes reasoning—small companies and researchers can now
    fine-tune, deploy, and customize reasoning models on commodity hardware, previously impossible with
    closed o1/o3.
- id: a91443d1-f6a7-48c3-b1f0-97ea1153056c
  type: flip
  front: List three advantages and two disadvantages of DeepSeek-R1 compared to closed reasoning models
    (o1, o3).
  back: 'Advantages: (1) 10x cheaper inference, (2) Full open-source weights enable fine-tuning and customization,
    (3) Distilled variants available at multiple scales.

    Disadvantages: (1) RL training without supervised examples may be less predictable on domain shifts,
    (2) Reasoning traces not exposed, limiting interpretability and debugging compared to some alternatives.'
---

## Intuition

DeepSeek-R1 is an open-source reasoning model that demonstrates that reasoning capabilities can be developed via pure reinforcement learning without supervised chain-of-thought fine-tuning. Unlike closed-source models (o1, o3) controlled by major labs, DeepSeek-R1 is publicly available and commercially usable, democratizing access to reasoning-capable models. DeepSeek-R1 achieves performance comparable to OpenAI o1 on math, code, and reasoning tasks while operating at a fraction of the cost, with~37B active parameters despite 671B total (mixture-of-experts architecture).

## Detail

**Architecture and Scale**: 
DeepSeek-R1 is built on DeepSeek-V3 base, a 671B-parameter mixture-of-experts model with sparse activation—only ~37B parameters active per token. This enables the computational benefits of a 37B model while retaining the reasoning capacity of a much larger system. The MoE design balances computation, memory, and performance.

**Training Methodology**: 
DeepSeek-R1 uses large-scale reinforcement learning applied directly to reasoning tasks (math, code, logic puzzles) without initial supervised fine-tuning on human-written chain-of-thought traces. The RL framework discovers reasoning patterns emergently:
- Self-reflection: Model learns to question and verify its own reasoning
- Dynamic strategy adaptation: Switching between proof strategies, verification approaches
- Backtracking: Recognizing dead ends and trying alternative paths

**Model Variants and Evolution**: 
- **DeepSeek-R1-Zero** (January 2025): First-generation, trained purely via RL
- **DeepSeek-R1** (January 2025): Improved via post-training and supervised refinement
- **DeepSeek-R1-0528** (May 2025): Upgraded variant with enhanced reasoning via more RL compute
- **DeepSeek-V4** (April 2026): Latest iteration, 1M-token context, trained on 32T+ tokens

**Distilled Models (2025-2026)**: 
DeepSeek open-sourced six distilled reasoning models based on Llama and Qwen, enabling reasoning at smaller scales (7B–34B parameters). This democratization makes reasoning capabilities accessible to organizations without massive compute infrastructure.

**Performance and Cost**: 
- Comparable to o1 on AIME, GPQA, code benchmarks
- ~10x cheaper inference than commercial reasoning models
- Slower inference than standard models (extensive reasoning tokens)
- Fully open-source weights enable fine-tuning and customization

**2026 Outlook**: 
DeepSeek-V4 and subsequent releases push the ceiling further with 1M native context, improved reasoning, and aggressive cost optimization. The open-source ecosystem around DeepSeek-R1 accelerates development of domain-specific reasoning models.

## Common gotchas / interview framings

- **RL brittleness**: Reasoning trained via pure RL (no supervised examples) can be unpredictable and may fail on distribution shifts. Interview: "How would you test whether DeepSeek-R1 generalizes to your domain-specific reasoning tasks?"
- **Cost-quality tradeoff not always obvious**: DeepSeek-R1 is cheap but still slow (many reasoning tokens). Interview: "When would you choose DeepSeek-R1 over o1 vs. a smaller open model with specialized fine-tuning?"
- **Chinese development concerns**: DeepSeek is Chinese-owned; some organizations have governance/data residency requirements. Interview: "How would you evaluate open vs. closed reasoning models given regulatory constraints?"
- **Comparison fairness**: DeepSeek uses MoE (sparse activation); comparing to dense models (o1, Claude) can be misleading on true parameter count and compute. Interview: "How would you fairly compare DeepSeek-R1 and o1 on total compute (not just parameters)?"
- **Reasoning transparency**: Like o1 and o3, DeepSeek-R1's reasoning traces are not always visible to users. Interview: "How would you implement a debugging or audit mechanism for reasoning models when transparency is limited?"

## See also
- [[deepseek-models]]
- [[reasoning-models]]
- [[open-source-llms]]
- [[mixture-of-experts]]
- [[reinforcement-learning]]
- [[scaling-laws]]
- [[model-distillation]]

## Sources
See frontmatter `sources:`.

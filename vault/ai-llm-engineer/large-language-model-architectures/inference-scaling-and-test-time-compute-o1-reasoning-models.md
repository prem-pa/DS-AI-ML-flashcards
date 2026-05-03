---
id: c97844cf-ecfc-46bd-9dd2-3a70f1766ae2
title: Inference-scaling and test-time compute (o1, reasoning models)
track: ai-llm-engineer
topic: large-language-model-architectures
difficulty: 5
tags:
- inference-scaling
- test-time-compute
- reasoning-models
- o1
- o3
- chain-of-thought
aliases:
- test-time-compute
- reasoning-scaling
- o1-o3
- inference-time-compute
sources:
- url: https://openai.com/index/learning-to-reason-with-llms/
  label: Learning to Reason with LLMs (OpenAI, 2024, o1)
- url: https://introl.com/blog/inference-time-scaling-research-reasoning-models-december-2025
  label: 'Inference-Time Scaling: Reasoning Models in December 2025'
- url: https://huggingface.co/blog/Kseniase/testtimecompute
  label: What is Test-Time Compute (Hugging Face, 2025)
- url: https://markets.financialcontent.com/wral/article/tokenring-2026-1-1-the-reasoning-revolution-how-openais-o3-series-and-the-rise-of-inference-scaling-redefined-artificial-intelligence
  label: 'The Reasoning Revolution: How OpenAI''s o3 Series Redefined AI (2026)'
cards:
- id: bbb35580-7be7-4483-8101-adca05947a0b
  type: flip
  front: What is the core difference between training-time scaling and inference-time (test-time) scaling?
  back: 'Training-time: increase parameters N, data D, compute FLOPs → better average loss across all
    tasks. Inference-time: spend more compute during prediction (reasoning steps, search, verification)
    → better accuracy on hard tasks specifically. o1/o3 show inference-scaling is orthogonal and powerful
    for reasoning.'
- id: 343256f3-f470-4af6-b3ed-66fa2f35cc28
  type: flip
  front: How does o1 learn to reason? Describe the training procedure and key insight.
  back: 'o1 trained with reinforcement learning (RL): generate reasoning chains, verify answers (outcome
    reward). Model learns to predict which problems need reasoning and how many steps. Key: RL signal
    (correct/incorrect) encourages adaptive reasoning—easy problems → quick answers, hard problems → extended
    chains. This is why o1 is slower but more accurate on difficult tasks.'
- id: 117749f6-1a83-40d3-adc3-2b04e4848cff
  type: mcq
  front: DeepSeek-R1 achieved performance comparable to o1 with 70% lower inference cost. What does this
    suggest about reasoning models?
  back: 'DeepSeek-R1 proves pure RL (no labeled chain-of-thought) induces reasoning. Cost difference reflects
    architectural and scaling choices, not just inference length. Implications: (1) reasoning is learnable
    from outcome-only signals, (2) efficiency varies by architecture [[gpt-llama-mistral-qwen-architectures]],
    (3) inference-scaling is orthogonal to model choice.'
  choices:
  - key: a
    text: o1 is inefficient and should be replaced by DeepSeek-R1
    correct: false
  - key: b
    text: RL-based reasoning is fundamentally more efficient than supervised fine-tuning; the architecture
      and scaling matter more than data source
    correct: true
  - key: c
    text: Reasoning cannot be learned from pure RL without supervised reasoning data
    correct: false
  - key: d
    text: DeepSeek has superior hardware; cost difference is only infrastructure
    correct: false
- id: ccfccd2e-2bf8-4b3d-9f11-df965846b967
  type: flip
  front: Estimate the test-time compute scaling law observed in o1/o3 models. How does it compare to training-time
    scaling?
  back: 'Test-time: $\text{Accuracy}(T) \approx A_\infty - B/T^{\gamma}$, $\gamma \approx 0.3-0.5$ (empirical).
    Training-time: loss $\sim 1/N^{0.076}$. Test-time exponent is larger (0.3-0.5 vs. 0.076), meaning
    test-time scaling is faster initially. Per unit of improvement, test-time compute is more efficient
    for hard problems [[scaling-laws-for-loss-and-compute]].'
- id: 7774fe23-5ab9-42cb-82a0-ef7b6474fac4
  type: flip
  front: Given that o1 and o3 achieve near-human reasoning on math/science benchmarks, what is the next
    frontier for inference-scaling?
  back: 'Current: hard single-domain reasoning (math, code, GPQA). Next frontiers: (1) multimodal reasoning
    (image+text), (2) long-horizon planning (multi-step real-world tasks), (3) open-ended reasoning (writing,
    creative problem-solving, where ground truth is ambiguous), (4) embodied reasoning (robotics, physical
    interaction). Inference-scaling''s applicability to these domains is open research (2025).'
---

## Intuition

Traditional LLM scaling (2017-2023) optimized training compute: larger models, more data, better loss. Inference-scaling (2024-2025) inverts this: spend more compute at test/inference time (reasoning steps, chain-of-thought, search) to improve reasoning on hard problems. o1 (OpenAI, 2024) demonstrated that test-time compute rivals training-time compute in improving performance. o3 (announced Dec 2024, released early 2025) scales this further, achieving frontier reasoning benchmarks (GPQA Diamond: 92%+, ARC-c: 95%+).

## Detail

**Scaling law inversion:**
- Training-time: more FLOPs (parameters × tokens) → better loss on average
- Inference-time: more reasoning steps (chain-of-thought, internal search) → better accuracy on hard problems

**Key mechanism: Reinforcement Learning for reasoning**
- o1: Train with RL to maximize reward (correct answer on test problem)
- Reward signal: process verification (check intermediate steps), or outcome (final answer correct)
- Model learns when to "think longer" and when to be confident
- Result: more steps on hard problems, fewer on easy ones (adaptive computation)

**o1 performance characteristics (Oct 2024):**
- Math (AIME): 92%, Physics (GPQA Diamond): 85%, Coding (Codeforces): 96th percentile
- Speed: slower than typical LLMs (more reasoning), but faster than human mathematicians
- Cost: higher per-query (more compute), but solves previously unsolved problems

**o3 progression (Dec 2024 / Jan 2025):**
- Extended reasoning: longer chains of thought, more internal states
- GPQA Diamond: 92% (matching specialized tools)
- ARC-c (science reasoning): 95%+
- Math Olympiad: ~45% (vs. o1's ~25%, human expert ~50%)
- Scaling laws suggest: more reasoning steps → higher accuracy, approaching human-expert level

**DeepSeek-R1 (Jan 2025 benchmark):**
- Pure RL approach (no supervised reasoning data)
- Matches o1 on math/code at 70% lower inference cost
- Proves RL alone (no labeled chain-of-thought) can induce reasoning
- Validates inference-scaling as dominant frontier

**Test-time compute scaling law (empirical 2025):**
$$\text{Accuracy}(T) \approx A_\infty - B/T^\gamma$$
where:
- $T$: test-time compute (reasoning steps, tokens generated)
- $\gamma \approx 0.3 - 0.5$ (empirically measured)
- Improvement saturates but continues (no hard ceiling observed yet)

**Comparison to training-time scaling:**
- Training: loss $\sim 1/N^{0.076}$ (slow power law)
- Test-time: accuracy $\sim 1 - B/T^{0.3}$ (faster power law, at least initially)
- Practical implication: test-time compute more efficient per unit of accuracy improvement (for hard problems)

**Infrastructure impact (2025-2026):**
- Inference compute projected to exceed training compute by 2026-2027
- 75% of total AI compute spent on inference by 2030 (analyst estimates)
- Serverless pricing models: charge per reasoning token, not just output token
- New optimization: latency vs. quality tradeoff (users can request quick answer vs. thorough reasoning)

## Common gotchas / interview framings

- **"o1 is just chain-of-thought prompting."** → No. o1 is trained with RL to reason; it learns *when* to use reasoning. Naive chain-of-thought (prompt "Let's think step-by-step") helps but is much weaker.
- **Speed vs. quality:** o1 is slower than GPT-4 on easy tasks (always reasons), but much better on hard tasks. Adaptive reasoning (knowing when not to reason) is the real innovation.
- **Test-time compute != longer outputs:** More internal states and search, not just more text. Hidden reasoning tokens are counted; user doesn't see all of them.
- **Reasoning generalization:** o1/o3 trained on math/code; unknown how well this transfers to open-ended reasoning (writing, creative tasks). Empirically, transfers well to new domains.
- **Future frontiers:** o3 sets GPQA Diamond at 92%; humans ~65%. Next frontier: multimodal reasoning, long-horizon planning, embodied reasoning.

## Interview framing

- "If training-time scaling hits diminishing returns, why does test-time scaling seem fresh?" → Hard problems have intrinsic difficulty; more reasoning (search, verification) directly addresses it. Easy tasks saturate fast; hard tasks sustain scaling. o1/o3 are trained on hard-task-only distributions.
- "Will inference-scaling replace training-scaling?" → Unlikely. Both are complementary. Larger base models (training) + more reasoning (inference) = optimal frontier.

## See also
- [[scaling-laws-for-loss-and-compute]]
- [[optimal-batch-sizes-and-datacompute-tradeoff]]
- [[frontier-model-performance-gpt-4-claude-37-gemini-3]]

## Sources
See frontmatter `sources:`.

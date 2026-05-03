---
id: 2c07f584-2f8f-4904-8ff0-64d630bd2ad8
title: GRPO (Group Relative Policy Optimization)
track: ai-llm-engineer
topic: fine-tuning-alignment
difficulty: 5
tags:
- grpo
- online-rl
- group-advantage
- deepseek
- ppo-variant
aliases:
- GRPO
- group relative
- online RL
sources:
- url: https://llm-stats.com/blog/research/post-training-techniques-2026
  label: 'LLM Stats: Post-Training 2026'
- url: https://arxiv.org/abs/2501.03262
  label: 'REINFORCE++: Stabilizing Critic-Free RL'
- url: https://medium.com/@ding.zhongqiang/human-alignment-in-large-language-models-041088692d00
  label: 'Dr. Zhongqiang DING: LLM Alignment (2026)'
cards:
- id: 01e013c9-07ff-45d0-a077-fc3aea3f89a5
  type: flip
  front: GRPO removes the critic (value function). How does it estimate advantages without V(s)?
  back: 'Group relative advantage: sample N responses from current policy, score each with reward model,
    compute advantage as: (response_score - group_mean) / group_std. Advantage is **relative rank within
    group**, not absolute value. Naturally normalized, no separate critic needed. Trade: must sample multiple
    responses per prompt (expensive), but avoids critic training overhead.'
- id: b8321f17-7211-442b-901c-b6dee716a2df
  type: mcq
  front: You implement GRPO with group size N=2. Advantages are near zero; policy barely updates. Why?
  back: With N=2, only 2 responses per prompt. If both score similarly (std small), advantage normalizes
    to ~0. Increase N to ≥4–8 to get meaningful variance within groups. Or audit reward model—if all responses
    score similarly, RM may be poorly calibrated.
  choices:
  - key: a
    text: Learning rate too low
    correct: false
  - key: b
    text: Group size too small; std(rewards) in small group is tiny, advantage ~0
    correct: true
  - key: c
    text: Reward model is too good (all scores similar)
    correct: false
  - key: d
    text: PPO clip range ε too tight
    correct: false
- id: de1cff90-2469-4f2d-a64b-2d3b1d370aa4
  type: flip
  front: 'GRPO vs DPO: you need reasoning/complex task, 8xA100, 100k preference pairs. Which is faster?'
  back: '**DPO is faster** (~2–3 hrs, off-policy). GRPO is slower (~12–16 hrs, on-policy—many samples
    needed). DPO preferred for standard alignment. GRPO shines when **reasoning quality matters** and
    you can afford compute; online sampling improves quality for tasks like math/code (DeepSeek-R1 case).
    For typical task: DPO wins speed.'
- id: ef6a6ede-cd73-419d-9a80-2768d609da82
  type: flip
  front: 'GRPO group size N: tradeoff between N=4 vs N=16?'
  back: 'N=4: fast (4 samples/prompt), but advantage estimates noisy (small group std). N=16: stable advantages
    (16-sample std well-estimated), but slower (16 inference passes per prompt). Sweet spot: N=8. Start
    there, grid-search if variance issues. Larger groups more stable but diminishing returns after N=16.'
---

## Intuition
**GRPO** is an online RL method designed for LLM alignment. Key innovation: **group relative advantage**. Instead of training a separate value function critic, compute advantages by comparing responses *within groups*. This removes the need for critic, reducing memory and complexity while maintaining stability.

Architecture: (1) For each prompt, generate N responses from current policy. (2) Score each response with reward model. (3) Compute advantage as relative rank within group (not absolute value). (4) Update policy via PPO-style clipped gradients.

Introduced by DeepSeek (2025) for training DeepSeek-R1; now widely adopted for reasoning tasks and online RL.

## Detail
**Group Advantage Computation:**
Traditional PPO:
$$A_t = r_t + \gamma V(s_{t+1}) - V(s_t)$$
requires training value network V (extra parameters, loss, complexity).

GRPO: for prompt $x$, sample N responses $(y_1, \ldots, y_N)$ from $\pi_\theta$. Score each: $r(x, y_i)$. Compute group advantage:
$$A_i = \frac{r(x, y_i) - \text{mean}(r(x, y_j) : j \in [1, N])}{\text{std}(r(x, y_j))}$$
Advantage is **relative rank within group**, not absolute. Naturally normalized, statistically stable.

Policy update (PPO-style):
$$\mathcal{L}_{GRPO} = -\mathbb{E}[\min(r_t A_t, \text{clip}(r_t, 1-\epsilon, 1+\epsilon) A_t)]$$
where $r_t = \frac{p_\theta(y_i|x)}{p_{\theta_{old}}(y_i|x)}$ (policy ratio).

**Advantages:**
- **No critic network**: Saves ~30% VRAM (no separate V parameters). Simpler codebase.
- **On-policy but efficient**: Sample multiple responses per prompt (amortizes inference cost).
- **Stable advantages**: Normalization by group mean/std naturally calibrates advantage scale. Less hyperparameter tuning than PPO.
- **Empirically strong**: DeepSeek-R1's reasoning improvements attributed partly to GRPO.

**Disadvantages:**
- Still on-policy (requires fresh samples per update, expensive). Not as efficient as DPO.
- Requires good reward model (same as PPO). Poor reward → bad advantages.
- Group size N is hyperparameter (typically 4–16). Smaller groups → noisier advantages; larger → more compute per step.

**Hyperparameters:**
- Group size N: 4–16 typical. Larger → more stable but slower.
- Learning rate: ~1e-5 (same as PPO).
- Epochs per group: 1–2 (to avoid overfitting to single group).
- Reward model: must be calibrated and reasonably accurate.

## Common gotchas / interview framings
- **Group size too small**: 2–4 responses, std is tiny, advantage estimates are noisy. Prefer N ≥ 4.
- **Reward model quality**: If reward model is poorly calibrated (e.g., all responses score similarly), advantages collapse to ~0 → no policy update. Requires good RM.
- **Comparison to REINFORCE++**: REINFORCE++ is critic-free with variance reduction tricks; GRPO is critic-free with group normalization. Both address PPO critic complexity; GRPO is newer (2025), REINFORCE++ is older variant.
- **Online RL cost**: Still on-policy, so ~3–5x cost of DPO per epoch. Used when reasoning/complex tasks need online samples.

## See also
- [[online-rl]]
- [[policy-optimization]]
- [[group-advantage]]
- [[critic-free]]
- [[deepseek]]
- [[rl-for-llms]]
- [[advantage-estimation]]

## Sources
See frontmatter `sources:`.

---
id: 96bc95b0-8657-4a6d-ba02-48a51235e81e
title: Policy optimization (REINFORCE, PPO)
track: ai-llm-engineer
topic: fine-tuning-alignment
difficulty: 5
tags:
- policy-gradient
- reinforce
- ppo
- on-policy-rl
- gradient-variance
aliases:
- PPO
- REINFORCE
- policy gradient
- on-policy RL
sources:
- url: https://arxiv.org/abs/1707.06347
  label: 'Schulman et al: PPO Algorithms'
- url: https://cameronrwolfe.substack.com/p/proximal-policy-optimization-ppo
  label: 'Cameron Wolfe: PPO Deep Dive'
- url: https://arxiv.org/abs/2501.03262
  label: REINFORCE++ Paper
- url: https://arxiv.org/abs/2601.11574
  label: 'GRADE: Replacing Policy Gradients with Backprop'
cards:
- id: b483541d-a0df-4ab5-8345-bd7188cf0623
  type: flip
  front: Why is PPO's clipped objective more stable than REINFORCE?
  back: REINFORCE uses $\log p(a|s) R$—unbounded. Single lucky rollout → massive positive update; one
    bad rollout → massive negative update. PPO clips the probability ratio to $[1-\epsilon, 1+\epsilon]$,
    bounding the update per step. This prevents catastrophic divergence and stabilizes training.
- id: 87375ec7-b521-4859-99ff-0f6f4e882dac
  type: mcq
  front: 'You run PPO with ε=0.5 (clip range ±50%) and observe: policy improves for 2 epochs, then diverges
    (nonsense outputs). What likely happened?'
  back: 'ε=0.5 is huge (typical: 0.2). Allows policy to shift 50% away from old policy per step—too aggressive.
    Reduce ε to 0.1–0.2. Gradient updates are clipped per-step; large ε defeats the purpose.'
  choices:
  - key: a
    text: ε is too large; policy is drifting too far from baseline per step
    correct: true
  - key: b
    text: Reward model is poorly calibrated
    correct: false
  - key: c
    text: KL weight β is too high
    correct: false
  - key: d
    text: Value function is overfitting
    correct: false
- id: 9a3de11d-101e-4d16-86eb-37c15b1ba550
  type: flip
  front: PPO requires a **value function** (critic). Why is training V(s) well important?
  back: 'Advantage $A_t = r_t + \gamma V(s_{t+1}) - V(s_t)$ depends on V. If V is poorly trained, A is
    noisy → policy gradients are uninformative. Good V reduces variance of advantage estimates, making
    policy updates reliable. Typically: use separate loss $\mathcal{L}_V = (V(s_t) - R_t)^2$ to train
    V alongside PPO.'
- id: f0d4f6b0-665a-42ed-9aaf-794ff152721d
  type: flip
  front: You have 1M tokens budget. Use RLHF (PPO) or DPO?
  back: Use **DPO**. PPO is on-policy (requires fresh samples per step), so 1M tokens might be 3–5 PPO
    steps depending on batch size. DPO is off-policy (batch-only), so same 1M tokens = deeper, more stable
    training. DPO converges faster and is 2–3x cheaper. RLHF reserved for when compute is abundant.
---

## Intuition
**REINFORCE**: Update policy by taking gradient of expected reward. Policy gradient theorem: $\nabla_\theta \mathbb{E}[R] = \mathbb{E}[\nabla_\theta \log p_\theta(a|s) R]$. Intuition: increase probability of actions that lead to high reward.

**PPO** (Proximal Policy Optimization): REINFORCE has high variance; PPO adds a clipped objective to stabilize training: don't update too far from current policy per step. This prevents catastrophic steps and divergence.

Both are **on-policy**: require fresh samples from current policy, so training is expensive. But they're reliable and interpretable.

## Detail
**REINFORCE Objective:**
$$\mathcal{L}_{REINFORCE} = \mathbb{E}_t [\log p_\theta(a_t | s_t) R_t]$$
where $R_t = \sum_{t'=t}^T \gamma^{t'-t} r_{t'}$ is cumulative reward. Problem: $R_t$ is a single sample—high variance. Small number of good rollouts by chance → large policy update; bad rollouts → large negative update. Training is unstable.

**PPO Objective (Clipped):**
$$\mathcal{L}_{PPO} = \mathbb{E}_t [\min(r_t(\theta) A_t, \text{clip}(r_t(\theta), 1-\epsilon, 1+\epsilon) A_t)]$$
where $r_t(\theta) = \frac{p_\theta(a_t|s_t)}{p_{\text{old}}(a_t|s_t)}$ (probability ratio), $A_t$ is advantage estimate, $\epsilon$ (typical: 0.2) clips ratio to stay near old policy. This prevents overly aggressive updates.

Advantage Estimation (crucial for stability):
$$A_t = r_t + \gamma V(s_{t+1}) - V(s_t)$$
where $V$ is a learned value function (critic). Reduces variance vs. raw cumulative reward.

**PPO with LLMs:**
- For each batch of prompts: generate multiple completions from current policy → score each with reward model → compute advantages → update policy via PPO. Repeat.
- Batch size, learning rate, $\beta$ (KL weight) are critical. Small batches → high variance; large batches → expensive per step.
- Number of epochs per batch: typically 1–4 (more epochs risks overtraining on stale rollouts).

**Recent Variants (2025–2026):**
- **REINFORCE++**: removes critic, stabilizes with variance reduction tricks (batch normalization of rewards, adaptive learning rate). Simpler than PPO, competitive performance.
- **GRADE**: replaces discrete token sampling with continuous relaxation (Gumbel-Softmax) → allows differentiable backprop instead of high-variance score function gradients. Promising but less adopted.
- **GRPO** (Group Relative Policy Optimization): groups samples, computes advantages within groups. Online RL, removes need for separate value function.

## Common gotchas / interview framings
- **On-policy inefficiency**: Every policy update requires fresh rollouts. Can't reuse old samples → 3–5x compute vs. off-policy methods (DPO).
- **PPO hyperparameter sensitivity**: $\epsilon$ (clip range), learning rate, $\beta$ (KL weight) are finicky. Poor choices → divergence or no improvement. Grid search advised.
- **Advantage estimation instability**: Value function $V$ must be trained well; if $V$ is bad, $A_t$ is noisy, policy updates are random. Requires careful V function training (separate loss).
- **Sequence length explosion**: Sampling long sequences from policy is expensive. Limit prompt length or use truncation.
- **Reward model dependency**: If reward model is poor, policy optimizes a corrupted signal. RLHF assumes good RM; if RM is weak, PPO amplifies bad signal.

## See also
- [[policy-gradient]]
- [[variance-reduction]]
- [[clipped-objective]]
- [[on-policy-learning]]
- [[rlhf]]
- [[advantage-function]]
- [[generalized-advantage-estimation]]

## Sources
See frontmatter `sources:`.

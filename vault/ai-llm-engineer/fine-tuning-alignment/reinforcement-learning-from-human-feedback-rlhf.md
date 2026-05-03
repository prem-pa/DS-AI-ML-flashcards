---
id: 623ea73e-95bf-42a6-8e77-711c95881585
title: Reinforcement learning from human feedback (RLHF)
track: ai-llm-engineer
topic: fine-tuning-alignment
difficulty: 3
tags:
- rlhf
- reward-modeling
- policy-optimization
- alignment
- human-feedback
aliases:
- RLHF
- human feedback alignment
- preference learning
sources:
- url: https://cameronrwolfe.substack.com/p/proximal-policy-optimization-ppo
  label: 'Cameron Wolfe: PPO for LLM Alignment'
- url: https://huggingface.co/blog/ariG23498/rlhf-to-dpo
  label: 'HuggingFace: Simplifying Alignment from RLHF to DPO'
- url: https://mljourney.com/rlhf-vs-dpo-vs-ppo-how-to-align-llms-without-losing-your-mind/
  label: 'ML Journey: RLHF vs DPO vs PPO'
cards:
- id: cdfd4902-53ff-4fe8-ba5c-39d1e8f8cec1
  type: flip
  front: Why does RLHF require both an SFT model AND a reference model during PPO?
  back: 'SFT model is the **policy** being optimized. Reference model is frozen copy of SFT model used
    to compute KL divergence penalty, preventing the policy from drifting too far. KL term: $\beta D_{KL}(\pi(y|x)
    \parallel \pi_{ref}(y|x))$. Without reference, policy could collapse to reward hacking or diverge
    to gibberish.'
- id: 90b16678-d987-45ed-93cd-e909e36e873f
  type: mcq
  front: You train a reward model on 10k binary preferences (A > B) and it predicts perfectly on training
    data (98% accuracy). At inference, it ranks irrelevant outputs as high-reward. Why?
  back: 'Reward model trained on only 10k examples has high train accuracy but poor OOD generalization.
    Solution: more preference data, ensemble rewards, or skip RLHF and use DPO (direct preference optimization)
    instead.'
  choices:
  - key: a
    text: The KL penalty is too weak
    correct: false
  - key: b
    text: Reward model overfit to training distribution; poor generalization to OOD prompts
    correct: true
  - key: c
    text: The PPO learning rate was too high
    correct: false
  - key: d
    text: Insufficient SFT data
    correct: false
- id: 9e1db50c-ff33-4422-960c-4416c80453ca
  type: flip
  front: 'RLHF pipeline: SFT → Reward Model → PPO. What is the computational cost multiplier vs. SFT?'
  back: 'Approximately **3x SFT cost**. Each PPO step requires forward/backward through: (1) policy model,
    (2) reference model (inference only but still compute), (3) reward model. Plus PPO''s clipped objectives
    add overhead. Modern shortcut: skip RM entirely, use DPO for ~1.5x cost.'
- id: 7fa2ad03-f2f1-4135-b337-adb652dc0e15
  type: flip
  front: You set KL weight β=0.001. Policy outputs gibberish. You set β=1.0. Policy barely improves over
    SFT. What went wrong?
  back: 'KL weight mismatch. β=0.001 is **way too low** (policy diverges). β=1.0 is **way too high** (no
    reward signal breakthrough). Proper range: 0.01–0.1. Start β=0.05, grid-search around it. Use validation
    set (human evals preferred) to find sweet spot between improvement and stability.'
---

## Intuition
RLHF is a three-step process: (1) **SFT**: train model on instruction data. (2) **Reward Modeling**: collect pairwise preferences (A vs B), train a reward model $r_\phi(x, y)$ to predict which response humans prefer. (3) **Policy Optimization**: use RL (typically PPO) to fine-tune the SFT model to maximize reward while staying close to the original model (KL penalty).

The appeal: reward model captures human values without hand-coding them. Once trained, it scales to any prompt.

## Detail
Step-by-step pipeline:

**1. Preference Data Collection:**
Collect triples: (prompt, response_A, response_B, preference). Example: prompt = "Summarize this article", A and B are two summaries, human marks A as better. Typically 10k–100k pairwise comparisons.

**2. Reward Model Training:**
$$\mathcal{L}_{RM} = -\mathbb{E}[(y_w, y_l) \sim \mathcal{D}] [\log \sigma(r_\phi(x, y_w) - r_\phi(x, y_l))]$$
where $y_w$ is preferred ("winner"), $y_l$ is dispreferred ("loser"), $\sigma$ is sigmoid. Reward model is typically a smaller LLM (~1/4 scale of policy) or linear head on frozen embeddings.

**3. Policy Optimization with PPO:**
$$\mathcal{L}_{RL} = \mathbb{E}_{y \sim \pi}[r_\phi(x, y)] - \beta D_{KL}(\pi(y|x) \parallel \pi_{SFT}(y|x))$$
PPO introduces clipped gradients to stabilize training. $\beta$ (KL weight, typical: 0.01–0.1) controls how far policy can drift from SFT baseline.

**Complexity:** Requires 3 forward passes per training step (policy, reference, reward model). Compute: ~3x SFT cost. Instability risks from reward model overfitting and PPO hyperparameter sensitivity.

## Common gotchas / interview framings
- **Reward hacking**: Model learns to game the reward signal. Example: verbose responses score higher (more text seems better). Mitigation: human review, domain-specific reward metrics, avoid reward simplicity.
- **Preference data bias**: Human annotators have inconsistent preferences (~80% agreement typical). Aggregating weak signals can train a weak reward model.
- **KL collapse**: If $\beta$ too low, policy diverges from SFT (unintelligible outputs). If $\beta$ too high, policy doesn't improve. Tune carefully (grid search 0.01–0.1).
- **Reward model overfitting**: Training on only 10k examples risks poor OOD generalization. Use data augmentation, ensemble rewards, or skip RLHF → DPO.

## See also
- [[reward-model]]
- [[policy-optimization]]
- [[preference-data]]
- [[ppo]]
- [[human-alignment]]
- [[kl-divergence]]
- [[rl-for-llms]]

## Sources
See frontmatter `sources:`.

---
id: 278707fe-251a-4084-8a3c-91973489a1d3
title: KL divergence constraint and reference model
track: ai-llm-engineer
topic: fine-tuning-alignment
difficulty: 3
tags:
- kl-divergence
- reference-model
- distribution-shift
- regularization
- policy-stability
aliases:
- KL penalty
- KL constraint
- reference policy
sources:
- url: https://cameronrwolfe.substack.com/p/proximal-policy-optimization-ppo
  label: 'Cameron Wolfe: PPO & KL Constraints'
- url: https://arxiv.org/abs/1707.06347
  label: 'Schulman et al: PPO (includes KL discussion)'
- url: https://huggingface.co/blog/ariG23498/rlhf-to-dpo
  label: HuggingFace RLHF Blog
cards:
- id: 3a1f4c09-3f34-4981-b610-669e8a3b1910
  type: flip
  front: During RLHF, why does the model need a **frozen reference** model in addition to the trainable
    policy?
  back: 'Reference provides a KL divergence baseline: $D_{KL}(\pi || \pi_{ref})$. Without it, reward maximization
    has no constraint—policy can diverge to reward hacking or gibberish. Reference keeps policy close
    to SFT distribution, preventing catastrophic forgetting and ensuring outputs remain coherent.'
- id: 96a85991-145b-466a-9971-c69b18e6a46b
  type: mcq
  front: You set β=0.5 during PPO. Policy improves 5% in reward but outputs become repetitive/low-quality.
    Diagnosis?
  back: β=0.5 is moderate, but combined with 5% improvement + degradation suggests reward hacking. Reward
    model may penalize short/diverse text, reward verbosity or repetition. Reduce β further? No—the real
    fix is audit reward model or add length/diversity penalties.
  choices:
  - key: a
    text: β is too high (too much constraint); policy is not free to improve
    correct: false
  - key: b
    text: β is too low (insufficient constraint); policy is drifting and reward hacking (e.g., repetition
      scores high)
    correct: true
  - key: c
    text: The reward model is bad
    correct: false
  - key: d
    text: Learning rate too high
    correct: false
- id: b1eedfe4-785e-4dff-9071-f26e4b47cea3
  type: flip
  front: Reference model takes 1.5GB VRAM. Can you use a quantized (4-bit) reference to save memory?
  back: 'In theory yes, practically **no**. Computing log probability $\log \pi_{ref}(y|x)$ requires accurate
    token probabilities. Quantization (4-bit) degrades accuracy significantly. KL term becomes noisy,
    policy updates unreliable. Mitigation: use smaller reference model (~half scale of policy) or accept
    VRAM cost. Better: use DPO (no reference needed).'
- id: 82c236b5-0a46-417f-b7dd-f9fad3721060
  type: flip
  front: Why does SFT quality matter before RLHF if RLHF is supposed to improve the model?
  back: Reference model is frozen SFT. If SFT is weak (poor instruction-following), reference is also
    weak. KL term then regularizes against weak baseline, limiting improvements. RLHF can only optimize
    against reward signal—if reference is garbage, KL is a weak regularizer. **Strong SFT is prerequisite
    for effective RLHF**.
---

## Intuition
After SFT, the model has learned to follow instructions. RLHF optimizes to maximize reward, but unbounded optimization can cause policy to diverge from SFT distribution—e.g., model collapses to reward hacking, outputs become incoherent, or language degrades. **KL divergence constraint** acts as a regularizer: keep the optimized policy close to the SFT baseline.

Intuition: "improve reward, but don't drift too far from your teacher." This prevents catastrophic forgetting and reward hacking.

## Detail
**KL Divergence:**
$$D_{KL}(\pi \parallel \pi_{ref}) = \mathbb{E}_{y \sim \pi} [\log \pi(y|x) - \log \pi_{ref}(y|x)]$$
Measures how much $\pi$ (optimized policy) differs from $\pi_{ref}$ (SFT baseline). If KL = 0, policies are identical. If KL >> 0, policies diverge.

**Loss with KL Penalty:**
Instead of maximizing reward alone, optimize:
$$\mathcal{L} = \mathbb{E}[r(x, y)] - \beta D_{KL}(\pi(y|x) \parallel \pi_{ref}(y|x))$$
where $\beta$ (weight) controls the trade-off. Higher $\beta$ → stay closer to SFT (safer, less improvement). Lower $\beta$ → aggressive reward optimization (risk divergence).

**Reference Model Implementation:**
The reference model is a **frozen copy** of the SFT model. At each training step:
1. Compute $\log \pi(y|x)$ from current policy.
2. Compute $\log \pi_{ref}(y|x)$ from frozen reference (inference only, no gradients).
3. KL term: compute log probability ratio, multiply by $\beta$, subtract from reward.

Memory cost: reference model takes up ~2x VRAM of policy during training (both loaded). Inference cost: zero (reference used only during training).

**Beta Tuning:**
- $\beta = 0$: no constraint, pure reward maximization → divergence.
- $\beta = 0.01$: very loose constraint, significant room for policy drift.
- $\beta = 0.05–0.1$: sweet spot for RLHF; balances improvement vs. stability.
- $\beta = 1.0$: strong constraint, policy barely improves but stays very safe.

**Why Reference Model?**
Without reference: policy has no regularization; RL collapse to reward hacking. With reference: KL term penalizes divergence, keeping policy interpretable and aligned to base model behavior.

## Common gotchas / interview framings
- **"Why not just lower LR?"** Lower learning rate slows convergence but doesn't prevent divergence. KL explicitly constrains distribution shift, more principled.
- **Reference model staleness**: Reference is frozen at start of RLHF. If SFT is weak, reference is also weak. **Always strong SFT before RLHF**.
- **Beta annealing**: Some pipelines start high $\beta$ (conservative) and decay it (allowing more drift). Risky; typically fixed $\beta$ is safer.
- **KL collapse with large batches**: Very large batches + small $\beta$ can cause KL to exceed policy capacity, leading to gibberish. Monitor KL during training.

## See also
- [[kl-divergence]]
- [[distribution-divergence]]
- [[regularization]]
- [[policy-drift]]
- [[rlhf]]
- [[ppo]]
- [[entropy-regularization]]

## Sources
See frontmatter `sources:`.

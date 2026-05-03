---
id: 99ba380d-b968-4ef0-8028-f239d4b0c7a9
title: SimPO, ORPO, KTO post-DPO landscape
track: ai-llm-engineer
topic: fine-tuning-alignment
difficulty: 5
tags:
- post-dpo
- simpo
- orpo
- kto
- preference-optimization
- modern-methods
aliases:
- SimPO
- ORPO
- KTO
- reference-free
- merged-objective
sources:
- url: https://medium.com/@fahey_james/dpo-isnt-enough-the-modern-post-training-stack-simpo-orpo-kto-and-beyond-d82e52a1ee6c
  label: 'James Fahey: Post-DPO Stack (Medium)'
- url: https://llm-stats.com/blog/research/post-training-techniques-2026
  label: 'LLM Stats: Post-Training 2026'
- url: https://aman.ai/primers/ai/preference-optimization/
  label: 'Aman Sanger: Preference Optimization Primer'
cards:
- id: cdb8cf06-b7f8-462a-b161-a94342603c89
  type: mcq
  front: You have budget for either 100k pairwise DPO annotations OR 500k binary (good/bad) KTO annotations.
    Which yields better alignment?
  back: 'Domain/annotator dependent. For high-quality annotations (expert raters): 100k DPO likely better.
    For crowd-sourced annotations (noisy): 500k KTO may have better signal-to-cost ratio. Empirically,
    modern best practice: hybrid—use both if possible, or prioritize KTO for production (cheaper, faster
    iteration).'
  choices:
  - key: a
    text: 100k pairwise DPO; higher signal per example
    correct: false
  - key: b
    text: 500k binary KTO; 5x more data outweighs signal loss
    correct: false
  - key: c
    text: Depends on domain and quality of annotators
    correct: true
  - key: d
    text: DPO always wins regardless of data scale
    correct: false
- id: 81fe93ed-c0c0-4265-857a-6c10754fbc0b
  type: flip
  front: SimPO removes the reference model. How does it prevent policy divergence without explicit KL
    penalty?
  back: 'SimPO uses implicit constraint: scaling by average log probability. The model is incentivized
    to assign high probability to preferred responses, naturally staying near the SFT distribution (high
    probability = coherent under SFT baseline). No explicit KL, but implicit regularization from score
    normalization. Trade: less explicit control vs. DPO.'
- id: 826f3abe-925b-4171-8ba4-6bffd853e3b4
  type: flip
  front: You train ORPO (merged SFT+preference) with lambda=2.0. Model reaches 95% preference accuracy
    but degraded instruction-following (MMLU -5%). Diagnosis?
  back: '**Lambda too high** (2.0 weighs preferences 2x over SFT loss). Model prioritizes preference signal,
    forgets instruction-following. Reduce lambda to 0.5–1.0 to rebalance. Or audit preference data for
    distribution shift—if preferences are on narrow domains, ORPO will overfit. Solution: mix general
    instructions into preference batches.'
- id: 2734707f-4187-4efd-a7f7-83537bb87e31
  type: flip
  front: KTO handles unpaired binary feedback (good/bad). Why is this a production game-changer vs pairwise
    DPO?
  back: 'Cost: pairwise ~$1/pair; binary ~$0.1/annotation. Speed: binary 10x faster to collect. Annotation
    complexity: binary (simple yes/no) vs pairwise (judge which of two is better, requires careful design).
    For production systems with tight iteration cycles (weekly model updates), KTO''s cheap feedback enables
    rapid improvement. DPO reserves for polished, one-time training.'
---

## Intuition
Post-DPO landscape (2026): researchers found DPO works but has limitations (requires pairwise data, reference model, beta sensitivity). Recent methods refine it:

- **SimPO**: Remove reference model, use implicit reward from average log probability. Simpler, fewer hyperparameters.
- **ORPO**: Merge SFT and preference objectives into single loss. Training SFT + preference in parallel instead of sequential.
- **KTO** (Kahneman-Tversky Optimization): Handle **unpaired data** (thumbs-up/down without comparisons). Useful for production systems where binary feedback is cheaper than pairwise annotations.

## Detail

**SimPO (Simple Preference Optimization):**
Key insight: DPO's implicit reward $r^* = \beta \log(\pi/\pi_{ref})$ suggests we don't need separate $\pi_{ref}$. Use **average log probability** of response as implicit reward signal. Loss:
$$\mathcal{L}_{SimPO} = -\log \sigma(\gamma (s(y_w) - s(y_l)))$$
where $s(y) = \frac{1}{|y|} \sum_t \log p_\theta(y_t|x, y_{<t})$ is normalized log prob. $\gamma$ scales the signal. No reference model needed → 30% VRAM saving vs DPO. Slightly simpler, competitive performance.

**ORPO (Odds-Ratio Preference Optimization):**
Merge SFT and preference loss into one objective: don't train SFT, then DPO—combine them. Loss:
$$\mathcal{L}_{ORPO} = \mathcal{L}_{SFT} + \lambda \mathcal{L}_{preference}$$
Single training run does both: model learns to follow instructions AND satisfy preferences. Saves ~30% wall-clock time (1 training run vs 2). Memory: comparable to DPO. Tradeoff: mixing objectives can sometimes hurt convergence (requires careful lambda tuning).

**KTO (Kahneman-Tversky Optimization):**
Key challenge: pairwise preference collection is expensive (~$1/pair). KTO handles **unpaired binary feedback** (good/bad only, no comparisons):
- Collect: (prompt, response, label ∈ {good, bad}). Cheap: $0.1/annotation, fast.
- Insight: Kahneman-Tversky prospect theory—human preferences asymmetric (loss aversion). KTO models this: penalize bad responses more than rewarding good ones.
- Loss (simplified):
$$\mathcal{L}_{KTO} = -\mathbb{E}_{y^+} [\log \sigma(\beta \log p_\theta(y^+))] - \mathbb{E}_{y^-} [\log \sigma(-\beta \log p_\theta(y^-))]$$
No reference model, no pairwise comparisons needed. Huge practical win for production systems. Slightly lower quality than DPO (asymmetric signal), but massive cost/speed advantage.

**Comparison Table (2026):**
| Method | Ref Model | Pairwise Data | VRAM | Speed | Quality | Production Ready |
|--------|-----------|---------------|------|-------|---------|------------------|
| DPO    | Yes       | Yes           | 100% | 100%  | 100%    | Yes              |
| SimPO  | No        | Yes           | 70%  | 105%  | 98%     | Yes              |
| ORPO   | Yes       | Yes           | 90%  | 70%   | 100%    | Yes              |
| KTO    | No        | No (binary)   | 50%  | 110%  | 90%     | Emerging         |

## Common gotchas / interview framings
- **SimPO's avg log prob**: Using average log probability as reward can be noisy for very long sequences. Works well for typical dialog/QA lengths (100–500 tokens).
- **ORPO lambda tuning**: Balance SFT vs preference—$\lambda$ too high favors preferences (forget instruction-following), too low ignores preferences. Default: 0.5, grid-search 0.3–1.0.
- **KTO binary label quality**: Binary feedback is cheaper but noisier than pairwise. Requires more examples for same quality. Also, asymmetric loss (loss aversion) may not hold universally across domains.
- **Reference model still useful**: Even in SimPO/KTO, many practitioners keep frozen reference in loss for stability (hidden KL). True "reference-free" is a gradient more than binary.

## See also
- [[dpo]]
- [[preference-optimization]]
- [[implicit-rewards]]
- [[unpaired-data]]
- [[merged-objectives]]
- [[modern-alignment]]
- [[2026-methods]]

## Sources
See frontmatter `sources:`.

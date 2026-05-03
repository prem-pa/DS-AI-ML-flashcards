---
id: c7d21bb4-87ba-4e9f-88b4-754aec074d50
title: Multiple arms and multi-armed bandit (MAB)
track: data-scientist
topic: ab-testing-experimentation
difficulty: 5
tags:
- multi-arm-test
- mab
- exploration-exploitation
- bandit-algorithms
- multiple-variants
aliases:
- A/B/C test
- multi-armed bandit
- exploration-exploitation
- thompson-sampling
sources:
- url: https://netflixtechblog.com/a-b-testing-and-beyond-improving-the-netflix-streaming-experience-with-experimentation-and-data-5b0ae9295bdf
  label: 'Netflix: Beyond A/B Testing'
- url: https://www.statsig.com/perspectives/power-analysis-ab-testing
  label: 'Statsig: Multiple Arms Guide'
cards:
- id: a8773030-416c-4183-9370-e49a4ef2ee80
  type: flip
  front: ''
  back: ''
- id: 4807f296-39af-410c-ac99-c09cf68bb9a8
  type: flip
  front: ''
  back: ''
- id: 84685b60-9fc1-4a3b-b146-c7741b29d8f5
  type: mcq
  front: ''
  back: ''
  choices:
  - key: a
    text: Variant A (only one significant)
    correct: false
  - key: b
    text: Variant A if Bonferroni-corrected (α = 0.05/3 = 0.017 per comparison)
    correct: false
  - key: c
    text: Variants A and B together
    correct: false
  - key: d
    text: None; multiple comparisons inflate error
    correct: false
- id: 416a1f46-e844-44c8-80b4-b0d12025835b
  type: flip
  front: ''
  back: ''
---

## Intuition

**A/B test**: one treatment vs control. **A/B/C test**: two treatments vs control. **MAB (multi-armed bandit)**: test many variants and adaptively allocate traffic to winners.

## Detail

**Multiple arms (A/B/C, A/B/C/D)**:
- Test 2+ variants against control
- Fixed allocation: 20% each arm
- Pros: test multiple ideas, captures interactions
- Cons: higher sample size needed per arm; multiple comparisons inflate error
- Example: test 3 colors of button (red, blue, green) with control (gray)
- Sample size: 4 arms × 25% = 100k users per arm (400k total for same power as 2-arm test)

**Multiple comparisons correction**:
- If testing k comparisons at α=0.05, family-wise error ≈ 1 - (1-0.05)^k
- Example: 3 comparisons → ~14% error (not 5%)
- Correction: Bonferroni (α = 0.05/3 = 0.017 per comparison), more lenient: Tukey, FDR

**Multi-armed bandit (MAB)**:
- Adaptive allocation: shift traffic toward winners as they emerge
- Algorithms: Thompson sampling, UCB, epsilon-greedy
- Pros: maximize learning + exploitation (win from winners while testing), faster decision
- Cons: complex, requires continuous monitoring, can miss good arms if unlucky early
- Example: start 25% each arm, after 1 week, shift to 10% / 40% / 30% / 20% as you learn

**Thompson sampling**:
- Assume each arm has a true reward
- Maintain posterior for each arm
- At each step, sample from posteriors, pick highest
- Naturally balances exploration (tries bad arms sometimes) and exploitation (mostly picks good arms)

**When to use**:
- **A/B/C test**: Test 2-3 strategic variants, want unbiased comparison, have power budget
- **MAB**: Want to maximize total reward during learning (less lost traffic on losers), can't wait for full test

## Common gotchas / interview framings
- "We test 5 variants. Should we compare all to control or pairwise?" → Compare all to control; multiple comparisons. Use Bonferroni or pre-register which comparisons matter (e.g., only variants 1,2 vs control)
- "MAB is better than A/B testing because it exploits winners." → MAB exploits during test (learn faster), but A/B test gives cleaner statistical answer. A/B better for decision-making; MAB better for revenue during test
- "Can we use MAB and then apply frequentist stats?" → Careful. Adaptive allocation biases estimates. Need special analysis (inverse probability weighting) or use Bayesian sequential methods built for MAB
- "Should we stop arms early if they're clearly bad?" → Risk: stopping arms removes samples for estimation (bias). Better: keep all arms until end, or use Bayesian decision rule (posterior P(arm > control) < 5% → stop)

## See also
- [[multiple-arms-and-multi-armed-bandit-mab]]
- [[sequential-testing-and-peek-penalties]]

## Sources
See frontmatter `sources:`.

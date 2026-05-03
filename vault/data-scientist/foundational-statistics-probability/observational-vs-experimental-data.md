---
id: 85ff4abf-0393-4fe3-bf57-747af10fbcaf
title: Observational vs experimental data
track: data-scientist
topic: foundational-statistics-probability
difficulty: 3
tags:
- causal-inference
- confounding
- randomization
- rct
- study-design
aliases:
- causal-data
- rct-vs-obs
- confounding-control
sources:
- url: https://link.springer.com/book/10.1007/978-0-387-21736-9
  label: Wasserman, L. All of Statistics (Ch. 8)
- url: https://www.statlearning.com/
  label: James et al. An Introduction to Statistical Learning (Ch. 13)
cards:
- id: 22573123-61bb-44c1-b378-e6798af898e6
  type: flip
  front: An observational study finds that people who exercise have 20% lower mortality. Does this mean
    exercise causes 20% lower mortality?
  back: ''
- id: 86e45ed7-b440-4dcc-b8eb-3b0edf34fa13
  type: flip
  front: An RCT randomizes students to online vs. in-person learning. Online students have better outcomes.
    Can you conclude online learning is better?
  back: ''
- id: c28dd2ab-8dfc-46b1-8835-39f68e382640
  type: flip
  front: Explain why randomization breaks confounding and enables causal inference. Write E[Y(1)] in terms
    of observables.
  back: ''
- id: 69c543d3-b119-4d68-b378-4d0b936c7f7f
  type: flip
  front: An RCT recruits from a university; 20% of randomized participants drop out. Does ITT (Intention-To-Treat)
    analysis still give causal estimates?
  back: ''
- id: d5af0bc3-32fe-4d9b-81b5-493ade5d3f49
  type: mcq
  front: Which method can identify causal effects without randomization?
  back: ''
  choices:
  - key: a
    text: Multiple regression on confounders
    correct: false
  - key: b
    text: Propensity score matching (assuming no hidden confounders)
    correct: false
  - key: c
    text: Observational study with observed confounders only
    correct: false
  - key: d
    text: All of the above assume no hidden confounding
    correct: true
---

## Intuition
RCTs (Randomized Controlled Trials) randomly assign treatment, breaking the link between confounders and treatment—enabling causal inference. Observational studies measure naturally occurring treatment/exposure without randomization; treatment is often correlated with confounders, so association ≠ causation. Example: online learners (treatment) have better job outcomes; but they're self-selected (motivated, educated), so confounding makes treatment seem effective. Only RCTs reliably isolate causal effects. When RCTs are infeasible (long-term health, ethical constraints), researchers use observational data with causal inference methods (propensity score matching, instrumental variables, difference-in-differences), but assumptions are required and results are sensitive to unobserved confounding.

## Detail
RCT: treatment T is assigned independently of potential outcomes and confounders. Estimand: Average Treatment Effect (ATE) = E[Y(1)] - E[Y(0)] (difference in potential outcomes). By randomization, $E[Y(1)] = E[Y(1)|T=1]$ (observable), so ATE is identified. Standard errors come from sampling variability; confidence intervals are straightforward.

Observational: treatment is correlated with confounders Z. Estimand remains ATE, but $E[Y(1)|T=1] \neq E[Y(1)]$ because treated units differ from untreated. Methods: (1) **Stratification**: within each level of Z, treatment is quasi-randomized. (2) **Propensity score**: model P(T=1|X), then match/weight treated and untreated by propensity (pseudo-randomization). (3) **Instrumental variables**: find a variable Z that affects T but not Y (except via T), enabling causal inference. (4) **Difference-in-differences**: use parallel trends and panel data to difference out unobserved confounders.

Key limitation of observational methods: requires no hidden (unmeasured) confounding. Unobserved confounders (ability, motivation) still bias estimates; robustness checks (sensitivity analysis) quantify bias risk.

## Common gotchas / interview framings
- Correlation ≠ causation; observational associations can reverse direction if you control for the right confounder (Simpson's Paradox)
- RCTs have high internal validity (estimates are causal) but may lack external validity (sample is non-representative)
- Propensity score matching doesn't handle unobserved confounding; neither do other observational methods
- Selection bias in RCTs (differential attrition) can reintroduce confounding; analyze by intention-to-treat (ITT) rather than per-protocol

## See also
- [[randomized-controlled-trial]]
- [[observational-study]]
- [[confounding]]
- [[causal-inference]]
- [[do-calculus]]

## Sources
See frontmatter `sources:`.

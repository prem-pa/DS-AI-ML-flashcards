---
id: e7a61a22-325a-43b5-9dac-361c3f6ef4b0
title: Stratified sampling and blocking
track: data-scientist
topic: foundational-statistics-probability
difficulty: 3
tags:
- experimental-design
- variance-reduction
- randomization
- confounding
- efficiency
aliases:
- stratification
- blocking
- design-of-experiments
sources:
- url: https://ocw.mit.edu/courses/18-05-introduction-to-probability-and-statistics-spring-2022/
  label: 'MIT 18.05: Introduction to Probability and Statistics'
- url: https://link.springer.com/book/10.1007/978-0-387-21736-9
  label: Wasserman, L. All of Statistics (Ch. 8)
cards:
- id: c981be72-8865-4659-a6bc-5cd4a12ae571
  type: flip
  front: You want to estimate the average height of people in a country. The population has 60% female,
    40% male. Should you use stratified sampling?
  back: ''
- id: 5c5f70ab-1959-44ae-9266-b7801b76b3d7
  type: flip
  front: In an RCT, you block by baseline disease severity (mild, moderate, severe). How does this improve
    causal inference?
  back: ''
- id: 6df20fb8-eca0-498c-89b2-9daea91f13b1
  type: flip
  front: Show that stratified sampling has lower variance than simple random sampling when strata have
    different means.
  back: ''
- id: 155328f7-6919-4ea2-94db-e3428297d303
  type: flip
  front: You conduct an RCT and block by baseline severity. After treatment, you find severity has changed.
    Should you re-block by post-treatment severity?
  back: ''
- id: f49b0784-9d04-424b-997b-dac2d0967156
  type: mcq
  front: 'Stratified sampling reduces variance most when:'
  back: ''
  choices:
  - key: a
    text: Strata have similar means
    correct: false
  - key: b
    text: Strata have very different means
    correct: true
  - key: c
    text: Sample sizes are equal
    correct: false
  - key: d
    text: Strata are small
    correct: false
---

## Intuition
Stratified sampling partitions the population into known subgroups (strata: age groups, regions, income levels) and samples randomly within each stratum. This ensures representation and reduces variance compared to simple random sampling. Blocking in experiments divides participants into blocks (e.g., by hospital, day, baseline severity) and randomly assigns treatments within each block. Both reduce noise from known confounders, improving precision and power without increasing sample size. Used in RCTs, surveys, and observational studies to isolate causal effects from confounding.

## Detail
Stratified sampling: if stratum i has size $N_i$ and you allocate $n_i$ samples proportionally ($n_i = n \cdot N_i / N$), the stratified estimator $\hat{\mu}_{st} = \sum_i W_i \bar{x}_i$ (weighted average of stratum means) has lower variance than simple random sampling if strata have different means. Variance reduction is $\text{Var}(\hat{\mu}_{st}) = \frac{\sigma^2_w}{n} - \frac{\sigma^2_b}{N}$ where $\sigma^2_w$ is within-stratum variance and $\sigma^2_b$ is between-stratum variance. Large $\sigma^2_b$ → large variance reduction.

Blocking (in experiments): randomize treatments within blocks so treatment assignment is balanced across block levels. Example: if patients differ in disease severity (low, medium, high), block by severity and randomize treatment within each block. This ensures both treatment and control groups have similar severity distribution, eliminating severity as a confounder. Equivalent to including block as a covariate in regression.

Pros: Reduced variance, balanced covariate distribution, more precise treatment effect estimates. Cons: Requires knowing stratification variable in advance; if many strata, sample sizes within strata become small.

## Common gotchas / interview framings
- Stratification is not a substitute for randomization; bias still occurs if sampling is non-random within strata
- Over-stratification: too many strata → small stratum sample sizes → loss of precision (curse of dimensionality)
- Blocking by a post-treatment variable (measured after treatment) induces bias (collider bias)
- Post-stratification (adjusting after data collection) can improve estimates if strata are defined a priori, but risks p-hacking if multiple post-strata are tried

## See also
- [[stratified-sampling]]
- [[blocking]]
- [[randomized-controlled-trial]]
- [[confounder-control]]
- [[experimental-design]]

## Sources
See frontmatter `sources:`.

---
id: 54149690-764a-463e-8e5a-fc3fc0933392
title: Hierarchical models
track: data-scientist
topic: probability-bayesian-thinking
difficulty: 5
tags:
- multilevel-models
- shrinkage
- partial-pooling
- group-structure
- variance-components
aliases:
- multilevel models
- mixed-effects models
- random-effects models
sources:
- url: https://vioshyvo.github.io/Bayesian_inference/hierarchical-models.html
  label: 'Chapter 6: Hierarchical Models - Bayesian Inference 2019'
- url: https://bayesball.github.io/BOOK/bayesian-hierarchical-modeling.html
  label: 'Chapter 10: Bayesian Hierarchical Modeling'
cards:
- id: 9a8ae11b-87a6-4bf1-a9f1-230b860dbbce
  type: flip
  front: In a hierarchical model for test scores across schools, explain what "partial pooling" means
    and why it improves estimation compared to (1) no pooling (separate models per school) and (2) complete
    pooling (all schools the same).
  back: '**No pooling:** Each school''s mean is estimated from its own data; high variance for small schools,
    no information sharing. **Complete pooling:** Assume all schools identical; biased if schools truly
    differ. **Partial pooling (hierarchical):** Estimates are pulled toward a common mean weighted by
    school size and between-school variance. Small schools shrink more toward the global mean (reducing
    variance), large schools rely more on own data (small bias). This balances bias-variance tradeoff
    and leverages the assumption that schools are similar but not identical.'
- id: 8b577c28-2d17-4923-9dc3-8e8b26b9f4a8
  type: mcq
  front: In a 2-level model where students $i$ are nested in schools $j$, the shrinkage weight for school
    $j$ is $w_j = \tau^2/(\tau^2 + \sigma^2/n_j)$. What happens if $n_j$ is small?
  back: 'If $n_j$ is small, $\sigma^2/n_j$ is large, so the denominator $\tau^2 + \sigma^2/n_j$ is larger,
    making $w_j$ smaller. This means school $j$''s estimate is shrunk more strongly toward the global
    mean. This is sensible: small samples are less informative, so we trust the global prior more.'
  choices:
  - key: a
    text: $w_j$ increases (more weight on school mean)
    correct: false
  - key: b
    text: $w_j$ decreases (more weight on global mean)
    correct: true
  - key: c
    text: $w_j$ stays the same regardless of $n_j$
    correct: false
  - key: d
    text: $w_j$ becomes negative
    correct: false
- id: 27466a3e-3547-464b-9d98-a27b09cf81e2
  type: flip
  front: In a hierarchical model with variance components, explain how the between-group variance $\tau^2$
    and within-group variance $\sigma^2$ control the degree of pooling.
  back: The shrinkage factor toward the global mean is $w = \tau^2 / (\tau^2 + \sigma^2/n)$. When $\tau^2
    \gg \sigma^2$ (groups very different), $w \approx 1$ (little pooling, trust group data). When $\tau^2
    \ll \sigma^2$ (groups similar), $w \approx 0$ (strong pooling, trust global mean). The Bayesian model
    learns $\tau^2$ from the data via the posterior, automatically adjusting pooling. If groups happen
    to be similar in the data, the posterior pulls $\tau^2$ toward 0, inducing strong shrinkage; if they
    diverge, $\tau^2$ grows and pooling weakens.
- id: c02fb04b-f870-417f-aa8c-8b50a2d057fa
  type: mcq
  front: In a hierarchical Poisson regression with group-level intercepts, how would you handle a group
    with zero observations?
  back: 'Hierarchical models handle sparse data gracefully. A group with zero observations has no group-specific
    likelihood term, so its posterior is the prior: $P(\theta_j|0\text{ obs}) = p(\theta_j|\mu,\tau)$.
    The posterior mean is shrunk fully toward $\mu$, and the posterior variance is $\tau^2$ (the between-group
    variance). This borrows strength from other groups, providing a regularized estimate rather than undefined
    ML.'
  choices:
  - key: a
    text: Drop the group; no data means no estimate
    correct: false
  - key: b
    text: Use the hyperprior mean as the estimate, accounting for information from other groups
    correct: true
  - key: c
    text: Set the intercept to $-\infty$ to encode no information
    correct: false
  - key: d
    text: Use maximum likelihood, which gives undefined intercept
    correct: false
- id: a52627b6-950e-49af-ae18-35cb8f3f1020
  type: flip
  front: Design a 3-level hierarchical model for patients nested in hospitals nested in health systems.
    Write the model structure and explain what each level captures.
  back: '$y_{ijk} | \theta_{jk} \sim \text{Likelihood}(\theta_{jk})$: patient $i$ in hospital $j$ in system
    $k$, with patient-level variance $\sigma^2$. $\theta_{jk} | \mu_{k}, \tau_j \sim \mathcal{N}(\mu_k,
    \tau_j^2)$: hospital effect $\theta_{jk}$ drawn from system mean $\mu_k$ with hospital-level variance
    $\tau_j^2$. $\mu_k | \mu_0, \tau_s \sim \mathcal{N}(\mu_0, \tau_s^2)$: system mean $\mu_k$ drawn from
    global mean with system-level variance $\tau_s^2$. **Levels:** Patient variation (Level 1), hospital
    effects $\theta_{jk}$ (Level 2), system effects $\mu_k$ (Level 3), global mean $\mu_0$ (Level 4).
    Patients borrow from hospitals, hospitals borrow from systems, systems borrow from global. Pooling
    degree depends on variance ratios at each level.'
---

## Intuition
Hierarchical models pool information across groups (e.g., students in schools, patients in hospitals) by assuming each group's parameter comes from a common distribution. This induces partial pooling: estimates are pulled toward the group mean, improving small-sample estimates while allowing group-level variation. Hierarchical structure is natural for many datasets and encodes domain knowledge about similarity across units.

## Detail
Basic two-level structure:
$$y_{ij} | \theta_j \sim p(y_{ij}|\theta_j) \quad \text{(data model)}$$
$$\theta_j | \mu, \tau \sim p(\theta_j|\mu, \tau) \quad \text{(group model)}$$
$$\mu, \tau \sim p(\mu, \tau) \quad \text{(hyperprior)}$$

Partial pooling weight between group estimate and global mean depends on: group sample size, within-group variance, and between-group variance. Small $\tau$ (groups similar) → more pooling. Large $\tau$ (groups different) → less pooling.

**Advantages:** Borrows strength across groups, reduces variance in small-sample regimes, infers uncertainty in group parameters (not just point estimates), natural for repeated measures.

**Extensions:** Multiple levels (students in classes in schools), multiple grouping factors (crossed designs), nonlinear models with hierarchical structure, Bayesian additive regression trees (BART).

## Common gotchas / interview framings
- Ignoring hierarchical structure and treating groups as independent, losing efficiency gains
- Setting hyperpriors that are too weak (e.g., very diffuse) and losing shrinkage benefits
- Forgetting that group-level parameters have posterior uncertainty; point estimates underestimate variability
- Misinterpreting group effect estimates: a small estimate may reflect strong shrinkage toward zero, not zero effect
- In unbalanced designs, failing to account for different group sizes in inference (smaller groups shrink more)

## See also
- [[hierarchical_bayes]]
- [[multilevel_model]]
- [[random_effects]]
- [[variance_components]]
- [[empirical_bayes]]
- [[shrinkage_estimator]]

## Sources
See frontmatter `sources:`.

---
id: 011a62fc-956c-483f-b3ea-97e05c609a7c
title: Gaussian Mixture Models (GMM)
track: data-scientist
topic: classical-machine-learning
difficulty: 3
tags:
- clustering
- probabilistic-model
- expectation-maximization
- soft-assignment
aliases:
- EM algorithm
- soft clustering
- mixture model
sources:
- url: https://scikit-learn.org/stable/modules/mixture.html
  label: scikit-learn Gaussian Mixture
- url: https://www.coursera.org/learn/machine-learning
  label: Andrew Ng ML Specialization
cards:
- id: a8454d3e-2215-4196-b9be-5969ca697a88
  type: flip
  front: GMM model. Role of $\pi_c$?
  back: '$p(x)=\sum \pi_c N(x|\mu_c,\Sigma_c)$. $\pi_c$: prior weight (proportion from cluster $c$). $\sum\pi_c=1$.'
- id: a5b40c7d-f23a-464c-940a-70de03d1b40d
  type: flip
  front: EM E-step, M-step GMM?
  back: 'E: responsibility $\gamma_{ic}$ (soft assignment). M: update $\pi_c$=mean $\gamma$, $\mu_c$=weighted
    mean, $\Sigma_c$=weighted cov. Repeat.'
- id: 1004cece-b299-4d26-8099-08edfe740a1e
  type: flip
  front: GMM vs k-means?
  back: 'K-means: hard assign, minimize variance. GMM: soft assign, maximize likelihood. GMM models covariance
    (shape). GMM flexible but slower.'
- id: 5a74e343-a2d2-40a4-8315-d448dd7ff530
  type: flip
  front: BIC in GMM model selection?
  back: $BIC=-2\log L + k\log n$. Penalizes complexity by $\log n$. Lower BIC better. Stricter than AIC.
---

## Intuition
$k$ Gaussian distributions mixture. Soft assignment (probabilistic membership). EM fits: E-step=responsibility, M-step=update params.

## Detail
$$p(x)=\sum_c \pi_c N(x|\mu_c,\Sigma_c)$$
**EM:** E: $\gamma_{ic}=\pi_c N(x_i|\mu_c,\Sigma_c)/\sum \pi_{c'}N(x_i|\mu_{c'},\Sigma_{c'})$. M: update $\pi_c,\mu_c,\Sigma_c$ from responsibilities.

## See also
- [[em-algorithm]]
- [[soft-clustering]]
- [[bic]]
- [[model-selection]]

## Sources
See frontmatter `sources:`.

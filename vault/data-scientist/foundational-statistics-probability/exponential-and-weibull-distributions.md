---
id: 236a7ad5-f786-4a36-927a-c9ccfc1868ef
title: Exponential and Weibull distributions
track: data-scientist
topic: foundational-statistics-probability
difficulty: 3
tags:
- time-to-event
- survival-analysis
- reliability
- failure-rate
- continuous-distribution
aliases:
- waiting-time
- survival-distribution
- lifetime-modeling
sources:
- url: https://link.springer.com/book/10.1007/978-0-387-21736-9
  label: Wasserman, L. All of Statistics (Ch. 3)
- url: https://ocw.mit.edu/courses/18-05-introduction-to-probability-and-statistics-spring-2022/
  label: 'MIT 18.05: Introduction to Probability and Statistics'
cards:
- id: 3e99e2a4-8a92-4252-940c-d7ce0277905e
  type: flip
  front: Customer churn times follow an exponential distribution with λ = 0.1/month. What is the mean
    lifetime?
  back: ''
- id: 95bb20ef-69a9-49a7-a588-1900c94519f8
  type: flip
  front: 'Prove the memoryless property of exponential: P(T > s+t | T > s) = P(T > t).'
  back: ''
- id: b37ba6f0-4a07-47fb-a797-e142386179aa
  type: flip
  front: Component lifetimes follow Weibull(k=2, λ=5). Is the failure rate increasing or decreasing with
    time?
  back: ''
- id: e79b5a2b-e328-4d10-b662-9098cbd51533
  type: flip
  front: In a clinical trial, 100 patients enroll. After 2 years, 40 experience the event; 60 have not
    yet experienced it and are still enrolled (censored data). Can you use a simple mean calculation to
    estimate E[T]?
  back: ''
- id: 9b1b10ae-386e-44bf-88d4-c403f07a5bee
  type: mcq
  front: 'The Weibull distribution reduces to exponential when:'
  back: ''
  choices:
  - key: a
    text: k → 0
    correct: false
  - key: b
    text: k = 1
    correct: true
  - key: c
    text: k → ∞
    correct: false
  - key: d
    text: λ = 0
    correct: false
---

## Intuition
The **Exponential distribution** models time until a rare event (failures, customer churn, webpage load time) with a constant hazard rate (memoryless). The **Weibull distribution** generalizes exponential to allow hazard rate to increase, decrease, or stay constant with time—used in reliability engineering (component lifetimes), survival analysis, and financial risk. Both are right-skewed with heavier tails than normal, reflecting realistic time-to-event scenarios where most events cluster early but rare long-duration outcomes are possible.

## Detail
**Exponential**: $f(t) = \lambda e^{-\lambda t}$ for $t \geq 0$. Mean $E[T] = 1/\lambda$, variance $\text{Var}(T) = 1/\lambda^2$. **Memoryless property**: $P(T > s + t | T > s) = P(T > t)$—the past does not affect future; used in queueing theory. Survival function: $S(t) = P(T > t) = e^{-\lambda t}$. Hazard rate: $h(t) = \lambda$ (constant).

**Weibull**: $f(t) = \frac{k}{\lambda}\left(\frac{t}{\lambda}\right)^{k-1} e^{-(t/\lambda)^k}$. Shape parameter k controls hazard: k < 1 (decreasing, e.g., "infant mortality"), k = 1 (constant, same as exponential), k > 1 (increasing, e.g., aging). Mean $\approx \lambda \Gamma(1 + 1/k)$. Used in reliability; components fail more frequently as they age (k > 1).

Connection: Exponential is Weibull with k = 1. Weibull accommodates a wider range of failure-rate patterns, making it more flexible for real-world time-to-event data.

## Common gotchas / interview framings
- Exponential's memoryless property is unrealistic for many survival scenarios (disease recurrence, equipment aging); use Weibull instead
- Censoring (partial observation) is common in survival analysis: if a patient leaves the study alive, we know T > observation time but not T exactly
- Hazard rate ≠ density; hazard is the instantaneous rate of failure given survival to that time
- Right-skew and heavy tails mean outliers are expected; transformation (log T) may help with model diagnostics

## See also
- [[exponential-distribution]]
- [[weibull-distribution]]
- [[survival-analysis]]
- [[memoryless-property]]
- [[hazard-rate]]

## Sources
See frontmatter `sources:`.

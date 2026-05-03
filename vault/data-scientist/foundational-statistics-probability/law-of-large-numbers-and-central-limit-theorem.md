---
id: 34148c1b-1c63-435d-a0aa-a1c48ce59659
title: Law of large numbers and Central Limit Theorem
track: data-scientist
topic: foundational-statistics-probability
difficulty: 3
tags:
- convergence
- limit-theorems
- sample-size
- normality
- asymptotics
aliases:
- LLN
- CLT
- asymptotic-normality
sources:
- url: https://link.springer.com/book/10.1007/978-0-387-21736-9
  label: Wasserman, L. All of Statistics (Ch. 4-5)
- url: https://ocw.mit.edu/courses/18-05-introduction-to-probability-and-statistics-spring-2022/
  label: 'MIT 18.05: Lecture Notes (Limit Theorems)'
cards:
- id: 163d0409-ec49-4df0-a66f-f201c779264d
  type: flip
  front: Explain why the Central Limit Theorem allows us to use t-tests on non-normal data.
  back: ''
- id: 453d0f45-4e2b-46a8-8a9c-579ed9f6baee
  type: flip
  front: State the Central Limit Theorem formally and explain what $\xrightarrow{d}$ (convergence in distribution)
    means.
  back: ''
- id: 046aeffd-e8e5-4cfa-8e3b-7743e3c1340c
  type: flip
  front: A dataset has n=30, mean=100, and SD=50. Can you assume the mean is normally distributed?
  back: ''
- id: 472a4a0a-fcdb-4065-8442-4b55c1e0eda1
  type: flip
  front: You compute a 95% CI using the normal approximation on n=1000 non-normal data and it's too narrow.
    Why?
  back: ''
- id: 80135f05-ac18-41d0-8d7d-4ef1d8696175
  type: mcq
  front: 'If a population is exponentially distributed, the Central Limit Theorem tells us:'
  back: ''
  choices:
  - key: a
    text: Sample data will be approximately normal
    correct: false
  - key: b
    text: Sample means will be approximately normal
    correct: true
  - key: c
    text: All samples must be very large
    correct: false
  - key: d
    text: The median will follow the normal distribution
    correct: false
---

## Intuition
The **Law of Large Numbers (LLN)** says the sample mean converges to the true mean as n → ∞: $\bar{X}_n \to \mu$ in probability. This justifies using sample statistics to estimate population parameters. The **Central Limit Theorem (CLT)** says the (properly scaled) sample mean is approximately normal for large n, regardless of the underlying distribution. This is the foundation for confidence intervals, hypothesis tests, and why normal approximations work so broadly.

## Detail
**Weak LLN**: $P(|\bar{X}_n - \mu| > \epsilon) \to 0$ as $n \to \infty$ (convergence in probability). **Strong LLN**: $\bar{X}_n \to \mu$ almost surely. Implies that sample moments converge to population moments.

**CLT**: If $X_1, X_2, \ldots$ are iid with mean $\mu$ and variance $\sigma^2$, then $\frac{\bar{X}_n - \mu}{\sigma/\sqrt{n}} \xrightarrow{d} N(0,1)$ as $n \to \infty$. Equivalently, $\bar{X}_n \approx N(\mu, \sigma^2/n)$ for large n. This holds **even if the underlying distribution is not normal** (uniform, exponential, etc.)—one of the most powerful results in statistics.

Implications: t-tests are valid even on non-normal data if n is large enough; confidence intervals based on the normal distribution work broadly; sample sums are approximately normal. The "large enough" depends on skewness/kurtosis (more skewed → larger n needed).

## Common gotchas / interview framings
- CLT does NOT say the data are normal; it says the **sample mean's distribution** is normal
- Heavy-tailed distributions (infinite variance) violate CLT assumptions; convergence is slower
- Dependent data (time series) has different asymptotics; CLT needs modification
- Multiple testing: CLT justifies normal approximation for individual tests, but not for controlling FWER across many tests

## See also
- [[law-of-large-numbers]]
- [[central-limit-theorem]]
- [[convergence-in-probability]]
- [[convergence-in-distribution]]
- [[asymptotic-theory]]

## Sources
See frontmatter `sources:`.

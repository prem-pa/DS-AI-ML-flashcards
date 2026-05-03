---
id: 8eb03b45-4e0b-46bc-9dfb-116288aefd95
title: Sampling bias and representative samples
track: data-scientist
topic: foundational-statistics-probability
difficulty: 1
tags:
- study-design
- sampling-method
- selection-bias
- survey-bias
- external-validity
aliases:
- selection-bias
- non-response-bias
- sampling-error
sources:
- url: https://ocw.mit.edu/courses/18-05-introduction-to-probability-and-statistics-spring-2022/
  label: 'MIT 18.05: Introduction to Probability and Statistics'
- url: https://link.springer.com/book/10.1007/978-0-387-21736-9
  label: Wasserman, L. All of Statistics (Ch. 8)
cards:
- id: b0cfc505-3d76-4920-ad0d-679326680a0e
  type: flip
  front: An online survey of 10,000 people finds 80% prefer Product A. Why might this be biased, and how
    would you investigate?
  back: ''
- id: 7948ac04-71a3-4b77-b6de-914ee2ef2f74
  type: flip
  front: A weight-loss study recruits volunteers from a fitness forum. Do you expect the results to generalize
    to the general population?
  back: ''
- id: c3586526-aeab-4d5b-9188-06fd679d0ef3
  type: flip
  front: You analyze historical stock returns and find a strong positive abnormal return to a trading
    strategy. Why might this be spurious?
  back: ''
- id: dfcbbd9a-1716-43fc-9453-b81b6556aa4b
  type: flip
  front: A company measures customer satisfaction via their mobile app. What biases might exist?
  back: ''
- id: 49bbb5b9-b1ff-40ee-9f9c-a6fc5e8ca8db
  type: mcq
  front: Which sampling method best eliminates selection bias?
  back: ''
  choices:
  - key: a
    text: Convenient sampling
    correct: false
  - key: b
    text: Volunteer sampling
    correct: false
  - key: c
    text: Random sampling
    correct: true
  - key: d
    text: Quota sampling
    correct: false
---

## Intuition
A sample is representative if it captures the distribution of the population. Sampling bias occurs when the sampling mechanism is not random or is correlated with the outcome, making estimates systematically wrong. Examples: (1) online survey (nonresponse bias: busy people don't respond), (2) clinical trial (selection bias: sickest patients volunteer), (3) historical data (survivorship bias: failed companies disappear). Bias cannot be fixed by collecting more data; it is systematic error in the sampling process itself. Awareness of bias sources drives study design choices and generalization bounds.

## Detail
Random sampling: each unit has equal probability of selection, eliminating systematic bias. Stratified sampling: divide population into strata (age groups, regions), then randomly sample within strata to ensure representation. Non-response bias: selected units decline to participate; if refusers differ from responders, estimates are biased. Selection bias: the mechanism for entering the study is correlated with the outcome (e.g., health conscious people buy supplements; confounding treatment effect).

Types of bias: (1) **Selection bias**: who is sampled. (2) **Measurement bias**: how variables are measured. (3) **Non-response bias**: who doesn't participate. (4) **Survivorship bias**: extinct entities (bankrupt firms) are missing. (5) **Volunteering bias**: voluntary participants differ from population. Bias leads to inconsistent estimators: $\hat{\theta} \not\to \theta$ even as n → ∞.

## Common gotchas / interview framings
- A large sample size does not cure bias; a million responses from a biased survey are still biased
- Simpson's Paradox arises partly from non-representative subsampling (confounding by group size/composition)
- Observational data often has selection bias (people self-select treatment); RCTs randomize to break the correlation
- Data availability is not the same as data quality; convenience samples (Twitter, Reddit) are fast but biased

## See also
- [[sampling-bias]]
- [[selection-bias]]
- [[representative-sample]]
- [[generalization]]
- [[study-design]]

## Sources
See frontmatter `sources:`.

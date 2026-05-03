---
id: 35d361ba-639d-47e1-83e9-436b16acd25b
title: Bayes' theorem and conditional probability
track: data-scientist
topic: foundational-statistics-probability
difficulty: 3
tags:
- probability-rules
- conditional-probability
- theorem
- inference
- classification
aliases:
- bayes-rule
- posterior
- posterior-odds
sources:
- url: https://ocw.mit.edu/courses/18-05-introduction-to-probability-and-statistics-spring-2022/
  label: 'MIT 18.05: Introduction to Probability and Statistics'
- url: https://link.springer.com/book/10.1007/978-0-387-21736-9
  label: Wasserman, L. All of Statistics (Ch. 1)
cards:
- id: 5ca6e4fe-6b97-4adc-9e3e-939a43f06b66
  type: flip
  front: Derive Bayes' theorem from the definition of conditional probability.
  back: ''
- id: c123c587-8d88-4c1f-8aac-b12e175037ab
  type: flip
  front: A medical test has sensitivity 99% (true positive rate) and specificity 98%. Disease prevalence
    is 0.1%. What is P(disease | positive)?
  back: ''
- id: 37bb0b2e-c0b7-420e-85d1-699098cc8c0a
  type: flip
  front: 'A lawyer argues: ''The defendant''s DNA matches the crime scene (match prob. = 0.01%). Therefore,
    guilt probability = 99.99%.'' What''s wrong?'
  back: ''
- id: f7bcd012-2ea1-4892-86e5-abdda03f2a29
  type: flip
  front: Write the law of total probability for P(E) in Bayes' theorem context with hypotheses H₁, H₂,
    H₃.
  back: ''
- id: 81da4957-e6fb-47c3-a275-46aa0d2f3f75
  type: mcq
  front: Which statement is true for independent events A and B?
  back: ''
  choices:
  - key: a
    text: P(A|B) = P(A)
    correct: true
  - key: b
    text: P(A∩B) = P(A) + P(B)
    correct: false
  - key: c
    text: P(A|B) = P(B|A)
    correct: false
  - key: d
    text: P(A∪B) = 1
    correct: false
---

## Intuition
Conditional probability $P(A|B)$ is the probability of A given B has occurred. Bayes' theorem inverts this: $P(B|A) = \frac{P(A|B)P(B)}{P(A)}$, enabling us to update beliefs from evidence. In ML, this is core to classification (posterior prob. of class given features), medical diagnostics (risk given test result), and Bayesian inference (posterior given data).

## Detail
Conditional probability: $P(A|B) = \frac{P(A \cap B)}{P(B)}$ (assuming $P(B) > 0$). Bayes' theorem: $P(H|E) = \frac{P(E|H)P(H)}{P(E)}$, where H is hypothesis (class), E is evidence (data). The law of total probability gives $P(E) = \sum_i P(E|H_i)P(H_i)$. In words: posterior $\propto$ likelihood × prior. Chain rule: $P(A, B, C) = P(A)P(B|A)P(C|A,B)$.

Key: $P(A|B) = 1$ does not imply $P(B|A) = 1$ (inverse fallacy). Example: $P(\text{positive test}|\text{disease}) = 0.99$ does not mean $P(\text{disease}|\text{positive test}) = 0.99$ if disease is rare (low prior). This is why screening tests have high false positive rates in low-prevalence populations.

## Common gotchas / interview framings
- Confusing $P(A|B)$ with $P(B|A)$: the "inverse fallacy" appears in medical test interpretation, legal reasoning, and model evaluation
- Ignoring base rates (priors): high likelihood alone doesn't guarantee high posterior without strong prior
- Independence vs. conditional independence: X ⊥ Y does not imply X ⊥ Y | Z (or vice versa)

## See also
- [[conditional-probability]]
- [[bayes-theorem]]
- [[likelihood]]
- [[posterior-odds]]
- [[classification]]

## Sources
See frontmatter `sources:`.

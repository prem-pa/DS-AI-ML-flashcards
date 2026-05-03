---
id: fd0f9d8b-6ab3-4d43-8afa-5a4de709d246
title: Naive Bayes
track: data-scientist
topic: classical-machine-learning
difficulty: 3
tags:
- classification
- probabilistic-model
- text-classification
- feature-independence
aliases:
- conditional independence
- Bayes classifier
- MultinomialNB
sources:
- url: https://scikit-learn.org/stable/modules/naive_bayes.html
  label: scikit-learn Naive Bayes
- url: https://www.coursera.org/learn/machine-learning
  label: Andrew Ng ML Specialization
cards:
- id: 11a88bef-8e67-4431-87ba-ac35d23072ed
  type: flip
  front: Bayes' theorem + naive assumption. Why 'naive'?
  back: '$P(y|X)\propto P(X|y)P(y)$. Naive: $P(X|y)=\prod P(X_j|y)$. Naive despite feature correlation
    violates assumption, robust in high-dim (correlations dilute).'
- id: 89b745b3-d344-4418-8a9c-f55ca6e03dd6
  type: flip
  front: Naive Bayes decision rule. Avoid underflow?
  back: '$\hat{y}=\arg\max P(y=c)\prod P(X_j|y=c)$. Underflow: take logs: $\hat{y}=\arg\max[\log P(y=c)
    + \sum\log P(X_j|y=c)]$.'
- id: 0613edf2-8cd9-4a6b-9795-9572d4e23e85
  type: flip
  front: Laplace smoothing. When necessary?
  back: Adds $\alpha$ to counts. Necessary for sparse data; without it, $P=0$ zeros posterior. Common
    in NLP.
- id: bf698cf2-7a32-454c-b168-4ad0ffc4eae1
  type: mcq
  front: 'Word counts in docs: Gaussian/Bernoulli/Multinomial NB?'
  back: Multinomial models word frequency distribution. For counts, not binary.
  choices:
  - key: a
    text: Gaussian.
    correct: false
  - key: b
    text: Bernoulli.
    correct: false
  - key: c
    text: Multinomial.
    correct: true
  - key: d
    text: Any.
    correct: false
---

## Intuition
Naive Bayes assumes feature independence given class. Despite strong assumption, surprisingly effective for text and high-dim data.

## Detail
**Bayes:** $P(y|X) = P(X|y)P(y)/P(X)$. **Naive:** $P(X|y) = \prod_j P(X_j|y)$. **Classifier:** $\hat{y}=\arg\max_c \log P(y=c) + \sum_j \log P(X_j|y=c)$. **Laplace smoothing:** Add $\alpha$ to avoid zero.

## See also
- [[bayes-theorem]]
- [[conditional-independence]]
- [[text-classification]]

## Sources
See frontmatter `sources:`.

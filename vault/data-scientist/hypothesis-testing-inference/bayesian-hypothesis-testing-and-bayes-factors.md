---
id: 0487da33-0ae0-4bf5-85f6-151cc3891cc6
title: Bayesian hypothesis testing and Bayes factors
track: data-scientist
topic: hypothesis-testing-inference
difficulty: 3
tags:
- Bayes factor
- Bayesian testing
- model comparison
- posterior odds
- likelihood ratio
aliases:
- BF
- Bayes factor
- model evidence
- posterior odds
sources:
- url: https://stat20.berkeley.edu/spring-2024/4-generalization/04-hypothesis-tests/notes.html
  label: 'Berkeley Stat 20: Bayesian Model Comparison'
- url: https://online.stat.psu.edu/statprogram/reviews/statistical-concepts/hypothesis-testing/p-value-approach
  label: 'Penn State STAT: Bayes Factors and Model Selection'
- url: https://physiology.med.cornell.edu/people/banfelder/qbio/resources_2008/1.5_Bonferroni_FDR.pdf
  label: 'Cornell: Bayesian Inference and Hypothesis Testing'
cards:
- id: 2104753c-a0ae-4616-aaae-605d28fd4e85
  type: flip
  front: Define the Bayes factor and explain what it measures.
  back: The Bayes factor is $BF_{10} = \frac{P(\text{data} | H_1)}{P(\text{data} | H_0)}$, the ratio of
    marginal likelihoods under two hypotheses. It quantifies how much the data support $H_1$ relative
    to $H_0$. A Bayes factor > 1 favors $H_1$; < 1 favors $H_0$. Unlike p-values, it treats both hypotheses
    symmetrically and is a direct measure of evidence.
- id: 8ecd7e59-0367-4022-8ac8-d14fece8b7d0
  type: flip
  front: How do you derive posterior odds from the Bayes factor?
  back: 'Posterior odds = Bayes factor × prior odds: $$\frac{P(H_1 | \text{data})}{P(H_0 | \text{data})}
    = BF_{10} \cdot \frac{P(H_1)}{P(H_0)}$$ If prior odds are 1:1 (equal belief), posterior odds = $BF_{10}$.
    If prior odds favor $H_0$, the posterior odds are more conservative. The Bayes factor updates your
    initial belief.'
- id: 96b41bf2-b1e0-475a-b0a7-1493375195a9
  type: mcq
  front: You compute $BF_{10} = 4$ for comparing a treatment effect ($H_1$) vs. no effect ($H_0$). With
    equal prior odds, what is the posterior probability of $H_1$?
  back: ''
  choices:
  - key: a
    text: '0.25'
    correct: false
  - key: b
    text: '0.50'
    correct: false
  - key: c
    text: '0.80'
    correct: true
  - key: d
    text: '0.95'
    correct: false
- id: 093bef48-751d-4e88-8392-49124303ae7b
  type: flip
  front: Compare Bayesian hypothesis testing (Bayes factors) with frequentist hypothesis testing (p-values).
  back: '**Bayes factor**: Compares two models directly; symmetric (can favor $H_0$ or $H_1$). Accounts
    for model complexity (marginalization over parameters). No multiple testing inflation. Depends on
    prior specification. **p-value**: Tests only $H_0$; cannot favor $H_0$ without additional context.
    Requires significance levels and multiple testing correction. Asymptotic; interpretation is about
    repeated sampling. Choose Bayes factors for exploratory analysis and model comparison; p-values for
    confirmatory, pre-specified tests.'
- id: a8c29d9b-aaa8-442a-9360-716f5724eb0e
  type: flip
  front: 'Design scenario: You A/B test a feature on two user cohorts. You want to use Bayes factors instead
    of p-values to declare a winner. How would you proceed?'
  back: 'Specify two models: $H_1$ (feature improves metric) and $H_0$ (no difference). Use conjugate
    priors (e.g., Beta for conversion) for closed-form Bayes factors. Compute $BF_{10}$ as data accumulate.
    When $BF_{10} > 10$ (strong evidence for $H_1$), launch the feature. Advantage: no multiple testing
    correction, and you can quantify evidence for $H_0$ if data are ambiguous. Always document the priors;
    sensitivity to prior choice is important.'
---

## Intuition

Bayesian hypothesis testing compares competing models or hypotheses directly via the **Bayes factor**, which measures how much the data support one hypothesis over another. Unlike p-values (which test $H_0$ under a repeated sampling framework), Bayes factors quantify evidence for both hypotheses simultaneously.

## Detail

The **Bayes factor** is the ratio of model likelihoods (marginal likelihoods, integrating over parameter uncertainty):
$$BF_{10} = \frac{P(\text{data} | H_1)}{P(\text{data} | H_0)}$$

Alternatively, using Bayes' theorem, the posterior odds of $H_1$ vs. $H_0$ are:
$$\frac{P(H_1 | \text{data})}{P(H_0 | \text{data})} = BF_{10} \cdot \frac{P(H_1)}{P(H_0)}$$
where $P(H_1) / P(H_0)$ are the prior odds.

**Interpretation of $BF_{10}$**:
- $BF > 10$: Strong evidence for $H_1$.
- $3 < BF < 10$: Moderate evidence for $H_1$.
- $1/3 < BF < 3$: Weak evidence (inconclusive).
- $BF < 1/10$: Strong evidence for $H_0$.

**Advantages over p-values**:
1. **Symmetric**: Can quantify evidence for $H_0$ (p-values only test $H_0$).
2. **No multiple testing inflation**: Bayes factors do not require correction for multiple comparisons (already integrated over model space).
3. **Interpretable**: Direct comparison of models, not a significance threshold.

## Common gotchas / interview framings

- **Model specification**: Bayes factors depend heavily on the prior within each hypothesis. Different priors yield different Bayes factors; sensitivity analysis is essential.
- **Computational burden**: Marginal likelihoods require integration; often approximated via MCMC or Laplace approximation.
- **Prior odds matter**: $BF_{10} = 5$ with equal prior odds ($P(H_1) = P(H_0) = 0.5$) gives posterior odds 5:1 for $H_1$. Different prior odds change the conclusion.
- **Not a replacement for p-values in all contexts**: In regulatory settings or pre-specified studies, p-values remain standard. Bayes factors are powerful for exploratory analysis and model comparison.
- **Information criteria alternatives**: AIC, BIC, and DIC approximate Bayes factors under certain conditions; simpler computationally but less flexible.

## See also
- [[bayes-factor]]
- [[posterior-odds]]
- [[model-comparison]]
- [[bayesian-hypothesis-test]]
- [[likelihood-ratio]]
- [[model-evidence]]

## Sources
See frontmatter `sources:`.

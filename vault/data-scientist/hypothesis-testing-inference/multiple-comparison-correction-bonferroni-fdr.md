---
id: ad0e97d3-99d5-4ae8-911d-02129e43e9a7
title: Multiple comparison correction (Bonferroni, FDR)
track: data-scientist
topic: hypothesis-testing-inference
difficulty: 3
tags:
- multiple testing
- Bonferroni
- FDR
- family-wise error
- false discovery rate
aliases:
- multiple hypothesis testing
- multiple testing correction
- p-value adjustment
- type I inflation
sources:
- url: https://edu.abi.am/statistics-theory/multiple-test-correction-bonferroni-fdr
  label: 'ABI Bioinformatics: Multiple Test Correction'
- url: https://physiology.med.cornell.edu/people/banfelder/qbio/resources_2008/1.5_Bonferroni_FDR.pdf
  label: 'Cornell: Bonferroni and FDR Methods'
- url: https://www.statsig.com/perspectives/bonferroni-correction-multiple-testing
  label: 'Statsig: Bonferroni and Multiple Testing in A/B Testing'
cards:
- id: 335a4008-664b-43b0-b824-a04fe157d53d
  type: flip
  front: Why does testing 20 independent hypotheses at $\alpha = 0.05$ without correction lead to a ~64%
    false positive rate?
  back: 'Each hypothesis has a 5% chance of a false positive (Type I error). The probability of getting
    at least one false positive among 20 independent tests is: FWER = $1 - (1 - 0.05)^{20} = 1 - 0.95^{20}
    \approx 0.64$ or 64%. This is the family-wise error rate. Correction is needed to control this inflation.'
- id: 48a6ee75-ec2c-46d1-860d-caec71b5799e
  type: flip
  front: Explain the Bonferroni correction and its main drawback.
  back: 'The Bonferroni correction divides the significance level by the number of tests: $\alpha_{\text{corrected}}
    = \alpha / m$. This guarantees FWER $\leq \alpha$. **Drawback**: It is very conservative. With $m
    = 50$ tests and $\alpha = 0.05$, each test uses $\alpha = 0.001$, making it hard to reject any hypothesis.
    Power drops sharply, so you may miss true discoveries (Type II errors).'
- id: 7d6a6279-dddd-42b1-989f-e0b49900cd54
  type: mcq
  front: In a gene expression study, you test 10,000 genes for differential expression at FDR-adjusted
    $\alpha = 0.05$. What does FDR = 0.05 mean?
  back: ''
  choices:
  - key: a
    text: Only 5% of all p-values are false positives.
    correct: false
  - key: b
    text: Among the genes you declare significant, expect ~5% to be false discoveries.
    correct: true
  - key: c
    text: 5% chance that the null hypothesis is true.
    correct: false
  - key: d
    text: The probability of at least one false positive across all tests is 5%.
    correct: false
- id: 91873cba-b6b4-4a08-9ce3-5515caafeea3
  type: flip
  front: Compare Bonferroni (FWER control) and Benjamini-Hochberg (FDR control). When would you use each?
  back: '**Bonferroni (FWER)**: Use when false positives are costly (clinical diagnosis, fraud detection).
    Guarantees ≤ $\alpha$ probability of any false positive. **Benjamini-Hochberg (FDR)**: Use when you
    tolerate some false positives among discoveries, guiding further research (genomics, screening, recommendation
    systems). Higher power and more discoveries. Choose based on the consequence of errors: high cost
    of any false positive → FWER; many candidates, further validation planned → FDR.'
- id: faaa55d1-a507-41b3-b401-ed25e6877e07
  type: flip
  front: 'Design scenario: You run an A/B test with 5 variants (1 control + 4 treatments) on a single
    metric. What multiple testing correction would you apply and why?'
  back: You have 4 treatment-vs-control comparisons or $\binom{5}{2} = 10$ pairwise comparisons. Use **Bonferroni**
    or **FDR** to adjust for multiple testing. If the business cost of a false positive (shipping a bad
    variant) is high, use Bonferroni. If variants advance to further testing, FDR is less conservative
    and preserves power. Always pre-specify the number of comparisons before analyzing.
---

## Intuition

When testing many hypotheses, the probability of making at least one Type I error (false positive) increases rapidly. If you test $m$ independent hypotheses at $\alpha = 0.05$, the family-wise error rate (FWER)—the probability of at least one false positive—is approximately $1 - (1 - 0.05)^m$. For $m = 20$, this is ~64%, not 5%!

Multiple comparison corrections adjust the rejection threshold to control this inflation. Two main approaches exist: **(1) FWER control** (Bonferroni): stringent, reduces false positives but low power. **(2) False Discovery Rate (FDR)** control (Benjamini-Hochberg): less stringent, balances discoveries with false positives, higher power.

## Detail

**Bonferroni Correction**: Adjust $\alpha$ by dividing by $m$: $\alpha_{\text{corrected}} = \alpha / m$. If $m = 20$ and $\alpha = 0.05$, each test uses $\alpha = 0.0025$. This guarantees FWER $\leq \alpha$ but is conservative—it rejects few hypotheses, increasing Type II error (low power). Alternatively, multiply each observed p-value by $m$ and cap at 1.

**False Discovery Rate (FDR)** (Benjamini-Hochberg): Define FDR as the expected proportion of false positives among all rejections. The BH procedure ranks p-values and uses a data-adaptive threshold. For the $i$-th smallest p-value, compute the threshold $\alpha_{(i)} = \alpha \cdot i/m$ and reject all hypotheses with p-values below the largest $\alpha_{(i)}$ that satisfies $p_{(i)} \leq \alpha_{(i)}$. **Key advantage**: Controls FDR (not FWER), allowing more discoveries while bounding false positives.

## Common gotchas / interview framings

- **When to apply**: Needed only when testing multiple related hypotheses (e.g., pairwise comparisons after ANOVA, gwas screening, A/B test multiple variants). Single pre-specified test does not require correction.
- **Bonferroni is too conservative**: In genomics or high-dimensional screening with thousands of tests, Bonferroni $\alpha / 1000$ leads to vanishingly small power. FDR is preferable.
- **FDR allows some false positives**: If you cannot tolerate any false discoveries (e.g., rare disease diagnosis), use FWER. If discoveries guide further research (e.g., gene screening), FDR is appropriate.
- **Adaptation to observed data**: BH procedure adapts to the p-value distribution; thus, uniform distribution of p-values under $H_0$ is assumed. Positively dependent p-values may require modified procedures.
- **Not a substitute for pre-registration**: Correction does not eliminate p-hacking; pre-register all hypotheses before data collection.

## See also
- [[bonferroni-correction]]
- [[false-discovery-rate]]
- [[family-wise-error-rate]]
- [[benjamini-hochberg]]
- [[multiple-testing-problem]]
- [[q-value]]

## Sources
See frontmatter `sources:`.

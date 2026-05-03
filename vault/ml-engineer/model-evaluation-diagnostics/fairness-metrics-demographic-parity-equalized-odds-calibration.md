---
id: 855d9a84-b879-4d1d-963c-e754e0110f1f
title: Fairness metrics (demographic parity, equalized odds, calibration)
track: ml-engineer
topic: model-evaluation-diagnostics
difficulty: 3
tags:
- fairness
- bias
- demographic parity
- equalized odds
- disparate impact
aliases:
- algorithmic fairness
- group fairness
- individual fairness
sources:
- url: https://developers.google.com/machine-learning/crash-course/classification/accuracy-precision-recall
  label: Google ML Crash Course
- url: https://www.evidentlyai.com/classification-metrics
  label: 'Evidently AI: Classification Metrics'
cards:
- id: 1b80f431-bb2a-413c-b86d-32c3e902a3cf
  type: flip
  front: Define demographic parity and explain when it's appropriate. What's a pitfall?
  back: 'Demographic parity: equal positive prediction rate across groups: P(ŷ=1|G=0) = P(ŷ=1|G=1). Appropriate
    when: past discrimination exists, legal mandate (e.g., hiring lawsuits), or outcomes should be equal.
    Pitfall: ignores base rates. If Group 1 is 90% qualified and Group 0 is 50%, DP forces same positive
    rate despite qualification gap, leading to false positives for Group 0 and false negatives for Group
    1.'
- id: ff253cd3-915b-47c3-a6c7-da7e99999ec8
  type: flip
  front: Explain equalized odds. How does it differ from demographic parity?
  back: 'Equalized odds (EO): equal TPR and FPR across groups. Condition on true label: P(ŷ=1|Y=1,G=g)
    and P(ŷ=0|Y=0,G=g) same for all g. Differs from DP: DP conditions only on prediction, ignoring true
    label. EO respects base rates. Example: if Group 0 and 1 both have 80% qualified rate, EO ensures
    80% of qualified in both groups are approved. Harder to achieve; requires group-specific thresholds.'
- id: 41df0166-2f8e-454d-b786-6f48c6fb7ce1
  type: mcq
  front: 'Hiring model: 80% TPR for male candidates, 70% for female. Fair under which metric?'
  back: 'TPR differs (80% vs 70%) → fails equalized odds. DP not satisfied if prediction rates differ.
    Predictive parity requires equal precision (P(hired | qualified)), not equal TPR. None satisfied.
    Fix: threshold tuning or reweighting to equalize TPR across groups.'
  choices:
  - key: a
    text: Demographic parity
    correct: false
  - key: b
    text: Equalized odds
    correct: false
  - key: c
    text: Predictive parity (precision)
    correct: false
  - key: d
    text: None of these
    correct: true
- id: ef3a5457-9ac9-49e5-b31c-7860ed681e71
  type: flip
  front: You're building a loan approval system. Would you optimize for DP, EO, or calibration? Why?
  back: 'Calibration is most appropriate. Reason: loan default risk must be equal across groups (P(default|approved,male)
    ≈ P(default|approved,female)). Bank''s risk exposure same regardless of gender. Calibration aligns
    with business logic (fair pricing). DP/EO might enforce approval rate parity despite underlying qualification
    disparity. Note: recidivism (COMPAS) case shows this: optimizing for equal TPR (EO) → worse calibration
    for minorities.'
- id: d134cfbf-fbe7-4133-bac4-c3744a22a979
  type: flip
  front: Describe the fairness-accuracy tradeoff. Give a numerical example.
  back: 'Enforcing fairness (e.g., group-specific thresholds) reduces overall accuracy. Example: original
    model 90% accurate, demographic-parity enforced → 85% accurate. Group A: 91% → 88% (3% drop), Group
    B: 88% → 82% (6% drop). Trade-off worse for minority group (larger threshold shift needed). Acceptable?
    Context: hiring (1-2% accuracy loss acceptable), fraud (unacceptable). Communicate trade-off to stakeholders.'
---

## Intuition
Fairness metrics quantify algorithmic bias across demographic groups (race, gender, age). Three main notions: demographic parity (equal outcomes), equalized odds (equal error rates), and calibration (equal confidence). No single definition fits all; choice depends on legal/ethical context. Fairness-accuracy tradeoff: enforcing fairness often reduces overall accuracy.

## Detail
**Demographic Parity (DP)**:
$$P(\hat{Y}=1|G=0) = P(\hat{Y}=1|G=1)$$
Equal positive prediction rate across groups. Fairness notion: equal outcomes regardless of group. Problem: ignores base rates. If Group 0 is 90% qualified, DP forces same 90% positive rate for Group 1 even if 50% qualified → false positives for Group 1.

**Equalized Odds (EO)**:
$$P(\hat{Y}=1|Y=1,G=0) = P(\hat{Y}=1|Y=1,G=1)$$ AND $$P(\hat{Y}=0|Y=0,G=0) = P(\hat{Y}=0|Y=0,G=1)$$
Equal TPR and FPR across groups. Fairness notion: equal opportunity (same treatment of positives, same treatment of negatives). Harder to achieve; often requires trade-off.

**Predictive Parity**:
$$P(Y=1|\hat{Y}=1,G=0) = P(Y=1|\hat{Y}=1,G=1)$$
Equal precision (reliability of positive predictions) across groups. If model predicts 'approve loan' for Group 0 with 90% approval rate and Group 1 with 80%, precision differs.

**Calibration**:
$$P(Y=1|\hat{Y}=p,G=0) ≈ p ≈ P(Y=1|\hat{Y}=p,G=1)$$
Equal calibration across groups. A 80%-confidence prediction should be ~80% accurate for both groups. Most relaxed definition; compatible with some disparity.

**Fairness-Accuracy Tradeoff**:
- Enforcing demographic parity reduces accuracy (especially minority group)
- Threshold tuning per group: lower threshold for disadvantaged group → more positives, lower precision
- Example: 5-10% accuracy drop to achieve demographic parity

## Common gotchas / interview framings
- Impossibility theorem: DP + EO incompatible unless base rates equal or classifier perfect; choose one
- Proxy discrimination: if using proxies for protected attribute (zipcode ≈ race), DP violated indirectly
- Measurement issues: fairness metrics sensitive to data quality (mislabeled minority), representation bias
- Context-dependent: hiring (EO preferred: equal TPR), lending (calibration: equal default risk), hiring (DP: equal success rate)
- Fairness washing: report best-looking metric; always report multiple (DP, EO, calibration) with context

## See also
- [[demographic-parity]]
- [[equalized-odds]]
- [[disparate-impact]]
- [[fairness-accuracy-tradeoff]]
- [[bias-mitigation]]

## Sources
See frontmatter `sources:`.

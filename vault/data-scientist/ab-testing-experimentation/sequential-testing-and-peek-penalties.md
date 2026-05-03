---
id: 0f3eca28-4ff5-4758-9577-da61407d94ce
title: Sequential testing and peek penalties
track: data-scientist
topic: ab-testing-experimentation
difficulty: 5
tags:
- multiple-testing
- early-stopping
- type-i-error
- sequential-analysis
- peek-penalty
aliases:
- optional stopping
- early stopping
- alpha spending
- Pocock boundaries
sources:
- url: https://netflixtechblog.com/sequential-a-b-testing-keeps-the-world-streaming-netflix-part-1-continuous-data-cba6c7ed49df
  label: 'Netflix: Sequential A/B Testing'
- url: https://blog.analytics-toolkit.com/2022/statistical-power-mde-and-designing-statistical-tests/
  label: 'Analytics Toolkit: Sequential Testing'
cards:
- id: 7f945e34-73eb-42cb-af31-73f7dde70834
  type: flip
  front: ''
  back: ''
- id: 792c4d08-4bd6-4d61-b1a5-b8384050b7ca
  type: flip
  front: ''
  back: ''
- id: 50ff664e-a5ad-482d-b44a-55119d1b197d
  type: mcq
  front: ''
  back: ''
  choices:
  - key: a
    text: Yes, p<0.05
    correct: false
  - key: b
    text: No, you pre-registered day 7; stopping early = p-hacking
    correct: false
  - key: c
    text: Yes, if you use Bonferroni correction (α = 0.05/2 = 0.025)
    correct: false
  - key: d
    text: Need more info about number of metrics
    correct: false
- id: a3a60c62-5e4b-4b0f-b776-f85e575012c0
  type: flip
  front: ''
  back: ''
---

## Intuition

**Peeking** (looking at results before the test ends) inflates Type I error. If you peek at day 3 and day 5 and day 7, the true false positive rate is ~20%, not 5%. **Peek penalties** correct for this inflation.

## Detail

**The peeking problem**:
- Fixed-horizon test: alpha = 0.05, Type I error = 5%
- Peek once at day 3: alpha = 0.05 at day 3 → inflates error to ~8% (if you stop early)
- Peek at days 1, 2, 3, ..., 7: error inflates to ~15-20%
- **Cause**: Each peek is an independent test; multiple tests inflate Type I error

**Solutions**:

1. **Fixed horizon**: Don't peek. Run full duration (7 days, 14 days), then analyze once. Simple, controls Type I error at α=0.05

2. **Sequential testing with corrections**:
   - **Bonferroni**: alpha_per_peek = 0.05 / number_of_peeks
   - **Pocock**: more permissive boundaries; adjust alpha at each peek
   - **O'Brien-Fleming**: tight early, loose late (good for early stopping)

3. **Truly sequential methods**:
   - **Likelihood ratio test** (sequential probability ratio test, SPRT): continuously monitor, stop when evidence is strong
   - **Bayesian sequential**: posterior reaches decision threshold
   - More efficient (faster decisions) but complex to implement

**Best practice**:
- **Default**: Fixed horizon (no peeking). Run the full duration, analyze once. Simple, robust.
- **If you must peek**: Use Bonferroni correction (conservative) or O'Brien-Fleming (moderate)
- **If you want early stopping**: Use truly sequential methods (SPRT, Bayesian), not manual peeking with corrections

## Common gotchas / interview framings
- "Our PM wants to look at metrics daily. What do you say?" → "We can look, but won't make decisions until day 7 or 14. Daily looks are for monitoring (crashes, bugs), not for statistical inference. If we make decisions daily, we inflate Type I error from 5% to 50%+"
- "We peeked and saw significance at day 3. Can we stop?" → Only if you pre-registered sequential testing (SPRT, Bayesian). If you didn't, waiting until day 7 controls error. Stopping early without pre-registration = p-hacking
- "Should we use Bonferroni for 100 peeks?" → No, alpha = 0.05/100 = 0.0005 is too strict, you'll never detect real effects. Better: use O'Brien-Fleming or truly sequential method
- "Bayesian sequential testing doesn't have Type I error." → Wrong. Bayesian can still have false positives if prior is misspecified or stopping rule is biased

## See also
- [[confidence-intervals-vs-bayesian-posterior]]
- [[pre-registration-and-redd-registered-experiment-design-document]]

## Sources
See frontmatter `sources:`.

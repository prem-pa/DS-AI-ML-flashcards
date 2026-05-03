---
id: 1a26a782-1fd9-4e18-96ed-245a1b5e0adb
title: Probabilistic forecasting
track: verticals
topic: time-series-forecasting
difficulty: 3
tags:
- probabilistic
- distribution
- likelihood
- generative
- foundation-models
- CRPS
aliases:
- distributional forecasting
- ensemble forecasting
- generative forecasting
- Bayesian forecasting
sources:
- url: https://github.com/amazon-science/chronos-forecasting
  label: Chronos Forecasting GitHub
- url: https://www.salesforce.com/blog/moirai/
  label: Moirai Foundation Model Blog
- url: https://arxiv.org/abs/2403.07815
  label: 'Chronos: Learning the Language of Time Series'
- url: https://arxiv.org/abs/2511.11698
  label: 'Moirai 2.0: When Less Is More for Time Series Forecasting'
cards:
- id: ec63fa0e-aa75-48f3-b74a-41391b0ff84b
  type: flip
  front: What is the key difference between optimizing for MSE and optimizing for negative log-likelihood
    (NLL)?
  back: MSE = mean squared error, optimizes point forecast (conditional mean). NLL = -log p(y | model),
    optimizes full distribution modeling. NLL penalizes overconfidence (narrow predicted distribution
    around true value) and underconfidence (wide distribution). A model can have low MSE but high NLL
    if it overestimates uncertainty. Use NLL for probabilistic forecasting, MSE for point forecasts only.
- id: 75aa5971-e367-4fbb-9b8f-cfbbf96483e0
  type: mcq
  front: Chronos tokenizes time series and trains a Transformer language model on the tokens. What is
    the advantage of this approach over predicting raw values?
  back: 'Tokenization converts continuous time series to discrete sequences, allowing reuse of LLM architectures
    and pretraining techniques. Language models trained on 100B+ tokens leverage self-supervised learning,
    enabling few-shot/zero-shot transfer. Trade-off: quantization loses some precision but gains generalization.
    This is why Chronos and TimeGPT (language-style models) achieve strong zero-shot performance.'
  choices:
  - key: a
    text: It reduces computational complexity
    correct: false
  - key: b
    text: Discrete tokens enable transfer learning from language; scales better to large pretraining corpora
    correct: true
  - key: c
    text: It improves forecast accuracy on all datasets
    correct: false
  - key: d
    text: It eliminates the need for normalization
    correct: false
- id: 2f92c6af-414a-4200-9670-0068d4406bbc
  type: flip
  front: What is CRPS and why is it better than MSE for evaluating probabilistic forecasts?
  back: 'CRPS = Continuous Ranked Probability Score = ∫(F(y) - 𝟙(y ≥ y_true))² dy (integral of squared
    difference between predicted CDF and empirical CDF at true value). MSE only scores point forecast.
    CRPS scores full distribution: rewards accurate intervals and calibration, not just point accuracy.
    Lower CRPS is better; it''s interpretable as expected distance between forecast and observation.'
- id: f8208b32-11da-49c4-b2a6-0f52f67eff87
  type: flip
  front: Moirai 2.0 uses a mixture distribution with multiple components. Why is this useful for time
    series forecasting?
  back: 'Mixture distributions (e.g., 4-component) capture multimodality: multiple plausible futures with
    different likelihoods. For example, demand can spike (promotion, event) or stay flat (normal). A single
    Normal distribution can''t represent this. Moirai''s mixture allows flexible prediction intervals:
    tight around likely scenarios, wide when future is genuinely uncertain. Trade-off: more parameters,
    needs careful fitting.'
---

## Intuition
Probabilistic forecasting models the full conditional distribution p(y_{t+1:t+h} | history), not just point estimates. This enables quantifying uncertainty, tail risk, and multimodality (multiple likely futures). Essential for planning under uncertainty: inventory decisions, resource allocation, risk assessment.

## Detail
**Core Approaches:**

1. **Quantile-Based (Covered in prev. concept):**
   - Directly predict quantiles τ = [0.05, ..., 0.95]
   - Flexible, no distributional assumption

2. **Distribution Estimation (Parametric):**
   - Assume conditional distribution: e.g., Normal, StudentT, NegativeBinomial
   - Network outputs parameters (μ, σ) or (α, β) for the distribution
   - Optimize via negative log-likelihood (NLL): -log p(y_t | μ, σ)
   - StudentT better than Normal for heavy tails; NegativeBinomial for counts

3. **Mixture Models:**
   - Output mixture: p(y) = Σ_k π_k N(μ_k, σ_k)
   - Captures multimodality (multiple plausible futures)
   - Moirai uses 4-component mixture for flexible intervals

4. **Generative/Autoregressive:**
   - Train to generate sequences sample-by-sample
   - WaveNet, PixelCNN style for time series
   - Chronos: treats time series as language, tokenizes values, trains transformer LM
   - Foundation models like Chronos-2, Moirai-2 scale this approach

**Evaluation Metrics:**
- **CRPS** (Continuous Ranked Probability Score): ∫(F(y) - 𝟙(y ≥ y_true))² dy (lower is better)
- **NLL** (Negative Log-Likelihood): -log p(y_true | model) (lower is better)
- **Prediction Interval Coverage** (PICP): fraction of observations in interval
- **Mean Interval Width** (MIW): average interval size (trade-off with coverage)

**Foundation Models for Time Series:**

1. **Chronos (Amazon):**
   - Tokenizes time series into discrete bins
   - Treats tokens as language → trains Transformer language model
   - Chronos-2: 120M parameters, zero-shot, multivariate
   - Outputs quantiles for uncertainty
   - URL: https://huggingface.co/amazon/chronos-2

2. **Moirai (Salesforce):**
   - Trained on 36M series, 27B+ observations
   - Decoder-only Transformer with quantile forecasting
   - Moirai-2: 30x smaller, 2x faster than 1.0-Large
   - Mixture distribution for flexible intervals
   - Handles multivariate, missing values, covariates
   - URL: https://github.com/SalesforceAIResearch/uni2ts

3. **TimeGPT (Nixtla):**
   - Generative pretrained model for time series
   - Trained on 100B+ data points across domains
   - Few-shot and zero-shot capabilities
   - Self-attention architecture

## Common gotchas / interview framings
- Point forecast (MSE loss) and probabilistic forecast (NLL) are different objectives; don't mix
- Assuming wrong distribution (e.g., Normal for count data) leads to poor calibration
- Overconfident models: predicted intervals too narrow, coverage <nominal level
- CRPS and NLL sensitive to long-tail behavior; may differ on same model
- Mixture models can overfit to multimodality in training data that's noise, not real
- Foundation models (Chronos, Moirai) impressive zero-shot but may underperform on domain-specific fine-tuning if poorly adapted
- Computational cost: sampling from generative models (Chronos) more expensive than parametric
- Hyperparameter tuning for distributions (e.g., df for StudentT) often ignored, defaulting to Normal

## See also
- [[continuous-ranked-probability-score]]
- [[negative-log-likelihood]]
- [[ensemble-methods]]
- [[mixture-distributions]]
- [[autoregressive-models]]
- [[chronos]]
- [[moirai]]

## Sources
See frontmatter `sources:`.

---
id: d74a0f61-02be-4f61-ace9-a58c53376787
title: Reward modeling and preference data
track: ai-llm-engineer
topic: fine-tuning-alignment
difficulty: 5
tags:
- reward-model
- preference-data
- annotation
- generalization
- data-quality
aliases:
- Reward model
- preference labeling
- human preferences
sources:
- url: https://arxiv.org/abs/2305.18290
  label: 'Rafailov et al: DPO (discusses reward model limitations)'
- url: https://huggingface.co/blog/ariG23498/rlhf-to-dpo
  label: 'HuggingFace: RLHF to DPO Blog'
- url: https://aman.ai/primers/ai/preference-optimization/
  label: 'Aman Sanger: Preference Optimization Primer'
cards:
- id: e2bcdcb2-4344-4438-ba91-9fd2f7db8998
  type: flip
  front: You collect 50k pairwise preferences for a domain. Annotators agree 82% of the time. Should you
    train a reward model or use DPO?
  back: Use **DPO**. 18% disagreement (7.2k mismatched pairs) will hurt reward model generalization. DPO
    directly optimizes on pairwise labels without training an intermediate RM, avoiding RM overfitting
    on weak signal. DPO is also 50% cheaper (no separate RM training) and more stable.
- id: a8431d27-b8ce-46ed-adac-7e49283e9a05
  type: mcq
  front: Reward model trained on 10k dialogue preferences, evaluated on 1k heldout dialogue, achieves
    94% accuracy. But it ranks irrelevant outputs highly on a medical dataset. Why?
  back: 'Classic OOD failure. High accuracy on in-domain val set masks poor generalization. Medical examples
    are outside the training distribution; model confidently ranks them wrong. Mitigation: (1) add medical
    preferences to training, (2) use ensemble/uncertainty estimates, or (3) skip RM → use DPO.'
  choices:
  - key: a
    text: The model is underfitting dialogue preferences
    correct: false
  - key: b
    text: Strong distribution shift; reward model has not seen medical examples and makes overconfident
      wrong predictions
    correct: true
  - key: c
    text: Insufficient regularization (weight decay, dropout)
    correct: false
  - key: d
    text: The preference data was mislabeled
    correct: false
- id: ba2defed-043c-4189-a580-d3ff57924516
  type: flip
  front: Describe two ways to mitigate reward hacking where the policy learns to output verbose text because
    longer responses score higher.
  back: '1. **Audit reward model**: plot reward vs response length; if correlation exists, add length-normalized
    reward $r(x, y) / |y|$ or length-penalized objective. 2. **Enforce diversity in preference data**:
    ensure dataset has good/bad examples of various lengths, so model learns quality ≠ length. 3. **Use
    task-specific metrics**: prefer BLEU/ROUGE (length-robust) over simple scalar rewards.'
- id: 8340139d-e9c9-46bc-84e5-b7f1668465a1
  type: flip
  front: Why is Bradley-Terry preference model preferred over naive scoring (reward_A - reward_B)?
  back: 'Bradley-Terry is a principled probabilistic model: $P(y_w > y_l) = \sigma(r(x, y_w) - r(x, y_l))$.
    It ensures probabilities are bounded [0, 1], handles intransitive preferences gracefully, and has
    theoretical grounding in ranking. Naive scoring can produce unbounded differences and is less statistically
    robust.'
---

## Intuition
A reward model is a learned proxy for human preferences. Given a prompt and response, it predicts a scalar score reflecting how much humans like that response. Training data: pairwise comparisons ("response A is better than response B"). The reward model learns a Bradley-Terry preference model that generalizes to rank unseen responses.

Key challenge: **preference data is expensive and noisy**. Human annotators disagree (~80% pairwise agreement typical), and the reward model must generalize beyond its training distribution—a hard problem.

## Detail
Preference Data:
- Collection: crowdsourced annotators label pairwise comparisons. Cost: $0.10–$1 per comparison. Time: 10k examples = 2–4 weeks. Bias: annotators from specific geographic/demographic groups may have skewed preferences.
- Format: triplet (prompt, response_A, response_B, label ∈ {A, B, tie}). Labels often noisy; ~20% inter-annotator disagreement.
- Filtering: remove tied votes (ambiguous), consensus only (>2 raters agree). Trade: fewer examples but higher signal.

Reward Model Architecture:
- Simplest: freeze a base model, attach a linear classification head to the last hidden state, train binary classifier on (y_w vs y_l). Output: scalar $r(x, y) = \text{head}(\text{model}_\text{frozen}(y))$.
- Parametric approach: trainable LM with classification head. More capacity but overfitting risk on small datasets.
- Training objective: binary cross-entropy:
$$\mathcal{L}_{RM} = -[y_w \log(\sigma(r(x, y_w) - r(x, y_l))) + (1 - y_w) \log(1 - \sigma(r(x, y_w) - r(x, y_l)))]$$
Where sigmoid ensures reward difference is calibrated [0, 1].

Generalization Challenges:
- **Distribution shift**: Model trained on code+chat preferences may fail on medical text.
- **Overconfidence**: High training accuracy (95%+) ≠ good generalization. OOD examples are ranked confidently but incorrectly.
- **Reward hacking**: Policy exploits reward model weaknesses (e.g., verbosity bias if longer responses get higher scores).
- **Preference inconsistency**: Cyclic preferences (A > B, B > C, C > A) can occur; model must interpolate inconsistent signals.

Modern approach (2026): Skip learned reward models entirely → **DPO, ORPO, or SimPO** directly fit pairwise losses without RM.

## Common gotchas / interview framings
- **"Does 10k preference pairs generalize?"** Usually poorly without careful curation. Aim for 50k+ or use synthetic augmentation.
- **Preference aggregation**: Majority vote discards minority opinion but improves signal. Probabilistic weighting (trust confident annotators) is superior but complex.
- **Cross-domain reward**: A reward model trained on dialogue doesn't transfer to summarization. Domain-specific RM's expensive; motivates preference-only methods (DPO).

## See also
- [[preference-learning]]
- [[binary-classification]]
- [[model-generalization]]
- [[data-annotation]]
- [[bradley-terry-model]]
- [[rlhf]]
- [[reward-signal]]

## Sources
See frontmatter `sources:`.

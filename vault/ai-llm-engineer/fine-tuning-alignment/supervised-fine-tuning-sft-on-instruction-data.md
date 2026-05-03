---
id: 5d858137-831d-4b57-a49b-97f5111f8d6a
title: Supervised fine-tuning (SFT) on instruction data
track: ai-llm-engineer
topic: fine-tuning-alignment
difficulty: 3
tags:
- instruction-following
- sft
- post-training
- data-quality
- base-models
aliases:
- SFT
- instruction tuning
- instruct-tuning
sources:
- url: https://llamafactory.readthedocs.io/en/latest/getting_started/sft.html
  label: LLaMA Factory SFT Guide
- url: https://huggingface.co/blog/mlabonne/sft-llama3
  label: 'HuggingFace: Fine-tune Llama 3 Ultra-Efficiently'
- url: https://www.jenova.ai/en/resources/llama-factory-complete-guide-to-llm-fine-tuning
  label: 'LLaMA Factory: Complete Fine-Tuning Guide'
cards:
- id: f9beece5-56e3-4e52-b28d-72ec1ab63998
  type: flip
  front: What is supervised fine-tuning (SFT) and why is it the prerequisite for RLHF/DPO?
  back: SFT trains a base model on {instruction, response} pairs using cross-entropy loss. It teaches
    the model to follow instructions. RLHF and DPO require an SFT-initialized model; training alignment
    from a base model directly is inefficient because the model first must learn how to respond to prompts.
    SFT is the intermediate step.
- id: 8e8dfd85-34b0-4fef-9fb7-304e1442823e
  type: flip
  front: You have 10k instruction examples. How do you decide how many epochs to train SFT?
  back: 'Monitor validation loss on a held-out set (~10% of data). Typical: 1–4 epochs. If val loss plateaus
    or increases, stop (overfitting). If you have limited data, use early stopping and/or regularization
    (weight decay, dropout). For very small datasets (<5k), 1 epoch with heavy augmentation is safer.'
- id: a2cd6199-a247-4495-947a-e9e4b4b75e2d
  type: mcq
  front: Which is NOT a primary failure mode of naive SFT?
  back: SFT degradation on long reasoning is not intrinsic; it's usually a data issue. The real failures
    are catastrophic forgetting (mitigated by mixed data), narrow distribution (mitigated by template
    diversity), and format sensitivity (mitigated by varying prompts).
  choices:
  - key: a
    text: Catastrophic forgetting of pretraining knowledge
    correct: false
  - key: b
    text: Overfitting to narrow instruction styles
    correct: false
  - key: c
    text: Long-horizon reasoning degradation
    correct: true
  - key: d
    text: Sensitivity to prompt format variation
    correct: false
- id: 00d3e8b0-ff2b-47ab-b3d1-b5d3bb241968
  type: flip
  front: Why does SFT alone not align models to human preferences?
  back: SFT optimizes for behavioral cloning of training data, not for preference satisfaction. A model
    can learn to follow instructions but still make harmful or low-quality choices when faced with ambiguity.
    Alignment requires learning to rank outputs or maximize reward—SFT cannot do this. Enter RLHF/DPO.
---

## Intuition
SFT converts a base model (trained on next-token prediction) into an instruction-following assistant. The model learns to map input instructions to desired outputs via supervised learning on curated {input, output} pairs. This is the foundation of modern LLMs: no SFT → model ignores instructions.

## Detail
Training objective is standard cross-entropy loss on target tokens:
$$\mathcal{L}_{SFT} = -\mathbb{E}_{(x, y) \sim \mathcal{D}} [\log p_\theta(y \mid x)]$$
where $\mathcal{D}$ is an instruction-response dataset. Data quality and diversity matter enormously: 500 carefully curated examples often outperform 5,000 noisy ones. Modern practice uses **template-driven generation** (system + user + assistant roles) to make the model robust to prompt variation.

Key hyperparameters: learning rate ($2 \times 10^{-5}$ typical), batch size (16–64), and gradient accumulation. Training time: 1–4 epochs on most 7B models. After SFT, the model has learned domain conventions but lacks alignment to human preferences; this is where RLHF or DPO enters.

## Common gotchas / interview framings
- **Data quality kills performance**: A 10k noisy dataset can underperform a 1k clean one. Always audit and filter.
- **Catastrophic forgetting**: Aggressive SFT on narrow data risks degrading the model's general knowledge. Use mixed datasets or replay original pretraining data.
- **Token distribution shift**: If target distribution differs from training (e.g., assistant responses are much longer), loss may appear to plateau.
- **Instruction diversity**: Models trained on narrow instruction styles (e.g., QA only) overfit and generalize poorly. Use multiple templates.

## See also
- [[base-language-models]]
- [[supervised-learning]]
- [[instruction-following]]
- [[training-data]]
- [[prompt-templates]]
- [[model-fine-tuning]]

## Sources
See frontmatter `sources:`.

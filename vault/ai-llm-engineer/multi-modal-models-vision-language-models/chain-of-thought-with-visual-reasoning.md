---
id: 55cfc2e5-3bca-431a-bc9b-5f6a3b90e5c9
title: Chain-of-thought with visual reasoning
track: ai-llm-engineer
topic: multi-modal-models-vision-language-models
difficulty: 3
tags:
- chain-of-thought
- visual-reasoning
- step-by-step
- interpretability
- accuracy-improvement
aliases:
- visual CoT
- step-by-step reasoning
- visual explanation
- reasoning prompts
sources:
- url: https://developers.openai.com/cookbook/examples/multimodal/document_and_multimodal_understanding_tips
  label: Getting the Most out of GPT-5.4 for Vision and Document Understanding
- url: https://medium.com/google-cloud/video-understanding-with-gemini-notes-from-the-field-82dd0cd130ea
  label: 'Video Understanding with Gemini: Notes From the Field'
- url: https://arxiv.org/html/2604.11177
  label: Do Thought Streams Matter? Evaluating Reasoning in Gemini Vision-Language Models
cards:
- id: 98003485-c34d-4748-833c-b5117e1adf7a
  type: flip
  front: Why does chain-of-thought prompting reduce hallucination in vision-language models?
  back: CoT forces the model to ground reasoning in visible elements before answering. By committing to
    intermediate observations ("I see a table with 3 columns"), it's harder to subsequently invent data.
- id: 508f5e44-740c-448b-b4a1-1c2494f39115
  type: flip
  front: Compare "Direct" vs. "Chain-of-thought" prompting for document extraction.
  back: '**Direct** (fast, lower accuracy): "Extract all values from this form." May invent missing fields.
    **CoT** (slower, higher accuracy): "What are the form labels? What values are filled? Extract each
    field." Structured steps reduce hallucination.'
- id: 466b6055-d369-4079-9520-5763f9008025
  type: flip
  front: What is the difference between descriptive CoT and few-shot CoT?
  back: '**Descriptive** (1-shot): ask model to describe image, then answer question. **Few-shot**: show
    examples of correct CoT reasoning, then ask model to apply same style to new image. Few-shot more
    consistent but requires examples.'
- id: 96fb1af7-9a92-4350-a128-388571aaf6a4
  type: mcq
  front: How does Claude Opus 4.7's verification layer (April 2026) implement verification CoT?
  back: 'Two-pass process: generate answer → re-read image to verify. If inconsistency detected (e.g.,
    "I said column A has 5 values but only see 4"), output flagged or corrected. Improves accuracy 10-20%
    on complex tasks.'
  choices:
  - key: a
    text: It uses a separate verification model to check answers
    correct: false
  - key: b
    text: It regenerates the same answer twice and compares
    correct: false
  - key: c
    text: 'Two-pass: generate answer, then re-read image to verify for inconsistencies; flags/corrects
      errors'
    correct: false
  - key: d
    text: It asks the user to manually verify each answer
    correct: false
- id: 3ce1b8fa-a2e7-42c8-85b2-ec877b6b48a5
  type: flip
  front: What is the trade-off of using chain-of-thought prompting?
  back: 'Speed vs. accuracy: CoT adds 2-3x inference latency due to extra reasoning tokens. Batch processing
    benefits more than real-time. Overkill for simple tasks (object detection); critical for complex reasoning
    (document interpretation, multi-step logic).'
---

## Intuition

Vision-language models sometimes hallucinate or misinterpret images. **Chain-of-thought (CoT)** prompting—asking the model to "explain your reasoning step-by-step before answering"—reduces errors by:
1. Breaking down complex visual reasoning into interpretable substeps
2. Forcing the model to ground its reasoning in visible elements
3. Enabling error correction (if reasoning is wrong, spot it early)

For documents: "What is the header? What columns? What is row 3?" beats "Extract all data." For charts: "What are axes? Which lines? Calculate trend?" For video: "What happens first? What happens next? How long?" CoT improves accuracy 5-15% on average; on complex tasks (reasoning, localization), gains exceed 30%.

## Detail

**Why CoT works for vision**:
- Vision tasks are inherently interpretable: describe what you see before answering
- Intermediate steps reduce hallucination: if the model commits to "table has 3 columns," it's less likely to invent data
- Alignment with human reasoning: humans describe observations before conclusions
- Error catching: reviewing intermediate steps reveals mistakes (e.g., "I said the table has 5 rows, but only counted 4")

**Prompting strategies**

1. **Descriptive CoT**: "Describe what you see in the image. Then answer: [question]"
   - Works well for open-ended tasks (chart interpretation, scene understanding)
   - Reduces hallucination
   - More tokens → slower inference

2. **Structural CoT**: "What are the main elements? What relationships? What conclusion?"
   - Useful for documents, diagrams, complex scenes
   - Forces decomposition
   - Example: [header], [sections], [connections], [answer]

3. **Few-shot CoT**: Show examples of reasoning before asking question
   - Improves consistency
   - Reduces hallucination further (model mimics examples)
   - Example: "Here's how I interpret a similar image: [CoT]. Now interpret this: [new image]"

4. **Verification CoT** (Claude Opus 4.7, April 2026): Generate answer, then re-read image to verify
   - Two passes increase accuracy 10-20%
   - Catches self-contradictions
   - Slowest but highest accuracy

**Gemini thought streams** (research direction):
- Model generates intermediate "thoughts" (not necessarily text) during inference
- Thoughts can be reviewed, re-weighted, or used to backtrack
- Improved complex reasoning on video understanding

## Common gotchas / interview framings
- CoT trades off speed for accuracy: 2-3x slower inference due to extra reasoning tokens
- Domain-specific prompts work better: generic "explain your reasoning" < tailored steps for domain ("for a table: identify columns first")
- Model size matters: CoT helps small models more than large models (GPT-5.4 strong without CoT, benefits less from CoT than Pixtral 12B)
- Reasoning transparency != correctness: model can reason correctly but reach wrong conclusion (or vice versa)
- Data annotation: few-shot CoT requires human-labeled examples (expensive); can use model-generated CoT for distillation
- Hallucination reduction is uneven: CoT reduces object hallucination (inventing objects), but doesn't eliminate text hallucination

## See also
- [[chain-of-thought-prompting]]
- [[visual-reasoning]]
- [[few-shot-learning]]
- [[prompting-techniques]]
- [[interpretability]]
- [[error-analysis]]

## Sources
See frontmatter `sources:`.

---
id: edfb5183-3107-4c52-9c38-149b273b930c
title: Visual question answering (VQA)
track: ai-llm-engineer
topic: multi-modal-models-vision-language-models
difficulty: 3
tags:
- VQA-benchmark
- visual-understanding
- question-answering
- reasoning-task
- evaluation-metric
aliases:
- VQA task
- image-QA
- visual-QA
- VQA benchmark
sources:
- url: https://www.labellerr.com/blog/top-open-source-vision-language-models/
  label: Best Open-Source Vision Language Models of 2026
- url: https://blog.roboflow.com/local-vision-language-models/
  label: Best Local Vision-Language Models for Offline AI
- url: https://arxiv.org/html/2604.11177
  label: Do Thought Streams Matter? Evaluating Reasoning in Gemini Vision-Language Models
cards:
- id: c9a65306-af1b-47c0-86da-7d855eaebb3a
  type: flip
  front: What is the VQA (Visual Question Answering) task, and why is it a useful benchmark?
  back: 'Given an image and question, predict the answer. Benchmark because: tests multiple capabilities
    (detection, reasoning, counting, relationships), scales to millions of examples, and correlates with
    real-world understanding of images.'
- id: 0c145fcd-a75f-49ac-9d6c-7e9ced5d71e7
  type: flip
  front: Name the main question types in VQA v2 and give an example of each.
  back: '**Counting** ("How many dogs?"), **Attribute** ("What color is the car?"), **Spatial** ("Is the
    cat left of the dog?"), **Reasoning** ("What is the person doing?"), **Yes/No** ("Is there a tree?").'
- id: 19ddf687-7cc0-4ab4-9a39-7e9feacd7c0d
  type: flip
  front: What is the key weakness of VQA v2 as a benchmark?
  back: 'Dataset bias: statistical shortcuts allow high accuracy without true visual understanding. E.g.,
    images with kitchens are always labeled "oven", so models can answer by detecting context, not actually
    understanding the scene.'
- id: 897fb501-9414-469e-a8bf-fc2edff49eaf
  type: mcq
  front: How does A-OKVQA (2023) differ from VQA v2?
  back: 'A-OKVQA: longer questions requiring **world knowledge** ("What is the occupation of the person?")
    vs. pure visual reasoning. Tests reasoning + knowledge grounding; harder and more realistic than VQA
    v2.'
  choices:
  - key: a
    text: A-OKVQA has 10x more images
    correct: false
  - key: b
    text: A-OKVQA uses video instead of images
    correct: false
  - key: c
    text: A-OKVQA requires world knowledge + visual grounding; tests longer questions
    correct: false
  - key: d
    text: A-OKVQA focuses only on counting
    correct: false
- id: c9e0ec03-80c7-48d3-b228-c53faefef074
  type: flip
  front: Why is counting a consistently hard task in VQA?
  back: 'Challenges: occlusion (hidden objects), small objects (hard to detect), similar instances (mistaken
    identity). Even GPT-5.4 errors on dense scenes (>15 objects). Requires precise detection + enumeration,
    not just classification.'
---

## Intuition

Visual Question Answering (VQA) is a benchmark task: given an image and a natural language question, predict the answer. For example:
- **Image**: photo of a dog on a beach
- **Question**: "What color is the dog?"
- **Answer**: "brown"

VQA tests multiple capabilities simultaneously: object detection, reasoning, counting, color recognition, relationships, scene understanding. It's become the primary benchmark for evaluating vision-language models because (a) it's intuitive, (b) it scales to millions of examples, and (c) it correlates with real-world understanding. VQA performance often predicts model quality on downstream tasks (document understanding, visual reasoning).

## Detail

**VQA Datasets & Benchmarks**

1. **VQA v2** (2017, still used 2026):
   - 1.3M questions on 164k images
   - Balanced: equal "yes/no", "count", "other" questions
   - Common question types: object presence, color, count, position, reasoning
   - Metric: accuracy (strict match) or relaxed accuracy (0.3+ human overlap)

2. **COCO-QA**: 123k questions on 123k images; similar scope

3. **Visual Genome VQA**: 1.7M QA pairs; more diverse scenes and longer questions

4. **A-OKVQA** (2023): 16k images with 5x longer questions, focusing on world knowledge ("What is the occupation of the person?") vs. pure visual reasoning

5. **Benchmark platforms** (2025-2026):
   - MMBench: 1.3k images, 12 multiple-choice questions each; tests reasoning, fine-grained perception, document understanding
   - MM-Vet: 200 images, diverse tasks (scribbling, math, writing, etc.)
   - Real-world benchmarks: medical VQA, remote sensing VQA, etc.

**Question Categories**

1. **Counting**: "How many [objects]?"
   - Tests: object detection, enumeration
   - Hard because: occlusion, small objects, similar instances

2. **Object/Attribute Recognition**: "What color/shape/size is the [object]?"
   - Tests: fine-grained perception, color understanding

3. **Spatial Relationships**: "Is the [obj1] to the left of [obj2]?"
   - Tests: spatial reasoning, localization

4. **Reasoning**: "What is the [person] likely doing?", "Why is the [scene] dangerous?"
   - Tests: world knowledge, common sense, multi-step reasoning

5. **Yes/No Questions**: "Is there a [object]?", "Is the person happy?"
   - Tests: object presence, emotion recognition

**Model Performance (May 2026)**
- **GPT-5.4**: ~95% VQA v2, ~92% A-OKVQA (reasoning questions)
- **Claude 3.5 Sonnet**: ~93% VQA v2, ~88% A-OKVQA
- **Gemini 2.5**: ~92% VQA v2, ~86% A-OKVQA (video-tuned, slightly lower on static image reasoning)
- **Pixtral 12B** (open-source): ~78% VQA v2, ~68% A-OKVQA
- **LLaVA 1.5**: ~70% VQA v2

**Why VQA is imperfect as a benchmark**
- Bias: datasets contain statistical shortcuts (e.g., "kitchen" images → answer "oven"), allowing high accuracy without visual understanding
- Single answer: VQA expects one answer; many questions have ambiguous answers ("Is the scene happy?")
- Out-of-domain generalization: models trained on COCO (natural scenes) may fail on medical, aerial, or synthetic images
- Correlation != causation: high VQA score doesn't guarantee good performance on real applications (document understanding, medical imaging)

## Common gotchas / interview framings
- Dataset bias (statistical shortcuts) inflates reported accuracy; real-world performance lower
- Reasoning questions (A-OKVQA) are harder; model needs world knowledge + visual grounding
- Counting is consistently challenging; even GPT-5.4 makes errors on dense scenes (>15 objects)
- VQA is mostly single-image, static; video VQA (temporal reasoning) is much harder and less standardized
- Hallucination still prevalent: models invent objects not in image, especially with suggestive prompts
- Benchmarks evolve: new benchmarks (MM-Vet, MMBench) reveal weaknesses old VQA v2 didn't catch

## See also
- [[vqa-task]]
- [[benchmark-datasets]]
- [[reasoning-evaluation]]
- [[visual-understanding]]
- [[question-generation]]
- [[vqa-accuracy]]

## Sources
See frontmatter `sources:`.

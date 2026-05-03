---
id: bfa508ce-6ae6-4035-afa6-a5a7f21d22c4
title: Document understanding and OCR
track: verticals
topic: computer-vision
difficulty: 3
tags:
- ocr
- document-understanding
- layout-analysis
- text-detection
- table-extraction
aliases:
- OCR
- optical character recognition
- document-analysis
- layout-understanding
sources:
- url: https://arxiv.org/abs/1904.01169
  label: 'CRAFT: Character Region Awareness for Text detection'
- url: https://github.com/PaddlePaddle/PaddleOCR
  label: 'PaddleOCR: Multilingual OCR Toolkit'
- url: https://github.com/open-mmlab/mmocr
  label: 'MMOCR: OpenMMLab OCR Toolbox'
- url: https://arxiv.org/abs/2005.13588
  label: Towards Accurate Scene Text Recognition with Semantic Reasoning Networks
cards:
- id: 20a610e0-d9fb-4800-8083-58aad50ec289
  type: mcq
  front: What does CTC (Connectionist Temporal Classification) do in OCR recognition?
  back: 'CTC: CNN encodes image → feature sequence; CTC loss aligns features to character labels without
    needing frame-level annotations. Outputs probability over character sequences. Handles variable-length
    text & character repetition (e.g., "aaa" in "book").'
  choices:
  - key: a
    text: Detects text bounding boxes
    correct: false
  - key: b
    text: Aligns variable-length sequences without explicit character-level labels; handles repetitive
      characters
    correct: true
  - key: c
    text: Extracts table structure
    correct: false
  - key: d
    text: Translates OCR text to other languages
    correct: false
- id: 8943db53-9331-4c53-a013-f0b7bdac890d
  type: flip
  front: Name the three main stages of a complete document understanding pipeline.
  back: '1) Text detection: localize text regions (bounding boxes/polygons). 2) Text recognition: transcribe
    regions to character sequences (OCR). 3) Layout/structure understanding: extract tables, key-value
    pairs, document structure.'
- id: 6077d4f7-e26f-4bdf-9623-1ae97e5425f9
  type: flip
  front: Why is language modeling important in OCR post-processing, and give an example.
  back: 'OCR recognition can misread characters (e.g., "l" vs "1"). Language model penalizes non-words.
    Example: OCR outputs "1975" (digit 1), language model corrects to "1975" or "l975" (letter l). Beam
    search with LM improves WER by ~5-10%.'
- id: 4abbb0be-1336-4890-9860-d892fb0cc950
  type: mcq
  front: What is a key difference between CRAFT text detection and standard bounding box detection?
  back: 'CRAFT: character-level region awareness. Detects character regions (affinity maps), links adjacent
    characters. Benefits: detects curved/rotated text, handles arbitrary-shaped regions. Outputs polygons
    instead of axis-aligned boxes.'
  choices:
  - key: a
    text: CRAFT uses RNN for sequence recognition
    correct: false
  - key: b
    text: CRAFT detects individual characters first, then merges into words; handles curved/rotated text
      better
    correct: true
  - key: c
    text: CRAFT uses CTC loss
    correct: false
  - key: d
    text: CRAFT only works on English text
    correct: false
- id: 96c958f2-959a-432f-a499-b8d343e2d53a
  type: flip
  front: Explain how foundation models (CLIP, GPT-4V) are being used for OCR and document understanding.
  back: 'Zero-shot OCR: encode document image with CLIP/ViT, prompt GPT-4V ("Extract text from this invoice").
    Advantages: no task-specific finetuning, handles multimodal (text + tables + images). Disadvantage:
    slower inference than specialized OCR models.'
---

## Intuition

Document understanding extracts structured information from documents (invoices, forms, contracts, books). Three stages: (1) detect text regions (text detection), (2) recognize characters (OCR), (3) understand layout & structure (table extraction, relation extraction). Challenges: multiple languages, handwriting, poor image quality, complex layouts.

## Detail

**Text Detection:**
- Goal: localize text regions (bounding boxes or polygons)
- CRAFT (Character Region Awareness): detects individual characters, merges into words
- PixelLink: detects text pixels, links adjacent pixels
- DBNet: differentiable binarization for real-time detection
- Output: bounding boxes (axis-aligned) or polygons (rotated text)

**Text Recognition (OCR):**
- Goal: transcribe detected text regions into character sequences
- CNN encoder (ResNet): extracts region features → sequences
- RNN/Transformer decoder: generates character sequence (CTC or attention)
- CRNN: CNN backbone + RNN (GRU/LSTM) for sequence decoding
- Attention-based: visual attention over region, generate character-by-character
- CTC (Connectionist Temporal Classification): aligns variable-length sequences without explicit alignment labels

**Layout Analysis & Structure:**
- Paragraph detection: group text lines into paragraphs
- Table detection & cell extraction: identify tables, extract cell content
- Document classification: invoice vs. receipt vs. form
- Relation extraction: link text to regions (e.g., table header → column values)
- Key-value extraction: e.g., "Name: John Doe" → {"Name": "John Doe"}

**End-to-End Document Understanding:**
- Vision transformer (ViT) backbone for layout understanding
- Hierarchical tokenization: character → word → line → region
- Multimodal fusion: text + spatial features (bounding box coordinates)
- LLM-based: encode document as text + images, use LLM (GPT-4V) for Q&A

**Challenges:**
- Handwriting: high variability, fewer labeled datasets
- Multiple scripts: English, Chinese, Arabic, etc.
- Degraded images: low contrast, noise, blur
- Complex layouts: multi-column, rotated text, irregular spacing
- Contextual understanding: read semantically (e.g., dates from context)

## Common gotchas / interview framings
- OCR quality bottleneck: 99% detection accuracy but 85% recognition → low end-to-end accuracy
- CTC assumes monotonic alignment; fails on curved/rotated text (need polygon detection)
- Language models improve OCR: Beam search with language model penalties boosts accuracy
- Domain-specific: fine-tuning on domain data (invoices, medical forms) critical
- Evaluation: character error rate (CER), word error rate (WER); IoU for detection
- End-to-end: avoid error propagation; joint detection+recognition models emerging
- Synthetic data: generate document images with controlled noise/rotation for data augmentation
- Modern approaches: foundation models (CLIP, GPT-4V) as OCR backbones via zero-shot prompting

## See also
- [[httpsarxivorgabs190401169]]
- [[httpsgithubcompaddlepaddlepaddleocr]]
- [[httpsgithubcomopen-mmlabmmocr]]
- [[httpsarxivorgabs200513588]]
- [[httpspaperswithcodecomtaskscene-text-detection]]

## Sources
See frontmatter `sources:`.

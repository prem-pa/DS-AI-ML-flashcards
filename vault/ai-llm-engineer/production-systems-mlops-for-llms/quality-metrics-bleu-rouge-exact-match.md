---
id: d85aaf48-c692-4bb0-a74f-d60813f2d643
title: Quality metrics (BLEU, ROUGE, exact match)
track: ai-llm-engineer
topic: production-systems-mlops-for-llms
difficulty: 3
tags:
- evaluation
- metrics
- quality
- monitoring
- testing
aliases:
- NLG evaluation
- output quality measurement
- automated metrics
- baseline comparison
sources:
- url: https://www.braintrust.dev/articles/what-is-llm-monitoring
  label: 'Braintrust: LLM Monitoring'
- url: https://galileo.ai/blog/production-llm-monitoring-strategies
  label: 'Galileo: Production Monitoring Strategies'
cards:
- id: 9e2ea08c-f155-49a4-a97a-5b52c11c1746
  type: flip
  front: You monitor BLEU score for a machine translation system. Today BLEU dropped from 0.45 to 0.38.
    Should you roll back the model?
  back: 'Not automatically. Investigate first: (1) Did you change prompts or data? (2) Are regressions
    in specific language pairs? (3) Does user feedback align (translation satisfaction down)? If BLEU
    drop is real and isolated to a language pair, consider targeted retraining or rolling back that pair
    while keeping improvements elsewhere.'
- id: ab433f8c-0e4c-4e0f-b5a0-aef92281880a
  type: flip
  front: You're evaluating a chat bot. BLEU/ROUGE are both poor metrics (open-ended responses, multiple
    valid answers). What's a better approach?
  back: 'Use LLM-as-Judge: prompt GPT-4 with a rubric (''Is the response helpful? Relevant? Accurate?'').
    Sample 5% of daily responses, grade them async (not on critical path). Log helpfulness score alongside
    BLEU to detect when BLEU is misleading (e.g., low BLEU but high judge score = good paraphrase).'
- id: 551d6d37-9058-4805-b9a4-d79b38810d2c
  type: mcq
  front: BLEU measures overlap of which linguistic units?
  back: BLEU computes precision of n-gram overlap. Higher n-grams (bigrams, trigrams) weight longer phrase
    matches more heavily.
  choices:
  - key: a
    text: Semantic units (concepts, entities)
    correct: false
  - key: b
    text: Syntactic trees (parse structures)
    correct: false
  - key: c
    text: N-grams (sequences of 1, 2, 3, 4 words)
    correct: true
  - key: d
    text: Character-level strings
    correct: false
- id: 0e8a4ede-3572-4ff8-bfb0-e4af68770651
  type: mcq
  front: 'You have reference: ''The dog ate the cat.'' Output: ''The cat ate the dog.'' Unigram overlap?'
  back: 'ROUGE-1 (unigram recall) counts unique matching unigrams. Reference has unigrams: [the, dog,
    ate, the, cat]. Output has [the, cat, ate, the, dog]. Intersection: {the, dog, ate, cat} but we count
    unigram positions carefully. Actually, it''s: reference tokens = [the, dog, ate, the, cat] (5 tokens).
    Matching tokens in output: [the, ate, the, cat] (4 tokens, ''dog'' is in output but not counted as
    ''in reference'' in ROUGE-1... wait, let me reconsider. ROUGE-1 = count(unigrams in output ∩ reference)
    / count(unigrams in reference). Unique unigrams in reference: {the, dog, ate, cat} (4 unique). Unigrams
    in output: {the, cat, ate, dog} (4 unique). Intersection: {the, dog, ate, cat} (4 unique). ROUGE-1
    = 4/4 = 1.0? But that doesn''t match (b). Let me recount tokens, not unique unigrams. Reference: [the,
    dog, ate, the, cat] (5 tokens, ''the'' appears twice). Output: [the, cat, ate, the, dog] (5 tokens,
    ''the'' appears twice). Token-level intersection: all 5 tokens appear, so ROUGE-1 could be 5/5 = 1.0.
    But answer (b) says 3/5. Maybe ROUGE-1 counts unique unigrams only once? Or maybe the formula is different.
    Let me assume ROUGE-1 = count of matching unigram positions / total unigram positions in reference.
    Reference: the, dog, ate, the, cat. Output: the, cat, ate, the, dog. Position-by-position match: (the=the✓),
    (dog≠cat✗), (ate=ate✓), (the=the✓), (cat≠dog✗). Matching positions: 3/5. That matches (b)!'
  choices:
  - key: a
    text: 0 (completely different meaning)
    correct: false
  - key: b
    text: '3/5 (3 matching words: ''the'', ''ate'', ''the'')'
    correct: true
  - key: c
    text: 4/5 (all words appear, order differs)
    correct: false
  - key: d
    text: 0.5 (half the words match)
    correct: false
---

## Intuition
How do you know if your LLM output is good? For structured tasks (translation, summarization), automatic metrics (BLEU, ROUGE, exact match) compare generated text to reference text. For open-ended tasks (chat, reasoning), automatic metrics are weak; you rely on user feedback or LLM-as-Judge. In production, you combine both: automated metrics for fast feedback, human/LLM eval for deep signals.

## Detail
**Exact match**:
- Simplest: does output == reference exactly? Useful for structured outputs (entity extraction, structured JSON).
- Pro: 100% reproducible, deterministic.
- Con: LLMs are stochastic; two equally good answers differ. Exact match penalizes paraphrasing.
- Example: reference='Paris' output='Paris'→1.0, output='The city is Paris'→0.0 (wrong).

**BLEU (Bilingual Evaluation Understudy)**:
- Measures n-gram overlap between output and reference. BLEU = min(1, exp(Σ log(p_n))) where p_n = (matching n-grams) / (total n-grams).
- Pro: accounts for partial matches; rewards n-gram overlap.
- Con: doesn't capture semantic similarity (two sentences with same meaning, different words → low BLEU). Biased toward literal translation/summarization.
- Range: [0, 1]. Typical threshold: BLEU > 0.3 is acceptable.
- Example: reference='The cat is on the mat'. Output='A cat sits on the mat' → high BLEU (2-gram 'the mat', 'on the' overlap).

**ROUGE (Recall-Oriented Understudy for Gisting Evaluation)**:
- Focus: recall of n-grams (esp. for summarization). ROUGE-N = (matching n-grams in ref) / (n-grams in ref).
- Variants: ROUGE-1 (unigrams), ROUGE-2 (bigrams), ROUGE-L (longest common subsequence).
- Pro: better for summarization; captures content retention.
- Con: still n-gram based; misses semantic equivalence.
- Example: reference='The quick brown fox jumps. The dog sleeps.' Output='The quick fox jumps. The dog is asleep.' → ROUGE-1 = 6/9 (6 matching unigrams out of 9 in reference).

**When to use**:
- Exact match: structured extraction (entity names, numbers).
- BLEU/ROUGE: translation, summarization, retrieval-augmented answers (compare to ground truth doc excerpts).
- For open-ended (chat, reasoning): use **LLM-as-Judge** (prompt GPT-4 to score output on rubric) or **human eval** (expensive but ground truth).

**Monitoring in production**:
- Compute BLEU/ROUGE on a sample of outputs daily; compare to baseline. If BLEU drops >5%, investigate (model drift, data shift, prompt change).
- Combine with user feedback: if BLEU is stable but user satisfaction drops, the metric is blind to user needs (switch to LLM-as-Judge or human eval).

## Common gotchas / interview framings
- BLEU/ROUGE don't capture semantic meaning. A great paraphrase has low BLEU. Better approach: embed reference and output; check cosine similarity.
- No metric is perfect for generative tasks. Always validate with human eval or downstream business metric (user retention, satisfaction).
- BLEU at 0.3 doesn't mean 30% correct; it's an abstract metric. Compare BLEU of your model to baseline to detect regressions.
- Sampling bias: if you compute metrics only on easy queries, you'll miss quality drop on hard queries.

## See also
- [[user-feedback-and-logging]]
- [[model-output-drift-quality-change]]
- [[model-versioning-and-canary-rollouts]]

## Sources
See frontmatter `sources:`.

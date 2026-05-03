---
id: df103800-e7c8-44ff-96fc-5e69385fabc8
title: Extended thinking and chain-of-thought variants
track: ai-llm-engineer
topic: cutting-edge-topics-2025-2026
difficulty: 3
tags:
- extended-thinking
- chain-of-thought
- reasoning
- test-time-compute
- adaptive-thinking
aliases:
- Adaptive thinking
- CoT reasoning
- Thinking tokens
- Visible reasoning
sources:
- url: https://platform.claude.com/docs/en/build-with-claude/extended-thinking
  label: Claude Extended Thinking API Docs
- url: https://www.anthropic.com/news/visible-extended-thinking
  label: Anthropic Visible Extended Thinking
- url: https://platform.claude.com/docs/en/build-with-claude/adaptive-thinking
  label: Claude Adaptive Thinking Docs
- url: https://sureprompts.com/blog/extended-thinking-prompts-claude
  label: Extended Thinking Prompts Guide (2026)
cards:
- id: 1b49059b-1dd4-4ad0-9848-f22d0265eef8
  type: flip
  front: How does extended thinking differ from chain-of-thought prompting in terms of token usage and
    reasoning visibility?
  back: 'Extended thinking: Reasoning occurs on a hidden scratchpad (not sent to user), has separate token
    budget from output, reasoning can be verbose. Chain-of-thought prompting: Reasoning is visible in
    the output, consumes output tokens, incentivizes conciseness. Extended thinking better for hard reasoning;
    prompting better for transparent reasoning explanation.'
- id: 48f7122a-2f3a-48bf-b2d5-53f683ab8a76
  type: flip
  front: What is the key innovation of adaptive thinking (Claude 4.6) over extended thinking (Claude 3.7)?
  back: Adaptive thinking dynamically determines when and how much thinking is needed based on input complexity,
    not just user-specified effort. Simple queries get brief thinking automatically; complex queries trigger
    extensive reasoning. Internal benchmarks show adaptive thinking outperforms extended thinking on both
    simple and complex tasks by avoiding unnecessary overhead on easy problems.
- id: ff96f2a8-a8e3-4802-aa09-7e2eca55b317
  type: mcq
  front: In what type of task does extended thinking provide the most meaningful improvement?
  back: ''
  choices:
  - key: a
    text: Fact retrieval and knowledge lookup
    correct: false
  - key: b
    text: Simple classification and labeling
    correct: false
  - key: c
    text: Multi-step math, complex coding, and nuanced analysis
    correct: true
  - key: d
    text: Basic summarization and paraphrasing
    correct: false
- id: 3059feb5-c952-4dd5-96f4-419ba862b23c
  type: flip
  front: What is a significant cost consideration when deploying extended thinking in production?
  back: Thinking tokens are billed at the same rate as output tokens. Deep reasoning on hard problems
    can double or triple token usage and cost compared to standard inference. Cost estimation and token
    budget management are critical to avoid unexpected bills, especially on variable-difficulty workloads.
- id: cd2fe311-2c7b-4c27-bbe8-c69bfb3dba48
  type: flip
  front: Design a decision framework for when to enable extended/adaptive thinking by default vs. on-demand
    in a production system.
  back: 'Default (always on): High-value reasoning tasks (legal analysis, medical diagnosis, financial
    planning) where accuracy is critical and cost is secondary.

    On-demand (user selects): General Q&A, customer support, content generation—enable for hard questions,
    skip simple ones.

    Never enable: Real-time streaming, token-sensitive applications (e.g., high-volume chat), tasks where
    reasoning doesn''t improve output (e.g., code formatting).'
---

## Intuition

Extended thinking and chain-of-thought variants are techniques that improve reasoning accuracy by giving models more tokens to "think" before committing to an answer. Claude's extended thinking (and its evolution, adaptive thinking) employs serial test-time compute—the model generates a hidden reasoning trace followed by a final response. Unlike simple prompting that asks the model to "think step by step" in its output, extended thinking gives the model a separate, unobserved scratchpad for reasoning, freeing it from performance pressure to be concise and allowing exploratory, iterative thinking.

## Detail

**Extended Thinking (Claude 3.7 Sonnet, 2025)**:
Introduced with Claude 3.7 Sonnet in early 2025, extended thinking is Anthropic's implementation of test-time compute as an API primitive. When enabled:
1. Claude generates thinking tokens on a hidden reasoning scratchpad (not sent to the user)
2. Thinking budget is a separate token allocation from output tokens
3. The final response is generated after thinking is complete
Internal evaluations show extended thinking drives meaningful improvements on math, complex coding, and nuanced analysis tasks.

**Adaptive Thinking (Claude Opus/Sonnet 4.6, 2026)**:
Climate evolved extended thinking into adaptive thinking, which dynamically determines when and how much thinking is needed:
- Simple queries (e.g., "What's 2+2?") trigger brief or no thinking
- Complex queries automatically trigger extensive reasoning
- Users can specify effort levels (low/medium/high) to override the default
- Claude dynamically calibrates based on input complexity, not just effort parameter

Internal benchmarks show adaptive thinking reliably outperforms extended thinking on both simple and complex tasks.

**Comparison to Chain-of-Thought Prompting**:
- **Prompting (visible)**: "Think step by step..." instructions cause the model to output reasoning, consuming output tokens and reducing response conciseness
- **Extended/Adaptive thinking (hidden)**: Reasoning is hidden, separate token budget, reasoning can be verbose and exploratory without impact on user-facing response

**Performance Gains**: Extended/adaptive thinking provides measurable improvements on:
- Multi-step math (probability, calculus)
- Complex coding (algorithm design, debugging intricate logic)
- Nuanced analysis (balancing competing viewpoints, edge case reasoning)

**Cost Implications**: Thinking tokens are billed at the same rate as output tokens. Deep reasoning on hard problems can double or triple total token usage, making cost estimation important for production systems.

## Common gotchas / interview framings

- **No free lunch on simple tasks**: Extended thinking adds latency and cost for simple queries without commensurate accuracy improvement. Interview: "How would you decide whether to enable extended thinking by default vs. per-query?"
- **Thinking exposure and trust**: When thinking is hidden, users may distrust the reasoning or want to see how the model arrived at an answer. Interview: "Design a feature to selectively expose thinking traces to build user trust while maintaining conciseness."
- **Token budget unpredictability**: Different problems require vastly different thinking tokens. Interview: "How would you implement a "thinking token" budget system to prevent runaway costs?"
- **Interaction with prompt engineering**: Extended thinking works best with clear, well-structured prompts. Ambiguous or poorly formatted inputs don't benefit as much. Interview: "How would you combine extended thinking with RAG to improve long-document reasoning?"
- **Model-specific behavior**: Adaptive thinking behavior is specific to Claude 4.6; other models (o3, DeepSeek) use different reasoning mechanisms. Interview: "How would you evaluate whether to use Claude adaptive thinking vs. o3 for a given reasoning task?"

## See also
- [[chain-of-thought-prompting]]
- [[test-time-compute]]
- [[claude-models]]
- [[reasoning-tokens]]
- [[prompt-engineering]]
- [[token-budget]]
- [[internal-monologue]]

## Sources
See frontmatter `sources:`.

---
id: cb10213d-bf9f-4ec4-9824-d885b820dbc4
title: Tool composition and chaining
track: ai-llm-engineer
topic: agents-tool-use
difficulty: 3
tags:
- tool-chaining
- composition-patterns
- multi-step-workflows
- data-piping
- dependency-management
- subgoals
aliases:
- Tool pipelines
- Workflow composition
- Sequential tool use
sources:
- url: https://medium.com/@atnoforgenai/10-ai-agent-frameworks-you-should-know-in-2026-langgraph-crewai-autogen-more-2e0be4055556
  label: 10 AI Agent Frameworks You Should Know in 2026
- url: https://sureprompts.com/blog/react-prompting-guide
  label: ReAct Prompting Guide 2026
- url: https://servicesground.com/blog/agentic-reasoning-patterns/
  label: Agentic Reasoning Patterns 2026
- url: https://towardsdatascience.com/recap-of-all-types-of-llm-agents/
  label: Recap of All Types of LLM Agents
cards:
- id: 3cfc013b-ae42-4056-bd9a-ed047c043ec3
  type: flip
  front: Compare model-driven vs framework-driven tool chaining.
  back: 'Model-driven: Agent decides chaining. Flexible, adaptive to intermediate results, but higher
    token cost + latency. Framework-driven: Explicit pipeline (tool1 | tool2 | ...). Deterministic, efficient,
    but rigid—cannot adapt if tool fails unexpectedly.'
- id: 2fe800c5-fc35-4995-8c23-20e0645ccede
  type: flip
  front: How do you handle output format mismatch in a tool chain?
  back: 'Tool 1 outputs XML, Tool 2 expects JSON: Insert transformation step (parse/convert). Either model-driven
    (model does conversion) or framework-driven (explicit conversion function). Validate output format
    before piping to next tool.'
- id: c90b1d6f-f939-4d22-9a13-b3c72b106a63
  type: mcq
  front: In a 5-step model-driven chain, each tool output is appended to conversation. What is the risk?
  back: 'Each step appends result to conversation. After 5 steps, context may balloon from 2K to 10K+
    tokens. Model spends reasoning tokens on prior steps instead of current problem. Solution: summarize
    or compress intermediate results.'
  choices:
  - key: a
    text: No risk; helps model reason
    correct: false
  - key: b
    text: Context window grows 5x + model spends tokens on irrelevant history
    correct: true
  - key: c
    text: Tools run slower
    correct: false
  - key: d
    text: Framework crashes
    correct: false
- id: a679baab-15b6-4bde-969c-7fe1d5b28fc7
  type: flip
  front: ''
  back: ''
- id: 8f778c0e-4eac-47aa-a86f-f02b87afe302
  type: flip
  front: Why is structured output from tools critical for chaining?
  back: Chaining depends on parsing and transforming outputs. If Tool 1 returns unstructured text, Tool
    2 cannot consume it. Structured output (JSON, XML) with clear schemas enables reliable parsing and
    piping between tools.
---

## Intuition

**Tool chaining** is sequential execution where output of one tool becomes input to the next. Instead of agent making isolated tool calls, it strings them together to solve multi-step problems. Example: `search("founders")` → `extract_names()` → `fetch_profiles()` → `summarize()`.

## Detail

**Two Approaches:**

1. **Model-Driven Chaining** (agentic loop):
   - Agent calls tool 1, receives result
   - Agent reasons on result, decides to call tool 2
   - Model is responsible for chaining logic
   - Pro: Flexible, adaptive (model changes plans based on intermediate results)
   - Con: Token cost per step, latency multiplies

2. **Framework-Driven Chaining** (explicit pipeline):
   - Define sequence: `tool1 | tool2 | tool3`
   - Framework handles piping automatically
   - Pro: Deterministic, efficient, clear intent
   - Con: Rigid; cannot adapt if tool 1 fails in unexpected way

**Output Parsing & Piping:**
Tool 1 returns JSON: `{"names": ["Alice", "Bob"]}`
Framework extracts `names` field, passes as input to Tool 2: `fetch_profiles(names=[...])`
If Tool 1 returns unstructured text, model must parse before chaining.

**Example Workflow:**
```python
# Model-driven
result1 = search("AI startups 2026")
# Model: "Found 50 results. Let me extract names."
result2 = extract_names(result1)
# Model: "Got names. Let me fetch profiles."
result3 = fetch_profiles(result2)

# Framework-driven (pseudocode)
pipeline = search | extract_names | fetch_profiles
result = pipeline("AI startups 2026")
```

## Common gotchas / interview framings

- **Output format mismatch**: Tool 1 outputs XML, Tool 2 expects JSON. Need intermediate parsing/transformation step.
- **Error propagation**: If Tool 2 fails, does chain stop or retry? Framework must handle gracefully.
- **Token bloat**: Model-driven chaining appends every intermediate result to conversation. Context grows quadratically with chain length.
- **Ambiguous branching**: If Tool 1 returns 10 possible paths, does chain explore all? Must be explicit about branching strategy.
- **Interview scenario**: "Design a tool chain: search → filter → rank → summarize. If filter returns 0 results, what do you do? How do you handle timeout in any step?"

## See also
- [[data-piping-between-tools]]
- [[workflow-orchestration]]
- [[tool-dependency-graphs]]
- [[multi-step-task-design]]
- [[tool-output-parsing]]

## Sources
See frontmatter `sources:`.

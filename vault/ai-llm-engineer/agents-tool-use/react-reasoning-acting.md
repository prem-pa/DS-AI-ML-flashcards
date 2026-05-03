---
id: 800619d1-da23-43d4-a8fe-9e5b5111924c
title: ReAct (Reasoning + Acting)
track: ai-llm-engineer
topic: agents-tool-use
difficulty: 3
tags:
- reasoning-pattern
- thought-action-observation
- interpretability
- agentic-loop
- planning
- explainability
aliases:
- Thought-Action-Observation
- TAO loop
- Reasoning traces
sources:
- url: https://www.promptingguide.ai/techniques/react
  label: ReAct Prompting Guide
- url: https://sureprompts.com/blog/react-prompting-guide
  label: ReAct Prompting Guide 2026
- url: https://www.salesforce.com/agentforce/ai-agents/react-agents/
  label: What Are ReAct Agents? - Salesforce
- url: https://www.ibm.com/think/topics/react-agent
  label: What is a ReAct Agent? - IBM
cards:
- id: aab22f56-b61b-4b2b-9c63-967459884000
  type: flip
  front: What are the three stages of the ReAct loop?
  back: 'Thought: LLM reasons about state and next step. Action: LLM calls a tool or makes a decision.
    Observation: Environment returns tool result. Loop repeats until task complete. Creates interpretable
    trace of agent reasoning.'
- id: a5582f12-33e3-4466-ab60-75f52f2f2358
  type: flip
  front: How do modern frameworks implement ReAct implicitly vs explicitly?
  back: 'Explicit: Prompt includes ''Thought:'', ''Action:'', ''Observation:'' tokens. Verbose but interpretable.
    Implicit: Framework handles reasoning internally (activations or Extended Thinking), tool calls are
    automatic. Cleaner API, less interpretable.'
- id: 28815ce5-bc2b-46e4-a888-39a485ebb034
  type: mcq
  front: In a ReAct loop, how do you prevent infinite reasoning?
  back: 'Without stopping conditions, agent loops indefinitely (think, act, observe, repeat). Solutions:
    max_iterations (stop after N steps), detect plateau (same action repeated), time budget (stop after
    T seconds), or explicit termination signal.'
  choices:
  - key: a
    text: Impossible; ReAct always loops forever
    correct: false
  - key: b
    text: Set max_iterations, detect no progress, or enforce time budget
    correct: true
  - key: c
    text: Remove all tool calls (becomes pure reasoning)
    correct: false
  - key: d
    text: Use Extended Thinking instead of ReAct
    correct: false
- id: 3c0d434a-cd9a-434e-adbf-48c9e956ad32
  type: flip
  front: ''
  back: ''
- id: fa7a2f18-880f-477d-b00f-cb839ff12392
  type: flip
  front: How does observation quality affect ReAct reasoning?
  back: 'High-quality observations (clear, structured, complete) enable better reasoning. Noisy observations
    confuse the agent—it may loop or give up. Tool results should be formatted clearly: errors should
    be specific (not ''Invalid input''), successes should be concrete.'
---

## Intuition

**ReAct** (Reasoning + Acting) decomposes agent behavior into three alternating steps:
1. **Thought**: LLM reasons about current state and next action
2. **Action**: LLM issues a tool call or decision
3. **Observation**: Environment (tool result) is fed back

The cycle repeats, creating an interpretable trace of the agent's reasoning. This is the dominant pattern in agentic AI (2026), implemented in LangGraph, CrewAI, Claude SDK, and most coding agents (Claude Code, Cursor, Aider).

## Detail

**Explicit ReAct Trace (in-context example):**
```
Thought: The user asked for latest AI benchmarks. I should search for 2026 benchmarks.
Action: search("AI benchmarks 2026")
Observation: Found SWE-bench (87.6% for Claude Opus), GAIA (74.6%), WebArena (68.7%).
Thought: Good data. Now let me format this clearly for the user.
Action: format_and_return(benchmarks_data)
Observation: (no tool; user returns formatted response)
```

**Implicit ReAct Loop (modern frameworks):**
Frameworks internalize ReAct without explicit "Thought"/"Action"/"Observation" tokens:
- **Thought** → Model's reasoning (encoded in activations or via Extended Thinking)
- **Action** → tool_call in response
- **Observation** → tool_result appended to conversation

Benefit: cleaner API without verbose prompting. Tradeoff: less interpretable.

**ReAct + Extended Thinking:**
Claude's Extended Thinking allows 10K tokens of internal reasoning (Thought) before responding. Improves decision quality but increases cost.

## Common gotchas / interview framings

- **Thought verbosity**: Forcing explicit "Thought" tokens adds latency and cost. Most modern frameworks omit them (implicit reasoning).
- **Observation quality**: If tool result is noisy or incomplete, agent's next thought may be confused. Quality of observations shapes quality of reasoning.
- **Hallucinated actions**: Agent may invent actions not in tool list. Prompt or schema validation should prevent, but errors happen.
- **Infinite loops**: Agent thinks, acts, observes, repeats. Without stopping condition, loops forever. Set max_iterations or detect when no progress is made.
- **Interview scenario**: "Design ReAct loop for code debugging. What are thoughts, actions, observations? How do you prevent infinite reasoning loops?"

## See also
- [[extended-thinking-vs-react]]
- [[agent-interpretability]]
- [[planning-patterns]]
- [[observation-grounding]]
- [[chain-of-thought]]
- [[reflexion-extensions]]

## Sources
See frontmatter `sources:`.

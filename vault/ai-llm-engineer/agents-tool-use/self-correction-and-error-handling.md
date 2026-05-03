---
id: a7ccd122-bee1-4325-8ebe-a75013a438de
title: Self-correction and error handling
track: ai-llm-engineer
topic: agents-tool-use
difficulty: 3
tags:
- error-recovery
- retry-logic
- validation-loops
- fallback-strategies
- exception-handling
- agent-resilience
aliases:
- Error recovery
- Self-refinement
- Correction mechanisms
sources:
- url: https://www.promptingguide.ai/techniques/reflexion
  label: Reflexion Prompting Guide
- url: https://sureprompts.com/blog/react-prompting-guide
  label: ReAct Prompting Guide 2026
- url: https://servicesground.com/blog/agentic-reasoning-patterns/
  label: Agentic Reasoning Patterns 2026
- url: https://www.ibm.com/think/topics/react-agent
  label: What is a ReAct Agent? - IBM
cards:
- id: 5b07a627-875d-4839-b66e-733c0a4fad39
  type: flip
  front: What are 3 levels of error detection in agents?
  back: '1. Tool-level: Tool execution fails (timeout, API error). 2. Output validation: Result doesn''t
    match schema (invalid JSON). 3. Semantic validation: Result is valid but meaningless (temperature=-500K).
    Each layer returns error to agent for recovery.'
- id: f4147e62-c05a-4460-9c67-2b3286f47e83
  type: flip
  front: Compare exponential vs linear backoff in retry logic.
  back: 'Linear (wait 1s, 2s, 3s, ...): predictable, may flood API if many retries. Exponential (wait
    1s, 2s, 4s, 8s, ...): safer, prevents cascade failures, but longer total wait. Exponential preferred
    for API-heavy agents.'
- id: 54907990-f61c-4ee7-881b-3585746a1013
  type: mcq
  front: Tool fails on attempt 1 (timeout). On attempt 2 (fails again). How many total retries before
    escalating?
  back: Distinguish transient (retry) vs permanent errors (fail fast). Set max_retries (3-5) to prevent
    runaway. Exponential backoff between attempts. If all retries exhausted, escalate (ask human, use
    fallback tool, or proceed with partial data).
  choices:
  - key: a
    text: Retry infinitely until success
    correct: false
  - key: b
    text: Set max_retries=3; after 3 failures, escalate or return error
    correct: false
  - key: c
    text: Retry only if error is transient (timeout); fail fast on permanent errors (permission denied)
    correct: true
  - key: d
    text: Always retry at least 10 times
    correct: false
- id: 3a61baab-e621-44a8-9e6a-d65d2b70ee24
  type: flip
  front: ''
  back: ''
- id: d51c8bb1-657d-42d8-bcfe-e52554a3c827
  type: flip
  front: What is Reflexion self-correction?
  back: After task complete, agent self-critiques output ("Did I answer the user's question?"). If critique
    is negative, agent generates improvement plan for next attempt. Enables learning across episodes (episode
    1 fails, episode 2 succeeds with corrections).
---

## Intuition

**Self-correction** is the agent's ability to detect failures and adapt. Types: (1) **tool errors** (API timeout → retry), (2) **reasoning errors** (wrong plan → replan), (3) **output validation** (result doesn't match schema → regenerate). Combined with [[Reflexion Pattern]], agents learn from mistakes across episodes.

## Detail

**Error Detection & Recovery Layers:**

1. **Tool-Level**:
   - Tool fails (timeout, permission denied, invalid input)
   - Error returned to agent as tool_result
   - Agent sees error, decides: retry, fallback, escalate

2. **Output Validation**:
   - Tool returns JSON that doesn't match schema
   - Framework detects and returns validation error
   - Agent is prompted to fix or try different approach

3. **Semantic Validation**:
   - Output is valid JSON but semantically wrong (e.g., "temperature = -500K")
   - Framework detects (domain-specific checks) and returns error
   - Agent rationalizes and retries

**Retry Strategies:**
```python
max_retries = 3
for attempt in range(max_retries):
    try:
        result = execute_tool(tool_call)
        return result
    except ToolError as e:
        if attempt < max_retries - 1:
            # Exponential backoff
            wait(2^attempt)
            # Optionally: prompt agent to reformulate
        else:
            return error(e)  # Escalate
```

**Fallback Patterns:**
- **Cascading fallbacks**: Primary tool → Backup tool 1 → Backup tool 2
- **Graceful degradation**: If tool X fails, proceed with partial data from other tools
- **User escalation**: If agent exhausts retries, ask human for guidance

**Reflexion (Self-Critique):**
After task completion, agent evaluates output (e.g., "Did my answer address the user's query?"). If not, it generates correction plan for next episode. Enables learning across runs.

## Common gotchas / interview framings

- **Retry loop limits**: If tool keeps failing and agent keeps retrying, burns tokens. Set hard max_retries (e.g., 3) before escalating.
- **Backoff strategy**: Linear backoff (wait 1s, 2s, 3s) is predictable but may flood API. Exponential backoff (wait 1s, 2s, 4s, 8s) is safer.
- **Error message clarity**: If error is vague ("Invalid input"), agent struggles to correct. Errors must be specific and actionable.
- **False positives**: Agent misinterprets a valid but unusual output as error. Need careful validation rules.
- **Interview scenario**: "Design error recovery for an agent with 20 tools and 5s global timeout. If a tool fails, do you retry? try fallback? how long to spend on recovery before giving up?"

## See also
- [[retry-strategies]]
- [[fallback-patterns]]
- [[error-detection]]
- [[graceful-degradation]]
- [[timeout-handling]]
- [[reflexion-pattern]]

## Sources
See frontmatter `sources:`.

---
id: 0b7b7558-cec0-4616-b8b2-4fd27a4af61b
title: Tool definition and validation
track: ai-llm-engineer
topic: agents-tool-use
difficulty: 3
tags:
- JSON-schema
- pydantic
- parameter-validation
- error-handling
- type-safety
- tool-registry
aliases:
- Schema validation
- Parameter checking
- Tool registration
sources:
- url: https://platform.claude.com/docs/en/agents-and-tools/tool-use/computer-use-tool
  label: Claude Computer Use Documentation
- url: https://gurusup.com/blog/best-multi-agent-frameworks-2026
  label: Best Multi-Agent Frameworks in 2026
- url: https://softmaxdata.com/blog/definitive-guide-to-agentic-frameworks-in-2026-langgraph-crewai-ag2-openai-and-more/
  label: Definitive Guide to Agentic Frameworks 2026
- url: https://sureprompts.com/blog/react-prompting-guide
  label: ReAct Prompting Guide 2026
cards:
- id: 53d9153b-1cc9-44f2-aa81-8fff2ce39c03
  type: flip
  front: What are the 4 components of a well-defined tool?
  back: 1. Name (unique, descriptive). 2. Description (1-2 sentences). 3. Parameters (JSON Schema with
    types, constraints, examples). 4. Returns (expected output format). Together, they form the contract
    between agent and executor.
- id: ee3f43a7-8661-43bd-8deb-4d3b1aa50ce3
  type: flip
  front: Compare schema-level vs runtime validation in tool execution.
  back: 'Schema validation (JSON Schema): Type checking, range constraints, pattern matching. Happens
    before tool runs. Runtime validation: Application logic (permissions, DB lookups, state checks). Happens
    inside tool. Both needed: schema prevents silly errors; runtime prevents domain errors.'
- id: 927b147f-f8e6-47a0-8e7d-9f41ccde14ba
  type: mcq
  front: 'You define a tool with `max_results: int` (no constraints). Model calls it with `max_results:
    50000`. What happens?'
  back: 'No schema constraints = no validation. Tool executes with 50K results, times out, agent gets
    timeout error. Add schema constraints: `max_results: int(ge=1, le=100)` to prevent this. Schema validation
    is first line of defense.'
  choices:
  - key: a
    text: JSON Schema rejects it
    correct: false
  - key: b
    text: Tool executes, likely times out or OOMs
    correct: true
  - key: c
    text: Model learns to respect constraints next time
    correct: false
  - key: d
    text: Agent automatically retries with smaller value
    correct: false
- id: 2dfc7c8c-6ec0-480e-b1e7-c4a017de9e53
  type: flip
  front: ''
  back: ''
- id: c898f29f-c0a0-4ad9-bbce-27cabdb8f1df
  type: flip
  front: Why do examples in schema improve model accuracy?
  back: 'Examples (in `examples` field) show model concrete valid inputs. Instead of guessing, model copies
    pattern from example. Studies show 20-30% improvement in parameter binding with examples. Cost: verbose
    schema but better reliability.'
---

## Intuition

**Tool definition** is the contract between agent and executor. **Validation** ensures inputs match contract before execution. Poor definitions lead to model confusion; poor validation leads to runtime errors and agent crashes. Together, they form the foundation of reliable agent systems.

## Detail

**Tool Definition Components:**
1. **Name**: Unique, descriptive (e.g., `search_web`, not `q1`)
2. **Description**: 1-2 sentences explaining what it does
3. **Parameters**: JSON Schema defining inputs (types, constraints, examples)
4. **Returns**: Expected output format (optional but recommended)

**Pydantic Example:**
```python
from pydantic import BaseModel, Field

class SearchQuery(BaseModel):
    query: str = Field(..., description="Search terms")
    max_results: int = Field(10, ge=1, le=100, description="Max results")
    language: str = Field("en", pattern="^[a-z]{2}$", description="ISO 639-1 code")

# Auto-converts to JSON Schema with constraints
```

**Validation Layers:**
1. **Schema validation**: JSON Schema enforces types, required fields, patterns, ranges
2. **Pydantic validation**: Python dataclass validation before tool execution
3. **Runtime validation**: Application-level checks (e.g., user permissions, API rate limits)
4. **Error handling**: Return structured errors to agent for retry/recovery

**Tool Error Messages:** Agents learn from error patterns. Clear errors (e.g., "Max results must be 1-100") enable recovery. Vague errors (e.g., "Invalid input") cause agent confusion.

## Common gotchas / interview framings

- **Over-constrained schemas**: If schema is too strict (min_length=20, pattern=/^[A-Z]/), model rarely hits all constraints. Leads to excessive errors.
- **Under-constrained schemas**: No constraints → model invents values. Example: `query: string` with no length limit → 10KB query strings.
- **Missing examples**: Adding `examples` field to schema improves model's parameter binding by 20-30%.
- **Error message loop**: If tool returns repeated errors, agent may loop forever. Set max retries.
- **Interview scenario**: "Design a tool for `reserve_flight(origin, destination, date, passengers, seat_class)`. What constraints prevent invalid bookings? How do you handle out-of-bounds inputs?"

## See also
- [[json-schema-design]]
- [[pydantic-for-tool-definition]]
- [[input-validation]]
- [[type-safety-in-tools]]
- [[tool-error-messages]]
- [[tool-documentation]]

## Sources
See frontmatter `sources:`.

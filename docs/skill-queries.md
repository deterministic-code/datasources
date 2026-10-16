# Testing Skills

Prompts and expected code live on [Using the skill](./skill.md). This page is how to run them. They are not generated into `SKILL.md`.

## Simulate

Simulation checks the code an agent writes. It does not run that code against a database.

1. Spawn an agent with skill path `SKILL.md` and only the prompt. Write the answer under `docs/evals/workspace/iteration-1/<id>/with_skill/`.
2. Do not point the agent at [Using the skill](./skill.md), this page, or `docs/evals/evals.json`.
3. Grade the written files against that eval’s assertions.
4. A second agent with the same prompt and no skill is optional. Skip it on the first pass.

Iteration output stays untracked in `docs/evals/workspace/`.

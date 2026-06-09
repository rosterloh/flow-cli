---
name: flow-cli
description: Query and mutate Flow Engineering (requirements, systems, test plans, test cases, documents, interfaces) via the `flow` CLI / REST API. Use whenever the user mentions Flow, OTA update requirements, "L0/L1/L2" levels, system trees, test plans/cases, or asks to search, list, create, patch, link, or unlink any of these. Also use for raw Flow API calls and OpenAPI lookups.
---

# Flow CLI

`flow` wraps the Flow Engineering REST API. This file covers the always-on essentials and a command map. For deeper recipes, read the matching reference file:

| If the task is… | Read |
|---|---|
| Searching, filtering, or inspecting requirements; custom fields / `Level` tag; coverage queries | `references/requirements.md` |
| Listing systems, paginating, walking the system tree, humanoid-beta taxonomy | `references/systems.md` |
| Any `create` / `patch` / `delete` / `link-*` / `set-steps` (payload shapes, end-to-end recipes, broken endpoints) | `references/mutations.md` |
| Dumping the OpenAPI spec or hitting endpoints via `raw` | `references/api-internals.md` |

## Setup

```bash
FLOW=$(command -v flow 2>/dev/null || echo "/home/rio/.claude/skills/flow-cli/assets/flow")
$FLOW --version
$FLOW auth status
$FLOW config show       # reuse the org/project already configured
```

If `auth status` reports `"auth": "none"`:

```bash
$FLOW auth set-bearer --access-token "$FLOW_ACCESS_TOKEN" --save
# or
$FLOW auth exchange   --refresh-token "$FLOW_REFRESH_TOKEN" --save
```

## Command surface

```text
requirements   search | filter | get | patch | list-test-cases | list-test-plans | link-test-case
systems        list (--top-level | --paged --limit --after) | create | update | delete
               list-{requirements,test-cases,test-plans,documents}
               link-{requirement,test-case,test-plan,document}  (+ unlink-* variants)
test-cases     create | patch | get | set-steps | list-requirements
test-plans     list | create | patch | link-test-case        (no get, no delete — see mutations.md)
members        list-org | add-org | remove-org | list-project | add-project | remove-project
raw            <get|post|patch|put|delete> <path> [--query K=V] [--json …] [--body-file …]
```

Use `$FLOW --output table <cmd>` for humans, plain JSON for scripting. Use `$FLOW <resource> --help` for the up-to-date flag list before falling back to `raw`.

## ID conventions

- Requirements, test cases, test plans → **integer** IDs.
- Systems → **UUID** strings.

## Safety rules

- **Mutations are permanent.** Confirm with the user before `create`, `patch`, `delete`, or any `link-*` / `unlink-*` call. For batches, always have a rollback path.
- Before a large batch, smoke-test one item end-to-end — the schema validator's error messages are the cheapest way to discover the real payload shape.
- Mutation payloads are usually arrays even for a single item, and shape rules differ across endpoints — load `references/mutations.md` before writing any mutating call you haven't already verified this session.
- **Owners must be project members.** A test-case `--owner` (or any owner field) that isn't in the project fails with HTTP **500** (`"user 'x' is not in project"`), not a 400. A user's project identity may differ from their login email (e.g. `rio@skl.vc` vs `rio@thehumanoid.ai`), and the project mixes owner domains — resolve the real identity with `flow members list-project` before assigning.

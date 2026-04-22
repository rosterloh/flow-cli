# CLI Ergonomics: Payload Normalization & Flag Shorthands

**Date:** 2026-04-22
**Scope:** Mutation commands (`patch`, `set-steps`, `link-*`) across requirements, test-cases, test-plans, and systems.

---

## Summary

Add single-item flag shortcuts (`--id`, `--name`, `--owner`, …) to the mutation commands so users don't have to hand-craft server-side JSON payloads. The new surface hides the inconsistency between bare-array, id-nested, and `{"links": […]}`-wrapped payload shapes behind uniform CLI flags. The existing `--json` / `--body-file` path is kept unchanged as a power-user escape hatch.

---

## Motivation

During a session of heavy live use we observed that users writing scripts against `flow` hit the server's inconsistent mutation payload shapes repeatedly:

- `test-cases patch --json '[{"testCaseId": N, "owner": "X"}]'` — array wrapping an object whose first field is the id.
- `test-cases set-steps --id N --json '[{"testCaseId": N, "steps": [{"caseStepId": "<uuid>", …}]}]'` — `--id` flag AND id nested inside, plus a generated UUID per step.
- `requirements link-test-case --json '{"links": [{"requirementId": R, "testCaseId": T}]}'` — object-wrapped array.
- `systems link-test-plan --id <uuid> --json '[{"testPlanId": P}]'` — bare array on the command side.

Each shape is discoverable only by triggering a `400 "doesn't match schema"` error. In practice users need 3–5 iterations to get a simple mutation working. The CLI can collapse this by accepting common per-field flags and building the correct payload itself.

---

## Goals

- Let common single-item mutations be performed with named flags, no JSON authorship.
- Preserve existing `--json` / `--body-file` behaviour exactly for scripts that already work.
- Keep the normalization mechanical and predictable — no silent array/object reshaping of user-provided JSON.
- Eliminate the manual UUID-minting step when setting test-case steps.

## Non-goals

- Automatic payload reshaping for user-supplied `--json` (the option-B variant we rejected).
- Batch-mode flags (`--ids 1,2,3`). Flag mode is deliberately single-item; batch stays on `--json`.
- Any change to GET / list endpoints, output format, or pagination.
- Fixing broken server-side endpoints (`test-plans get/delete`). Captured in the backlog, out of scope here.
- New compound workflow commands (e.g. `create-full`). Backlog.

---

## Design

### Input modes

Each mutation command accepts exactly one of:

1. **Flag mode** — `--id` plus at least one per-field flag (`--name`, `--owner`, …) or, for link commands, the partner-id flag.
2. **Body mode** — `--json "<inline>"` or `--body-file <path>`.

Flag mode and body mode are mutually exclusive via a clap `ArgGroup`. Passing both errors with:

```
error: pass either --json/--body-file or per-field flags, not both
```

Passing neither keeps today's error:

```
error: request body required; pass --json or --body-file
```

Flag mode is always single-item. If a user wants to patch multiple items in one call, they use `--json` with an array — unchanged from today.

### Scope — commands getting flag shortcuts

Flag sets are aligned to each endpoint's `*PatchInput` / link input schema in the OpenAPI spec. Fields that exist on the schema but are awkward to express as flags (rich-text `statement_raw`, structured `customFields`, arrays like `reviewers`) stay JSON-only.

| Command | New flags | Target payload shape |
|---|---|---|
| `test-cases patch` | `--id`, `--name`, `--description`, `--owner` | `[{"testCaseId": N, …}]` |
| `test-cases set-steps` | `--steps-file <path>` (already has `--id`) | `[{"testCaseId": N, "steps": [{"action": …, "expected": …}]}]` |
| `test-plans patch` | `--id`, `--name`, `--description` | `[{"testPlanId": N, …}]` |
| `requirements patch` | `--id`, `--name`, `--owner` | `[{"requirementId": N, …}]` |
| `requirements link-test-case` | `--requirement-id`, `--test-case-id` | `{"links": [{"requirementId": R, "testCaseId": T}]}` |
| `test-plans link-test-case` | `--test-plan-id`, `--test-case-id` | `{"links": [{"testPlanId": P, "testCaseId": T}]}` |
| `systems link-test-plan` | `--test-plan-id` (already has `--id`) | `[{"testPlanId": P}]` |
| `systems link-test-case` | `--test-case-id` (already has `--id`) | `[{"testCaseId": T}]` |
| `systems link-requirement` | `--requirement-id` (already has `--id`) | `[{"id": R}]` (`AddRequirementToSystemInput`) |
| `systems link-document` | `--document-id` (already has `--id`) | `[{"documentId": D}]` |

`set-import-id` / `set-stage` / `set-value` on requirements, test-cases, and test-plans follow the same collection-patch shape; their flag shorthands are deferred to the backlog to keep scope tight.

### Payload-builder rules

For each command the handler builds the server payload from flags as follows.

**Patch commands** (`test-cases`, `test-plans`, `requirements`):
- Start with the id-key object: `{<idKey>: <id>}` where `<idKey>` is `testCaseId` / `testPlanId` / `requirementId`.
- For each present optional flag, add a property: `--name` → `"name"`, `--description` → `"description"`, `--owner` → `"owner"`.
- Omit flags the user did not set.
- Error if `--id` is the only flag (the payload would be a no-op patch). Message: `at least one field flag required (--name, --description, --owner as applicable)`.
- Final payload: `[<object>]`.

**`test-cases set-steps` with `--steps-file`**:
- File must be a JSON array.
- Each element must be an object with `action` (string) and `expected` (string). Those two fields are the entire server schema (`TestCaseStepInput`); the server generates `caseStepId` itself. Any extra keys on an element are passed through unchanged for forward compatibility.
- Final payload: `[{"testCaseId": <id>, "steps": <array>}]`.
- No client-side UUID generation. No `uuid` crate dependency.

**Link commands on `requirements` / `test-plans`** (`{"links": [...]}` wrapper):
- `flow requirements link-test-case --requirement-id R --test-case-id T` → `{"links": [{"requirementId": R, "testCaseId": T}]}`.
- `flow test-plans link-test-case --test-plan-id P --test-case-id T` → `{"links": [{"testPlanId": P, "testCaseId": T}]}`.
- Both id flags are required in flag mode.

**Link commands on `systems`** (bare array):
- The system id stays on `--id`, which these commands already require.
- Add `--<entity>-id` per variant (`--test-plan-id`, `--test-case-id`, `--requirement-id`, `--document-id`). Required in flag mode.
- Final payload: `[{<entityKey>: <value>}]`. The `<entityKey>` matches the server's `Add<Entity>ToSystemInput` schema — `testPlanId`, `testCaseId`, `documentId` — except `link-requirement` which uses the literal key `"id"` (per `AddRequirementToSystemInput`).

### Error handling

Wrong-mode combinations fail at argument parsing (clap) with the messages above. Post-parse validation (e.g. "patch needs at least one field flag") fires from the handler with an `anyhow::bail!`. No handler reaches the HTTP client unless the payload is complete.

---

## Backward compatibility

- `--json` and `--body-file` are unchanged. Existing scripts keep working bit-for-bit.
- All HTTP paths, methods, and query parameters are unchanged.
- No command is renamed, no flag is removed.
- Output format is unchanged.

---

## Testing strategy

- **Unit tests** per payload-builder function:
  - Flag set → exact JSON payload equality.
  - `set-steps` with a minimal two-field steps file → payload contains `{"testCaseId", "steps": [{"action", "expected"}]}` with no extra keys injected.
  - Error paths: id only (no field flags) → specific error; conflicting modes → specific error.
- **Integration tests** using the existing `helpers.rs` mock client:
  - One happy-path test per new flag surface verifying HTTP method, URL, and body.
  - A mixed test per command asserting `--json` mode still works identically to today.
  - A negative test per command for the mode-conflict error.
- No test regressions allowed — the full existing suite must pass unchanged.

---

## Dependencies

No new crate dependencies. The payload builders are pure `serde_json::Value` construction, which the codebase already uses throughout.

## Rollout

This is additive behaviour with no version-gated compatibility. Ship in a single PR (likely a handful of commits along module boundaries: cli args, handler wiring, tests, docs). Update the README with one new example per command family.

---

## Open questions

- Should `--owner` validate that the supplied email looks well-formed client-side? Current answer: **no** — the server's validation already errors clearly on unknown users, and client-side validation duplicates rules without adding value. Captured here so it doesn't become a review surprise.

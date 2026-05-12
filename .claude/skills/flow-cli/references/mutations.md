# Mutations

Read this before running any `create`, `patch`, `delete`, `link-*`, `unlink-*`, or `set-steps` command. Mutation payload shapes are **inconsistent** across endpoints — the table below reflects what works in live calls, not what the docs imply.

## Payload-shape cheat sheet

Most commands accept per-field flags that build the payload for you. Raw-JSON forms remain for batch and custom-field cases.

| Operation | Endpoint command | Flag mode (single item) | Raw payload (batch / custom fields) |
|---|---|---|---|
| Create test case | `test-cases create --name ... --description ...` | flags are native | — |
| Patch test case | `test-cases patch` | `--id N --name ... --description ... --owner ...` | `[{"testCaseId": N, "owner": "..."}]` |
| Set test-case steps | `test-cases set-steps --id N` | `--steps-file path.json` (file is `[{"action","expected"}]`) | `[{"testCaseId": N, "steps": [{"action","expected"}]}]` |
| Patch test plan | `test-plans patch` | `--id N --name ... --description ...` | `[{"testPlanId": N, "name": "..."}]` |
| Patch requirement | `requirements patch` | `--id N --name ... --owner ...` | `[{"requirementId": N, "owner": "..."}]` |
| Link TC → requirement | `requirements link-test-case` | `--requirement-id R --test-case-id T` | `{"links": [{"requirementId": R, "testCaseId": T}]}` |
| Link TC → test plan | `test-plans link-test-case` | `--test-plan-id P --test-case-id T` | `{"links": [{"testPlanId": P, "testCaseId": T}]}` |
| Link plan → system | `systems link-test-plan --id <uuid>` | `--test-plan-id P` | `[{"testPlanId": P}]` |
| Link TC → system | `systems link-test-case --id <uuid>` | `--test-case-id T` | `[{"testCaseId": T}]` |
| Link req → system | `systems link-requirement --id <uuid>` | `--requirement-id R` | `[{"id": R}]` (`AddRequirementToSystemInput`) |
| Link doc → system | `systems link-document --id <uuid>` | `--document-id D` | `[{"documentId": "..."}]` |

## Common rules

- **Mutations take arrays** even for a single item. `400 "value must be an array"` → wrap in `[...]`.
- **Link endpoints on `requirements` / `test-plans` take `{"links": [...]}`** — an object wrapping the array.
- **Link endpoints on `systems` take a bare array** with `--id` for the system side.
- **`set-steps` needs a `caseStepId` UUID per step** — generate with `str(uuid.uuid4())`.
- **Create responses are arrays**, not objects: `id = json.loads(out)[0]["id"]`.
- On `400 "doesn't match schema"`, read the error — it names the missing/wrong property. If still stuck, dump the OpenAPI spec (see `api-internals.md`).

## End-to-end: create, own, step, link a test case

This is a **4-call** sequence because owner/steps/links aren't part of `create`:

```python
import subprocess, json
FLOW = "/home/rio/.claude/skills/flow-cli/assets/flow"

def run(cmd):
    p = subprocess.run(cmd, capture_output=True, text=True)
    if p.returncode: raise RuntimeError(p.stderr or p.stdout)
    return p.stdout

# 1. Create — returns an array
out = run([FLOW, "test-cases", "create",
           "--name", "Internal Network Bandwidth",
           "--description", "Measure sustained throughput..."])
tc_id = json.loads(out)[0]["id"]

# 2. Set owner
run([FLOW, "test-cases", "patch",
     "--id", str(tc_id),
     "--owner", "rio@skl.vc"])

# 3. Set steps (write [{"action","expected"}, ...] to /tmp/steps.json first)
run([FLOW, "test-cases", "set-steps",
     "--id", str(tc_id),
     "--steps-file", "/tmp/steps.json"])

# 4. Link to a requirement
run([FLOW, "requirements", "link-test-case",
     "--requirement-id", "2855",
     "--test-case-id", str(tc_id)])
```

For batch creation, wrap in `try/except` with a rollback that calls `test-cases delete --id N` on each partial creation.

## Test plans: two hierarchies, one exposed

Flow's UI shows two independent trees:

1. **System tree** — plans grouped by linked system(s) via `systems link-test-plan`. **Exposed by the REST API.**
2. **All Test Plans tree** — plans can parent other plans. **Not exposed.** The `TestPlan` OpenAPI schema has no `parentId`; create/patch don't accept one.

If a user asks for plans "under L2. Component Level > Internal Network", confirm which tree they mean:

- **System-tree grouping** is achievable: `systems link-test-plan --id <internal-network-uuid>` puts the plan under the system view.
- **Plan-tree parenting** must be set in the UI — REST can't create or read it.

## Known broken endpoints

Server-side issues, not CLI bugs. Flag these to the user if you hit them:

- `test-plans get --id N` — always 404. Workaround: find it in `test-plans list` output.
- `test-plans delete --id N` — always 404 on current API version. Workaround: rename/redescribe via `patch` and unlink its test cases / systems.

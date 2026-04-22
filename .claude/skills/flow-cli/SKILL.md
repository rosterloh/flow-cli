---
name: flow-cli
description: Use this skill whenever the user wants to interact with Flow Engineering — searching requirements, listing systems, managing test cases, checking OTA update requirements, linking items, managing documents or interfaces, or doing anything that involves the Flow Engineering REST API. Triggers on requests like "find requirements for X", "list systems", "show me test cases", "what does the Flow project say about Y", "create a requirement", "link test case to requirement", "search Flow for Z", or any mention of Flow requirements, systems, test plans, or test cases. Also use when the user asks to run a flow CLI command or query the Flow API.
---

# Flow CLI Skill

The `flow` binary wraps the Flow Engineering REST API. Use it to query and manage requirements, systems, test cases, test plans, and more.

## Finding the binary

```bash
FLOW=$(command -v flow 2>/dev/null || echo "/home/rio/.claude/skills/flow-cli/assets/flow")
$FLOW --version
$FLOW auth status
```

If auth status shows `"auth": "none"`, tell the user to run:
```bash
$FLOW auth set-bearer --access-token "$FLOW_ACCESS_TOKEN" --save
# or
$FLOW auth exchange --refresh-token "$FLOW_REFRESH_TOKEN" --save
```

## Context defaults

Most commands need an org and project. Read the current config once at the start:
```bash
$FLOW config show
```
If org/project are already set, reuse them — don't ask the user to repeat them.

## Discovering the API

When you hit an unfamiliar endpoint or a 400 "doesn't match schema" error, dump the OpenAPI spec. It's the source of truth and saves a lot of guessing:

```bash
$FLOW raw get /openapi.json > /tmp/flow-openapi.json
python3 - <<'EOF'
import json
d = json.load(open('/tmp/flow-openapi.json'))
# List all paths
for p, methods in sorted(d['paths'].items()):
    for m, op in methods.items():
        print(f"{m.upper():6s} {p}  |  {op.get('summary','')}")
# Inspect a specific schema
print(json.dumps(d['components']['schemas']['TestPlanPatchInput'], indent=2))
EOF
```

`raw` method names are lowercase: `get`, `post`, `patch`, `put`, `delete`.

## Searching requirements

`search` fetches all requirements and filters by name (case-insensitive). It returns a compact `[{id, name, owner}]` array:

```bash
$FLOW requirements search "OTA"
$FLOW --output table requirements search "battery"
$FLOW requirements search "charging" | python3 -c "import json,sys; [print(r['id'], r['name']) for r in json.load(sys.stdin)]"
```

For multi-word, regex, or custom-field filters, fetch all requirements once and filter in Python:
```bash
$FLOW requirements filter --json '{}' > /tmp/flow-reqs.json
python3 - <<'EOF'
import json, re
reqs = json.load(open('/tmp/flow-reqs.json'))
hits = [r for r in reqs if re.search(r'\b(OTA|over.the.air)\b', r.get('name',''), re.IGNORECASE)]
for r in hits:
    print(r['id'], r['name'])
EOF
```

The full fetch is 3–4 MB (~2000 requirements). Save to `/tmp/flow-reqs.json` and reuse across queries in the same session.

## Getting full requirement detail

```bash
$FLOW requirements get --id 1234 > /tmp/req.json
python3 - <<'EOF'
import json, re
r = json.load(open('/tmp/req.json'))
print(f"Name:  {r['name']}")
print(f"Owner: {r.get('owner') or 'unowned'}")
# Extract plain text from Flow rich-text JSON
texts = re.findall(r'"text":"((?:[^"\\]|\\.)*)"', r.get('statement_raw',''))
statement = ' '.join(t for t in texts if t.strip()).replace('\\n','\n').replace('\\"','"')
print(f"\nStatement:\n  {statement}")
print(f"\nSystems: {len(r.get('systemIds', []))}")
EOF
```

## Custom fields and the Level tag

A requirement's key metadata lives in `customFields`, not on the top-level object. The common custom fields in the humanoid project are `Level` (TAG), `Tags` (TAG: `Subsystem_*`), `Relevance (HW Design)` (TAG), `Rationale` (TEXT), `Comments` (TEXT), `Documents` (FILE).

Extract them like this:
```python
level = next((f['value'] for f in r.get('customFields', [])
              if f['name']=='Level'), None)
tags  = next((f['value'] for f in r.get('customFields', [])
              if f['name']=='Tags'), None)
```

The `Level` vocabulary is **flat**: `L0-Product`, `L1-System`, `L2-Constructional/Technical`. **There is no L2.1 / L2.2 etc. on the Level tag.** When a user says "L2.1 requirements for X", they mean requirements linked to a *system* named `L2.1 something` — i.e. system-tree hierarchy, not the Level field. Clarify if ambiguous.

## Systems and the system tree

```bash
# Top-level systems only
$FLOW --output table systems list --top-level

# Paginated JSON — cursor flag is --after (NOT --cursor)
$FLOW systems list --paged --limit 200
$FLOW systems list --paged --limit 200 --after <cursor>

# List requirements / test plans / test cases linked to a system (system id is a UUID)
$FLOW systems list-requirements --id <uuid>
$FLOW systems list-test-plans --id <uuid>
$FLOW systems list-test-cases --id <uuid>
```

**There is no `systems get` subcommand** — the CLI exposes only `list`, `create`, `update`, `delete`, `list-documents/requirements/test-cases/test-plans`, and the link/unlink variants. To inspect one system, grab it from the full list:

```bash
python3 - <<'EOF'
import subprocess, json
FLOW = "/home/rio/.claude/skills/flow-cli/assets/flow"
all_sys, after = [], None
while True:
    args = [FLOW, "systems", "list", "--paged", "--limit", "200"]
    if after: args += ["--after", after]
    d = json.loads(subprocess.check_output(args))
    all_sys.extend(d["results"])
    if not d.get("hasMore"): break
    after = d["cursor"]
json.dump(all_sys, open('/tmp/flow-sys.json','w'))
print(f"Total systems: {len(all_sys)}")
EOF
```

Each system record includes `parentId` and `parentIds` (a single system can have multiple parents). To walk the tree:

```python
import json
systems = json.load(open('/tmp/flow-sys.json'))
by_id = {s['id']: s for s in systems}

def descendants(root_id):
    out, frontier = [], [root_id]
    while frontier:
        cur = frontier.pop()
        kids = [s for s in systems
                if cur in (s.get('parentIds') or
                           ([s['parentId']] if s.get('parentId') else []))]
        for k in kids:
            if k['id'] not in [d['id'] for d in out]:
                out.append(k); frontier.append(k['id'])
    return out
```

The top-level system taxonomy in the humanoid-beta project:
- `L0 Product Level` → `L1 System Level` → `L2 Constructional Level` → `L2.1 Robot` → subsystems (Head, Internal Network, Arms, …)
- `x.ARCHIVE`, `Work In Progress`, `Input Product Requirements` — housekeeping roots

## Coverage analysis

To find requirements missing test cases / test plans:

```bash
# All test cases linked to a requirement
$FLOW requirements list-test-cases --id 1234

# All requirements linked to a test case
$FLOW test-cases list-requirements --id 321

# All test plans linked to a requirement
$FLOW requirements list-test-plans --id 1234
```

For bulk coverage: iterate over the requirement IDs and flag the empty responses.

## Mutation payload shapes (cheat sheet)

Payload shapes are **inconsistent** across endpoints. This table reflects what actually works — verified in live calls, not inferred from docs:

| Operation | Endpoint command | Payload shape |
|---|---|---|
| Create test case | `test-cases create --name ... --description ...` | (flags, no JSON) |
| Patch test case | `test-cases patch --json ...` | `[{"testCaseId": N, "owner": "..."}]` |
| Set test-case steps | `test-cases set-steps --id N --json ...` | `[{"testCaseId": N, "steps": [{"caseStepId": "<uuid>", "action": "...", "expected": "..."}]}]` |
| Link TC → requirement | `requirements link-test-case --json ...` | `{"links": [{"requirementId": R, "testCaseId": T}]}` |
| Create test plan | `test-plans create --json ...` | `[{"name": "...", "description": "..."}]` |
| Patch test plan | `test-plans patch --json ...` | `[{"testPlanId": N, "name": "...", "description": "..."}]` |
| Link TC → test plan | `test-plans link-test-case --json ...` | `{"links": [{"testPlanId": P, "testCaseId": T}]}` |
| Link plan → system | `systems link-test-plan --id <uuid> --json ...` | `[{"testPlanId": P}]` |
| Link TC → system | `systems link-test-case --id <uuid> --json ...` | `[{"testCaseId": T}]` |

Common rules:
- **Mutations take arrays**, even for a single item. A `400 "value must be an array"` error means wrap your payload in `[...]`.
- **Link endpoints on `requirements` / `test-plans` take `{"links": [...]}`** — an object wrapping the array.
- **Link endpoints on `systems` take a bare array** with `--id` for the system side.
- **`set-steps` needs a `caseStepId` UUID per step** — generate with `str(uuid.uuid4())`.
- **Create responses are arrays**, not objects: `id = json.loads(out)[0]["id"]`.
- On a 400 "doesn't match schema", read the error — it names the missing/wrong property.

## End-to-end: create a test case, own it, step it, link it

Creating a properly-owned test case that's linked to requirements is a **4-call** sequence because owner/steps/links aren't part of create. Verified recipe:

```python
import subprocess, json, uuid
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

# 2. Set owner (patch takes array wrapping testCaseId)
run([FLOW, "test-cases", "patch",
     "--json", json.dumps([{"testCaseId": tc_id, "owner": "rio@skl.vc"}])])

# 3. Set steps (each needs a caseStepId UUID)
steps = [{"caseStepId": str(uuid.uuid4()),
          "action": "Set up iperf3", "expected": "Link up"},
         {"caseStepId": str(uuid.uuid4()),
          "action": "Run saturating test", "expected": ">=10 Gbps"}]
run([FLOW, "test-cases", "set-steps", "--id", str(tc_id),
     "--json", json.dumps([{"testCaseId": tc_id, "steps": steps}])])

# 4. Link to requirements (object-wrapped)
run([FLOW, "requirements", "link-test-case",
     "--json", json.dumps({"links": [{"requirementId": 2855, "testCaseId": tc_id}]})])
```

For batch creation, wrap in `try/except` with a rollback that calls `test-cases delete --id N` on each partial creation.

## Test plans: two hierarchies, one exposed

Flow's UI shows two independent trees:
1. **System tree** — plans grouped by the system(s) they're linked to (via `systems link-test-plan`). Exposed by the REST API.
2. **All Test Plans tree** — plans can be parents of other plans (plan-to-plan hierarchy). **Not exposed by the REST API.** The `TestPlan` OpenAPI schema has no `parentId` field, and create/patch don't accept one.

If a user asks for plans "under L2. Component Level > Internal Network", confirm which tree they mean:
- **System-tree grouping** is achievable: `systems link-test-plan --id <internal-network-uuid>` puts the plan under the `L2 Constructional Level > L2.1 Robot > Internal Network` branch in the UI's system view.
- **Plan-tree parenting** (one plan as the parent of others) must be set in the UI — the REST API can't create or read the relationship.

## Known broken endpoints

These return 404 or otherwise misbehave server-side:

- `test-plans get --id N` — always 404. Workaround: find the plan in `test-plans list` output.
- `test-plans delete --id N` — always 404 on current API version. Workaround: repurpose via `patch` (rename/re-describe), and unlink its test cases / systems if needed.

Flag these to the user if you hit them — the server state, not the CLI, is the problem.

## Raw API access

```bash
$FLOW raw get /org/my-org/project/my-project/someEndpoint
$FLOW raw post /org/my-org/project/my-project/requirements --json '{"name":"New req"}'
$FLOW raw get /org/my-org/project/my-project/testPlans --query "limit=50"
```

Method must be **lowercase** (`get`, `post`, `patch`, `put`, `delete`). `--query K=V` appends to the URL. Use `--json` or `--body-file` for request bodies.

## Tips

- **IDs**: requirements / test cases / test plans use integer IDs; systems use UUID strings.
- **`--output table`** is good for humans; JSON is better for scripting.
- **Mutations are permanent** — confirm with the user before running `create`, `patch`, `delete`, or any linking command. For batch operations, always have a rollback path.
- **Before running a large batch**, do one end-to-end smoke run on a single item first (especially for shapes you haven't verified in this session) — the error messages from the schema validator are the cheapest way to discover the real payload shape.
- The `raw` command covers any endpoint not yet wrapped by a subcommand — check `$FLOW <resource> --help` first.

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

## Searching requirements

The native `search` command fetches all requirements and filters by name (case-insensitive). It returns a compact `[{id, name, owner}]` array — pipe to table output for readability:

```bash
$FLOW requirements search "OTA"
$FLOW --output table requirements search "battery"
$FLOW requirements search "charging" | python3 -c "import json,sys; [print(r['id'], r['name']) for r in json.load(sys.stdin)]"
```

For multi-word or regex patterns, use the filter endpoint and pipe to Python:
```bash
$FLOW requirements filter --json '{}' > /tmp/flow-reqs.json
python3 << 'EOF'
import json, re
reqs = json.load(open('/tmp/flow-reqs.json'))
hits = [r for r in reqs if re.search(r'\b(OTA|over.the.air)\b', r.get('name',''), re.IGNORECASE)]
for r in hits:
    print(r['id'], r['name'])
EOF
```

Save to `/tmp/flow-reqs.json` when running multiple searches — the full fetch is 3–4 MB and reusing it saves time.

## Systems

```bash
# Top-level systems only (clean hierarchy view)
$FLOW --output table systems list --top-level

# All systems (paginated JSON)
$FLOW systems list --paged --limit 50

# Single system by UUID
$FLOW systems get --id <uuid>
```

Note: `systems list` returns `{"results": [...], "hasMore": ..., "cursor": ...}` — not a flat array. Use `--top-level` for a filtered flat list, or extract `results` manually when scripting.

## Getting full requirement detail

```bash
$FLOW requirements get --id 1234 > /tmp/req.json
python3 << 'EOF'
import json, re
r = json.load(open('/tmp/req.json'))
print(f"Name:  {r['name']}")
print(f"Owner: {r.get('owner') or 'unowned'}")
# Extract plain text from Flow rich-text JSON
texts = re.findall(r'"text":"(.*?)"(?=[,}])', r.get('statement_raw',''))
statement = ' '.join(t.replace('\\n', ' ').strip() for t in texts if t.strip())
print(f"\nStatement:\n  {statement}")
print(f"\nSystems: {len(r.get('systemIds', []))}")
EOF
```

## Other common commands

### List by scope
```bash
$FLOW requirements list --scope org            # organisation-level requirements
$FLOW requirements list --scope without-system # requirements not linked to any system
```

### Test cases and test plans
```bash
$FLOW --output table test-cases list --paged --limit 20
$FLOW test-cases get --id 4321
$FLOW test-plans list
$FLOW test-plans get --id 12
```

### Create and link
```bash
$FLOW requirements create --name "New requirement"
$FLOW requirements link-test-case --json '{"requirementId": 1234, "testCaseId": 4321}'
```

### Raw API access
```bash
$FLOW raw GET /org/my-org/project/my-project/someEndpoint
$FLOW raw POST /org/my-org/project/my-project/requirements --json '{"name":"New req"}'
```

## Handling large responses

The full requirements set is 3–4 MB (~2000 items). Always:
1. Use `requirements search <term>` for name-based lookups
2. Save the full dataset to `/tmp/flow-reqs.json` if running multiple queries
3. Use `--paged --limit N` when exploring other resources

## Tips

- **IDs**: requirements use integer IDs; systems use UUID strings
- **`--output table`** is good for quick overviews; JSON is better for scripting
- **Mutations are permanent** — confirm with the user before running `create`, `patch`, `delete`, or any linking command
- The `raw` command covers any endpoint not yet wrapped — check `$FLOW --help` for the full list

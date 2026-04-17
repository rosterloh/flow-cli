---
name: flow-cli
description: Use this skill whenever the user wants to interact with Flow Engineering — searching requirements, listing systems, managing test cases, checking OTA update requirements, linking items, managing documents or interfaces, or doing anything that involves the Flow Engineering REST API. Triggers on requests like "find requirements for X", "list systems", "show me test cases", "what does the Flow project say about Y", "create a requirement", "link test case to requirement", "search Flow for Z", or any mention of Flow requirements, systems, test plans, or test cases. Also use when the user asks to run a flow CLI command or query the Flow API.
---

# Flow CLI Skill

The `flow` binary wraps the Flow Engineering REST API. Use it to query and manage requirements, systems, test cases, test plans, and more.

## Finding the binary

Always resolve the binary before running commands. Prefer the system-installed version; fall back to the one bundled with this skill:

```bash
FLOW=$(command -v flow 2>/dev/null || echo "$(dirname "$0")/../assets/flow")
# Or as a one-liner to use in any command:
FLOW=$(command -v flow 2>/dev/null || echo "/home/rio/.claude/skills/flow-cli/assets/flow")
```

Confirm the binary works:
```bash
$FLOW --version
$FLOW auth status
```

If auth status shows `"auth": "none"`, tell the user to run `$FLOW auth set-bearer --access-token "$FLOW_ACCESS_TOKEN" --save` or `$FLOW auth exchange --refresh-token "$FLOW_REFRESH_TOKEN" --save`.

## Context defaults

Most commands need an org and project. They read from config (`$FLOW config show`) or accept `--org` / `--project` flags. If the user's context is already configured (visible in `auth status`), reuse it — don't ask them to repeat it.

If the org/project is not set, check the environment:
```bash
$FLOW config show
```

## Key commands

### Search / filter requirements
```bash
# Returns all requirements (can be large — pipe to filter)
$FLOW requirements filter --json '{}'

# Paginated list
$FLOW requirements list --paged --limit 50

# Then filter with Python for keyword matches:
$FLOW requirements filter --json '{}' | python3 -c "
import json, sys, re
data = json.load(sys.stdin)
term = 'YOUR TERM'
hits = [r for r in data if re.search(term, r.get('name',''), re.IGNORECASE)]
for r in sorted(hits, key=lambda x: x['id']):
    print(f\"[{r['id']:>4}] {r['name']}  (owner: {r.get('owner') or 'unowned'})\")
print(f'{len(hits)} of {len(data)} requirements matched')
"
```

The filter endpoint returns all requirements by default — always pipe large responses through Python or `jq` rather than printing them raw.

### Get a single requirement
```bash
$FLOW requirements get --id 1234
```

### List requirements by scope
```bash
$FLOW requirements list --scope org            # organisation-level
$FLOW requirements list --scope without-system # unassigned
```

### Systems
```bash
$FLOW systems list --output table      # quick overview
$FLOW systems list --paged --limit 50  # paginated JSON
$FLOW systems get --id <uuid>          # single system (id is a UUID string)
```

### Test cases and test plans
```bash
$FLOW test-cases list --paged --limit 50
$FLOW test-cases get --id 4321
$FLOW test-plans list
$FLOW test-plans get --id 12
```

### Table output for readability
```bash
$FLOW --output table requirements list --paged --limit 20
$FLOW --output table systems list
```

### Raw API access (for endpoints not yet in the CLI)
```bash
$FLOW raw GET /org/my-org/project/my-project/someEndpoint
$FLOW raw POST /org/my-org/project/my-project/requirements --json '{"name":"New req"}'
```

## Handling large responses

The Flow API returns full requirement objects (custom fields, links, reviewers, etc.). A project with ~2000 requirements produces 3–4 MB of JSON. Always:
1. Pipe through Python for filtering
2. Use `--paged --limit N` when exploring
3. Only fetch individual items (`get --id`) when you need full detail

## Common workflows

### Find requirements about a topic
1. Run `$FLOW requirements filter --json '{}'` and pipe to Python with `re.search(r'\bTERM\b', name, re.IGNORECASE)` for whole-word matching.
2. Show the user a clean table: id, name, owner.
3. Offer to `get --id` for full detail on any specific result.

### Link a test case to a requirement
```bash
$FLOW requirements link-test-case --json '{"requirementId": 1234, "testCaseId": 4321}'
```

### Create a requirement
```bash
$FLOW requirements create --name "My new requirement"
# Or with multiple names:
$FLOW requirements create --name "REQ-A" --name "REQ-B"
```

### Check OTA / topic requirements and get full statements
```bash
# 1. Find matching requirements
$FLOW requirements filter --json '{}' | python3 -c "
import json, sys, re
for r in json.load(sys.stdin):
    if re.search(r'\bOTA\b', r.get('name',''), re.IGNORECASE):
        print(r['id'], r['name'])
"

# 2. Get full statement for a specific one
$FLOW requirements get --id 1711 | python3 -c "
import json, sys
r = json.load(sys.stdin)
print('Name:', r['name'])
print('Owner:', r.get('owner'))
# statement_raw is Flow rich text JSON — extract plain text:
import re
raw = r.get('statement_raw','')
text = re.sub(r'{\"text\":\"(.*?)\"', lambda m: m.group(1), raw)
print('Statement:', text[:500])
"
```

## Tips

- **IDs**: requirements use integer IDs; systems use UUID strings.
- **Stages and custom fields**: returned as UUIDs/raw JSON — use `get --id` to fetch the full object and inspect `stage`, `customFields`.
- **Mutations are permanent**: confirm with the user before running `create`, `patch`, `delete`, or any linking command.
- **`--output table`** is good for quick overviews; JSON is better for scripting.
- The `raw` command covers any endpoint not yet wrapped — check `$FLOW --help` for the full list.

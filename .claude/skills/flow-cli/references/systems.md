# Systems and the system tree

System IDs are UUIDs. The CLI exposes only `list`, `create`, `update`, `delete`, and the `list-*` / `link-*` / `unlink-*` variants — **there is no `systems get`**. To inspect one system, grab it out of the full list.

## Listing

```bash
# Top-level systems only
$FLOW --output table systems list --top-level

# Paginated JSON — cursor flag is --after (NOT --cursor)
$FLOW systems list --paged --limit 200
$FLOW systems list --paged --limit 200 --after <cursor>

# Children of a system
$FLOW systems list-requirements --id <uuid>
$FLOW systems list-test-plans --id <uuid>
$FLOW systems list-test-cases --id <uuid>
```

## Full-tree snapshot

Cache the full system list once per session, then traverse locally:

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

## Walking the tree

Each system has `parentId` and `parentIds` (a system can have multiple parents):

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

## Taxonomy (humanoid-beta)

- `L0 Product Level` → `L1 System Level` → `L2 Constructional Level` → `L2.1 Robot` → subsystems (Head, Internal Network, Arms, …)
- `x.ARCHIVE`, `Work In Progress`, `Input Product Requirements` — housekeeping roots

When a user says "L2.1 requirements", they mean requirements linked to a system named `L2.1 …`, not the `Level` custom-field tag (see `requirements.md`).

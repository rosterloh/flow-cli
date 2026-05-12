# Requirements

## Searching by name

`requirements search` fetches all requirements and filters by name (case-insensitive). It returns a compact `[{id, name, owner}]` array:

```bash
$FLOW requirements search "OTA"
$FLOW --output table requirements search "battery"
$FLOW requirements search "charging" | python3 -c "import json,sys; [print(r['id'], r['name']) for r in json.load(sys.stdin)]"
```

For multi-word, regex, or custom-field filters, fetch everything once and filter in Python:

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

The full fetch is 3–4 MB (~2000 requirements). Cache to `/tmp/flow-reqs.json` and reuse across queries in the same session.

## Getting full detail

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

## Custom fields and the `Level` tag

Key metadata lives in `customFields`, not on the top-level object. Common custom fields in the humanoid project:

- `Level` (TAG) — flat vocabulary: `L0-Product`, `L1-System`, `L2-Constructional/Technical`
- `Tags` (TAG: `Subsystem_*`)
- `Relevance (HW Design)` (TAG)
- `Rationale` (TEXT)
- `Comments` (TEXT)
- `Documents` (FILE)

Extract them like this:

```python
level = next((f['value'] for f in r.get('customFields', [])
              if f['name']=='Level'), None)
tags  = next((f['value'] for f in r.get('customFields', [])
              if f['name']=='Tags'), None)
```

**There is no `L2.1` / `L2.2` etc. on the `Level` tag.** When a user says "L2.1 requirements for X", they mean requirements linked to a *system* named `L2.1 something` — i.e. system-tree hierarchy, not the `Level` field. Clarify if ambiguous, and see `systems.md` for tree traversal.

## Coverage queries

```bash
# Test cases / plans linked to a requirement
$FLOW requirements list-test-cases --id 1234
$FLOW requirements list-test-plans --id 1234

# Requirements linked back to a test case
$FLOW test-cases list-requirements --id 321
```

For bulk coverage analysis, iterate over requirement IDs from the cached `/tmp/flow-reqs.json` and flag the empty responses.

# API internals

## Dumping the OpenAPI spec

When you hit an unfamiliar endpoint or a `400 "doesn't match schema"` error, the OpenAPI spec is the source of truth and saves a lot of guessing:

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

## Raw API access

`raw` covers any endpoint not yet wrapped by a subcommand. Methods must be **lowercase** (`get`, `post`, `patch`, `put`, `delete`):

```bash
$FLOW raw get  /org/my-org/project/my-project/someEndpoint
$FLOW raw post /org/my-org/project/my-project/requirements --json '{"name":"New req"}'
$FLOW raw get  /org/my-org/project/my-project/testPlans --query "limit=50"
```

- `--query K=V` appends to the URL.
- `--json '<body>'` or `--body-file path.json` for request bodies.

Always check `$FLOW <resource> --help` before reaching for `raw` — the wrapped commands handle the org/project URL prefix and the array-wrapping rules.

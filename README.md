# flow

`flow` is a Rust command-line client for the Flow Engineering REST API.

It provides first-class commands for every resource in the API:

- authentication and local config
- organisations, projects, members, and configurations
- requirements, systems, documents, and interfaces
- test cases, test plans, test cycles, and test runs
- design values
- HTML-to-FlowText conversion
- arbitrary API calls through a raw request command

Output defaults to formatted JSON. Pass `--output table` for a human-readable table view.

The CLI targets the Flow REST API documented at `https://api.flowengineering.com/rest/v1/docs`.

## Requirements

- Rust toolchain with Cargo
- A Flow API credential (bearer token or basic auth)

## Build

```bash
cargo build
```

Run directly during development:

```bash
cargo run -- --help
```

Or install the `flow` binary to your Cargo bin directory:

```bash
cargo install --path .
```

## Configuration

The CLI stores local configuration at:

```text
~/.config/flow-cli/config.json
```

Config fields:

| Field | Env override |
|---|---|
| `base_url` | `FLOW_BASE_URL` |
| `org_alias` | `FLOW_ORG` |
| `project_alias` | `FLOW_PROJECT` |
| `access_token` | `FLOW_ACCESS_TOKEN` |
| `refresh_token` | `FLOW_REFRESH_TOKEN` |
| `username` | `FLOW_USERNAME` |
| `password` | `FLOW_PASSWORD` |

Show active config:

```bash
flow config show
```

Set default org and project:

```bash
flow config set-context --org my-org --project my-project
```

## Authentication

Exchange a refresh token for an access token and save it:

```bash
flow auth exchange --refresh-token "$FLOW_REFRESH_TOKEN" --save
```

Also save the refresh token:

```bash
flow auth exchange --refresh-token "$FLOW_REFRESH_TOKEN" --save --save-refresh-token
```

Save a bearer token directly:

```bash
flow auth set-bearer --access-token "$FLOW_ACCESS_TOKEN" --save
```

Save basic auth credentials:

```bash
flow auth set-basic --username "$FLOW_USERNAME" --password "$FLOW_PASSWORD" --save
```

Inspect or clear auth:

```bash
flow auth status
flow auth clear
flow auth clear --all   # also clears the saved refresh token
```

## Output format

All commands output formatted JSON by default. Pass `--output table` before the subcommand for a human-readable table:

```bash
flow --output table requirements list --paged --limit 20
flow --output table systems list
```

## Commands

| Command | Description |
|---|---|
| `auth` | Authentication management |
| `config` | Local config management |
| `orgs` | List organisations |
| `projects` | List and create projects |
| `members` | Org and project member management |
| `configurations` | Project configurations |
| `requirements` | Requirements CRUD and linking |
| `systems` | Systems CRUD and linking |
| `documents` | Documents CRUD |
| `interfaces` | Interfaces CRUD |
| `test-cases` | Test case CRUD and linking |
| `test-plans` | Test plan CRUD and linking |
| `test-cycles` | Test cycle management |
| `test-runs` | Test run management |
| `values` | Design value management |
| `util` | Utility commands (HTML conversion) |
| `raw` | Arbitrary API requests |

Built-in help for any command:

```bash
flow <command> --help
flow <command> <subcommand> --help
```

### Flag-mode shortcuts

For single-item mutations, common fields are available as flags — no need to hand-craft the server's JSON payload.

```bash
# Patch
flow test-cases patch --id 326 --owner rio@skl.vc
flow test-plans patch --id 202 --name "Internal Network Performance Test"
flow requirements patch --id 2855 --owner rio@skl.vc

# Set steps from a plain step array (caseStepIds generated server-side)
flow test-cases set-steps --id 326 --steps-file steps.json
#   where steps.json = [{"action": "...", "expected": "..."}, ...]

# Cross-resource links
flow requirements link-test-case --requirement-id 2855 --test-case-id 326
flow test-plans    link-test-case --test-plan-id 203    --test-case-id 329

# System links (the system uses --id; the partner uses --<entity>-id)
flow systems link-test-plan    --id <sys-uuid> --test-plan-id 203
flow systems link-test-case    --id <sys-uuid> --test-case-id 326
flow systems link-requirement  --id <sys-uuid> --requirement-id 2855
flow systems link-document     --id <sys-uuid> --document-id  <doc-uuid>
```

`--json` / `--body-file` continue to work unchanged for batch or custom-field payloads.

## Requirements

```bash
flow requirements list
flow requirements list --paged --limit 50
flow requirements list --scope org          # organisation-level requirements
flow requirements list --scope without-system

flow requirements get --id 1234
flow requirements create --name "REQ-1" --name "REQ-2"
flow requirements patch --json '[{"id":1234,"name":"Updated"}]'
flow requirements delete --id 1234

flow requirements filter --json '{"status":"open"}'
flow requirements set-stage --json '[{"id":1234,"stage":"verified"}]'
flow requirements set-value --json '[{"id":1234,"valueId":5,"value":42}]'
flow requirements set-import-id --json '[{"id":1234,"importId":"EXT-001"}]'

flow requirements list-test-cases --id 1234
flow requirements list-test-plans --id 1234

flow requirements link-jira --id 1234 --json '{"jiraIssueId":"PROJ-42"}'
flow requirements unlink-jira --id 1234 --jira-issue-id "PROJ-42"

flow requirements get-custom-fields
flow requirements patch-custom-fields --json '[...]'
```

## Systems

```bash
flow systems list
flow systems create --name "Vehicle" --prefix VEH
flow systems update --id <uuid> --name "Vehicle Core"
flow systems delete --id <uuid>
flow systems bulk-update --json '[...]'

flow systems list-requirements --id <uuid>
flow systems link-requirement --id <uuid> --json '{"requirementId":1234}'
flow systems unlink-requirement --id <uuid> --requirement-id 1234

flow systems list-test-cases --id <uuid>
flow systems link-test-case --id <uuid> --json '{"testCaseId":4321}'
flow systems unlink-test-case --id <uuid> --test-case-id 4321
```

## Documents and Interfaces

```bash
flow documents list
flow documents get --id 7
flow documents create --json '{"name":"ICD"}'
flow documents delete --id 7

flow interfaces list
flow interfaces create --json '{"name":"CAN Bus"}'
flow interfaces delete --id 5
```

## Members

```bash
flow members list-org --org my-org
flow members add-org --org my-org --json '{"email":"user@example.com","role":"member"}'
flow members remove-org --org my-org --email user@example.com

flow members list-project
flow members add-project --json '{"email":"user@example.com","role":"viewer"}'
flow members remove-project --email user@example.com
```

## Test Cases and Test Plans

```bash
flow test-cases list --paged --limit 50
flow test-cases get --id 4321
flow test-cases create --name "Brake pedal test"
flow test-cases patch --json '[{"id":4321,"name":"Updated"}]'
flow test-cases delete --id 4321

flow test-cases set-steps --id 4321 --json '[{"action":"Press brake","expected":"Car stops"}]'
flow test-cases create-test-run --id 4321 --json '{}'
flow test-cases list-requirements --id 4321

flow test-plans list
flow test-plans create --json '{"name":"Sprint 1 plan"}'
flow test-plans get --id 12
flow test-plans delete --id 12
flow test-plans create-cycle --id 12 --json '{"name":"Cycle 1"}'
flow test-plans link-test-case --json '{"testPlanId":12,"testCaseId":4321}'
```

## Test Cycles and Test Runs

```bash
flow test-cycles get --id 99
flow test-cycles delete --id 99

flow test-runs get --cycle-id 99 --id 1
flow test-runs patch --cycle-id 99 --id 1 --json '{"status":"passed"}'
flow test-runs delete --cycle-id 99 --id 1
flow test-runs set-steps --cycle-id 99 --id 1 --json '[{"result":"pass"}]'
```

## Values

```bash
flow values list
flow values list --numeric
flow values get --id 77
flow values set-number --id 77 --value 42.5
flow values set-import-id --json '[{"id":77,"importId":"VAL-001"}]'
```

## Utilities

```bash
flow util convert-html --html '<h1>Hello</h1>' --html '<p>World</p>'
```

## Raw Requests

Call any endpoint directly:

```bash
flow raw GET /orgs
flow raw GET /org/my-org/project/my-project/requirements/paged --query limit=25 --query after=cursor123
flow raw POST /org/my-org/projects --json '{"name":"New Project"}'
flow raw POST /org/my-org/project/my-project/requirements --body-file payload.json
```

## Notes

- Most resource commands require an org and project. Pass `--org`/`--project` flags or set defaults with `flow config set-context`.
- JSON responses are pretty-printed by default; use `--output table` for a columnar view.
- All commands respect `FLOW_ORG` and `FLOW_PROJECT` environment variables.

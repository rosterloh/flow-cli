# flow-cli

`flow-cli` is a Rust command-line client for the Flow Engineering REST API.

It wraps a practical subset of the API with first-class commands for:

- authentication and local config
- organisations and projects
- requirements
- systems
- test cases and test plans
- design values
- HTML-to-FlowText conversion
- arbitrary API calls through a raw request command

The CLI targets the Flow REST API documented at `https://api.flowengineering.com/rest/v1/docs`.

## Requirements

- Rust toolchain with Cargo
- A Flow API credential

The CLI supports two auth modes documented by the API:

- bearer token auth
- basic auth

It also supports exchanging a Flow refresh token for a bearer access token using `/auth/exchange`.

## Build

```bash
cargo build
```

Run directly during development:

```bash
cargo run -- --help
```

## Configuration

The CLI stores local configuration at:

```text
~/.config/flow-cli/config.json
```

Config can include:

- `base_url`
- `org_alias`
- `project_alias`
- `access_token`
- `refresh_token`
- `username`
- `password`

Environment variables override saved values where applicable:

- `FLOW_BASE_URL`
- `FLOW_ORG`
- `FLOW_PROJECT`
- `FLOW_REFRESH_TOKEN`
- `FLOW_ACCESS_TOKEN`
- `FLOW_USERNAME`
- `FLOW_PASSWORD`

Show the active config:

```bash
cargo run -- config show
```

Print the config path:

```bash
cargo run -- config path
```

Set default org and project context:

```bash
cargo run -- config set-context --org my-org --project my-project
```

## Authentication

### Exchange a refresh token

If you have a Flow refresh token, exchange it for an access token and save it:

```bash
cargo run -- auth exchange --refresh-token "$FLOW_REFRESH_TOKEN" --save
```

To save the refresh token in config as well:

```bash
cargo run -- auth exchange --refresh-token "$FLOW_REFRESH_TOKEN" --save --save-refresh-token
```

### Save a bearer token directly

```bash
cargo run -- auth set-bearer --access-token "$FLOW_ACCESS_TOKEN" --save
```

### Save basic auth credentials

```bash
cargo run -- auth set-basic --username "$FLOW_USERNAME" --password "$FLOW_PASSWORD" --save
```

### Inspect or clear auth

```bash
cargo run -- auth status
cargo run -- auth clear
cargo run -- auth clear --all
```

## Commands

Top-level commands:

- `auth`
- `config`
- `orgs`
- `projects`
- `requirements`
- `systems`
- `test-cases`
- `test-plans`
- `values`
- `util`
- `raw`

Use built-in help for any command:

```bash
cargo run -- <command> --help
```

Examples:

```bash
cargo run -- orgs list
cargo run -- projects list --org my-org
cargo run -- projects create --org my-org --name "Platform"
```

## Requirements

List requirements:

```bash
cargo run -- requirements list
```

Use the paginated endpoint:

```bash
cargo run -- requirements list --paged --limit 50
```

Fetch a single requirement:

```bash
cargo run -- requirements get --id 1234
```

Create requirements:

```bash
cargo run -- requirements create --name "REQ-1" --name "REQ-2" --description "Imported from CLI"
```

Patch requirements from inline JSON:

```bash
cargo run -- requirements patch --json '[{"id":1234,"name":"Updated name"}]'
```

Patch requirements from a file:

```bash
cargo run -- requirements patch --body-file patches/requirements.json
```

Delete a requirement:

```bash
cargo run -- requirements delete --id 1234
```

## Systems

List systems:

```bash
cargo run -- systems list
```

Create a system:

```bash
cargo run -- systems create --name "Vehicle" --prefix VEH
```

Update a system:

```bash
cargo run -- systems update --id 550e8400-e29b-41d4-a716-446655440000 --name "Vehicle Core"
```

Or provide a full JSON body:

```bash
cargo run -- systems update --id 550e8400-e29b-41d4-a716-446655440000 --json '{"name":"Vehicle Core"}'
```

Delete a system:

```bash
cargo run -- systems delete --id 550e8400-e29b-41d4-a716-446655440000
```

## Test Cases and Test Plans

List test cases:

```bash
cargo run -- test-cases list
```

Get a test case:

```bash
cargo run -- test-cases get --id 4321
```

Create test cases:

```bash
cargo run -- test-cases create --name "Brake pedal test" --description "CLI-created test"
```

Patch test cases:

```bash
cargo run -- test-cases patch --json '[{"id":4321,"name":"Updated test case"}]'
```

Delete a test case:

```bash
cargo run -- test-cases delete --id 4321
```

List test plans:

```bash
cargo run -- test-plans list
```

Patch test plans:

```bash
cargo run -- test-plans patch --json '[{"id":12,"name":"Updated plan"}]'
```

## Values

List all values:

```bash
cargo run -- values list
```

List numeric values only:

```bash
cargo run -- values list --numeric
```

Update a numeric value:

```bash
cargo run -- values set-number --id 77 --value 42.5
```

## Utilities

Convert HTML to FlowText:

```bash
cargo run -- util convert-html --html '<h1>Hello</h1>' --html '<p>World</p>'
```

## Raw Requests

Use `raw` to call any endpoint that is not wrapped yet.

Simple GET:

```bash
cargo run -- raw GET /orgs
```

GET with query parameters:

```bash
cargo run -- raw GET /org/my-org/project/my-project/requirements/paged --query limit=25 --query after=cursor123
```

POST with JSON body:

```bash
cargo run -- raw POST /org/my-org/projects --json '{"name":"New Project"}'
```

POST with JSON from file:

```bash
cargo run -- raw POST /org/my-org/project/my-project/requirements --body-file payload.json
```

## Notes

- Most resource commands require an org and project. Pass `--org` and `--project`, or set them once with `config set-context`.
- The CLI prints API responses as formatted JSON.
- Some Flow endpoints documented in the API are not wrapped as dedicated commands yet. Use `raw` for those.
- Live API behavior still depends on your credentials and Flow permissions.

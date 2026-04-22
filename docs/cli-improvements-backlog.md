# CLI improvement backlog

Notes captured from a session of heavy live use (2026-04-22). The two highest-impact items — **payload-shape normalization** and **single-item flag shorthands** — are being taken forward first; everything else is captured here for later.

## In flight

- **#1 Payload-shape normalization.** Hide server-side inconsistency (bare arrays vs `{links: [...]}` wrappers vs `--id` + nested id) behind a consistent CLI surface.
- **#5 Single-item flag shorthands.** `test-cases patch --id N --owner X --description Y` instead of hand-crafting `[{"testCaseId": N, ...}]` JSON.

## Not yet scheduled

### Tier 1 — API / CLI design

- **#2 Return a single object from single-item `create`.** `test-cases create --name X` currently returns `[{...}]` with one element. Every caller does `json.loads(out)[0]["id"]`. Return the object directly when a single `--name` is passed; keep array output when multiple.
- **#3 `test-plans get` / `test-plans delete` are wired to paths the server doesn't implement** (`/testPlan/{id}`). Both return 404. Options: remove the subcommands, or have `get` transparently fall back to `list` + client-side filter. Needs confirmation from server-side whether the endpoints should exist.
- **#4 `systems get --id <uuid>`** doesn't exist. Currently the only way to inspect a single system is to fetch the full paginated list. Add the subcommand even if it means client-side filtering.

### Tier 2 — UX conveniences

- **#6 `--all` / auto-pagination on `list --paged`.** Every user-land script repeats the same `while hasMore: after = cursor` loop. Implement once in the CLI and return a concatenated array.
- **#7 `--plain` on `get` for `*_raw` rich-text fields.** Flatten Plate/Slate JSON (`statement_raw`, `description_raw`) to plain text. Eliminates a regex every user writes.
- **#8 `--dry-run` on every mutation.** Print the planned HTTP request (method, URL, body) without executing. Cheap insurance for batch operations.
- **#9 `--verbose` / request echoing.** When a schema error fires, the cheapest tool is seeing what went on the wire. Print method + URL + body alongside the response.

### Tier 3 — completeness

- **#10 Workflow bundle: create-and-link.** Collapse the 4-call sequence (create → patch owner → set-steps → link) into one `test-cases create-full --owner X --steps-file p.json --link-requirements 1,2,3`. Covers the most common workflow.
- **#11 Expose the All-Test-Plans tree.** Plan-to-plan parent hierarchy exists in Flow's UI (likely via GraphQL) but is absent from REST. Either expose via a `test-plans set-parent` subcommand (if the server supports it) or document the limitation inline.
- **#12 Case-insensitive HTTP method in `raw`.** `raw GET /foo` should work as well as `raw get /foo`.

### Meta

- **#13 Embed the OpenAPI version in `flow --version`.** Lets users correlate "this shape worked yesterday, fails today" against a server API rev.
- **#14 Commit `openapi.json` to the repo.** Server exposes it; checking it in lets users diff schema changes across CLI versions without running the server. Could also be used by build-time checks to catch route drift between CLI and server.

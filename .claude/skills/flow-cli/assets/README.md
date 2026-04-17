# flow-cli skill — binary installation

Place the `flow` binary for your platform in this directory before using the skill.

## Download

Pre-built binaries are available on the [GitHub Releases page](https://github.com/rosterloh/flow-cli/releases/latest):

| Platform | File |
|---|---|
| Linux x86_64 | `flow-x86_64-unknown-linux-gnu.tar.gz` |
| macOS Apple Silicon | `flow-aarch64-apple-darwin.tar.gz` |
| Windows x86_64 | `flow-x86_64-pc-windows-msvc.zip` |

## Install

Extract the archive and place the binary here:

```bash
# Linux / macOS
tar -xzf flow-<target>.tar.gz
cp flow ~/.claude/skills/flow-cli/assets/flow
chmod +x ~/.claude/skills/flow-cli/assets/flow

# Windows — extract the zip and place flow.exe here
```

## Alternative

If you install `flow` system-wide via `cargo install --path .`, the skill will find it on `$PATH` automatically and the bundled binary is not needed.

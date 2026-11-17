# @@ID@@

What `@@BINARY@@` does, when to run it, and what its output means.

## Installing the binary

Grimoire never installs binaries. Until release builds exist, build it from this repository:

```bash
cargo install --locked --path catalog/tools/@@ID@@
```

## Exit status

| Status | Meaning |
|---|---|
| 0 | success |
| 1 | the check failed |
| 2 | usage error |

## Security notes

What the tool can cause, and why it needs each permission it declares.

# Node-Watcher
[![CI](https://github.com/freq-data/node-watcher/workflows/CI/badge.svg)](https://github.com/freq-data/node-watcher)
[![CodeFactor](https://www.codefactor.io/repository/github/freq-data/node-watcher/badge)](https://www.codefactor.io/repository/github/freq-data/node-watcher)
[![codecov](https://codecov.io/gh/freq-data/node-watcher/branch/main/graph/badge.svg?token=HHBhutxpnG)](https://codecov.io/gh/freq-data/node-watcher)
[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://github.com/freq-data/node-watcher/blob/main/LICENSE)
[![Crates.io](https://img.shields.io/crates/v/node-watcher?color=%23FF4500)](https://crates.io/crates/node-watcher)
[![Docs.rs](https://docs.rs/node-watcher/badge.svg)](https://docs.rs/crate/node-watcher)

Node watcher, watches a ETH node and publishes the data.

## Development

### Requirements
We try to keep all local dev tools inside the Rust environment to make the
setup and tooling as simple as possible.

* [Rust](https://www.rust-lang.org/tools/install)
* [Cargo Make](https://github.com/sagiegurari/cargo-make)

### Build

This will format, lint and build:
```
cargo make build
```

If any fmt issues are found they can be fixed with:
```
cargo make styleFix
```

### Commits

We follow [The Conventional Commits specification](https://www.conventionalcommits.org/en/v1.0.0-beta.4/#summary).
This is also enforced by CI.

### Deploy

Deployment is automated with [Semantic Release](https://github.com/semantic-release/semantic-release).
As soon as commits are merged into `main` the release workflow begins.

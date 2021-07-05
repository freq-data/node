# node
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

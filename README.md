# algae

[![crates.io](https://img.shields.io/crates/v/algae.svg)](https://crates.io/crates/algae)
[![docs.rs](https://docs.rs/algae/badge.svg)](https://docs.rs/algae)
[![License](https://img.shields.io/crates/l/algae.svg)](https://crates.io/crates/algae)

[![Clippy](https://github.com/FL03/algae/actions/workflows/clippy.yml/badge.svg)](https://github.com/FL03/algae/actions/workflows/clippy.yml)
[![Rust](https://github.com/FL03/algae/actions/workflows/rust.yml/badge.svg)](https://github.com/FL03/algae/actions/workflows/rust.yml)

***

_**Warning: the library is currently in development and the API is subject to heavy changes!**_

Welcome to algae, a collection of optimized data-structures and algorithms intended for use within blockchain environments.

## Features

- [ ] `graph` - a hyper-graph implementation written in pure Rust.
- [ ] `merkle` - an optimized merkle tree library
- [ ] `mmr` - a merkle mountain range implementation

## Getting Started

### Build from the source

Start by cloning the repository:

```bash
    git clone https://github.com/FL03/algae.git
```

Then, build the project using cargo:

```bash
    cargo build --all-features --workspace
```

### Usage

Add this to your `Cargo.toml`:

```toml
[dependencies.algae]
features = []
version = "0.1"
```

### Examples

#### _HyperGraph: Basic Usage_

```rust
extern crate algae;

use algae::graph::HyperGraph;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new graph
    let mut graph = HyperGraph::new();

    Ok(())
}
```

#### _MerkleTree: Basic Usage_

```rust
extern crate algae;

use algae::merkle::MerkleTree;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new merkle tree
    let mut merkle = MerkleTree::new();

    Ok(())
}
```

## Contributors

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.

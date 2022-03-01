# FeOs - PeTS

[![crate](https://img.shields.io/crates/v/feos-pets.svg)](https://crates.io/crates/feos-pets)
[![documentation](https://docs.rs/feos-pets/badge.svg)](https://docs.rs/feos-pets)
[![documentation](https://img.shields.io/badge/docs-github--pages-blue)](https://feos-org.github.io/feos/)

Implementation of the PeTS equation of state and corresponding Helmholtz energy functional[^heier2018] within the FeOs project. This project contains a Rust implementation as well as bindings to Python.


## Usage in Python

If you want to use `feos-pets` in Python, take a look at the [`feos`-repository](https://github.com/feos-org/feos). `feos` contains multiple equation of state implementations in a single, easy-to-use Python package.


## FeOs

> FeOs is a framework for equations of state and classical density function theory

You can learn more about the principles behind `FeOs` [here](https://feos-org.github.io/feos/).


## Installation

Add this to your `Cargo.toml`

```toml
[dependencies]
feos-pets = "0.1"
```

## Test building python wheel

From within a Python virtual environment with `maturin` installed, type

```
maturin build --release --out dist --no-sdist -m build_wheel/Cargo.toml
pip install dist/feos_pets-0.1.0-[...].whl --force-reinstall
```

or 

```
maturin develop --release -m build_wheel/Cargo.toml
```

[^heier2018]: [M. Heier, S. Stephan, J. Liu, W.G. Chapman, H. Hasse & K. Langenbac (2018). *Molecular Physics*, 116(15-16), 2083-2094.](https://doi.org/10.1080/00268976.2018.1447153)

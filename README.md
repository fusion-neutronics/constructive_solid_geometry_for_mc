[![Python tests](https://github.com/fusion-neutronics/constructive_solid_geometry_for_mc/actions/workflows/ci-python.yml/badge.svg)](https://github.com/fusion-neutronics/constructive_solid_geometry_for_mc/actions/workflows/ci-python.yml)
[![Rust tests](https://github.com/fusion-neutronics/constructive_solid_geometry_for_mc/actions/workflows/ci-rust.yml/badge.svg)](https://github.com/fusion-neutronics/constructive_solid_geometry_for_mc/actions/workflows/ci-rust.yml)

# Constructive Solid Geometry for MC

Constructive Solid Geometry for MC is a package for making CSG geometry for Monte Carlo Codes.

🦀Rust back end for speed
🐍Python API for ease of use

Developer install
```bash
 python3 -m venv .env

 source .env/bin/activate

 cargo build

 pip install maturin

 maturin develop --features pyo3
 ```
# Wrapper crate for [KaHyPar](https://github.com/kahypar/kahypar)

This crate provides Rust bindings for the **KaHyPar library** (Karlsruhe Hypergraph Partitioning) written in C++. The library can do hypergraph partitioning with different objectives and hyperparameters. Check the original repository for details.

## Using this crate

This crate makes use of the `kahypar-sys` crate which ships with the C++ code of KaHyPar and takes care of building it when the crate is being built. For this, **cmake must be installed on the system**. The build can take a while.

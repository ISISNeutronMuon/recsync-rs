# Recsync-rs

A rust implementation of [recsync](https://github.com/ChannelFinder/recsync) with python bindings.Aiming for bug to bug compatibility. 
See the [recsync](https://github.com/ChannelFinder/recsync) original repository for details about the protocol.

## Project status 
The project initially would implement only **Recaster** in Rust with Python binding to be used along with [p4p](https://github.com/mdavidsaver/p4p).

## Requirements
* Rust 1.54.0 or later
* Python 3.7 or later
* [Maturin](https://github.com/PyO3/maturin) 

## Build and Installation

Rust library
```bash
cargo build
```

### Building Python bindings

Ensure that [Maturin](https://github.com/PyO3/maturin) is installed.

```bash
pip install maturin
```

```bash
cd pyreccaster
maturin build
# to install the python bindings
pip install . 
```

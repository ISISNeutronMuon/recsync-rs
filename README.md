# Recsync-rs

A rust implementation of [recsync](https://github.com/ChannelFinder/recsync) with python bindings.Aiming for bug to bug compatibility.  
See the [recsync](https://github.com/ChannelFinder/recsync) original repository for details about the protocol and theory of operation.

## Project status 
The project initially would implement only **ReCaster** in Rust with Python binding to be used along with [p4p](https://github.com/mdavidsaver/p4p). 
**RecCeiver** is not implemented yet. Recsync-rs is split into different sections. First part is `wire` which implements only the protocol definition, encoders and decoders. 
It used by **ReCaster** and **RecCeiver** (not implemented yet). Second part is `reccaster` which is **ReCaster** implementation, as it will be used as rust library. 
Finally, `pyreccaster` is a [pyo3](https://github.com/PyO3/pyo3) Rust-wrapped Python library of `reccaster`.

### RecCeiver functionality

[X] Announcement Message
[X] Ping
[X] Add Record
[ ] Delete Record
[X] Add Info

## Usage Example 

Using Rust
```rust
use reccaster::{record::Record, Reccaster};

#[tokio::main]
async fn main() {

    let mut record = Record::new("DEV:RECASTER:RUST".to_string(), "ai".to_string());
    record.properties.insert("recordDesc".to_string(), "Rust Recaster".to_string());
    let records: Vec<Record> = vec![record];

    let mut caster = Reccaster::new(records).await;
    caster.run().await;
}
```

Using Python bindings
```python
import asyncio
from pyreccaster import PyReccaster, PyRecord
from p4p.nt import NTScalar
from p4p.server.asyncio import SharedPV
from p4p.server import Server


async def main():
    pv = SharedPV(nt=NTScalar('d'), initial=0.0)

    @pv.put
    def handle(pv, op):
        pv.post(op.value())
        print(f"{op.value()}")
        op.done()

    records = [
        PyRecord(name="DEV:P4P:VAL", type="ai", alias="DEV:P4P:TEST", properties={"recordDesc": "P4P Recaster"}),
    ]

    with Server(providers=[{"DEV:P4P:VAL": pv}]):
        py_reccaster = await PyReccaster.setup(records)
        await py_reccaster.run()


if __name__ == "__main__":
    asyncio.run(main())
```

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

### Cross-Compile Python bindings for Windows

Ensure that [Maturin](https://github.com/PyO3/maturin) is installed.

Add rust windows target
```bash
rustup target add x86_64-pc-windows-gnu 
```

Install `mingw-w64` for windowws cross-compilation
```bash
apt install mingw-w64
```

Build for specific target and python version
```bash
maturin build --release --target x86_64-pc-windows-gnu --interpreter python3.9
```
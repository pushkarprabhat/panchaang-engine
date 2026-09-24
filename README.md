# panchang-engine
High-performance Rust library (Wasm-ready) for bi-directional Panchang calculations (Gregorian ↔ Hindu Calendar).

![CI](https://github.com/pushkarprabhat/panchaang-engine/actions/workflows/ci.yml/badge.svg)

Features:

- `ephem`: enable high-precision ephemeris support via `siderust` (pure-Rust). Note: `siderust` is AGPL-licensed.
- `wasm`: enable WebAssembly JS bindings via `wasm-bindgen`.

Rust usage (native):

```bash
cargo add panchaang-engine --path .
cargo test --features ephem -- --nocapture
```

WASM usage (build):

```bash
cargo build --target wasm32-unknown-unknown --features wasm
wasm-bindgen --out-dir pkg --target web target/wasm32-unknown-unknown/debug/panchaang_engine.wasm
```

CLI:

```bash
cargo run --bin panchaang -- "2026-09-23T00:00:00Z" 28.6139 77.2090
```

License & Commercial Contact:

This project is licensed under AGPLv3. If you require a commercial, non-GPL license, please contact the maintainers at legal@example.com to discuss dual-licensing options.


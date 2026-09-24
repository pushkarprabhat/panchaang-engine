# panchaang-engine

Owned bi-directional Panchang engine (Gregorian ↔ Hindu calendar).

Standards: [`STANDARDS.md`](STANDARDS.md). In-tree Sun/Moon (Meeus), Lahiri ayanamsa, civil tithi at **local sunrise**. No siderust. No Drik.

![CI](https://github.com/pushkarprabhat/panchaang-engine/actions/workflows/ci.yml/badge.svg)

## Build and test

```bash
cargo test
cargo run --bin panchaang -- "2023-11-12T00:00:00Z" 28.6139 77.2090
```

API (standalone):

```bash
cargo run --bin panchaang-api --features server
# POST http://127.0.0.1:8088/v1/panchang
# {"date_time":"2023-03-22T00:00:00Z","latitude":19.076,"longitude":72.8777}
```

Wasm:

```bash
cargo build --target wasm32-unknown-unknown --features wasm
```

`--features ephem` is accepted and ignored (kept so older CI flags still run).

## License

AGPL-3.0-only. Commercial dual-license: replace `legal@example.com` with the LLP address before publishing.

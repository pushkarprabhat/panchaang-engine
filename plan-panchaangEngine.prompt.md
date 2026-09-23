## Plan: Panchaang Next Phase

TL;DR: Add a test suite first to lock API behavior, then replace crude astronomical approximations with a high-precision ephemeris library, and finally harden edge-case handling (high latitudes, polar day/night). Implementations will keep `Paksha + Tithi` as the public tithi representation and reuse the current `panchaang` module layout.

**Steps**
1. Add unit tests (Highest priority): write `tests/panchaang_tests.rs` with reverse and forward checks against known reference dates. (*blocks step 2*)
2. Integrate precise ephemeris (Meeus / Swiss Ephemeris): replace mean-longitude helpers with vetted library calls and add `ayanamsa` handling. (*depends on step 1 for regression tests; parallelizable with step 3*)
3. Handle edge cases and validation: polar regions, longitudes/latitudes out of range, missing sunrise/sunset, explicit `Err` returns. (*parallel with step 2*)
4. API polish & extras: add helper types, documentation, example CLI, and (optional) Wasm bindings for browser use. (*after steps 1–3*)

**Relevant files**
- `src/panchaang/forward.rs` — refine `calculate_panchaang()` to use precise ephemeris and robust sunrise calculations.
- `src/panchaang/reverse.rs` — keep bisection search but call the improved low-level ephemeris and sunrise helpers.
- `src/types.rs` — public types for `PanchangInput`, `PanchangQuery`, `Paksha`, `MonthSystem`.
- `Cargo.toml` — add chosen astronomy crate (e.g., `meeus`, `swisseph` or `astronomy`) and feature flags.
- `tests/panchaang_tests.rs` — new test harness with a small dataset of known Panchang ↔ Gregorian mappings.

**Verification**
1. Run `cargo test` and assert exact match for paksha/tithi and sunrise window within a conservative tolerance (e.g., ±5 minutes) for initial tests.
2. After integrating ephemeris, tighten tolerances (e.g., ±10 seconds) and verify against several historical almanac entries (Diwali, Holi, Gudhi Padwa, etc.).
3. Add CI step to run `cargo test` and optionally run sample builds for Wasm target.

**Decisions**
- Use `Paksha + Tithi(1..=15)` for public API (already implemented). This will remain the canonical representation in bindings.
- Replace simple mean-longitude math with a high-precision library before claiming production accuracy.
- Tests are the gating criterion: don't change core algorithms until tests exist and pass.

**Further Considerations**
1. Astronomy library choice: `meeus` is pure-Rust and easy to integrate; `swisseph`/Swiss Ephemeris is authoritative but requires native bindings. I can prototype with `meeus` and optionally add `swisseph` later.
2. Reference dataset: do you have a small set of trusted Panchang reference dates to embed in tests? If not, I can add canonical festival dates for recent years.
3. CI/Platform targets: confirm whether you want to support `wasm32-unknown-unknown` early; it affects library choices for native bindings.

**Next action**
- Confirm which task to start: `Unit tests`, `Integrate ephemeris`, `Edge-case handling`, or `Wasm bindings`.

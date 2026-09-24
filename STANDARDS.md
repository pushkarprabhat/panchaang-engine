# panchaang-engine standards

These are the **default product standards** until the owner replaces a line.
Anything marked **OWNER** needs an explicit yes/no.

## Calendar

| Rule | Default | Owner must confirm |
|---|---|---|
| Tithi numbering | `Paksha` + `1..=15` (15 = Purnima or Amavasya) | No unless you want 1..=30 |
| Civil-day tithi | Tithi **at local sunrise** for that lat/lon | **OWNER**: sunrise vs sunset vs midnight |
| Month system default | Amanta. `is_purnimanta=true` shifts the reverse search window | **OWNER** |
| Ayanamsa | Lahiri (Chitrapaksha), applied to Sun/Moon before nakshatra and yoga | **OWNER** if you want a different ayanamsa |
| Tithi / karana | From **elongation** (Moon − Sun). Ayanamsa cancels. | — |
| Time scale | Input UTC. Local civil day uses `timezone_offset_hours`. | — |

## Astronomy (in-repo, no siderust, no Drik)

- Sun: Meeus apparent ecliptic longitude (equation of centre + low-order apparent correction).
- Moon: Meeus truncated ELP periodic terms (in-tree).
- Sunrise / sunset: NOAA geometric algorithm, solar altitude −0.83°.
- Polar day/night → `PanchaangError::PolarDayNight`.

This is **owned** math. It is not JPL DE440. Accuracy target for v0: tithi correct at sunrise for certified gold rows; muhurat bounds within a few minutes.

## Festival assignment (separate from civil tithi)

A festival is **not** “whatever tithi the engine printed at 00:00 UTC”.

| Festival | Civil rule we use in gold tests | Owner must confirm |
|---|---|---|
| Diwali | Kartika Amavasya prevailing at **local sunset / Pradosh**, not sunrise | **OWNER** |
| Gudi Padwa / Ugadi / Chaitra Navratri day 1 | Chaitra Shukla Pratipada at **sunrise** | **OWNER** |

Gold tests live in `tests/panchaang_tests.rs` (`gold_sunrise_tithi`).

## Service

Standalone HTTP API in this repo (`panchaang-api`). No other product is coupled here.

## License

AGPL-3.0-only on **this** source. Commercial dual-license is possible only for this tree (no siderust).

**OWNER**: replace `legal@example.com` with the LLP address before any public publish.

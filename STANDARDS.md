# panchaang-engine standards

Confirmed by owner on 2026-09-24. These are product rules, not drafts.

## Calendar

| Rule | Locked value |
|---|---|
| Tithi numbering | `Paksha` + `1..=15` (15 = Purnima or Amavasya) |
| Civil-day tithi | Tithi at **local sunrise** for that lat/lon |
| Month system default | **Amanta**. `is_purnimanta=true` shifts the reverse search window |
| Ayanamsa | **Lahiri (Chitrapaksha)**, applied before nakshatra and yoga |
| Tithi / karana | From elongation (Moon − Sun). Ayanamsa cancels. |
| Time scale | Input UTC. Local civil day uses `timezone_offset_hours`. |

## Astronomy (in-repo, no siderust, no Drik)

- Sun: Meeus apparent ecliptic longitude.
- Moon: Meeus truncated ELP periodic terms.
- Sunrise / sunset: NOAA, solar altitude −0.83°.
- Polar day/night → `PanchaangError::PolarDayNight`.

Accuracy target for v0: certified gold rows match at sunrise; tithi bounds within a few minutes.

## Festival assignment

A festival is not “whatever tithi printed at 00:00 UTC”.

| Festival | Locked rule |
|---|---|
| Diwali | Kartika Amavasya prevailing at **local sunset / Pradosh**. Sunrise that morning may still be Krishna 14. |
| Gudi Padwa / Ugadi / Chaitra Navratri day 1 | Chaitra Shukla Pratipada at **sunrise** |

## Certified cities

Gold tests must pass at sunrise for:

| City | Lat | Lon |
|---|---|---|
| Delhi | 28.6139 | 77.2090 |
| Mumbai | 19.0760 | 72.8777 |
| Ujjain | 23.1765 | 75.7849 |
| Jaipur | 26.9124 | 75.7873 |

Rows: `tests/panchaang_tests.rs`.

## Service

Standalone HTTP API in this repo (`panchaang-api`). No other product is coupled here.

## License

AGPL-3.0-only on this source. Commercial dual-license enquiries: **legal@theiaone-ai.com**.

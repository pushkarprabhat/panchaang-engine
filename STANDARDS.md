# panchaang-engine standards

Confirmed by owner on 2026-09-24. These are product rules, not drafts.

## Calendar — three independent axes

Do not mix these. A Shaka book and a Gujarati wall calendar can disagree on
“when Shravan starts” while agreeing on the Moon.

### 1. Paksha (sky)

Shukla = waxing, Krishna = waning. Always computed from elongation.
Not a regional choice.

### 2. Month system (name of the month)

| | Amanta (default, Gujarat / MH / most South) | Purnimanta (common North Indian print) |
|---|---|---|
| Month ends | Amavasya | Purnima |
| Shukla paksha month name | Same in both | Same in both |
| Krishna paksha month name | Still the month that just completed Shukla | Already the *next* month |

Worked example — why Ahmedabad Shravan ≠ home Shaka 1948 Shravan:

- After Ashadha Purnima, Amanta still calls the waning fortnight **Ashadha Krishna**.
- Purnimanta already calls that same fortnight **Shravan Krishna**.
- So “Shravan begins” is ~15 days apart. Tithi number and paksha are identical.
- Shaka 1948 is only the *year label* (≈ 2026 CE). It does not pick Amanta or Purnimanta.
- A second, rarer clash: some Gujarati almanacs are Kartikadi (year starts Kartika),
  not Chaitradi. That shifts the *year number* at Kartika, not the tithi.

Resolution in this engine: compute tithi at local sunrise from lat/lon; attach a
**month name only after** the caller states `month_system`. Default Amanta.
Home Shaka Purnimanta: `era=shaka` + `month_system=purnimanta`.
Ahmedabad Gujarati: `era=vikrama` + `month_system=amanta`.

### 3. Era (year number only)

| Era | Rough conversion |
|---|---|
| Vikrama | Gregorian + 57 |
| Shaka | Gregorian − 78 (Shaka 1948 ≈ 2026 CE) |

## Locked computation rules

| Rule | Locked value |
|---|---|
| Tithi numbering | `Paksha` + `1..=15` |
| Civil-day tithi | Tithi at **local sunrise** for that lat/lon |
| Month system default | **Amanta** |
| Ayanamsa | **Lahiri** |
| Time scale | Input UTC |

## Place

The sky is computed for **any** lat/lon at **any** UTC instant.
Named cities (`src/geo/cities.rs`) are only a gazetteer so a client can send
`city=Ahmedabad` instead of coordinates. Missing city ≠ missing panchang:
pass latitude and longitude.

Certified gold tests (sunrise tithi): Delhi, Mumbai, Ujjain, Jaipur.

## Festival assignment

| Festival | Locked rule |
|---|---|
| Diwali | Kartika Amavasya at **local sunset / Pradosh** |
| Gudi Padwa / Ugadi / Chaitra Navratri day 1 | Chaitra Shukla Pratipada at **sunrise** |

## Service

`POST /v1/panchang` — city name *or* raw lat/lon.
`GET /v1/cities?q=` — gazetteer search.

## License

AGPL-3.0-only. Commercial dual-license: **legal@theiaone-ai.com**.

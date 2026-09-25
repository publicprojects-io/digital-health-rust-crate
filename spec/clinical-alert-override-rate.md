# Spec: Clinical Alert Override Rate

- **Module**: [`src/clinical_alert_override_rate.rs`](../src/clinical_alert_override_rate.rs)
- **Status**: implemented
- **Upstream topic**: `digital-health-metrics/locales/en-gb-oxendict/topics/clinical-alert-override-rate/index.md`

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `override_rate(overridden: f64, total_fired: f64) -> Option<f64>`

- Formula: `overridden / total_fired * 100`
- Returns `None` iff `total_fired == 0.0`
- Intended to be called once per severity tier or alert type, not once for a
  blended total — see the module's rustdoc "Pitfalls" section.
- Worked examples:
  - `override_rate(8_700.0, 10_000.0) == Some(87.0)` (overall)
  - `override_rate(60.0, 500.0) == Some(12.0)` (contraindicated tier)
  - `override_rate(5_700.0, 6_000.0) == Some(95.0)` (moderate tier)

### `documented_override_rate(documented: f64, overridden: f64) -> Option<f64>`

- Formula: `documented / overridden * 100`
- Returns `None` iff `overridden == 0.0`
- Worked example: `documented_override_rate(340.0, 500.0) == Some(68.0)`

## Invariants

- `override_rate` and `documented_override_rate` are independent
  calculations over different denominators (`total_fired` vs. `overridden`)
  — they are not meant to be chained without re-checking which count is the
  denominator at each step.

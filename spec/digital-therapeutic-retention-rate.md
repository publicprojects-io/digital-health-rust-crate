# Spec: Digital Therapeutic Retention Rate

- **Module**: [`src/digital_therapeutic_retention_rate.rs`](../src/digital_therapeutic_retention_rate.rs)
- **Status**: implemented
- **Upstream topic**: none — independent topic, not present in
  [digital-health-metrics](https://github.com/digital-health-metrics/digital-health-metrics)
  at the time this module was written; see the module's rustdoc `## Sources`
  section.

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `day_7_retention_rate(active_at_day_7: f64, enrolled: f64) -> Option<f64>`

- Formula: `active_at_day_7 / enrolled * 100`
- Returns `None` iff `enrolled == 0.0`
- Worked example: `day_7_retention_rate(640.0, 1_000.0) == Some(64.0)`

### `day_30_retention_rate(active_at_day_30: f64, enrolled: f64) -> Option<f64>`

- Formula: `active_at_day_30 / enrolled * 100`
- Returns `None` iff `enrolled == 0.0`
- Worked example: `day_30_retention_rate(280.0, 1_000.0) == Some(28.0)`

## Invariants

- Both functions share the same denominator (`enrolled`) when called on the
  same cohort, so `day_30_retention_rate(a, d) <= day_7_retention_rate(b, d)`
  is expected but not enforced — a caller passing inconsistent counts (e.g.
  a day-30 active count larger than day-7) gets no validation error.

# Spec: Long-Term Retention Rate

- **Module**: [`src/long_term_retention_rate.rs`](../src/long_term_retention_rate.rs)
- **Status**: implemented
- **Upstream topic**: none — independent topic, not present in
  [digital-health-metrics](https://github.com/digital-health-metrics/digital-health-metrics)
  at the time this module was written; see the module's rustdoc `## Sources`
  section.

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `day_60_retention_rate(retained_at_day_60: f64, cohort: f64) -> Option<f64>`

- Formula: `retained_at_day_60 / cohort * 100`
- Returns `None` iff `cohort == 0.0`
- Worked example: `day_60_retention_rate(540.0, 1_000.0) == Some(54.0)`

### `day_90_retention_rate(retained_at_day_90: f64, cohort: f64) -> Option<f64>`

- Formula: `retained_at_day_90 / cohort * 100`
- Returns `None` iff `cohort == 0.0`
- Worked example: `day_90_retention_rate(410.0, 1_000.0) == Some(41.0)`

## Invariants

- Every function's result is a plain value derived directly from its arguments; no function chains into another.

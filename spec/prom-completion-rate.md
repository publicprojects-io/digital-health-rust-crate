# Spec: PROM Completion Rate

- **Module**: [`src/prom_completion_rate.rs`](../src/prom_completion_rate.rs)
- **Status**: implemented
- **Upstream topic**: none — independent topic, not present in
  [digital-health-metrics](https://github.com/digital-health-metrics/digital-health-metrics)
  at the time this module was written; see the module's rustdoc `## Sources`
  section.

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `prom_completion_rate(completed: f64, eligible: f64) -> Option<f64>`

- Formula: `completed / eligible * 100`
- Returns `None` iff `eligible == 0.0`
- Worked example: `prom_completion_rate(560.0, 800.0) == Some(70.0)`

### `digital_completion_share(completed_digitally: f64, completed: f64) -> Option<f64>`

- Formula: `completed_digitally / completed * 100`
- Returns `None` iff `completed == 0.0`
- Worked example: `digital_completion_share(420.0, 560.0) == Some(75.0)`

## Invariants

- `prom_completion_rate` and `digital_completion_share` have different
  denominators (`eligible` vs. `completed`) — they are not meant to be
  chained beyond `digital_completion_share`'s denominator being drawn from
  `prom_completion_rate`'s numerator.

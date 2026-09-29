# Spec: Digital Access Rate

- **Module**: [`src/digital_access_rate.rs`](../src/digital_access_rate.rs)
- **Status**: implemented
- **Upstream topic**: none — independent topic, not present in
  [digital-health-metrics](https://github.com/digital-health-metrics/digital-health-metrics)
  at the time this module was written; see the module's rustdoc `## Sources`
  section.

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `digital_access_rate(with_access: f64, population: f64) -> Option<f64>`

- Formula: `with_access / population * 100`
- Returns `None` iff `population == 0.0`
- Worked example: `digital_access_rate(8_400.0, 12_000.0) == Some(70.0)`

### `access_parity_ratio(group_rate: f64, reference_rate: f64) -> Option<f64>`

- Formula: `group_rate / reference_rate`
- Returns `None` iff `reference_rate == 0.0`
- Worked example: `access_parity_ratio(60.0, 80.0) == Some(0.75)`

## Invariants

- `access_parity_ratio` returns a plain ratio, **not** a percentage; it takes
  percentage rates (typically the output of `digital_access_rate`) for both
  arguments, so the units cancel.

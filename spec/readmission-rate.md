# Spec: Readmission Rate

- **Module**: [`src/readmission_rate.rs`](../src/readmission_rate.rs)
- **Status**: implemented
- **Upstream topic**: none — independent topic, not present in
  [digital-health-metrics](https://github.com/digital-health-metrics/digital-health-metrics)
  at the time this module was written; see the module's rustdoc `## Sources`
  section.

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `readmission_rate(readmissions: f64, index_discharges: f64) -> Option<f64>`

- Formula: `readmissions / index_discharges * 100`
- Returns `None` iff `index_discharges == 0.0`
- Worked example: `readmission_rate(400.0, 2_500.0) == Some(16.0)`

### `readmission_reduction(baseline_rate: f64, current_rate: f64) -> Option<f64>`

- Formula: `(baseline_rate - current_rate) / baseline_rate * 100`
- Returns `None` iff `baseline_rate == 0.0`
- Worked example: `readmission_reduction(16.0, 13.6) == Some(15.0)`

## Invariants

- `readmission_reduction` takes percentage rates (typically the output of
  `readmission_rate`), not counts. Its result is negative when the current
  rate exceeds the baseline.

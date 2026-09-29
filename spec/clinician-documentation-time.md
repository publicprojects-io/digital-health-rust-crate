# Spec: Clinician Documentation Time

- **Module**: [`src/clinician_documentation_time.rs`](../src/clinician_documentation_time.rs)
- **Status**: implemented
- **Upstream topic**: none — independent topic, not present in
  [digital-health-metrics](https://github.com/digital-health-metrics/digital-health-metrics)
  at the time this module was written; see the module's rustdoc `## Sources`
  section.

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `documentation_minutes_per_encounter(documentation_minutes: f64, encounters: f64) -> Option<f64>`

- Formula: `documentation_minutes / encounters`
- Returns `None` iff `encounters == 0.0`
- Worked example: `documentation_minutes_per_encounter(240_000.0, 12_000.0) == Some(20.0)`

### `documentation_time_reduction(baseline_minutes: f64, current_minutes: f64) -> Option<f64>`

- Formula: `(baseline_minutes - current_minutes) / baseline_minutes * 100`
- Returns `None` iff `baseline_minutes == 0.0`
- Worked example: `documentation_time_reduction(20.0, 15.0) == Some(25.0)`

## Invariants

- `documentation_minutes_per_encounter` returns minutes, not a percentage.
  `documentation_time_reduction` is negative when the current time exceeds
  the baseline.

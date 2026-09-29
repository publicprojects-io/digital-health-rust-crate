# Spec: Virtual Ward Bed Days

- **Module**: [`src/virtual_ward_bed_days.rs`](../src/virtual_ward_bed_days.rs)
- **Status**: implemented
- **Upstream topic**: none — independent topic, not present in
  [digital-health-metrics](https://github.com/digital-health-metrics/digital-health-metrics)
  at the time this module was written; see the module's rustdoc `## Sources`
  section.

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `bed_days_saved(patients: f64, conventional_stay_days: f64, actual_stay_days: f64) -> f64`

- Formula: `patients * (conventional_stay_days - actual_stay_days)`
- Never returns `None` — there is no denominator.
- Worked example: `bed_days_saved(200.0, 6.0, 4.5) == 300.0`

### `bed_day_reduction_rate(baseline_bed_days: f64, current_bed_days: f64) -> Option<f64>`

- Formula: `(baseline_bed_days - current_bed_days) / baseline_bed_days * 100`
- Returns `None` iff `baseline_bed_days == 0.0`
- Worked example: `bed_day_reduction_rate(1_200.0, 900.0) == Some(25.0)`

## Invariants

- Every function's result is a plain value derived directly from its arguments; no function chains into another.

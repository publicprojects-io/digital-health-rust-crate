# Spec: Biometric Improvement

- **Module**: [`src/biometric_improvement.rs`](../src/biometric_improvement.rs)
- **Status**: implemented
- **Upstream topic**: none — independent topic, not present in
  [digital-health-metrics](https://github.com/digital-health-metrics/digital-health-metrics)
  at the time this module was written; see the module's rustdoc `## Sources`
  section.

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `estimated_a1c(mean_glucose_mg_dl: f64) -> f64`

- Formula: `(mean_glucose_mg_dl + 46.7) / 28.7`
- Never returns `None` — there is no denominator.
- Worked example: `estimated_a1c(154.2) == 7.0`

### `biometric_reduction(baseline: f64, current: f64) -> Option<f64>`

- Formula: `(baseline - current) / baseline * 100`
- Returns `None` iff `baseline == 0.0`
- Worked example: `biometric_reduction(9.0, 7.65) == Some(15.0)`

### `target_attainment_rate(at_target: f64, measured: f64) -> Option<f64>`

- Formula: `at_target / measured * 100`
- Returns `None` iff `measured == 0.0`
- Worked example: `target_attainment_rate(420.0, 600.0) == Some(70.0)`

## Invariants

- Every function's result is a plain value derived directly from its arguments; no function chains into another.

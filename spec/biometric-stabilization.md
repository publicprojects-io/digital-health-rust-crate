# Spec: Biometric Stabilization

- **Module**: [`src/biometric_stabilization.rs`](../src/biometric_stabilization.rs)
- **Status**: implemented
- **Upstream topic**: none — independent topic, not present in
  [digital-health-metrics](https://github.com/digital-health-metrics/digital-health-metrics)
  at the time this module was written; see the module's rustdoc `## Sources`
  section.

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `blood_pressure_control_rate(controlled: f64, monitored: f64) -> Option<f64>`

- Formula: `controlled / monitored * 100`
- Returns `None` iff `monitored == 0.0`
- Worked example: `blood_pressure_control_rate(540.0, 900.0) == Some(60.0)`

### `time_in_range(readings_in_range: f64, readings: f64) -> Option<f64>`

- Formula: `readings_in_range / readings * 100`
- Returns `None` iff `readings == 0.0`
- Worked example: `time_in_range(8_400.0, 12_000.0) == Some(70.0)`

## Invariants

- Every function's result is a plain value derived directly from its arguments; no function chains into another.

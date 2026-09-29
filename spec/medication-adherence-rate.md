# Spec: Medication Adherence Rate

- **Module**: [`src/medication_adherence_rate.rs`](../src/medication_adherence_rate.rs)
- **Status**: implemented
- **Upstream topic**: none — independent topic, not present in
  [digital-health-metrics](https://github.com/digital-health-metrics/digital-health-metrics)
  at the time this module was written; see the module's rustdoc `## Sources`
  section.

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `proportion_of_days_covered(days_covered: f64, days_in_period: f64) -> Option<f64>`

- Formula: `days_covered / days_in_period * 100`
- Returns `None` iff `days_in_period == 0.0`
- Worked example: `proportion_of_days_covered(72.0, 90.0) == Some(80.0)`

### `adherent_patient_rate(adherent_patients: f64, patients: f64) -> Option<f64>`

- Formula: `adherent_patients / patients * 100`
- Returns `None` iff `patients == 0.0`
- Worked example: `adherent_patient_rate(560.0, 800.0) == Some(70.0)`

## Invariants

- Every function's result is a plain value derived directly from its arguments; no function chains into another.

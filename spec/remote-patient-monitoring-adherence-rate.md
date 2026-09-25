# Spec: Remote Patient Monitoring Adherence Rate

- **Module**: [`src/remote_patient_monitoring_adherence_rate.rs`](../src/remote_patient_monitoring_adherence_rate.rs)
- **Status**: implemented
- **Upstream topic**: none — independent topic, not present in
  [digital-health-metrics](https://github.com/digital-health-metrics/digital-health-metrics)
  at the time this module was written; see the module's rustdoc `## Sources`
  section.

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `monitoring_adherence_rate(days_with_reading: f64, days_in_period: f64) -> Option<f64>`

- Formula: `days_with_reading / days_in_period * 100`
- Returns `None` iff `days_in_period == 0.0`
- Worked example: `monitoring_adherence_rate(24.0, 30.0) == Some(80.0)`

### `billing_threshold_met_rate(patients_meeting_threshold: f64, total_enrolled_patients: f64) -> Option<f64>`

- Formula: `patients_meeting_threshold / total_enrolled_patients * 100`
- Returns `None` iff `total_enrolled_patients == 0.0`
- Worked example: `billing_threshold_met_rate(280.0, 400.0) == Some(70.0)`

## Invariants

- The two functions have independent denominators (`days_in_period` is a
  count of days for one patient/period; `total_enrolled_patients` is a
  count of patients for a cohort) — they are not meant to be chained.
- Neither function encodes the payer's specific day threshold (e.g. CMS's
  commonly used ≥16-of-30); the caller determines which patients meet the
  threshold before calling `billing_threshold_met_rate`.

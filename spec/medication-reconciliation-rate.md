# Spec: Medication Reconciliation Rate

- **Module**: [`src/medication_reconciliation_rate.rs`](../src/medication_reconciliation_rate.rs)
- **Status**: implemented
- **Upstream topic**: none — independent topic, not present in
  [digital-health-metrics](https://github.com/digital-health-metrics/digital-health-metrics)
  at the time this module was written; see the module's rustdoc `## Sources`
  section.

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `admission_reconciliation_rate(admissions_reconciled: f64, total_admissions: f64) -> Option<f64>`

- Formula: `admissions_reconciled / total_admissions * 100`
- Returns `None` iff `total_admissions == 0.0`
- Worked example: `admission_reconciliation_rate(850.0, 1_000.0) == Some(85.0)`

### `discharge_reconciliation_rate(discharges_reconciled: f64, total_discharges: f64) -> Option<f64>`

- Formula: `discharges_reconciled / total_discharges * 100`
- Returns `None` iff `total_discharges == 0.0`
- Worked example: `discharge_reconciliation_rate(760.0, 950.0) == Some(80.0)`

## Invariants

- `admission_reconciliation_rate` and `discharge_reconciliation_rate` have
  independent denominators (`total_admissions` vs. `total_discharges`) over
  potentially different patient cohorts in the same period — they are not
  meant to be combined into a single blended rate.

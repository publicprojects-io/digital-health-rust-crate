# Spec: Remote Patient Monitoring Billing Revenue

- **Module**: [`src/remote_patient_monitoring_billing_revenue.rs`](../src/remote_patient_monitoring_billing_revenue.rs)
- **Status**: implemented
- **Upstream topic**: none — independent topic, not present in
  [digital-health-metrics](https://github.com/digital-health-metrics/digital-health-metrics)
  at the time this module was written; see the module's rustdoc `## Sources`
  section.

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc, and its `## Money convention` section for how this
file's contract differs from an `f64`-based one.

## Contract

### `billable_revenue<T: FormattableCurrency>(billing_eligible_patients: u32, reimbursement_per_patient: Money<'_, T>) -> Result<Money<'_, T>, MoneyError>`

- Formula: `reimbursement_per_patient * billing_eligible_patients` (via
  `Money::mul`)
- Returns `Err(MoneyError::Overflow)` only if the multiplication overflows.
- Worked example: `billable_revenue(280, Money::from_major(50, iso::USD)) ==
  Ok(Money::from_major(14_000, iso::USD))`

### `revenue_at_risk<T: FormattableCurrency>(non_eligible_patients: u32, reimbursement_per_patient: Money<'_, T>) -> Result<Money<'_, T>, MoneyError>`

- Formula: `reimbursement_per_patient * non_eligible_patients` (via
  `Money::mul`)
- Returns `Err(MoneyError::Overflow)` only if the multiplication overflows.
- Worked example: `revenue_at_risk(120, Money::from_major(50, iso::USD)) ==
  Ok(Money::from_major(6_000, iso::USD))`

## Invariants

- `billing_eligible_patients` and `non_eligible_patients` are `u32` counts
  of the same cohort at the same `reimbursement_per_patient`; calling both
  functions with the same rate and patient counts that sum to the cohort
  total yields `billable_revenue.add(revenue_at_risk) ==
  reimbursement_per_patient.mul(cohort_total)`.
- Neither function encodes a specific CPT/HCPCS reimbursement figure — see
  [`spec/remote-patient-monitoring-adherence-rate.md`](remote-patient-monitoring-adherence-rate.md)
  for the underlying billing-threshold definition this module's counts come
  from.

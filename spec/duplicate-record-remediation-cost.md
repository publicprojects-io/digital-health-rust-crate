# Spec: Duplicate Record Remediation Cost

- **Module**: [`src/duplicate_record_remediation_cost.rs`](../src/duplicate_record_remediation_cost.rs)
- **Status**: implemented
- **Upstream topic**: none — independent topic, not present in
  [digital-health-metrics](https://github.com/digital-health-metrics/digital-health-metrics)
  at the time this module was written; see the module's rustdoc `## Sources`
  section.

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc, and its `## Money convention` section for how this
file's contract differs from an `f64`-based one.

## Contract

### `remediation_cost(duplicates_resolved: u32, cost_per_resolution: Money<'static, iso::Currency>) -> Result<Money<'static, iso::Currency>, MoneyError>`

- Formula: `cost_per_resolution * duplicates_resolved` (via `Money::mul`,
  called directly)
- Returns `Err(MoneyError::Overflow)` only if the multiplication overflows.
- Worked example: `remediation_cost(31_500, Money::from_major(18, iso::USD))
  == Ok(Money::from_major(567_000, iso::USD))`

### `backlog_cost(duplicates_unresolved: u32, cost_per_resolution: Money<'static, iso::Currency>) -> Result<Money<'static, iso::Currency>, MoneyError>`

- Formula: `cost_per_resolution * duplicates_unresolved` (via `Money::mul`,
  called directly)
- Returns `Err(MoneyError::Overflow)` only if the multiplication overflows.
- Worked example: `backlog_cost(10_500, Money::from_major(18, iso::USD)) ==
  Ok(Money::from_major(189_000, iso::USD))`

## Invariants

- Same shape as
  [`spec/remote-patient-monitoring-billing-revenue.md`](remote-patient-monitoring-billing-revenue.md):
  two independent `mul` calls over a cohort split at the same rate, not a
  `mul`-then-`sub` pair. `remediation_cost(a, r).add(backlog_cost(b, r))` ==
  `r.mul(a + b)` when `a` and `b` are drawn from the same identified-duplicate
  cohort as
  [`duplicate_patient_record_rate`](../src/duplicate_patient_record_rate.rs).
- The `Money` type is concrete (`Money<'static, iso::Currency>`), not
  generic over `rusty_money::FormattableCurrency` — see
  [`spec/README.md`](README.md)'s Money convention for why.

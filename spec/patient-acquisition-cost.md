# Spec: Patient Acquisition Cost

- **Module**: [`src/patient_acquisition_cost.rs`](../src/patient_acquisition_cost.rs)
- **Status**: implemented
- **Upstream topic**: none — independent topic, not present in
  [digital-health-metrics](https://github.com/digital-health-metrics/digital-health-metrics)
  at the time this module was written; see the module's rustdoc `## Sources`
  section.

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc, and its `## Money convention` section for how this
file's contract differs from an `f64`-based one.

## Contract

### `total_acquisition_cost(ad_spend: Money<'static, iso::Currency>, agency_fees: Money<'static, iso::Currency>, technology_cost: Money<'static, iso::Currency>, intake_labor_cost: Money<'static, iso::Currency>) -> Result<Money<'static, iso::Currency>, MoneyError>`

- Formula: `ad_spend + agency_fees + technology_cost + intake_labor_cost` (via
  chained `Money::add`, called directly)
- Returns `Err(MoneyError::CurrencyMismatch { .. })` if the four arguments
  are not all in the same currency, or `Err(MoneyError::Overflow)` if the
  sum overflows.
- Worked example: `total_acquisition_cost(Money::from_major(40_000, iso::USD),
  Money::from_major(8_000, iso::USD), Money::from_major(6_000, iso::USD),
  Money::from_major(16_000, iso::USD)) == Ok(Money::from_major(70_000,
  iso::USD))`

### `cost_per_new_patient(total_acquisition_cost: Money<'static, iso::Currency>, new_patients: u32) -> Result<Money<'static, iso::Currency>, MoneyError>`

- Formula: `total_acquisition_cost / new_patients` (via `Money::div`, called
  directly)
- Returns `Err(MoneyError::DivisionByZero)` iff `new_patients == 0`, or
  `Err(MoneyError::Overflow)` if the division overflows. Unlike every other
  `Money` function in this crate it can fail on a zero count — that is
  `rusty_money`'s own division behaviour, surfaced rather than converted to
  `None`.
- Worked example: `cost_per_new_patient(Money::from_major(70_000, iso::USD),
  140) == Ok(Money::from_major(500, iso::USD))`

## Invariants

- The `Money` type is concrete (`Money<'static, iso::Currency>`), not
  generic over `rusty_money::FormattableCurrency` — see
  [`spec/README.md`](README.md)'s Money convention for why.
- `cost_per_new_patient`'s result is the CAC input to
  [`patient_acquisition_efficiency::ltv_to_cac_ratio`](patient-acquisition-efficiency.md)
  once converted to a plain amount by the caller; this crate does not
  convert `Money` to `f64`.

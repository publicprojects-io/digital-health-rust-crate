# Spec: Patient Self-Scheduling Cost Savings

- **Module**: [`src/patient_self_scheduling_cost_savings.rs`](../src/patient_self_scheduling_cost_savings.rs)
- **Status**: implemented
- **Upstream topic**: none — independent topic, not present in
  [digital-health-metrics](https://github.com/digital-health-metrics/digital-health-metrics)
  at the time this module was written; see the module's rustdoc `## Sources`
  section.

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc, and its `## Money convention` section for how this
file's contract differs from an `f64`-based one.

## Contract

### `staff_time_saved_cost(self_scheduled: u32, avoided_cost_per_booking: Money<'static, iso::Currency>) -> Result<Money<'static, iso::Currency>, MoneyError>`

- Formula: `avoided_cost_per_booking * self_scheduled` (via `Money::mul`,
  called directly)
- Returns `Err(MoneyError::Overflow)` only if the multiplication overflows;
  never returns a currency-mismatch error, since `self_scheduled` is a plain
  count.
- Worked example: `staff_time_saved_cost(640, Money::from_major(6, iso::USD))
  == Ok(Money::from_major(3_840, iso::USD))`

### `net_savings(staff_time_saved_cost: Money<'static, iso::Currency>, platform_cost: Money<'static, iso::Currency>) -> Result<Money<'static, iso::Currency>, MoneyError>`

- Formula: `staff_time_saved_cost - platform_cost` (via `Money::sub`, called
  directly)
- Returns `Err(MoneyError::CurrencyMismatch { .. })` if the two arguments are
  denominated in different currencies.
- Worked example:
  `net_savings(Money::from_major(3_840, iso::USD),
  Money::from_major(900, iso::USD)) ==
  Ok(Money::from_major(2_940, iso::USD))`

## Invariants

- Same shape as [`spec/no-show-lost-revenue.md`](no-show-lost-revenue.md)
  and [`spec/telehealth-cost-avoidance.md`](telehealth-cost-avoidance.md): a
  `mul` step over a plain count that can only overflow, followed by a `sub`
  step over two caller-supplied `Money` values that can mismatch currency.
- The `Money` type is concrete (`Money<'static, iso::Currency>`), not
  generic over `rusty_money::FormattableCurrency` — see
  [`spec/README.md`](README.md)'s Money convention for why.

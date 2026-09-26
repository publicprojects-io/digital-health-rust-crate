# Spec: E-Consult Cost Avoidance

- **Module**: [`src/econsult_cost_avoidance.rs`](../src/econsult_cost_avoidance.rs)
- **Status**: implemented
- **Upstream topic**: none — independent topic, not present in
  [digital-health-metrics](https://github.com/digital-health-metrics/digital-health-metrics)
  at the time this module was written; see the module's rustdoc `## Sources`
  section.

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc, and its `## Money convention` section for how this
file's contract differs from an `f64`-based one.

## Contract

### `cost_avoided(econsults_completed: u32, avoided_cost_per_consult: Money<'static, iso::Currency>) -> Result<Money<'static, iso::Currency>, MoneyError>`

- Formula: `avoided_cost_per_consult * econsults_completed` (via
  `Money::mul`, called directly)
- Returns `Err(MoneyError::Overflow)` only if the multiplication overflows;
  never returns a currency-mismatch error, since `econsults_completed` is a
  plain count.
- Worked example: `cost_avoided(400, Money::from_major(120, iso::USD)) ==
  Ok(Money::from_major(48_000, iso::USD))`

### `net_savings(cost_avoided: Money<'static, iso::Currency>, platform_cost: Money<'static, iso::Currency>) -> Result<Money<'static, iso::Currency>, MoneyError>`

- Formula: `cost_avoided - platform_cost` (via `Money::sub`, called directly)
- Returns `Err(MoneyError::CurrencyMismatch { .. })` if the two arguments are
  denominated in different currencies.
- Worked example:
  `net_savings(Money::from_major(48_000, iso::USD),
  Money::from_major(10_000, iso::USD)) ==
  Ok(Money::from_major(38_000, iso::USD))`

## Invariants

- Same shape as [`spec/no-show-lost-revenue.md`](no-show-lost-revenue.md),
  [`spec/telehealth-cost-avoidance.md`](telehealth-cost-avoidance.md), and
  [`spec/patient-self-scheduling-cost-savings.md`](patient-self-scheduling-cost-savings.md):
  a `mul` step over a plain count that can only overflow, followed by a
  `sub` step over two caller-supplied `Money` values that can mismatch
  currency.
- The `Money` type is concrete (`Money<'static, iso::Currency>`), not
  generic over `rusty_money::FormattableCurrency` — see
  [`spec/README.md`](README.md)'s Money convention for why.

# Spec: Telehealth Cost Avoidance

- **Module**: [`src/telehealth_cost_avoidance.rs`](../src/telehealth_cost_avoidance.rs)
- **Status**: implemented
- **Upstream topic**: none — independent topic, not present in
  [digital-health-metrics](https://github.com/digital-health-metrics/digital-health-metrics)
  at the time this module was written; see the module's rustdoc `## Sources`
  section.

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc, and its `## Money convention` section for how this
file's contract differs from an `f64`-based one.

## Contract

### `cost_avoided(telehealth_encounters: u32, avoided_cost_per_encounter: Money<'static, iso::Currency>) -> Result<Money<'static, iso::Currency>, MoneyError>`

- Formula: `avoided_cost_per_encounter * telehealth_encounters` (via
  `Money::mul`, called directly)
- Returns `Err(MoneyError::Overflow)` only if the multiplication overflows;
  never returns a currency-mismatch error, since `telehealth_encounters` is
  a plain count.
- Worked example: `cost_avoided(1_600, Money::from_major(40, iso::USD)) ==
  Ok(Money::from_major(64_000, iso::USD))`

### `net_savings(cost_avoided: Money<'static, iso::Currency>, platform_cost: Money<'static, iso::Currency>) -> Result<Money<'static, iso::Currency>, MoneyError>`

- Formula: `cost_avoided - platform_cost` (via `Money::sub`, called directly)
- Returns `Err(MoneyError::CurrencyMismatch { .. })` if the two arguments are
  denominated in different currencies.
- Worked example:
  `net_savings(Money::from_major(64_000, iso::USD),
  Money::from_major(18_000, iso::USD)) ==
  Ok(Money::from_major(46_000, iso::USD))`

## Invariants

- Same shape as [`spec/no-show-lost-revenue.md`](no-show-lost-revenue.md):
  a `mul` step over a plain count that can only overflow, followed by a
  `sub` step over two caller-supplied `Money` values that can mismatch
  currency.
- The `Money` type is concrete (`Money<'static, iso::Currency>`), not
  generic over `rusty_money::FormattableCurrency` — see
  [`spec/README.md`](README.md)'s Money convention for why.

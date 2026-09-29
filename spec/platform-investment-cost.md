# Spec: Platform Investment Cost

- **Module**: [`src/platform_investment_cost.rs`](../src/platform_investment_cost.rs)
- **Status**: implemented
- **Upstream topic**: none — independent topic, not present in
  [digital-health-metrics](https://github.com/digital-health-metrics/digital-health-metrics)
  at the time this module was written; see the module's rustdoc `## Sources`
  section.

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc, and its `## Money convention` section for how this
file's contract differs from an `f64`-based one.

## Contract

### `total_platform_cost(licensing_cost: Money<'static, iso::Currency>, hardware_cost: Money<'static, iso::Currency>, staffing_cost: Money<'static, iso::Currency>) -> Result<Money<'static, iso::Currency>, MoneyError>`

- Formula: `licensing_cost + hardware_cost + staffing_cost` (via chained `Money::add`, called directly)
- Errors: Returns `MoneyError::CurrencyMismatch` if the three costs are not all denominated in the same currency, or `MoneyError::Overflow` if the sum overflows.
- Worked example: $120,000.00 + $80,000.00 + $100,000.00 = $300,000.00.

### `net_benefit(total_benefit: Money<'static, iso::Currency>, total_cost: Money<'static, iso::Currency>) -> Result<Money<'static, iso::Currency>, MoneyError>`

- Formula: `total_benefit - total_cost` (via `Money::sub`, called directly)
- Errors: Returns `MoneyError::CurrencyMismatch` if `total_benefit` and `total_cost` are not denominated in the same currency.
- Worked example: $450,000.00 − $300,000.00 = $150,000.00.

## Invariants

- The `Money` type is concrete (`Money<'static, iso::Currency>`), not
  generic over `rusty_money::FormattableCurrency` — see
  [`spec/README.md`](README.md)'s Money convention for why.

# Spec: Bed-Day Cost Avoidance

- **Module**: [`src/bed_day_cost_avoidance.rs`](../src/bed_day_cost_avoidance.rs)
- **Status**: implemented
- **Upstream topic**: none — independent topic, not present in
  [digital-health-metrics](https://github.com/digital-health-metrics/digital-health-metrics)
  at the time this module was written; see the module's rustdoc `## Sources`
  section.

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc, and its `## Money convention` section for how this
file's contract differs from an `f64`-based one.

## Contract

### `bed_day_cost_avoided(bed_days_saved: u32, cost_per_bed_day: Money<'static, iso::Currency>) -> Result<Money<'static, iso::Currency>, MoneyError>`

- Formula: `cost_per_bed_day * bed_days_saved` (via `Money::mul`, called directly)
- Errors: Returns `MoneyError::Overflow` if the multiplication overflows `cost_per_bed_day`'s underlying decimal representation.
- Worked example: 300 × $500.00 = $150,000.00.

### `net_savings(cost_avoided: Money<'static, iso::Currency>, virtual_ward_cost: Money<'static, iso::Currency>) -> Result<Money<'static, iso::Currency>, MoneyError>`

- Formula: `cost_avoided - virtual_ward_cost` (via `Money::sub`, called directly)
- Errors: Returns `MoneyError::CurrencyMismatch` if `cost_avoided` and `virtual_ward_cost` are not denominated in the same currency.
- Worked example: $150,000.00 − $60,000.00 = $90,000.00.

## Invariants

- The `Money` type is concrete (`Money<'static, iso::Currency>`), not
  generic over `rusty_money::FormattableCurrency` — see
  [`spec/README.md`](README.md)'s Money convention for why.

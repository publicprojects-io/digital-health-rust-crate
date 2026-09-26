# Spec: CPOE Cost Impact

- **Module**: [`src/cpoe_cost_impact.rs`](../src/cpoe_cost_impact.rs)
- **Status**: implemented
- **Upstream topic**: none — independent topic, not present in
  [digital-health-metrics](https://github.com/digital-health-metrics/digital-health-metrics)
  at the time this module was written; see the module's rustdoc `## Sources`
  section.

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc, and its `## Money convention` section for how this
file's contract differs from an `f64`-based one.

## Contract

### `transcription_cost_avoided(electronic_orders: u32, avoided_cost_per_order: Money<'static, iso::Currency>) -> Result<Money<'static, iso::Currency>, MoneyError>`

- Formula: `avoided_cost_per_order * electronic_orders` (via `Money::mul`,
  called directly)
- Returns `Err(MoneyError::Overflow)` only if the multiplication overflows.
- Worked example: `transcription_cost_avoided(9_400,
  Money::from_major(1, iso::USD)) == Ok(Money::from_major(9_400, iso::USD))`

### `verbal_order_review_cost(verbal_orders: u32, review_cost_per_order: Money<'static, iso::Currency>) -> Result<Money<'static, iso::Currency>, MoneyError>`

- Formula: `review_cost_per_order * verbal_orders` (via `Money::mul`, called
  directly)
- Returns `Err(MoneyError::Overflow)` only if the multiplication overflows.
- Worked example: `verbal_order_review_cost(500,
  Money::from_major(15, iso::USD)) == Ok(Money::from_major(7_500, iso::USD))`

## Invariants

- Unlike most of this crate's money-module pairs, these two functions are
  **not** complementary halves of one cohort at one rate — they take
  different per-order costs (`avoided_cost_per_order` vs.
  `review_cost_per_order`) for different order types
  (`electronic_orders` vs. `verbal_orders`, from
  [`cpoe_adoption_rate`](../src/cpoe_adoption_rate.rs)) and must not be
  netted against each other without stating explicitly that one is avoided
  cost and the other is incurred cost.
- The `Money` type is concrete (`Money<'static, iso::Currency>`), not
  generic over `rusty_money::FormattableCurrency` — see
  [`spec/README.md`](README.md)'s Money convention for why.

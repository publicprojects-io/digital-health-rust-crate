# Spec: No-Show Lost Revenue

- **Module**: [`src/no_show_lost_revenue.rs`](../src/no_show_lost_revenue.rs)
- **Status**: implemented
- **Upstream topic**: none — independent topic, not present in
  [digital-health-metrics](https://github.com/digital-health-metrics/digital-health-metrics)
  at the time this module was written; see the module's rustdoc `## Sources`
  section.

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc, and its `## Money convention` section for how this
file's contract differs from an `f64`-based one.

## Contract

### `lost_revenue<T: FormattableCurrency>(no_shows: u32, revenue_per_appointment: Money<'_, T>) -> Result<Money<'_, T>, MoneyError>`

- Formula: `revenue_per_appointment * no_shows` (via `Money::mul`)
- Returns `Err(MoneyError::Overflow)` only if the multiplication overflows;
  never returns a currency-mismatch error, since `no_shows` is a plain count.
- Worked example: `lost_revenue(180, Money::from_major(150, iso::USD)) ==
  Ok(Money::from_major(27_000, iso::USD))`

### `net_revenue_impact<T: FormattableCurrency>(gross_scheduled_revenue: Money<'_, T>, lost_revenue: Money<'_, T>) -> Result<Money<'_, T>, MoneyError>`

- Formula: `gross_scheduled_revenue - lost_revenue` (via `Money::sub`)
- Returns `Err(MoneyError::CurrencyMismatch { .. })` if the two arguments are
  denominated in different currencies.
- Worked example:
  `net_revenue_impact(Money::from_major(300_000, iso::USD),
  Money::from_major(27_000, iso::USD)) ==
  Ok(Money::from_major(273_000, iso::USD))`

## Invariants

- `no_shows` is a `u32` count, not an `f64` — a fractional no-show has no
  meaning, and `u32` converts to `rust_decimal::Decimal` exactly, with no
  precision-loss concern the way an `f64` count would carry.
- Neither function validates that `revenue_per_appointment` or
  `gross_scheduled_revenue` is positive; a negative amount (e.g. modelling a
  refund) is passed through unchanged.

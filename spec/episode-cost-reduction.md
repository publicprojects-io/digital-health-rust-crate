# Spec: Episode Cost Reduction

- **Module**: [`src/episode_cost_reduction.rs`](../src/episode_cost_reduction.rs)
- **Status**: implemented
- **Upstream topic**: none — independent topic, not present in
  [digital-health-metrics](https://github.com/digital-health-metrics/digital-health-metrics)
  at the time this module was written; see the module's rustdoc `## Sources`
  section.

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc, and its `## Money convention` section for how this
file's contract differs from an `f64`-based one.

## Contract

### `cost_per_episode(total_cost: Money<'static, iso::Currency>, episodes: u32) -> Result<Money<'static, iso::Currency>, MoneyError>`

- Formula: `total_cost / episodes` (via `Money::div`, called directly)
- Errors: Returns `MoneyError::DivisionByZero` if `episodes` is zero, or `MoneyError::Overflow` if the division overflows.
- Worked example: $1,800,000.00 / 2,000 = $900.00.

### `total_episode_savings(baseline_cost_per_episode: Money<'static, iso::Currency>, current_cost_per_episode: Money<'static, iso::Currency>, episodes: u32) -> Result<Money<'static, iso::Currency>, MoneyError>`

- Formula: `(baseline_cost_per_episode - current_cost_per_episode) * episodes` (via `Money::sub` then `Money::mul`, called directly)
- Errors: Returns `MoneyError::CurrencyMismatch` if the two costs are not denominated in the same currency, or `MoneyError::Overflow` if the multiplication overflows.
- Worked example: ($1,000.00 − $900.00) × 2,000 = $200,000.00.

## Invariants

- The `Money` type is concrete (`Money<'static, iso::Currency>`), not
  generic over `rusty_money::FormattableCurrency` — see
  [`spec/README.md`](README.md)'s Money convention for why.
- `cost_per_episode` divides by a count, so it is the second `Money` function that can return `MoneyError::DivisionByZero`, alongside `patient_acquisition_cost::cost_per_new_patient`.

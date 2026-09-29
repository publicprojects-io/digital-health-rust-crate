# Spec: ED Diversion Cost Avoidance

- **Module**: [`src/ed_diversion_cost_avoidance.rs`](../src/ed_diversion_cost_avoidance.rs)
- **Status**: implemented
- **Upstream topic**: none — independent topic, not present in
  [digital-health-metrics](https://github.com/digital-health-metrics/digital-health-metrics)
  at the time this module was written; see the module's rustdoc `## Sources`
  section.

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc, and its `## Money convention` section for how this
file's contract differs from an `f64`-based one.

## Contract

### `cost_avoided(safely_diverted_visits: u32, avoided_cost_per_visit: Money<'static, iso::Currency>) -> Result<Money<'static, iso::Currency>, MoneyError>`

- Formula: `avoided_cost_per_visit * safely_diverted_visits` (via `Money::mul`, called directly)
- Errors: Returns `MoneyError::Overflow` if the multiplication overflows `avoided_cost_per_visit`'s underlying decimal representation.
- Worked example: 1,150 × $800.00 = $920,000.00.

### `net_savings(cost_avoided: Money<'static, iso::Currency>, triage_service_cost: Money<'static, iso::Currency>) -> Result<Money<'static, iso::Currency>, MoneyError>`

- Formula: `cost_avoided - triage_service_cost` (via `Money::sub`, called directly)
- Errors: Returns `MoneyError::CurrencyMismatch` if `cost_avoided` and `triage_service_cost` are not denominated in the same currency.
- Worked example: $920,000.00 − $120,000.00 = $800,000.00.

## Invariants

- The `Money` type is concrete (`Money<'static, iso::Currency>`), not
  generic over `rusty_money::FormattableCurrency` — see
  [`spec/README.md`](README.md)'s Money convention for why.

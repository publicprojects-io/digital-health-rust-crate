# Spec: Return on Investment

- **Module**: [`src/return_on_investment.rs`](../src/return_on_investment.rs)
- **Status**: implemented
- **Upstream topic**: none — independent topic, not present in
  [digital-health-metrics](https://github.com/digital-health-metrics/digital-health-metrics)
  at the time this module was written; see the module's rustdoc `## Sources`
  section.

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `roi_percent(total_benefit: f64, total_cost: f64) -> Option<f64>`

- Formula: `(total_benefit - total_cost) / total_cost * 100`
- Returns `None` iff `total_cost == 0.0`
- Worked example: `roi_percent(450_000.0, 300_000.0) == Some(50.0)`

### `benefit_cost_ratio(total_benefit: f64, total_cost: f64) -> Option<f64>`

- Formula: `total_benefit / total_cost`
- Returns `None` iff `total_cost == 0.0`
- Worked example: `benefit_cost_ratio(450_000.0, 300_000.0) == Some(1.5)`

## Invariants

- Every function's result is a plain value derived directly from its arguments; no function chains into another.

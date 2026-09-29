# Spec: Patient Acquisition Efficiency

- **Module**: [`src/patient_acquisition_efficiency.rs`](../src/patient_acquisition_efficiency.rs)
- **Status**: implemented
- **Upstream topic**: none — independent topic, not present in
  [digital-health-metrics](https://github.com/digital-health-metrics/digital-health-metrics)
  at the time this module was written; see the module's rustdoc `## Sources`
  section.

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `ltv_to_cac_ratio(lifetime_value: f64, acquisition_cost: f64) -> Option<f64>`

- Formula: `lifetime_value / acquisition_cost`
- Returns `None` iff `acquisition_cost == 0.0`
- Worked example: `ltv_to_cac_ratio(1_500.0, 500.0) == Some(3.0)`

### `marketing_efficiency_ratio(total_revenue: f64, total_marketing_spend: f64) -> Option<f64>`

- Formula: `total_revenue / total_marketing_spend`
- Returns `None` iff `total_marketing_spend == 0.0`
- Worked example: `marketing_efficiency_ratio(900_000.0, 300_000.0) == Some(3.0)`

## Invariants

- Both functions return plain ratios, **not** percentages (`3.0` means 3:1).
- Arguments are plain `f64` currency amounts in one currency; the currency
  cancels in the ratio. For a currency-safe CAC build-up see
  [`spec/patient-acquisition-cost.md`](patient-acquisition-cost.md).

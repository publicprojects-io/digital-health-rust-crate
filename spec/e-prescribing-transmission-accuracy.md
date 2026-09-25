# Spec: E-Prescribing Transmission Accuracy

- **Module**: [`src/e_prescribing_transmission_accuracy.rs`](../src/e_prescribing_transmission_accuracy.rs)
- **Status**: implemented
- **Upstream topic**: none — independent topic, not present in
  [digital-health-metrics](https://github.com/digital-health-metrics/digital-health-metrics)
  at the time this module was written; see the module's rustdoc `## Sources`
  section.

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `first_pass_transmission_rate(clean_transmissions: f64, total_transmissions: f64) -> Option<f64>`

- Formula: `clean_transmissions / total_transmissions * 100`
- Returns `None` iff `total_transmissions == 0.0`
- Worked example: `first_pass_transmission_rate(4_850.0, 5_000.0) == Some(97.0)`

### `pharmacy_callback_rate(callbacks: f64, total_transmissions: f64) -> Option<f64>`

- Formula: `callbacks / total_transmissions * 100`
- Returns `None` iff `total_transmissions == 0.0`
- Worked example: `pharmacy_callback_rate(150.0, 5_000.0) == Some(3.0)`

## Invariants

- `first_pass_transmission_rate(a, d) + pharmacy_callback_rate(d - a, d) ==
  100.0` only when every transmission is classified into exactly one of
  "clean" or "call-back" — if a caller's data has other outcome categories
  (e.g. a transmission still pending review), the two rates are not
  complementary and must not be assumed to sum to 100.

# Spec: Interoperability Document Exchange Rate

- **Module**: [`src/interoperability_document_exchange_rate.rs`](../src/interoperability_document_exchange_rate.rs)
- **Status**: implemented
- **Upstream topic**: none — independent topic, not present in
  [digital-health-metrics](https://github.com/digital-health-metrics/digital-health-metrics)
  at the time this module was written; see the module's rustdoc `## Sources`
  section.

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `exchange_success_rate(successful_transmissions: f64, total_exchange_attempts: f64) -> Option<f64>`

- Formula: `successful_transmissions / total_exchange_attempts * 100`
- Returns `None` iff `total_exchange_attempts == 0.0`
- Worked example: `exchange_success_rate(47_500.0, 50_000.0) == Some(95.0)`

### `structured_data_parse_rate(successfully_parsed: f64, successful_transmissions: f64) -> Option<f64>`

- Formula: `successfully_parsed / successful_transmissions * 100`
- Returns `None` iff `successful_transmissions == 0.0`
- Worked example: `structured_data_parse_rate(38_000.0, 47_500.0) == Some(80.0)`

## Invariants

- `exchange_success_rate` and `structured_data_parse_rate` have different
  denominators (`total_exchange_attempts` vs. `successful_transmissions`) —
  they are not meant to be chained beyond `structured_data_parse_rate`'s
  denominator being drawn from `exchange_success_rate`'s numerator.

# Spec: Duplicate Patient Record Rate

- **Module**: [`src/duplicate_patient_record_rate.rs`](../src/duplicate_patient_record_rate.rs)
- **Status**: implemented
- **Upstream topic**: none — independent topic, not present in
  [digital-health-metrics](https://github.com/digital-health-metrics/digital-health-metrics)
  at the time this module was written; see the module's rustdoc `## Sources`
  section.

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `duplicate_record_rate(duplicate_records: f64, total_records: f64) -> Option<f64>`

- Formula: `duplicate_records / total_records * 100`
- Returns `None` iff `total_records == 0.0`
- Worked example: `duplicate_record_rate(42_000.0, 500_000.0) == Some(8.4)`

### `resolved_duplicate_rate(resolved_duplicates: f64, identified_duplicates: f64) -> Option<f64>`

- Formula: `resolved_duplicates / identified_duplicates * 100`
- Returns `None` iff `identified_duplicates == 0.0`
- Worked example: `resolved_duplicate_rate(31_500.0, 42_000.0) == Some(75.0)`

## Invariants

- `duplicate_record_rate` and `resolved_duplicate_rate` have different
  denominators (`total_records` vs. `identified_duplicates`) — they are not
  meant to be chained beyond `resolved_duplicate_rate`'s denominator being
  drawn from `duplicate_record_rate`'s numerator.

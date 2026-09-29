# Spec: ED Diversion Rate

- **Module**: [`src/ed_diversion_rate.rs`](../src/ed_diversion_rate.rs)
- **Status**: implemented
- **Upstream topic**: none — independent topic, not present in
  [digital-health-metrics](https://github.com/digital-health-metrics/digital-health-metrics)
  at the time this module was written; see the module's rustdoc `## Sources`
  section.

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `ed_diversion_rate(diverted_contacts: f64, triage_contacts: f64) -> Option<f64>`

- Formula: `diverted_contacts / triage_contacts * 100`
- Returns `None` iff `triage_contacts == 0.0`
- Worked example: `ed_diversion_rate(1_250.0, 5_000.0) == Some(25.0)`

### `safe_diversion_rate(diverted_without_return: f64, diverted_contacts: f64) -> Option<f64>`

- Formula: `diverted_without_return / diverted_contacts * 100`
- Returns `None` iff `diverted_contacts == 0.0`
- Worked example: `safe_diversion_rate(1_150.0, 1_250.0) == Some(92.0)`

## Invariants

- Every function's result is a plain value derived directly from its arguments; no function chains into another.

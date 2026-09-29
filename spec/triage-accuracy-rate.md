# Spec: Triage Accuracy Rate

- **Module**: [`src/triage_accuracy_rate.rs`](../src/triage_accuracy_rate.rs)
- **Status**: implemented
- **Upstream topic**: none — independent topic, not present in
  [digital-health-metrics](https://github.com/digital-health-metrics/digital-health-metrics)
  at the time this module was written; see the module's rustdoc `## Sources`
  section.

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `triage_accuracy_rate(matching_reference: f64, reviewed: f64) -> Option<f64>`

- Formula: `matching_reference / reviewed * 100`
- Returns `None` iff `reviewed == 0.0`
- Worked example: `triage_accuracy_rate(870.0, 1_000.0) == Some(87.0)`

### `under_triage_rate(under_triaged: f64, reviewed: f64) -> Option<f64>`

- Formula: `under_triaged / reviewed * 100`
- Returns `None` iff `reviewed == 0.0`
- Worked example: `under_triage_rate(60.0, 1_000.0) == Some(6.0)`

## Invariants

- Both functions share the `reviewed` denominator; `under_triage_rate`'s
  numerator is a subset of the decisions not counted by
  `triage_accuracy_rate`'s numerator.

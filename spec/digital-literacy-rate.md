# Spec: Digital Literacy Rate

- **Module**: [`src/digital_literacy_rate.rs`](../src/digital_literacy_rate.rs)
- **Status**: implemented
- **Upstream topic**: none — independent topic, not present in
  [digital-health-metrics](https://github.com/digital-health-metrics/digital-health-metrics)
  at the time this module was written; see the module's rustdoc `## Sources`
  section.

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `unassisted_completion_rate(completed_unassisted: f64, attempted: f64) -> Option<f64>`

- Formula: `completed_unassisted / attempted * 100`
- Returns `None` iff `attempted == 0.0`
- Worked example: `unassisted_completion_rate(640.0, 800.0) == Some(80.0)`

### `assistance_rate(needed_assistance: f64, attempted: f64) -> Option<f64>`

- Formula: `needed_assistance / attempted * 100`
- Returns `None` iff `attempted == 0.0`
- Worked example: `assistance_rate(120.0, 800.0) == Some(15.0)`

## Invariants

- Every function's result is a plain value derived directly from its arguments; no function chains into another.

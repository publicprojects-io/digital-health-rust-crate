# Spec: Telehealth Technical Failure Rate

- **Module**: [`src/telehealth_technical_failure_rate.rs`](../src/telehealth_technical_failure_rate.rs)
- **Status**: implemented
- **Upstream topic**: none — independent topic, not present in
  [digital-health-metrics](https://github.com/digital-health-metrics/digital-health-metrics)
  at the time this module was written; see the module's rustdoc `## Sources`
  section.

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `technical_failure_rate(failed_attempts: f64, total_attempts: f64) -> Option<f64>`

- Formula: `failed_attempts / total_attempts * 100`
- Returns `None` iff `total_attempts == 0.0`
- Worked example: `technical_failure_rate(35.0, 500.0) == Some(7.0)`

### `rebooking_rate(rebooked_after_failure: f64, failed_attempts: f64) -> Option<f64>`

- Formula: `rebooked_after_failure / failed_attempts * 100`
- Returns `None` iff `failed_attempts == 0.0`
- Worked example: `rebooking_rate(28.0, 35.0) == Some(80.0)`

## Invariants

- `technical_failure_rate` and `rebooking_rate` have different denominators
  (`total_attempts` vs. `failed_attempts`) — they are not meant to be
  chained arithmetically beyond `rebooking_rate`'s numerator being drawn
  from `technical_failure_rate`'s numerator.

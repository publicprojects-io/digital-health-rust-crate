# Spec: Engagement Consistency Rate

- **Module**: [`src/engagement_consistency_rate.rs`](../src/engagement_consistency_rate.rs)
- **Status**: implemented
- **Upstream topic**: none — independent topic, not present in
  [digital-health-metrics](https://github.com/digital-health-metrics/digital-health-metrics)
  at the time this module was written; see the module's rustdoc `## Sources`
  section.

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `active_week_rate(active_weeks: f64, enrolled_weeks: f64) -> Option<f64>`

- Formula: `active_weeks / enrolled_weeks * 100`
- Returns `None` iff `enrolled_weeks == 0.0`
- Worked example: `active_week_rate(312.0, 480.0) == Some(65.0)`

### `dropout_rate(dropouts: f64, enrolled: f64) -> Option<f64>`

- Formula: `dropouts / enrolled * 100`
- Returns `None` iff `enrolled == 0.0`
- Worked example: `dropout_rate(180.0, 1_200.0) == Some(15.0)`

## Invariants

- Every function's result is a plain value derived directly from its arguments; no function chains into another.

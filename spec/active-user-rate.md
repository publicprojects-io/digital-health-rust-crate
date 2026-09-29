# Spec: Active User Rate

- **Module**: [`src/active_user_rate.rs`](../src/active_user_rate.rs)
- **Status**: implemented
- **Upstream topic**: none — independent topic, not present in
  [digital-health-metrics](https://github.com/digital-health-metrics/digital-health-metrics)
  at the time this module was written; see the module's rustdoc `## Sources`
  section.

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `active_user_rate(active_users: f64, enrolled_users: f64) -> Option<f64>`

- Formula: `active_users / enrolled_users * 100`
- Returns `None` iff `enrolled_users == 0.0`
- Worked example: `active_user_rate(3_200.0, 8_000.0) == Some(40.0)`

### `stickiness_ratio(daily_active_users: f64, monthly_active_users: f64) -> Option<f64>`

- Formula: `daily_active_users / monthly_active_users * 100`
- Returns `None` iff `monthly_active_users == 0.0`
- Worked example: `stickiness_ratio(1_800.0, 6_000.0) == Some(30.0)`

## Invariants

- Every function's result is a plain value derived directly from its arguments; no function chains into another.

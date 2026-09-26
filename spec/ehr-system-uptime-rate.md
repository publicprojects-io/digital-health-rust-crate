# Spec: EHR System Uptime Rate

- **Module**: [`src/ehr_system_uptime_rate.rs`](../src/ehr_system_uptime_rate.rs)
- **Status**: implemented
- **Upstream topic**: none — independent topic, not present in
  [digital-health-metrics](https://github.com/digital-health-metrics/digital-health-metrics)
  at the time this module was written; see the module's rustdoc `## Sources`
  section.

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `uptime_rate(uptime_minutes: f64, total_minutes: f64) -> Option<f64>`

- Formula: `uptime_minutes / total_minutes * 100`
- Returns `None` iff `total_minutes == 0.0`
- Worked example: `uptime_rate(39_800.0, 40_000.0) == Some(99.5)`

### `unplanned_downtime_share(unplanned_downtime_minutes: f64, total_downtime_minutes: f64) -> Option<f64>`

- Formula: `unplanned_downtime_minutes / total_downtime_minutes * 100`
- Returns `None` iff `total_downtime_minutes == 0.0`
- Worked example: `unplanned_downtime_share(150.0, 200.0) == Some(75.0)`

## Invariants

- `uptime_rate` and `unplanned_downtime_share` have different denominators
  (`total_minutes` vs. `total_downtime_minutes`) — they are not meant to be
  chained; `total_downtime_minutes` is `total_minutes - uptime_minutes`,
  computed by the caller, not by either function.

# Spec: Clinical Alert Firing Rate

- **Module**: [`src/clinical_alert_firing_rate.rs`](../src/clinical_alert_firing_rate.rs)
- **Status**: implemented
- **Upstream topic**: none — independent topic, not present in
  [digital-health-metrics](https://github.com/digital-health-metrics/digital-health-metrics)
  at the time this module was written; see the module's rustdoc `## Sources`
  section.

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `alert_firing_rate(alerts_fired: f64, orders_placed: f64) -> Option<f64>`

- Formula: `alerts_fired / orders_placed * 100`
- Returns `None` iff `orders_placed == 0.0`
- Worked example: `alert_firing_rate(4_000.0, 10_000.0) == Some(40.0)`

### `high_severity_firing_share(high_severity_alerts: f64, alerts_fired: f64) -> Option<f64>`

- Formula: `high_severity_alerts / alerts_fired * 100`
- Returns `None` iff `alerts_fired == 0.0`
- Worked example: `high_severity_firing_share(500.0, 4_000.0) == Some(12.5)`

## Invariants

- `alert_firing_rate` and `high_severity_firing_share` have different
  denominators (`orders_placed` vs. `alerts_fired`) — they are not meant to
  be chained beyond `high_severity_firing_share`'s denominator being drawn
  from `alert_firing_rate`'s numerator.

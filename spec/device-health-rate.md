# Spec: Device Health Rate

- **Module**: [`src/device_health_rate.rs`](../src/device_health_rate.rs)
- **Status**: implemented
- **Upstream topic**: none — independent topic, not present in
  [digital-health-metrics](https://github.com/digital-health-metrics/digital-health-metrics)
  at the time this module was written; see the module's rustdoc `## Sources`
  section.

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `device_uptime_rate(uptime_hours: f64, expected_hours: f64) -> Option<f64>`

- Formula: `uptime_hours / expected_hours * 100`
- Returns `None` iff `expected_hours == 0.0`
- Worked example: `device_uptime_rate(2_138.4, 2_160.0) == Some(99.0)`

### `data_transmission_success_rate(received: f64, expected: f64) -> Option<f64>`

- Formula: `received / expected * 100`
- Returns `None` iff `expected == 0.0`
- Worked example: `data_transmission_success_rate(9_600.0, 10_000.0) == Some(96.0)`

### `resource_saturation_rate(samples_above_threshold: f64, samples: f64) -> Option<f64>`

- Formula: `samples_above_threshold / samples * 100`
- Returns `None` iff `samples == 0.0`
- Worked example: `resource_saturation_rate(180.0, 3_600.0) == Some(5.0)`

## Invariants

- Every function's result is a plain value derived directly from its arguments; no function chains into another.

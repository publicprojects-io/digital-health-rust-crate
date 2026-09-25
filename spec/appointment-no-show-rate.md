# Spec: Appointment No-Show Rate

- **Module**: [`src/appointment_no_show_rate.rs`](../src/appointment_no_show_rate.rs)
- **Status**: implemented
- **Upstream topic**: `digital-health-metrics/locales/en-gb-oxendict/topics/appointment-no-show-rate/index.md`

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `no_show_rate(no_shows: f64, total_scheduled: f64) -> Option<f64>`

- Formula: `no_shows / total_scheduled * 100`
- Returns `None` iff `total_scheduled == 0.0`
- Worked example: `no_show_rate(180.0, 2_000.0) == Some(9.0)`

### `late_cancellation_rate(late_cancellations: f64, total_scheduled: f64) -> Option<f64>`

- Formula: `late_cancellations / total_scheduled * 100`
- Returns `None` iff `total_scheduled == 0.0`
- Worked example: `late_cancellation_rate(60.0, 2_000.0) == Some(3.0)`

### `combined_non_attendance_rate(no_shows: f64, late_cancellations: f64, total_scheduled: f64) -> Option<f64>`

- Formula: `(no_shows + late_cancellations) / total_scheduled * 100`
- Returns `None` iff `total_scheduled == 0.0`
- Worked example: `combined_non_attendance_rate(180.0, 60.0, 2_000.0) == Some(12.0)`

## Invariants

- `total_scheduled` is expected to already exclude appointments cancelled
  with sufficient notice (see the module's rustdoc for the notice-period
  convention); this crate does not perform that exclusion itself.
- `combined_non_attendance_rate(a, b, d)` is always `>=` both
  `no_show_rate(a, d)` and `late_cancellation_rate(b, d)` for non-negative
  `a`, `b`.

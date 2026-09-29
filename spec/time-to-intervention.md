# Spec: Time to Intervention

- **Module**: [`src/time_to_intervention.rs`](../src/time_to_intervention.rs)
- **Status**: implemented
- **Upstream topic**: none — independent topic, not present in
  [digital-health-metrics](https://github.com/digital-health-metrics/digital-health-metrics)
  at the time this module was written; see the module's rustdoc `## Sources`
  section.

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `elapsed_minutes(alert_minute: f64, intervention_minute: f64) -> f64`

- Formula: `intervention_minute - alert_minute`
- Total function: never returns `None`; both arguments are timestamps in
  minutes on any consistent clock.
- Worked example: `elapsed_minutes(100.0, 118.0) == 18.0`

### `within_target_rate(within_target: f64, alerts: f64) -> Option<f64>`

- Formula: `within_target / alerts * 100`
- Returns `None` iff `alerts == 0.0`
- Worked example: `within_target_rate(720.0, 800.0) == Some(90.0)`

### `percentile(response_times: &[f64], p: f64) -> Option<f64>`

- Delegates to the crate-internal `internal::percentile` helper (shared with
  `digital_referral_turnaround_time::percentile`,
  `secure_messaging_response_time::percentile`, and
  `econsult_turnaround_time::percentile`): sorts a copy with
  `f64::total_cmp`, then linearly interpolates between the two order
  statistics bracketing rank `(p / 100).clamp(0, 1) * (len - 1)`.
- `p` is clamped to `[0, 100]`.
- Returns `None` iff `response_times` is empty.
- Worked example: `percentile(&[5.0, 10.0, 15.0, 20.0, 25.0], 50.0) == Some(15.0)`

## Invariants

- `percentile` never panics, on any `f64` input including `NaN`, `inf`, or
  an empty slice — see `spec/digital-referral-turnaround-time.md`'s
  invariants, which apply identically.

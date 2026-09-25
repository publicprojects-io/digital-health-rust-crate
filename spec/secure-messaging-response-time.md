# Spec: Secure Messaging Response Time

- **Module**: [`src/secure_messaging_response_time.rs`](../src/secure_messaging_response_time.rs)
- **Status**: implemented
- **Upstream topic**: none — independent topic, not present in
  [digital-health-metrics](https://github.com/digital-health-metrics/digital-health-metrics)
  at the time this module was written; see the module's rustdoc `## Sources`
  section.

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `response_hours(sent_hour: f64, replied_hour: f64) -> f64`

- Formula: `replied_hour - sent_hour`
- Total function: never returns `None`; both arguments are timestamps on
  any consistent clock (e.g. hours since an epoch).
- Worked example: `response_hours(9.0, 12.5) == 3.5`

### `percentile(response_times: &[f64], p: f64) -> Option<f64>`

- Delegates to the crate-internal `internal::percentile` helper (shared
  with `digital_referral_turnaround_time::percentile`): sorts a copy with
  `f64::total_cmp`, then linearly interpolates between the two order
  statistics bracketing rank `(p / 100).clamp(0, 1) * (len - 1)`.
- `p` is clamped to `[0, 100]`.
- Returns `None` iff `response_times` is empty.
- Worked examples, for
  `response_times = [0.5, 1.0, 1.5, 2.0, 3.0, 3.0, 5.0, 8.0, 20.0, 20.0]`:
  - `percentile(&response_times, 50.0) == Some(3.0)` (median)
  - `percentile(&response_times, 90.0) == Some(20.0)` (90th percentile)

## Invariants

- `percentile` never panics, on any `f64` input including `NaN`, `inf`, or
  an empty slice — see `spec/digital-referral-turnaround-time.md`'s
  invariants, which apply identically since both delegate to the same
  internal helper.

# Spec: E-Consult Turnaround Time

- **Module**: [`src/econsult_turnaround_time.rs`](../src/econsult_turnaround_time.rs)
- **Status**: implemented
- **Upstream topic**: none — independent topic, not present in
  [digital-health-metrics](https://github.com/digital-health-metrics/digital-health-metrics)
  at the time this module was written; see the module's rustdoc `## Sources`
  section.

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `elapsed_hours(submission_hour: f64, response_hour: f64) -> f64`

- Formula: `response_hour - submission_hour`
- Total function: never returns `None`; both arguments are timestamps on
  any consistent clock (e.g. hours since an epoch).
- Worked example: `elapsed_hours(100.0, 104.0) == 4.0`

### `percentile(turnaround_times: &[f64], p: f64) -> Option<f64>`

- Delegates to the crate-internal `internal::percentile` helper (shared with
  `digital_referral_turnaround_time::percentile` and
  `secure_messaging_response_time::percentile`): sorts a copy with
  `f64::total_cmp`, then linearly interpolates between the two order
  statistics bracketing rank `(p / 100).clamp(0, 1) * (len - 1)`.
- `p` is clamped to `[0, 100]`.
- Returns `None` iff `turnaround_times` is empty.
- Worked examples, for
  `turnaround_times = [1.0, 2.0, 3.0, 4.0, 5.0, 5.0, 8.0, 12.0, 48.0, 48.0]`:
  - `percentile(&turnaround_times, 50.0) == Some(5.0)` (median)
  - `percentile(&turnaround_times, 90.0) == Some(48.0)` (90th percentile)

## Invariants

- `percentile` never panics, on any `f64` input including `NaN`, `inf`, or
  an empty slice — see `spec/digital-referral-turnaround-time.md`'s
  invariants, which apply identically since both delegate to the same
  internal helper.

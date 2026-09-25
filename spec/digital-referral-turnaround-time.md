# Spec: Digital Referral Turnaround Time

- **Module**: [`src/digital_referral_turnaround_time.rs`](../src/digital_referral_turnaround_time.rs)
- **Status**: implemented
- **Upstream topic**: `digital-health-metrics/locales/en-gb-oxendict/topics/digital-referral-turnaround-time/index.md`

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `elapsed_days(submission_day: f64, decision_day: f64) -> f64`

- Formula: `decision_day - submission_day`
- Total function: never returns `None`, both arguments are timestamps on any
  consistent clock (e.g. days since an epoch).
- Worked example: `elapsed_days(10.0, 11.8) == 1.8`

### `hours_to_days(hours: f64) -> f64`

- Formula: `hours / 24.0`
- Total function.
- Worked example: `hours_to_days(4.0) == 4.0 / 24.0`

### `percentile(durations: &[f64], p: f64) -> Option<f64>`

- Sorts a copy of `durations` with `f64::total_cmp` (a total order over all
  `f64`, including NaN, so the sort itself cannot panic), then linearly
  interpolates between the two order statistics bracketing rank
  `(p / 100).clamp(0, 1) * (len - 1)`.
- `p` is clamped to `[0, 100]`; out-of-range values are not an error.
- Returns `None` iff `durations` is empty.
- Worked examples, for
  `durations = [0.3, 0.5, 0.8, 1.2, 1.8, 1.8, 2.5, 3.5, 6.0, 6.0]`:
  - `percentile(&durations, 50.0) == Some(1.8)` (median)
  - `percentile(&durations, 90.0) == Some(6.0)` (90th percentile)

## Invariants

- `percentile` never panics, on any `f64` input including `NaN`, `inf`, or
  an empty slice.
- `hours_to_days` composes with `percentile`: converting each duration with
  `hours_to_days` before calling `percentile` is equivalent to calling
  `percentile` first and converting the single result, since both operations
  are linear.

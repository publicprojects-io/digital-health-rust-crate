# Spec: Digital Intake Form Completion Rate

- **Module**: [`src/digital_intake_form_completion_rate.rs`](../src/digital_intake_form_completion_rate.rs)
- **Status**: implemented
- **Upstream topic**: none — independent topic, not present in
  [digital-health-metrics](https://github.com/digital-health-metrics/digital-health-metrics)
  at the time this module was written; see the module's rustdoc `## Sources`
  section.

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `intake_completion_rate(completed_online: f64, total_scheduled: f64) -> Option<f64>`

- Formula: `completed_online / total_scheduled * 100`
- Returns `None` iff `total_scheduled == 0.0`
- Worked example: `intake_completion_rate(1_200.0, 2_000.0) == Some(60.0)`

### `pre_visit_completion_rate(completed_before_arrival: f64, completed_online: f64) -> Option<f64>`

- Formula: `completed_before_arrival / completed_online * 100`
- Returns `None` iff `completed_online == 0.0`
- Worked example: `pre_visit_completion_rate(900.0, 1_200.0) == Some(75.0)`

## Invariants

- `intake_completion_rate` and `pre_visit_completion_rate` have different
  denominators (`total_scheduled` vs. `completed_online`) — they are not
  meant to be chained beyond `pre_visit_completion_rate`'s denominator being
  drawn from `intake_completion_rate`'s numerator.

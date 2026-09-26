# Spec: Patient Self-Scheduling Rate

- **Module**: [`src/patient_self_scheduling_rate.rs`](../src/patient_self_scheduling_rate.rs)
- **Status**: implemented
- **Upstream topic**: none — independent topic, not present in
  [digital-health-metrics](https://github.com/digital-health-metrics/digital-health-metrics)
  at the time this module was written; see the module's rustdoc `## Sources`
  section.

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `self_scheduling_rate(self_scheduled: f64, total_scheduled: f64) -> Option<f64>`

- Formula: `self_scheduled / total_scheduled * 100`
- Returns `None` iff `total_scheduled == 0.0`
- Worked example: `self_scheduling_rate(640.0, 2_000.0) == Some(32.0)`

### `same_day_self_scheduling_rate(same_day_self_scheduled: f64, self_scheduled: f64) -> Option<f64>`

- Formula: `same_day_self_scheduled / self_scheduled * 100`
- Returns `None` iff `self_scheduled == 0.0`
- Worked example: `same_day_self_scheduling_rate(192.0, 640.0) == Some(30.0)`

## Invariants

- `self_scheduling_rate` and `same_day_self_scheduling_rate` have different
  denominators (`total_scheduled` vs. `self_scheduled`) — they are not meant
  to be chained beyond `same_day_self_scheduling_rate`'s denominator being
  drawn from `self_scheduling_rate`'s numerator.

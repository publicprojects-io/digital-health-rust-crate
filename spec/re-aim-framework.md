# Spec: RE-AIM Framework

- **Module**: [`src/re_aim_framework.rs`](../src/re_aim_framework.rs)
- **Status**: implemented
- **Upstream topic**: none — independent topic, not present in
  [digital-health-metrics](https://github.com/digital-health-metrics/digital-health-metrics)
  at the time this module was written; see the module's rustdoc `## Sources`
  section.

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `reach_rate(participants: f64, eligible: f64) -> Option<f64>`

- Formula: `participants / eligible * 100`
- Returns `None` iff `eligible == 0.0`
- Worked example: `reach_rate(2_400.0, 6_000.0) == Some(40.0)`

### `adoption_rate(adopting_settings: f64, invited_settings: f64) -> Option<f64>`

- Formula: `adopting_settings / invited_settings * 100`
- Returns `None` iff `invited_settings == 0.0`
- Worked example: `adoption_rate(18.0, 30.0) == Some(60.0)`

### `implementation_fidelity_rate(delivered_as_intended: f64, planned: f64) -> Option<f64>`

- Formula: `delivered_as_intended / planned * 100`
- Returns `None` iff `planned == 0.0`
- Worked example: `implementation_fidelity_rate(1_350.0, 1_500.0) == Some(90.0)`

### `maintenance_rate(still_active: f64, initial_participants: f64) -> Option<f64>`

- Formula: `still_active / initial_participants * 100`
- Returns `None` iff `initial_participants == 0.0`
- Worked example: `maintenance_rate(1_080.0, 2_400.0) == Some(45.0)`

## Invariants

- Every function's result is a plain value derived directly from its arguments; no function chains into another.

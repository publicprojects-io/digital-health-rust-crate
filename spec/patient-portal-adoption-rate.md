# Spec: Patient Portal Adoption Rate

- **Module**: [`src/patient_portal_adoption_rate.rs`](../src/patient_portal_adoption_rate.rs)
- **Status**: implemented
- **Upstream topic**: `digital-health-metrics/locales/en-gb-oxendict/topics/patient-portal-adoption-rate/index.md`

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `registration_rate(registered: f64, eligible: f64) -> Option<f64>`

- Formula: `registered / eligible * 100`
- Returns `None` iff `eligible == 0.0`
- Worked example: `registration_rate(32_000.0, 50_000.0) == Some(64.0)`

### `activation_rate(activated: f64, registered: f64) -> Option<f64>`

- Formula: `activated / registered * 100`
- Returns `None` iff `registered == 0.0`
- Worked example: `activation_rate(27_000.0, 32_000.0) == Some(84.375)`

### `active_use_rate(active_in_window: f64, eligible: f64) -> Option<f64>`

- Formula: `active_in_window / eligible * 100`
- Returns `None` iff `eligible == 0.0`
- Worked example: `active_use_rate(21_000.0, 50_000.0) == Some(42.0)`

## Invariants

- The three functions form a funnel with two different denominators:
  `registration_rate` and `active_use_rate` are both over `eligible`, while
  `activation_rate` is over `registered` — callers must not assume all three
  share one denominator.
- `active_use_rate <= registration_rate` does not hold structurally (they're
  independent counts passed by the caller), but in the crate's worked
  example it does, and the module's rustdoc asserts it as a property of that
  example specifically, not as a general invariant of the functions.

# Spec: Patient Identity Verification Rate

- **Module**: [`src/patient_identity_verification_rate.rs`](../src/patient_identity_verification_rate.rs)
- **Status**: implemented
- **Upstream topic**: none — independent topic, not present in
  [digital-health-metrics](https://github.com/digital-health-metrics/digital-health-metrics)
  at the time this module was written; see the module's rustdoc `## Sources`
  section.

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `verification_success_rate(verified: f64, attempts: f64) -> Option<f64>`

- Formula: `verified / attempts * 100`
- Returns `None` iff `attempts == 0.0`
- Worked example: `verification_success_rate(4_250.0, 5_000.0) == Some(85.0)`

### `step_up_verification_rate(step_up_required: f64, attempts: f64) -> Option<f64>`

- Formula: `step_up_required / attempts * 100`
- Returns `None` iff `attempts == 0.0`
- Worked example: `step_up_verification_rate(750.0, 5_000.0) == Some(15.0)`

## Invariants

- `verification_success_rate` and `step_up_verification_rate` share the same
  denominator (`attempts`) but count different, possibly overlapping
  subsets (a step-up attempt can still succeed or fail) — the two are not
  complementary and must not be assumed to sum to 100%.

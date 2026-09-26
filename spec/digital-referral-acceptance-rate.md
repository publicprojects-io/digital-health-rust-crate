# Spec: Digital Referral Acceptance Rate

- **Module**: [`src/digital_referral_acceptance_rate.rs`](../src/digital_referral_acceptance_rate.rs)
- **Status**: implemented
- **Upstream topic**: none — independent topic, not present in
  [digital-health-metrics](https://github.com/digital-health-metrics/digital-health-metrics)
  at the time this module was written; see the module's rustdoc `## Sources`
  section.

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `acceptance_rate(accepted: f64, total_triaged: f64) -> Option<f64>`

- Formula: `accepted / total_triaged * 100`
- Returns `None` iff `total_triaged == 0.0`
- Worked example: `acceptance_rate(780.0, 1_000.0) == Some(78.0)`

### `returned_for_information_rate(returned: f64, total_triaged: f64) -> Option<f64>`

- Formula: `returned / total_triaged * 100`
- Returns `None` iff `total_triaged == 0.0`
- Worked example: `returned_for_information_rate(150.0, 1_000.0) == Some(15.0)`

## Invariants

- `acceptance_rate` and `returned_for_information_rate` share the same
  denominator (`total_triaged`) and count disjoint outcome categories along
  with a third, uncounted category (rejected as clinically inappropriate);
  the two functions' results sum to less than 100% whenever rejections
  exist, and callers must not assume they are complementary.

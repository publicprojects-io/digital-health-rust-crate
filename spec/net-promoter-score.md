# Spec: Net Promoter Score

- **Module**: [`src/net_promoter_score.rs`](../src/net_promoter_score.rs)
- **Status**: implemented
- **Upstream topic**: none — independent topic, not present in
  [digital-health-metrics](https://github.com/digital-health-metrics/digital-health-metrics)
  at the time this module was written; see the module's rustdoc `## Sources`
  section.

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `net_promoter_score(promoters: f64, detractors: f64, respondents: f64) -> Option<f64>`

- Formula: `(promoters - detractors) / respondents * 100`
- Returns `None` iff `respondents == 0.0`
- Worked example: `net_promoter_score(600.0, 150.0, 1_000.0) == Some(45.0)`

### `survey_response_rate(respondents: f64, invited: f64) -> Option<f64>`

- Formula: `respondents / invited * 100`
- Returns `None` iff `invited == 0.0`
- Worked example: `survey_response_rate(1_000.0, 4_000.0) == Some(25.0)`

## Invariants

- `net_promoter_score` is **not** bounded to `[0, 100]` like the other rate
  functions: it ranges from `-100.0` to `100.0` for valid inputs, and a
  negative result is meaningful.
- `respondents` includes passives, so `promoters + detractors <= respondents`.

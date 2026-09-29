# Spec: System Usability Scale

- **Module**: [`src/system_usability_scale.rs`](../src/system_usability_scale.rs)
- **Status**: implemented
- **Upstream topic**: none — independent topic, not present in
  [digital-health-metrics](https://github.com/digital-health-metrics/digital-health-metrics)
  at the time this module was written; see the module's rustdoc `## Sources`
  section.

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `sus_score(responses: [f64; 10]) -> f64`

- Formula: `sum(i even (0-based): responses[i] - 1, i odd: 5 - responses[i]) * 2.5`
  — i.e. odd-numbered items (1, 3, 5, 7, 9) contribute `response - 1`,
  even-numbered items (2, 4, 6, 8, 10) contribute `5 - response`.
- Total function: never returns `None`; there is no denominator. Responses
  are trusted to be 1–5 (no validation, per `AGENTS.md`).
- Worked example: `sus_score([5.0, 1.0, 5.0, 1.0, 4.0, 2.0, 5.0, 1.0, 4.0, 2.0]) == 90.0`

### `mean_sus_score(total_scores: f64, respondents: f64) -> Option<f64>`

- Formula: `total_scores / respondents`
- Returns `None` iff `respondents == 0.0`
- Worked example: `mean_sus_score(4_080.0, 60.0) == Some(68.0)`

## Invariants

- SUS is scored 0–100 but is **not** a percentage: about 68 is the commonly
  cited average across products.
- `mean_sus_score` takes the sum of already-scored respondents (each from
  `sus_score`), not raw responses.

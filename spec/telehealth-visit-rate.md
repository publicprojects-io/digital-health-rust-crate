# Spec: Telehealth Visit Rate

- **Module**: [`src/telehealth_visit_rate.rs`](../src/telehealth_visit_rate.rs)
- **Status**: implemented
- **Upstream topic**: `digital-health-metrics/locales/en-gb-oxendict/topics/telehealth-visit-rate/index.md`

See [`spec/README.md`](README.md) for what belongs in this file versus in
the module's rustdoc.

## Contract

### `telehealth_visit_rate(telehealth_encounters: f64, in_person_encounters: f64) -> Option<f64>`

- Formula: `telehealth_encounters / (telehealth_encounters + in_person_encounters) * 100`
- Returns `None` iff `telehealth_encounters + in_person_encounters == 0.0`
- Worked example: `telehealth_visit_rate(2_800.0, 1_200.0) == Some(70.0)`
  (2,800 = 1,600 video + 1,200 telephone)

### `video_rate(video_encounters: f64, total_encounters: f64) -> Option<f64>`

- Formula: `video_encounters / total_encounters * 100`
- Returns `None` iff `total_encounters == 0.0`
- Worked example: `video_rate(1_600.0, 4_000.0) == Some(40.0)`

### `telephone_rate(telephone_encounters: f64, total_encounters: f64) -> Option<f64>`

- Formula: `telephone_encounters / total_encounters * 100`
- Returns `None` iff `total_encounters == 0.0`
- Worked example: `telephone_rate(1_200.0, 4_000.0) == Some(30.0)`

## Invariants

- `telehealth_visit_rate`'s denominator is derived internally
  (`telehealth_encounters + in_person_encounters`), unlike `video_rate` and
  `telephone_rate`, which take the total as an explicit, separate argument —
  callers must pass `total_encounters` consistently with that sum for the
  three rates to be comparable.
- For a consistent set of counts, `video_rate + telephone_rate ==
  telehealth_visit_rate` when `total_encounters` in the first two calls
  equals `telehealth_encounters + in_person_encounters` in the third.

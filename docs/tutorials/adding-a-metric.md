# Adding a new metric module

This is the human-readable walkthrough of the same steps
[`AGENTS.md`](../../AGENTS.md) gives an AI agent. Follow it when adding an
seventeenth topic alongside the crate's existing sixteen.

Most topics are `f64`-based (see the earlier tutorials); a topic centred on
cost or revenue is `Money`-based instead, following
[`no_show_lost_revenue.rs`](../../src/no_show_lost_revenue.rs)'s pattern.
The steps below note where the two diverge.

## 1. Write the spec first

Add `spec/<topic-name>.md` before writing any code. Copy the shape of an
existing file — if the topic exists in
[digital-health-metrics](https://github.com/digital-health-metrics/digital-health-metrics),
copy e.g. [`spec/telehealth-visit-rate.md`](../../spec/telehealth-visit-rate.md);
if it doesn't, copy e.g.
[`spec/secure-messaging-response-time.md`](../../spec/secure-messaging-response-time.md),
which sets `Upstream topic: none — independent topic ...` and cites the
module's own `## Sources` instead:

- `Module`, `Status` (`implemented` once done, `planned` until then),
  `Upstream topic` (the path under digital-health-metrics, or `none —
  independent topic` with a pointer to the module's own sources).
- One `### function_signature(...)` subsection per function, each with its
  formula, its exact `None` condition, and at least one worked-example
  input/output pair.

Writing this first forces the formula and the zero-denominator condition to
be decided before any Rust code exists.

## 2. Implement the module

Create `src/<topic_name>.rs`. Every existing module follows the same shape;
copy the closest match (e.g. `telehealth_visit_rate.rs` for a rate with a
derived denominator, `digital_referral_turnaround_time.rs` for a
non-percentage metric):

- Module doc (`//!`): `# <Title>`, `## How it's calculated`, `## Why it
  matters`, `## Worked example` (a doctest reproducing the spec's example),
  `## Data sources and caveats`, `## Pitfalls`, `## Sources`, then a trailing
  `Topic doc:` line pointing at the upstream path (topics that exist
  upstream) or an `Independent topic:` line pointing back at `## Sources`
  (topics that don't).
- Each `pub fn`: `f64` in, `f64` or `Option<f64>` out, `#[must_use]`, and
  doc comments with `# Arguments`, `# Returns`, `# Examples` (a doctest). If
  the function sorts and interpolates over a slice (a percentile-style
  function), delegate to `crate::internal::percentile` rather than
  reimplementing it — see `secure_messaging_response_time::percentile` for
  the pattern.
- `#[cfg(test)] mod tests`: one test per worked-example value plus a
  `zero_denominator_returns_none` test (or an `empty_..._returns_none` test
  for a slice-based function).

## 3. Wire it in

- `src/lib.rs`: add `pub mod <topic_name>;` and an entry under the correct
  `## Module index` theme heading.
- `spec/README.md`: add a row to the "Files" table.
- `README.md`: add the module to "Module index by theme", and bump the
  module count in the opening paragraph.
- `llms.txt`: add it under "## Modules".
- `llms.json`: add a module entry with `name`, `path`, `spec`,
  `upstream_topic` (`true`/`false`), `summary`, and a `functions` array
  matching the shape of the existing entries.
- `digital-health-skill/SKILL.md`: add rows to the "Picking a function"
  table.

## 4. Verify

```sh
cargo test
cargo clippy --all-targets --all-features
cargo doc --no-deps
python3 -m json.tool llms.json > /dev/null   # confirm llms.json still parses
```

All four must be clean. `[lints]` in `Cargo.toml` denies both
`clippy::pedantic` and `missing_docs`, so a new module with an undocumented
item or a pedantic violation fails the build.

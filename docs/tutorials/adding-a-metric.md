# Adding a new metric module

This is the human-readable walkthrough of the same steps
[`AGENTS.md`](../../AGENTS.md) gives an AI agent. Follow it when adding a
sixth topic alongside the crate's existing five.

## 1. Write the spec first

Add `spec/<topic-name>.md` before writing any code. Copy the shape of an
existing file, e.g.
[`spec/telehealth-visit-rate.md`](../../spec/telehealth-visit-rate.md):

- `Module`, `Status` (`implemented` once done, `planned` until then),
  `Upstream topic` (the path under
  [digital-health-metrics](https://github.com/digital-health-metrics/digital-health-metrics)).
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
  `## Data sources and caveats`, `## Pitfalls`, `## Sources`, then a
  trailing `Topic doc:` line.
- Each `pub fn`: `f64` in, `f64` or `Option<f64>` out, `#[must_use]`, and
  doc comments with `# Arguments`, `# Returns`, `# Examples` (a doctest).
- `#[cfg(test)] mod tests`: one test per worked-example value plus a
  `zero_denominator_returns_none` test.

## 3. Wire it in

- `src/lib.rs`: add `pub mod <topic_name>;` and an entry under the correct
  `## Module index` theme heading.
- `README.md`: add the module to "Module index by theme".
- `llms.txt`: add it under "## Modules".
- `llms.json`: add a module entry with `name`, `path`, `spec`, `summary`,
  and a `functions` array matching the shape of the existing entries.
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

# AGENTS.md

Instructions for any AI coding agent working in this repository. See
[`CLAUDE.md`](CLAUDE.md) for Claude Code-specific supplements.

## What this crate is

`digital-health` is a Rust crate of pure calculation functions for digital
health KPIs — patient portal adoption, telehealth visit rate, appointment
no-show rate, clinical alert override rate, digital referral turnaround
time. One module per metric topic in `src/`, each mirroring a topic in the
upstream [Digital Health
Metrics](https://github.com/digital-health-metrics/digital-health-metrics)
project (see `Topic doc:` at the end of each module's rustdoc, and each
metric's `Upstream topic:` line under [`spec/`](spec/README.md)).

The crate is `std`-only with **zero external dependencies**, by design — do
not add a dependency without asking first.

## Build, test, lint

```sh
cargo test                                 # unit tests + doctests
cargo clippy --all-targets --all-features  # must be zero warnings — see below
cargo doc --no-deps                        # must be zero warnings
```

Run all three before considering any change to `src/` complete. `Cargo.toml`
sets `[lints.rust] missing_docs = "deny"` and `[lints.clippy] pedantic =
"deny"` — a clippy-pedantic or missing-docs violation fails the build, not
just a lint pass. If a pedantic lint is a false positive for a specific line,
scope an `#[allow(...)]` to that item with a one-line comment explaining why,
rather than weakening the crate-wide lint level.

## Spec-driven workflow

[`spec/`](spec/README.md) holds the calculation contract (formula, function
signature, `None` conditions) for every metric — the single source of truth
for "what the function computes," independent of the prose explaining why it
matters. A change to a formula, signature, or `None` condition must update,
together, in the same change:

1. The spec file under `spec/`.
2. The function in `src/`.
3. That function's rustdoc (`# Arguments` / `# Returns` / `# Examples`).
4. Its unit test(s), and the module doctest if the worked example changed.

A narrative-only edit (why a metric matters, its pitfalls, its data sources)
touches only the module's rustdoc and doesn't require touching `spec/`.

## Adding a new metric module

1. Add `spec/<topic-name>.md` (kebab-case), following the format of an
   existing spec file: `Module`, `Status`, `Upstream topic`, then a
   `## Contract` section per function with formula, `None` condition, and at
   least one worked example.
2. Add `src/<topic_name>.rs` (snake_case), following the shape every
   existing module uses:
   - Module-level rustdoc (`//!`) with `# <Title>`, `## How it's
     calculated`, `## Why it matters`, `## Worked example` (as a runnable
     doctest), `## Data sources and caveats`, `## Pitfalls`, `## Sources`,
     and a trailing `Topic doc:` line pointing at the upstream path.
   - Each `pub fn` takes and returns `f64`, uses `Option<f64>` if any
     argument is a denominator that can be zero, carries `#[must_use]`, and
     has rustdoc with `# Arguments`, `# Returns`, and a doctest under `#
     Examples` that reproduces the module's worked example.
   - A `#[cfg(test)] mod tests` block with one test per worked-example
     value (comment the test with the doc line it reproduces) and one
     `zero_denominator_returns_none` test.
3. Add `pub mod <topic_name>;` to `src/lib.rs`, and an entry in its `##
   Module index` doc section under the correct theme heading.
4. Update `README.md`'s module list, `llms.txt`, `llms.json`, and
   `digital-health-skill/SKILL.md` to include the new module.

## Code conventions

- No panics in any public function. Prefer a total order (`f64::total_cmp`)
  over `partial_cmp().unwrap()` when sorting floats, so NaN input can't
  panic — see `digital_referral_turnaround_time::percentile`.
- No input validation beyond the zero-denominator check. This crate does not
  verify that a numerator is non-negative or `<=` its denominator; it trusts
  the caller's counts. Do not add that validation without discussing it
  first — it's a deliberate scope boundary (see `spec/README.md`), not a gap.
- Comments in function bodies are rare on purpose: most functions are a
  one-line formula that the doctest above it already demonstrates. Add a
  body comment only for genuinely non-obvious logic (e.g. the interpolation
  arithmetic in `percentile`), not to restate the formula.
- Every `#[allow(clippy::...)]` needs a comment explaining why the lint
  doesn't apply, not just that it was silenced.

## Things not to do without asking first

- Add a dependency (the crate is intentionally zero-dependency).
- Change the `license` field in `Cargo.toml` or add a new license option.
- Change the repository URL (canonical: the `origin` git remote —
  currently `https://github.com/publicprojects-io/digital-health-rust-crate/`;
  keep `Cargo.toml`, `CITATION.cff`, and `README.md` in agreement with it).
- Scaffold or modify a `*.github.io` / SvelteKit / Lily Design System site —
  that's a separate repository, out of scope here.

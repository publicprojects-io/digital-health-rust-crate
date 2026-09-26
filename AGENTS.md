# AGENTS.md

Instructions for any AI coding agent working in this repository. See
[`CLAUDE.md`](CLAUDE.md) for Claude Code-specific supplements.

## What this crate is

`digital-health` is a Rust crate of pure calculation functions for digital
health KPIs — ten modules under `src/`, one per metric topic. Five mirror a
topic in the upstream [Digital Health
Metrics](https://github.com/digital-health-metrics/digital-health-metrics)
project directly (`patient_portal_adoption_rate`, `telehealth_visit_rate`,
`appointment_no_show_rate`, `clinical_alert_override_rate`,
`digital_referral_turnaround_time`; each ends its rustdoc with a `Topic
doc:` line to the upstream source). Five more cover well-evidenced metrics
not yet in that project (`remote_patient_monitoring_adherence_rate`,
`secure_messaging_response_time`, `e_prescribing_transmission_accuracy`,
`no_show_lost_revenue`, `remote_patient_monitoring_billing_revenue`; each
ends with an `Independent topic:` line pointing back at its own `## Sources`
section instead). Every module's `spec/` file states which kind it is on
its `Upstream topic:` line.

Most modules take and return `f64`, returning `Option<f64>` when a
denominator can be zero — see [`spec/README.md`](spec/README.md)'s
"Conventions used in every `f64` spec file". The two cost-and-revenue
modules (`no_show_lost_revenue`, `remote_patient_monitoring_billing_revenue`)
instead take `u32` counts and a concrete
[`rusty_money::Money<'static, iso::Currency>`](https://docs.rs/rusty-money)
amount, returning `Result<Money, rusty_money::MoneyError>` — see
[`spec/README.md`](spec/README.md)'s "Money convention". Two rules there
matter most: these functions use `rusty_money` directly (each is a one-line
call straight through to `Money::mul`/`Money::sub`, not a generic wrapper
type — do not reintroduce a `T: FormattableCurrency` type parameter), and
every worked example, doctest, and unit test uses `iso::USD` unless it's
specifically demonstrating a currency-mismatch error. `rusty-money` is this
crate's only dependency; adding a further one still needs to be raised with
the user first, it isn't now open-ended just because the first one was
approved.

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
signature, `None`/`Err` conditions) for every metric — the single source of
truth for "what the function computes," independent of the prose explaining
why it matters. A change to a formula, signature, or `None`/`Err` condition
must update, together, in the same change:

1. The spec file under `spec/`.
2. The function in `src/`.
3. That function's rustdoc (`# Arguments` / `# Returns` / `# Examples`).
4. Its unit test(s), and the module doctest if the worked example changed.

A narrative-only edit (why a metric matters, its pitfalls, its data sources)
touches only the module's rustdoc and doesn't require touching `spec/`.

## Adding a new metric module

Most new topics are `f64`-based; a topic centred on cost or revenue is
`Money`-based instead (see `spec/README.md`'s "Money convention"). Steps 1–2
below note where the two diverge.

1. Add `spec/<topic-name>.md` (kebab-case), following the format of an
   existing spec file: `Module`, `Status`, `Upstream topic` (the upstream
   path if the topic exists in digital-health-metrics, or `none —
   independent topic` pointing at the module's own `## Sources` if it
   doesn't — every `Money`-based topic is independent, since currency isn't
   part of digital-health-metrics), then a `## Contract` section per
   function with formula, `None`/`Err` condition, and at least one worked
   example.
2. Add `src/<topic_name>.rs` (snake_case), following the shape every
   existing module uses:
   - Module-level rustdoc (`//!`) with `# <Title>`, `## How it's
     calculated`, `## Why it matters`, `## Worked example` (as a runnable
     doctest), `## Data sources and caveats`, `## Pitfalls`, `## Sources`,
     and a trailing `Topic doc:` line (upstream topics) or `Independent
     topic:` line pointing back at `## Sources` (topics that aren't upstream).
   - `f64`-based: each `pub fn` takes and returns `f64`, uses `Option<f64>`
     if any argument is a denominator that can be zero, carries
     `#[must_use]`. A percentile-style function delegates to
     `crate::internal::percentile` rather than reimplementing the
     interpolation.
   - `Money`-based: each `pub fn` takes counts as `u32` (never `f64` — a
     fractional count has no meaning) and a concrete
     `rusty_money::Money<'static, iso::Currency>` amount, returning
     `Result<Money<'static, iso::Currency>, rusty_money::MoneyError>`. Call
     `rusty_money`'s own arithmetic (`Money::mul`, `Money::sub`, ...)
     directly in the function body — don't add a `T: FormattableCurrency`
     generic parameter, that's wrapping the library rather than using it.
     Don't add `#[must_use]`: `Result` already carries it, and clippy's
     pedantic `must_use_candidate` skips types that already do.
   - Either way: rustdoc with `# Arguments`, `# Returns` (`f64`-based) or
     `# Errors` (`Money`-based, naming which `MoneyError` variants apply),
     and a doctest under `# Examples` that reproduces the module's worked
     example.
   - A `#[cfg(test)] mod tests` block with one test per worked-example
     value (comment the test with the doc line it reproduces) and one
     `zero_denominator_returns_none` / `empty_..._returns_none` (`f64`-based)
     or `currency_mismatch_returns_err` (`Money`-based) test.
3. Add `pub mod <topic_name>;` to `src/lib.rs`, and an entry in its `##
   Module index` doc section under the correct theme heading (add a `Cost
   and revenue` heading if this is the first `Money`-based topic there
   isn't already one for).
4. Update `spec/README.md`'s table (including its `Numeric type` column),
   `README.md`'s module list and module count, `llms.txt`, `llms.json`
   (including its `upstream_topic` field), and `digital-health-skill/SKILL.md`
   to include the new module.

## Code conventions

- No panics in any public function. Prefer a total order (`f64::total_cmp`)
  over `partial_cmp().unwrap()` when sorting floats, so NaN input can't
  panic — see the shared `internal::percentile` helper, used by both
  `digital_referral_turnaround_time::percentile` and
  `secure_messaging_response_time::percentile`.
- No input validation beyond the zero-denominator check (`f64`-based) or what
  `rusty_money::Money`'s own arithmetic already enforces (`Money`-based).
  This crate does not verify that a numerator is non-negative or `<=` its
  denominator, nor that a `Money` amount is positive. It trusts the caller's
  counts. Do not add that validation without discussing it first — it's a
  deliberate scope boundary (see `spec/README.md`), not a gap.
- Comments in function bodies are rare on purpose: most functions are a
  one-line formula that the doctest above it already demonstrates. Add a
  body comment only for genuinely non-obvious logic (e.g. the interpolation
  arithmetic in `percentile`), not to restate the formula.
- Every `#[allow(clippy::...)]` needs a comment explaining why the lint
  doesn't apply, not just that it was silenced.

## Things not to do without asking first

- Add a dependency beyond `rusty-money` (the crate is otherwise
  dependency-free by design).
- Change the `license` field in `Cargo.toml` or add a new license option.
- Change the repository URL (canonical: the `origin` git remote —
  currently `https://github.com/publicprojects-io/digital-health-rust-crate/`;
  keep `Cargo.toml`, `CITATION.cff`, and `README.md` in agreement with it).
- Scaffold or modify a `*.github.io` / SvelteKit / Lily Design System site —
  that's a separate repository, out of scope here.

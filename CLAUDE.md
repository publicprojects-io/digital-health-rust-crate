# CLAUDE.md

Claude Code-specific supplement. [`AGENTS.md`](AGENTS.md) is the primary,
tool-agnostic guide — read it first: crate overview, build/test/lint
commands, the spec-driven change workflow, how to add a new metric module,
code conventions, and things not to change without asking.

## Skill

[`digital-health-skill/SKILL.md`](digital-health-skill/SKILL.md) explains
how to pick the right function for a metrics question and how to read
`Option<f64>::None` in this crate's functions (always "zero denominator,"
never an error condition). Load it when a task asks you to compute, explain,
or write code against one of this crate's metrics.

## Session checklist for any change under `src/`

1. Read the relevant `spec/<topic>.md` file before touching a formula or
   signature — it's the contract, not the rustdoc prose.
2. Make the change.
3. Run, in order: `cargo test`, `cargo clippy --all-targets --all-features`,
   `cargo doc --no-deps`. All three must be clean (`[lints]` in
   `Cargo.toml` denies pedantic clippy and missing docs, so a warning is a
   failure here, not a suggestion).
4. If a formula, signature, or `None`/`Err` condition changed, confirm
   `spec/`, the rustdoc, and the tests all agree with each other and with
   the worked-example numbers in `llms.json`.

## AI-discovery files to keep in sync

If you add, remove, or rename a public function or module, update all of:
`src/lib.rs` (module index doc), `spec/README.md` (file table), `README.md`,
`llms.txt`, `llms.json`, `digital-health-skill/SKILL.md`. These are
redundant with each other by
design — different consumers (a human reading GitHub, an LLM crawling
`llms.txt`, an agent loading the skill) read different ones, so accuracy
matters in each independently rather than in just one canonical place.

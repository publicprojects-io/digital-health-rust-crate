# Digital Health Rust crate

Digital health metrics models, structs, calculations, and examples — 13
modules covering patient engagement and access, digital care operations and
safety, and cost and revenue. One module per topic. Five modules follow
[Digital Health
Metrics](https://github.com/digital-health-metrics/digital-health-metrics)
directly; eight more cover well-evidenced metrics not yet in that project,
sourced independently (see each module's rustdoc `Sources` section).

Most modules are rate calculations: `f64` in, `Option<f64>` out, `None`
wherever a denominator can be zero. The three cost-and-revenue modules
instead use [`rusty-money`](https://docs.rs/rusty-money) for currency-safe
arithmetic: they take counts and a `Money` amount, and return
`Result<Money, MoneyError>` rather than a plain `f64`, so a currency
mismatch or overflow is a typed error instead of a silent rounding bug.

## Install

Add the dependency to your `Cargo.toml`:

```toml
[dependencies]
digital-health = "0.3"
```

## Quickstart

The portal-adoption funnel and the no-show rate it helps drive down — two
metrics that show up in almost every digital health business case:

```rust
use digital_health::patient_portal_adoption_rate as portal;
use digital_health::appointment_no_show_rate as no_show;

// A primary care network's portal funnel: registration, activation, active use.
let registration = portal::registration_rate(32_000.0, 50_000.0).unwrap();
let active_use = portal::active_use_rate(21_000.0, 50_000.0).unwrap();
assert!((registration - 64.0).abs() < 1e-9);
assert!((active_use - 42.0).abs() < 1e-9);

// Reporting registration alone would considerably overstate real engagement.
assert!(active_use < registration);

// Portal-based self-scheduling and reminders are a primary lever on no-shows.
let rate = no_show::no_show_rate(180.0, 2_000.0).unwrap();
assert!((rate - 9.0).abs() < 1e-9);
```

Putting a currency amount on that same no-show rate, at an illustrative
$150.00 per completed appointment:

```rust
use rusty_money::{Money, iso};
use digital_health::no_show_lost_revenue::lost_revenue;

// 180 no-shows × $150.00 = $27,000.00.
let revenue_per_appointment = Money::from_major(150, iso::USD);
let lost = lost_revenue(180, revenue_per_appointment).unwrap();
assert_eq!(lost, Money::from_major(27_000, iso::USD));
```

## Learning path

Each module's rustdoc explains its topic — what the metric is, how it's
calculated, why it matters, and a worked example that runs as a doctest.
Start with `cargo doc --open`, or with
[`docs/tutorials/getting-started.md`](docs/tutorials/getting-started.md) for
a narrated walkthrough of two worked examples.

New here? Start with `patient_portal_adoption_rate` and
`appointment_no_show_rate` — two metrics that show up in almost every
digital health business case.

## Module index by theme

### Patient engagement and access

- `patient_portal_adoption_rate` — registration, activation, and active use;
  three different rates too often conflated as one
- `telehealth_visit_rate` — the share of care delivered remotely, and why
  video and telephone should never be reported as one number
- `appointment_no_show_rate` — the oldest operational metric in healthcare,
  and one of the best-evidenced targets for digital reminders
- `remote_patient_monitoring_adherence_rate` — the metric that decides
  whether a monitoring period is even billable, not just how engaged a
  patient is
- `telehealth_technical_failure_rate` — the metric behind
  `telehealth_visit_rate`'s own warning against counting attempted rather
  than completed visits

### Digital care operations and safety

- `clinical_alert_override_rate` — the standard signal for alert fatigue in
  clinical decision support
- `digital_referral_turnaround_time` — the process metric that shows whether
  an e-referral system is actually saving time
- `secure_messaging_response_time` — patient-portal in-basket response time,
  a well-documented driver of clinician burnout
- `e_prescribing_transmission_accuracy` — the pharmacy call-back rate that
  catches errors a clean network transmission can't
- `cpoe_adoption_rate` — the precondition for `clinical_alert_override_rate`
  to mean anything, and the verbal-order rate hiding beneath a healthy
  override rate

### Cost and revenue (`Money`, not `f64`)

- `no_show_lost_revenue` — the currency amount behind
  `appointment_no_show_rate`, for the business case a reminders programme is
  justified against
- `remote_patient_monitoring_billing_revenue` — the currency amount behind
  `remote_patient_monitoring_adherence_rate`'s cohort billing-eligible rate
- `telehealth_cost_avoidance` — the currency amount behind
  `telehealth_visit_rate`, netted against the platform cost of running it

## Testing

Every module reproduces its topic's worked example in unit tests, and every
doc example compiles and asserts under `cargo test --doc`:

```sh
cargo test
cargo clippy --all-targets --all-features  # denies clippy::pedantic
cargo doc --no-deps                        # denies missing_docs
```

## For contributors and AI agents

- [`spec/`](spec/README.md) — the single-source calculation contract
  (formula, signature, `None`/`Err` condition) for every function,
  independent of the narrative rustdoc.
- [`AGENTS.md`](AGENTS.md) — build/test/lint commands, the spec-driven
  change workflow, and conventions for modifying this crate.
- [`CLAUDE.md`](CLAUDE.md) — Claude Code-specific supplement to `AGENTS.md`.
- [`digital-health-skill/SKILL.md`](digital-health-skill/SKILL.md) — how an
  AI agent should pick a function and interpret its `Option<f64>` or
  `Result<Money, MoneyError>` result.
- [`llms.txt`](llms.txt) / [`llms.json`](llms.json) — prose and structured
  machine-readable summaries of the crate for LLM tooling.
- [`docs/tutorials/`](docs/tutorials/) — a getting-started walkthrough and a
  guide to adding a new metric module.

## Citation

See [`CITATION.cff`](CITATION.cff) for citation metadata.

## License

Any of MIT, Apache-2.0, BSD-3-Clause, GPL-2.0-only, or GPL-3.0-only, at your
option — or contact us for custom license options. See
[`LICENSE.md`](LICENSE.md).

## Tracking

- Package: [digital-health](https://crates.io/crates/digital-health)
- Repository: [github.com/publicprojects-io/digital-health-rust-crate](https://github.com/publicprojects-io/digital-health-rust-crate)
- Source of truth for metric definitions: [digital-health-metrics](https://github.com/digital-health-metrics/digital-health-metrics)
- Source of truth for this crate's calculation contracts: [`spec/`](spec/README.md)
- Author: [Joel Parker Henderson](https://joelparkerhenderson.com) — joel@joelparkerhenderson.com

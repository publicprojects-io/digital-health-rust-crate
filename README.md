# Digital Health Rust crate

Digital health metrics models, structs, calculations, and examples — 55
modules covering patient engagement and access, digital care operations and
safety, and cost and revenue. One module per topic. Five modules follow
[Digital Health
Metrics](https://github.com/digital-health-metrics/digital-health-metrics)
directly; fifty more cover well-evidenced metrics not yet in that
project, sourced independently (see each module's rustdoc `Sources`
section).

Most modules are rate calculations: `f64` in, `Option<f64>` out, `None`
wherever a denominator can be zero. The thirteen cost-and-revenue modules
instead use [`rusty-money`](https://docs.rs/rusty-money) for currency-safe
arithmetic: they take counts and a `Money` amount, and return
`Result<Money, MoneyError>` rather than a plain `f64`, so a currency
mismatch or overflow is a typed error instead of a silent rounding bug.

## Install

Add the dependency to your `Cargo.toml`:

```toml
[dependencies]
digital-health = "1.0"
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
- `patient_self_scheduling_rate` — the natural continuation of
  `patient_portal_adoption_rate`'s funnel, and one of the levers
  `appointment_no_show_rate` itself points to
- `digital_intake_form_completion_rate` — the digital front door before the
  portal-adoption funnel even begins
- `prom_completion_rate` — outcome data that's only trustworthy if the
  completing population is representative of everyone treated
- `digital_therapeutic_retention_rate` — day-7 versus day-30 retention, the
  standard early-attrition checkpoints in digital therapeutics
- `patient_identity_verification_rate` — the gate before
  `patient_portal_adoption_rate`'s funnel starts at all
- `net_promoter_score` — the single-number satisfaction measure for
  telehealth and app experience, and the response rate that qualifies it
- `digital_access_rate` — the equity precondition for everything in this
  theme: who can reach a digital service at all, and how unevenly
- `patient_acquisition_efficiency` — whether growth spending pays back, via
  LTV-to-CAC and marketing efficiency ratios
- `engagement_consistency_rate` — how steadily patients keep using a tool,
  and the dropout that ends it
- `digital_literacy_rate` — whether patients can actually use a service they
  have access to, measured by unassisted completion
- `medication_adherence_rate` — proportion of days covered and the share of
  patients reaching the 80% adherence threshold
- `active_user_rate` — who shows up, and how often: active users and DAU/MAU
  stickiness
- `long_term_retention_rate` — day-60 and day-90 retention, where clinical
  benefit is decided
- `system_usability_scale` — the standard 10-item usability score for patients
  and providers

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
- `duplicate_patient_record_rate` — the silent failure mode behind
  interoperability, and a direct patient-safety risk in its own right
- `econsult_turnaround_time` — the asynchronous-specialist-advice
  counterpart to `digital_referral_turnaround_time`
- `medication_reconciliation_rate` — one of the highest-yield digital
  medication-safety checks in this crate
- `interoperability_document_exchange_rate` — the gap between "the document
  arrived" and "the data became usable"
- `clinical_alert_firing_rate` — the other half of the alert-fatigue picture
  from `clinical_alert_override_rate`
- `ehr_system_uptime_rate` — the availability every other metric in this
  crate implicitly assumes
- `digital_referral_acceptance_rate` — what triage actually decides, not
  just how fast it decides it
- `triage_accuracy_rate` — whether a digital triage tool routes patients to
  the right care level, and the under-triage that carries the safety risk
- `readmission_rate` — the outcome payers accept as proof a digital
  programme improved care, and its relative reduction
- `clinician_documentation_time` — the paperwork burden behind clinician
  burnout, per encounter and as a reduction
- `biometric_improvement` — the clinical proof point: relative fall in
  HbA1c or BMI, target attainment, and estimated A1c from mean glucose
- `device_health_rate` — device uptime, data transmission success, and
  resource saturation behind every monitoring programme
- `biometric_stabilization` — blood pressure control and glucose time in range
  from connected devices
- `virtual_ward_bed_days` — the hospital bed days saved by moving recovery
  into a virtual ward
- `time_to_intervention` — how fast a clinical team acts on an automated
  alert, not just that it fired
- `ed_diversion_rate` — avoided emergency department visits, and how many were
  safely avoided
- `re_aim_framework` — Reach, Adoption, Implementation and Maintenance from
  the RE-AIM evaluation framework

### Cost and revenue (`Money`, not `f64`)

- `no_show_lost_revenue` — the currency amount behind
  `appointment_no_show_rate`, for the business case a reminders programme is
  justified against
- `remote_patient_monitoring_billing_revenue` — the currency amount behind
  `remote_patient_monitoring_adherence_rate`'s cohort billing-eligible rate
- `telehealth_cost_avoidance` — the currency amount behind
  `telehealth_visit_rate`, netted against the platform cost of running it
- `patient_self_scheduling_cost_savings` — the currency amount behind
  `patient_self_scheduling_rate`, netted against the scheduling platform's
  own cost
- `econsult_cost_avoidance` — the currency amount behind
  `econsult_turnaround_time`'s avoided in-person referrals
- `digital_intake_cost_savings` — the currency amount behind
  `digital_intake_form_completion_rate`'s avoided manual data entry
- `duplicate_record_remediation_cost` — the currency amount behind
  `duplicate_patient_record_rate`'s resolved-versus-backlog split
- `cpoe_cost_impact` — the staffing-cost case behind `cpoe_adoption_rate`'s
  safety case
- `patient_acquisition_cost` — the fully loaded cost of acquiring a
  patient, the denominator `patient_acquisition_efficiency` needs
- `bed_day_cost_avoidance` — the currency amount behind
  `virtual_ward_bed_days`, netted against the virtual ward's cost
- `ed_diversion_cost_avoidance` — the currency amount behind
  `ed_diversion_rate`'s safely avoided visits
- `episode_cost_reduction` — cost per episode of care against a baseline
  cohort, the unit value-based contracts use
- `platform_investment_cost` — the fully loaded licensing, hardware and
  staffing cost `return_on_investment` needs
- `return_on_investment` — ROI and benefit-cost ratio, the payback question
  every budget holder asks

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

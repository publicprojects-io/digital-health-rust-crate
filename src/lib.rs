//! # Digital Health
//!
//! Rust implementations of the metrics from [Digital Health
//! Metrics](https://github.com/digital-health-metrics/digital-health-metrics)
//! — one module per topic, covering patient engagement and access, digital
//! care operations and safety, and cost and revenue.
//!
//! Most modules are rate calculations: `f64` in, `Option<f64>` out (`None`
//! wherever a denominator can be zero), with no dependencies beyond `std`.
//! The cost-and-revenue modules instead take counts and a
//! [`rusty_money::Money`] amount, and return
//! `Result<Money, rusty_money::MoneyError>` — money arithmetic uses
//! [`rusty_money`](https://docs.rs/rusty-money) rather than `f64`, so that a
//! currency mismatch or overflow is a typed error instead of a silent
//! rounding bug.
//!
//! ## Quickstart
//!
//! New to the crate? Start with the portal-adoption funnel and the no-show
//! rate it helps drive down — two metrics that show up in almost every
//! digital health business case:
//!
//! ```
//! use digital_health::patient_portal_adoption_rate as portal;
//! use digital_health::appointment_no_show_rate as no_show;
//!
//! // A primary care network's portal funnel: registration, activation, active use.
//! let registration = portal::registration_rate(32_000.0, 50_000.0).unwrap();
//! let active_use = portal::active_use_rate(21_000.0, 50_000.0).unwrap();
//! assert!((registration - 64.0).abs() < 1e-9);
//! assert!((active_use - 42.0).abs() < 1e-9);
//!
//! // Reporting registration alone would considerably overstate real engagement.
//! assert!(active_use < registration);
//!
//! // Portal-based self-scheduling and reminders are a primary lever on no-shows.
//! let rate = no_show::no_show_rate(180.0, 2_000.0).unwrap();
//! assert!((rate - 9.0).abs() < 1e-9);
//! ```
//!
//! ## Module index
//!
//! Five modules mirror a topic in [Digital Health
//! Metrics](https://github.com/digital-health-metrics/digital-health-metrics)
//! directly (each ends its rustdoc with a `Topic doc:` line to the upstream
//! source); twenty-five more cover well-evidenced metrics not yet in that
//! project (each ends with an `Independent topic:` line instead, and cites
//! its own sources).
//!
//! ### Patient engagement and access
//!
//! - [`patient_portal_adoption_rate`] — registration, activation, and active
//!   use; three different rates too often conflated as one
//! - [`telehealth_visit_rate`] — the share of care delivered remotely, and
//!   why video and telephone should never be reported as one number
//! - [`appointment_no_show_rate`] — the oldest operational metric in
//!   healthcare, and one of the best-evidenced targets for digital reminders
//! - [`remote_patient_monitoring_adherence_rate`] — the metric that decides
//!   whether a monitoring period is even billable, not just how engaged a
//!   patient is
//! - [`telehealth_technical_failure_rate`] — the metric behind [`telehealth_visit_rate`]'s
//!   own warning against counting attempted rather than completed visits
//! - [`patient_self_scheduling_rate`] — the natural continuation of
//!   [`patient_portal_adoption_rate`]'s funnel, and one of the levers
//!   [`appointment_no_show_rate`] itself points to
//! - [`digital_intake_form_completion_rate`] — the digital front door before
//!   the portal-adoption funnel even begins
//! - [`prom_completion_rate`] — outcome data that's only trustworthy if the
//!   completing population is representative of everyone treated
//! - [`digital_therapeutic_retention_rate`] — day-7 versus day-30 retention,
//!   the standard early-attrition checkpoints in digital therapeutics
//! - [`patient_identity_verification_rate`] — the gate before
//!   [`patient_portal_adoption_rate`]'s funnel starts at all
//!
//! ### Digital care operations and safety
//!
//! - [`clinical_alert_override_rate`] — the standard signal for alert
//!   fatigue in clinical decision support
//! - [`digital_referral_turnaround_time`] — the process metric that shows
//!   whether an e-referral system is actually saving time
//! - [`secure_messaging_response_time`] — patient-portal in-basket response
//!   time, a well-documented driver of clinician burnout
//! - [`e_prescribing_transmission_accuracy`] — the pharmacy call-back rate
//!   that catches errors a clean network transmission can't
//! - [`cpoe_adoption_rate`] — the precondition for
//!   [`clinical_alert_override_rate`] to mean anything, and the verbal-order
//!   rate that hides beneath a healthy override rate
//! - [`duplicate_patient_record_rate`] — the silent failure mode behind
//!   interoperability, and a direct patient-safety risk in its own right
//! - [`econsult_turnaround_time`] — the asynchronous-specialist-advice
//!   counterpart to [`digital_referral_turnaround_time`]
//! - [`medication_reconciliation_rate`] — one of the highest-yield digital
//!   medication-safety checks in this crate
//! - [`interoperability_document_exchange_rate`] — the gap between "the
//!   document arrived" and "the data became usable"
//! - [`clinical_alert_firing_rate`] — the other half of the alert-fatigue
//!   picture from [`clinical_alert_override_rate`]
//! - [`ehr_system_uptime_rate`] — the availability every other metric in
//!   this crate implicitly assumes
//! - [`digital_referral_acceptance_rate`] — what triage actually decides,
//!   not just how fast it decides it
//!
//! ### Cost and revenue
//!
//! - [`no_show_lost_revenue`] — the currency amount behind
//!   [`appointment_no_show_rate`], for the business case a reminders
//!   programme is justified against
//! - [`remote_patient_monitoring_billing_revenue`] — the currency amount
//!   behind [`remote_patient_monitoring_adherence_rate`]'s cohort
//!   billing-eligible rate
//! - [`telehealth_cost_avoidance`] — the currency amount behind
//!   [`telehealth_visit_rate`], netted against the platform cost of running
//!   it
//! - [`patient_self_scheduling_cost_savings`] — the currency amount behind
//!   [`patient_self_scheduling_rate`], netted against the scheduling
//!   platform's own cost
//! - [`econsult_cost_avoidance`] — the currency amount behind
//!   [`econsult_turnaround_time`]'s avoided in-person referrals
//! - [`digital_intake_cost_savings`] — the currency amount behind
//!   [`digital_intake_form_completion_rate`]'s avoided manual data entry
//! - [`duplicate_record_remediation_cost`] — the currency amount behind
//!   [`duplicate_patient_record_rate`]'s resolved-versus-backlog split
//! - [`cpoe_cost_impact`] — the staffing-cost case behind
//!   [`cpoe_adoption_rate`]'s safety case
//!
//! ## Testing
//!
//! Every module reproduces its topic's worked example in unit tests, and
//! every doc example compiles and asserts under `cargo test --doc`.

pub mod appointment_no_show_rate;
pub mod clinical_alert_firing_rate;
pub mod clinical_alert_override_rate;
pub mod cpoe_adoption_rate;
pub mod cpoe_cost_impact;
pub mod digital_intake_cost_savings;
pub mod digital_intake_form_completion_rate;
pub mod digital_referral_acceptance_rate;
pub mod digital_referral_turnaround_time;
pub mod digital_therapeutic_retention_rate;
pub mod duplicate_patient_record_rate;
pub mod duplicate_record_remediation_cost;
pub mod e_prescribing_transmission_accuracy;
pub mod econsult_cost_avoidance;
pub mod econsult_turnaround_time;
pub mod ehr_system_uptime_rate;
mod internal;
pub mod interoperability_document_exchange_rate;
pub mod medication_reconciliation_rate;
pub mod no_show_lost_revenue;
pub mod patient_identity_verification_rate;
pub mod patient_portal_adoption_rate;
pub mod patient_self_scheduling_cost_savings;
pub mod patient_self_scheduling_rate;
pub mod prom_completion_rate;
pub mod remote_patient_monitoring_adherence_rate;
pub mod remote_patient_monitoring_billing_revenue;
pub mod secure_messaging_response_time;
pub mod telehealth_cost_avoidance;
pub mod telehealth_technical_failure_rate;
pub mod telehealth_visit_rate;

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
//! source); five more cover well-evidenced metrics not yet in that project
//! (each ends with an `Independent topic:` line instead, and cites its own
//! sources).
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
//!
//! ### Cost and revenue
//!
//! - [`no_show_lost_revenue`] — the currency amount behind
//!   [`appointment_no_show_rate`], for the business case a reminders
//!   programme is justified against
//! - [`remote_patient_monitoring_billing_revenue`] — the currency amount
//!   behind [`remote_patient_monitoring_adherence_rate`]'s cohort
//!   billing-eligible rate
//!
//! ## Testing
//!
//! Every module reproduces its topic's worked example in unit tests, and
//! every doc example compiles and asserts under `cargo test --doc`.

pub mod appointment_no_show_rate;
pub mod clinical_alert_override_rate;
pub mod digital_referral_turnaround_time;
pub mod e_prescribing_transmission_accuracy;
mod internal;
pub mod no_show_lost_revenue;
pub mod patient_portal_adoption_rate;
pub mod remote_patient_monitoring_adherence_rate;
pub mod remote_patient_monitoring_billing_revenue;
pub mod secure_messaging_response_time;
pub mod telehealth_visit_rate;

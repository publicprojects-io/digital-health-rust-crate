//! # Digital Health
//!
//! Rust implementations of the metrics from [Digital Health
//! Metrics](https://github.com/digital-health-metrics/digital-health-metrics)
//! — one module per topic, covering patient engagement, access, and digital
//! care operations and safety.
//!
//! The crate is `std`-only with **zero external dependencies**. All
//! quantities are `f64`, and functions return `Option<f64>` wherever a
//! denominator can be zero.
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
//! ### Patient engagement and access
//!
//! - [`patient_portal_adoption_rate`] — registration, activation, and active
//!   use; three different rates too often conflated as one
//! - [`telehealth_visit_rate`] — the share of care delivered remotely, and
//!   why video and telephone should never be reported as one number
//! - [`appointment_no_show_rate`] — the oldest operational metric in
//!   healthcare, and one of the best-evidenced targets for digital reminders
//!
//! ### Digital care operations and safety
//!
//! - [`clinical_alert_override_rate`] — the standard signal for alert
//!   fatigue in clinical decision support
//! - [`digital_referral_turnaround_time`] — the process metric that shows
//!   whether an e-referral system is actually saving time
//!
//! ## Testing
//!
//! Every module reproduces its topic's worked example in unit tests, and
//! every doc example compiles and asserts under `cargo test --doc`.

pub mod appointment_no_show_rate;
pub mod clinical_alert_override_rate;
pub mod digital_referral_turnaround_time;
pub mod patient_portal_adoption_rate;
pub mod telehealth_visit_rate;

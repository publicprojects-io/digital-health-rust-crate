---
name: digital-health-metrics
description: Compute digital health KPIs — patient portal adoption, telehealth visit rate, appointment no-show rate, clinical alert override rate, digital referral turnaround time, and their cost/revenue counterparts — using the `digital-health` Rust crate's pure calculation functions. Use when asked to calculate one of these metrics from raw counts, to explain what one means, or to write Rust code against this crate.
---

# Digital Health Metrics

`digital-health` (this repository) is a Rust crate: one module per metric,
each a small set of pure functions. Most take and return `f64`; eight
cost-and-revenue modules take `u32` counts and a
[`rusty_money::Money`](https://docs.rs/rusty-money) amount instead (see
"Reading the result" below for how the two differ). Full contracts live in
[`spec/`](../spec/README.md); the machine-readable form is
[`llms.json`](../llms.json). This file is the quick-reference for picking
the right function and interpreting its result.

## Picking a function

| Question being asked | Function | Module |
| --- | --- | --- |
| Share of eligible patients registered for the portal | `registration_rate` | `patient_portal_adoption_rate` |
| Share of registered patients who took a first action | `activation_rate` | `patient_portal_adoption_rate` |
| Share of eligible patients who logged in recently | `active_use_rate` | `patient_portal_adoption_rate` |
| Share of encounters delivered remotely (video + phone) | `telehealth_visit_rate` | `telehealth_visit_rate` |
| Share of encounters by video specifically | `video_rate` | `telehealth_visit_rate` |
| Share of encounters by telephone specifically | `telephone_rate` | `telehealth_visit_rate` |
| True no-show rate | `no_show_rate` | `appointment_no_show_rate` |
| Late-cancellation rate | `late_cancellation_rate` | `appointment_no_show_rate` |
| No-shows and late cancellations combined | `combined_non_attendance_rate` | `appointment_no_show_rate` |
| Share of monitoring days with a qualifying reading | `monitoring_adherence_rate` | `remote_patient_monitoring_adherence_rate` |
| Share of a cohort meeting the RPM billing-day threshold | `billing_threshold_met_rate` | `remote_patient_monitoring_adherence_rate` |
| Share of CDS alerts overridden | `override_rate` | `clinical_alert_override_rate` |
| Share of overrides with a documented reason | `documented_override_rate` | `clinical_alert_override_rate` |
| Time from referral submission to triage decision | `elapsed_days` | `digital_referral_turnaround_time` |
| Converting an hours figure to days for comparison | `hours_to_days` | `digital_referral_turnaround_time` |
| Median or Nth-percentile turnaround time over many referrals | `percentile` | `digital_referral_turnaround_time` |
| Time from a patient portal message to a clinician's reply | `response_hours` | `secure_messaging_response_time` |
| Median or Nth-percentile message response time | `percentile` | `secure_messaging_response_time` |
| Share of e-prescriptions filled with no pharmacy call-back | `first_pass_transmission_rate` | `e_prescribing_transmission_accuracy` |
| Share of e-prescriptions that generated a pharmacy call-back | `pharmacy_callback_rate` | `e_prescribing_transmission_accuracy` |
| Share of orders placed electronically via CPOE | `electronic_order_rate` | `cpoe_adoption_rate` |
| Share of orders given verbally, bypassing CPOE checks | `verbal_order_rate` | `cpoe_adoption_rate` |
| Share of telehealth attempts that fail to connect | `technical_failure_rate` | `telehealth_technical_failure_rate` |
| Share of failed attempts successfully rebooked | `rebooking_rate` | `telehealth_technical_failure_rate` |
| Currency amount lost to no-shows | `lost_revenue` | `no_show_lost_revenue` |
| Gross scheduled revenue minus no-show losses | `net_revenue_impact` | `no_show_lost_revenue` |
| Currency amount billable for an RPM cohort | `billable_revenue` | `remote_patient_monitoring_billing_revenue` |
| Currency amount at risk from a non-adherent RPM cohort | `revenue_at_risk` | `remote_patient_monitoring_billing_revenue` |
| Currency amount of travel/facility cost avoided via telehealth | `cost_avoided` | `telehealth_cost_avoidance` |
| Cost avoided minus telehealth platform's own operating cost | `net_savings` | `telehealth_cost_avoidance` |
| Share of appointments booked by the patient online | `self_scheduling_rate` | `patient_self_scheduling_rate` |
| Share of self-scheduled visits booked same-day/next-day | `same_day_self_scheduling_rate` | `patient_self_scheduling_rate` |
| Share of a patient index identified as duplicate records | `duplicate_record_rate` | `duplicate_patient_record_rate` |
| Share of identified duplicates that have been merged | `resolved_duplicate_rate` | `duplicate_patient_record_rate` |
| Currency amount of staff time saved by self-scheduling | `staff_time_saved_cost` | `patient_self_scheduling_cost_savings` |
| Staff time saved minus scheduling platform's own operating cost | `net_savings` | `patient_self_scheduling_cost_savings` |
| Share of patients completing intake forms online | `intake_completion_rate` | `digital_intake_form_completion_rate` |
| Share of online intake forms completed before arrival | `pre_visit_completion_rate` | `digital_intake_form_completion_rate` |
| Time from e-consult submission to specialist response | `elapsed_hours` | `econsult_turnaround_time` |
| Median or Nth-percentile e-consult response time | `percentile` | `econsult_turnaround_time` |
| Share of admissions with completed medication reconciliation | `admission_reconciliation_rate` | `medication_reconciliation_rate` |
| Share of discharges with completed medication reconciliation | `discharge_reconciliation_rate` | `medication_reconciliation_rate` |
| Share of document exchanges that transmit successfully | `exchange_success_rate` | `interoperability_document_exchange_rate` |
| Share of transmitted documents that parse into usable data | `structured_data_parse_rate` | `interoperability_document_exchange_rate` |
| Share of eligible patients completing a PROM | `prom_completion_rate` | `prom_completion_rate` |
| Share of PROMs completed via a digital channel | `digital_completion_share` | `prom_completion_rate` |
| Digital therapeutic retention at day 7 | `day_7_retention_rate` | `digital_therapeutic_retention_rate` |
| Digital therapeutic retention at day 30 | `day_30_retention_rate` | `digital_therapeutic_retention_rate` |
| Share of orders that trigger a CDS alert | `alert_firing_rate` | `clinical_alert_firing_rate` |
| Share of fired alerts that are high severity | `high_severity_firing_share` | `clinical_alert_firing_rate` |
| Share of a measurement period a system is available | `uptime_rate` | `ehr_system_uptime_rate` |
| Share of downtime that was unplanned | `unplanned_downtime_share` | `ehr_system_uptime_rate` |
| Share of identity verification attempts that succeed | `verification_success_rate` | `patient_identity_verification_rate` |
| Share of attempts requiring step-up verification | `step_up_verification_rate` | `patient_identity_verification_rate` |
| Share of triaged referrals accepted and booked | `acceptance_rate` | `digital_referral_acceptance_rate` |
| Share of referrals returned for missing information | `returned_for_information_rate` | `digital_referral_acceptance_rate` |
| Currency amount avoided by e-consults replacing referrals | `cost_avoided` | `econsult_cost_avoidance` |
| E-consult savings minus platform's own operating cost | `net_savings` | `econsult_cost_avoidance` |
| Currency amount of staff time saved by online intake | `staff_time_saved_cost` | `digital_intake_cost_savings` |
| Intake savings minus platform's own operating cost | `net_savings` | `digital_intake_cost_savings` |
| Currency amount already spent resolving duplicate records | `remediation_cost` | `duplicate_record_remediation_cost` |
| Currency amount of outstanding duplicate-record backlog | `backlog_cost` | `duplicate_record_remediation_cost` |
| Currency amount of transcription cost avoided by CPOE | `transcription_cost_avoided` | `cpoe_cost_impact` |
| Currency amount of verbal-order review cost incurred | `verbal_order_review_cost` | `cpoe_cost_impact` |

## Reading the result

Every rate function returns `Option<f64>` as a **percentage in `[0, 100]`
for valid inputs** — not a fraction. `None` means exactly one thing in this
crate: the denominator argument was `0.0`. It is never an error condition to
surface as a bug; treat it as "rate is undefined for zero appointments (or
zero eligible patients, or zero alerts fired, etc.)" and handle it
accordingly, e.g. by omitting that row from a report rather than showing
`0%`.

```rust
use digital_health::appointment_no_show_rate::no_show_rate;

match no_show_rate(180.0, 2_000.0) {
    Some(rate) => println!("no-show rate: {rate:.1}%"),
    None => println!("no-show rate: undefined (no appointments scheduled)"),
}
```

The eight cost-and-revenue modules (`no_show_lost_revenue`,
`remote_patient_monitoring_billing_revenue`, `telehealth_cost_avoidance`,
`patient_self_scheduling_cost_savings`, `econsult_cost_avoidance`,
`digital_intake_cost_savings`, `duplicate_record_remediation_cost`,
`cpoe_cost_impact`) instead return
`Result<Money<'static, iso::Currency>, rusty_money::MoneyError>` — each is a
one-line call straight through to `rusty_money`'s own `Money::mul` or
`Money::sub`, not a wrapper type. `Err` is not a zero-denominator condition
here — there's no division — it's either a `CurrencyMismatch` (only possible
when subtracting two caller-supplied `Money` values, e.g.
`net_revenue_impact` or a module's `net_savings`) or an `Overflow`. Each
function's rustdoc `# Errors` section says which apply.

```rust
use rusty_money::{Money, iso};
use digital_health::no_show_lost_revenue::lost_revenue;

let revenue_per_appointment = Money::from_major(150, iso::USD);
match lost_revenue(180, revenue_per_appointment) {
    Ok(lost) => println!("lost revenue: {lost}"),
    Err(e) => println!("could not compute lost revenue: {e}"),
}
```

## Before computing a rate

- Check which count is the denominator — several modules have functions
  that look similar but divide by different totals (e.g.
  `patient_portal_adoption_rate::registration_rate` and `active_use_rate`
  divide by `eligible`, but `activation_rate` divides by `registered`; see
  the table above and each function's `spec/` entry).
- `clinical_alert_override_rate::override_rate` should usually be called
  once per severity tier or alert type, not once with a single blended
  total — a blended figure conflates well-justified and unsafe overrides.
- `e_prescribing_transmission_accuracy`'s `first_pass_transmission_rate` and
  `pharmacy_callback_rate` only sum to 100% when every transmission is
  classified into exactly one of those two buckets; don't assume that if
  the caller's data has other outcome categories.
- This crate does not validate that a numerator is non-negative or `<=` its
  denominator, nor that a `Money` amount is positive; it trusts the
  caller's counts. Validate upstream if the data source might not guarantee
  that.
- Every reimbursement, cost, or per-unit figure in a money-based module
  (`reimbursement_per_patient`, `revenue_per_appointment`,
  `avoided_cost_per_encounter`, `platform_cost`,
  `avoided_cost_per_booking`, ...) is always caller-supplied — this crate
  never hard-codes a specific reimbursement, cost, or CPT/HCPCS rate, since
  those change annually and vary by payer, locality, and service line. Look
  the current figure up rather than reusing a worked example's illustrative
  number.
- A platform that only logs a telehealth session's final successful
  connection makes `telehealth_technical_failure_rate` unmeasurable —
  failed attempts must be retained as distinct events, not silently
  retried away, before this metric can be calculated at all.
- `duplicate_patient_record_rate`'s `duplicate_record_rate` is not
  comparable across organizations with different patient-matching
  algorithms — a stricter probabilistic matcher surfaces more candidate
  duplicates than a looser deterministic one, independent of true
  underlying data quality.
- `interoperability_document_exchange_rate`'s `exchange_success_rate` is a
  transport-layer measure only — a document can transmit successfully and
  still fail `structured_data_parse_rate`; don't infer usability from
  transmission success.
- `prom_completion_rate` alone doesn't establish that PROM data is
  trustworthy — a high completion rate can still be skewed toward one
  channel or population; check representativeness, not just the rate.
- `cpoe_cost_impact`'s `transcription_cost_avoided` and
  `verbal_order_review_cost` are not complementary and must not be netted
  against each other — one is avoided cost, the other is incurred cost, for
  different order types.
- Most of this crate's modules are not present in [Digital Health
  Metrics](https://github.com/digital-health-metrics/digital-health-metrics)
  and are sourced independently from general literature and standards
  instead; see [`spec/README.md`](../spec/README.md)'s file table (`Upstream
  topic` column) for exactly which, and each such module's rustdoc `##
  Sources` section for what it's sourced from.

## Further reading

- [`spec/`](../spec/README.md) — exact formula and `None` condition per
  function
- [`AGENTS.md`](../AGENTS.md) — conventions for modifying this crate
- [`docs/tutorials/getting-started.md`](../docs/tutorials/getting-started.md)
  — a worked walkthrough

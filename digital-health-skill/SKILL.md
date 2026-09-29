---
name: digital-health-metrics
description: Compute digital health KPIs — patient portal adoption, telehealth visit rate, appointment no-show rate, clinical alert override rate, digital referral turnaround time, and their cost/revenue counterparts — using the `digital-health` Rust crate's pure calculation functions. Use when asked to calculate one of these metrics from raw counts, to explain what one means, or to write Rust code against this crate.
---

# Digital Health Metrics

`digital-health` (this repository) is a Rust crate: one module per metric,
each a small set of pure functions. Most take and return `f64`; thirteen
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
| Net Promoter Score from promoter, detractor and respondent counts | `net_promoter_score` | `net_promoter_score` |
| Share of invited patients who answered the survey | `survey_response_rate` | `net_promoter_score` |
| Share of a population with connectivity or device access | `digital_access_rate` | `digital_access_rate` |
| A subgroup's access relative to a reference group | `access_parity_ratio` | `digital_access_rate` |
| Lifetime value relative to acquisition cost | `ltv_to_cac_ratio` | `patient_acquisition_efficiency` |
| Total revenue relative to total marketing spend | `marketing_efficiency_ratio` | `patient_acquisition_efficiency` |
| Share of triage decisions matching a clinician reference | `triage_accuracy_rate` | `triage_accuracy_rate` |
| Share of triage decisions routed below the needed care level | `under_triage_rate` | `triage_accuracy_rate` |
| 30-day readmission rate | `readmission_rate` | `readmission_rate` |
| Relative fall in readmission rate against a baseline | `readmission_reduction` | `readmission_rate` |
| Clinician EHR documentation minutes per encounter | `documentation_minutes_per_encounter` | `clinician_documentation_time` |
| Relative fall in documentation time against a baseline | `documentation_time_reduction` | `clinician_documentation_time` |
| Currency amount of fully loaded patient acquisition cost | `total_acquisition_cost` | `patient_acquisition_cost` |
| Acquisition cost per new patient | `cost_per_new_patient` | `patient_acquisition_cost` |
| The estimated A1c (eA1c) implied by a mean glucose reading, using the ADAG regression | `estimated_a1c` | `biometric_improvement` |
| The biometric reduction: the relative fall from a baseline value to a current one, as a percentage of the baseline | `biometric_reduction` | `biometric_improvement` |
| The target attainment rate: patients at their clinical target as a percentage of patients measured | `target_attainment_rate` | `biometric_improvement` |
| The proportion of days covered (PDC): days a patient had medication on hand as a percentage of days in the measurement period | `proportion_of_days_covered` | `medication_adherence_rate` |
| The adherent patient rate: patients whose PDC meets the adherence threshold, as a percentage of patients | `adherent_patient_rate` | `medication_adherence_rate` |
| The unassisted completion rate: tasks completed without help as a percentage of tasks attempted | `unassisted_completion_rate` | `digital_literacy_rate` |
| The assistance rate: tasks needing staff or family help as a percentage of tasks attempted | `assistance_rate` | `digital_literacy_rate` |
| The device uptime rate: hours a device was operating as a percentage of hours it was expected to operate | `device_uptime_rate` | `device_health_rate` |
| The data transmission success rate: transmissions received as a percentage of transmissions expected | `data_transmission_success_rate` | `device_health_rate` |
| The resource saturation rate: samples with CPU or memory above a threshold as a percentage of all samples | `resource_saturation_rate` | `device_health_rate` |
| The active week rate: enrolled weeks with meaningful activity as a percentage of all enrolled weeks | `active_week_rate` | `engagement_consistency_rate` |
| The dropout rate: patients who stopped engaging as a percentage of patients enrolled | `dropout_rate` | `engagement_consistency_rate` |
| The blood pressure control rate: patients with controlled blood pressure as a percentage of patients monitored | `blood_pressure_control_rate` | `biometric_stabilization` |
| The time in range: readings within the target range as a percentage of readings taken | `time_in_range` | `biometric_stabilization` |
| The bed days saved: patients multiplied by the conventional stay minus the actual hospital stay | `bed_days_saved` | `virtual_ward_bed_days` |
| The bed-day reduction rate: the relative fall from baseline bed days to current bed days, as a percentage of the baseline | `bed_day_reduction_rate` | `virtual_ward_bed_days` |
| The active user rate: users active in the period as a percentage of enrolled users | `active_user_rate` | `active_user_rate` |
| The stickiness ratio: average daily active users as a percentage of monthly active users | `stickiness_ratio` | `active_user_rate` |
| The day-60 retention rate: cohort users active at day 60 as a percentage of the cohort | `day_60_retention_rate` | `long_term_retention_rate` |
| The day-90 retention rate: cohort users active at day 90 as a percentage of the cohort | `day_90_retention_rate` | `long_term_retention_rate` |
| The ED diversion rate: contacts that avoided an ED visit as a percentage of triage contacts | `ed_diversion_rate` | `ed_diversion_rate` |
| The safe diversion rate: diverted contacts with no ED return in the follow-up window, as a percentage of diverted contacts | `safe_diversion_rate` | `ed_diversion_rate` |
| The return on investment: net benefit as a percentage of total cost | `roi_percent` | `return_on_investment` |
| The benefit-cost ratio: total benefit divided by total cost | `benefit_cost_ratio` | `return_on_investment` |
| The reach rate: participants as a percentage of the eligible population | `reach_rate` | `re_aim_framework` |
| The adoption rate: settings that adopted the intervention as a percentage of settings invited | `adoption_rate` | `re_aim_framework` |
| The implementation fidelity rate: components delivered as intended as a percentage of components planned | `implementation_fidelity_rate` | `re_aim_framework` |
| The maintenance rate: participants still active at the follow-up point as a percentage of initial participants | `maintenance_rate` | `re_aim_framework` |
| The cost avoided: bed days saved multiplied by the marginal cost of one bed day | `bed_day_cost_avoided` | `bed_day_cost_avoidance` |
| The net savings: cost avoided minus the virtual ward's own operating cost for the period | `net_savings` | `bed_day_cost_avoidance` |
| The cost avoided: safely diverted ED visits multiplied by the avoided cost of one visit | `cost_avoided` | `ed_diversion_cost_avoidance` |
| The net savings: cost avoided minus the triage service's own operating cost for the period | `net_savings` | `ed_diversion_cost_avoidance` |
| The cost per episode of care: total cost divided by the number of episodes | `cost_per_episode` | `episode_cost_reduction` |
| The total episode savings: the per-episode cost reduction against baseline, multiplied by the number of episodes | `total_episode_savings` | `episode_cost_reduction` |
| The total platform cost: licensing, hardware, and staffing costs summed | `total_platform_cost` | `platform_investment_cost` |
| The net benefit: total quantified benefit minus total platform cost | `net_benefit` | `platform_investment_cost` |
| Minutes from an automated alert to a clinician's first action | `elapsed_minutes` | `time_to_intervention` |
| Share of alerts answered within the target time | `within_target_rate` | `time_to_intervention` |
| Median or Nth-percentile time to intervention | `percentile` | `time_to_intervention` |
| SUS score for one respondent's ten answers | `sus_score` | `system_usability_scale` |
| Mean SUS score across respondents | `mean_sus_score` | `system_usability_scale` |

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

The thirteen cost-and-revenue modules (`no_show_lost_revenue`,
`remote_patient_monitoring_billing_revenue`, `telehealth_cost_avoidance`,
`patient_self_scheduling_cost_savings`, `econsult_cost_avoidance`,
`digital_intake_cost_savings`, `duplicate_record_remediation_cost`,
`cpoe_cost_impact`, `patient_acquisition_cost`, `bed_day_cost_avoidance`,
`ed_diversion_cost_avoidance`, `episode_cost_reduction`,
`platform_investment_cost`) instead return
`Result<Money<'static, iso::Currency>, rusty_money::MoneyError>` — each is a
one-line call straight through to `rusty_money`'s own `Money::mul`,
`Money::sub`, or `Money::add`, not a wrapper type. `Err` is not a
zero-denominator condition here — with one exception, below — it's either a `CurrencyMismatch` (only possible
when subtracting two caller-supplied `Money` values, e.g.
`net_revenue_impact` or a module's `net_savings`) or an `Overflow`. Each
function's rustdoc `# Errors` section says which apply. The exceptions are
`patient_acquisition_cost::cost_per_new_patient` and
`episode_cost_reduction::cost_per_episode`, which divide by a count and
return `Err(MoneyError::DivisionByZero)` when it is zero.

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
- `net_promoter_score` ranges from -100 to +100, not 0 to 100, and
  `access_parity_ratio`, `ltv_to_cac_ratio`, and `marketing_efficiency_ratio`
  are plain ratios (3.0 means 3:1), not percentages — don't format them as
  `%` or clamp them.
- `estimated_a1c` is the ADAG regression estimate, not a laboratory `HbA1c`, and never returns `None`
  (it has no denominator). `device_health_rate`'s missing transmissions are
  a device signal first — rule out device failure before reading them as
  patient non-adherence.
- `roi_percent` and `benefit_cost_ratio` take plain `f64` amounts in one currency; `platform_investment_cost` builds the cost side as `Money`, but this crate
  doesn't convert `Money` to `f64` — extract the amount yourself. `sus_score` is 0–100 but not a percentage, and `re_aim_framework` covers only the four
  proportional RE-AIM dimensions: Effectiveness is measured with the outcome modules (`biometric_improvement`, `readmission_rate`).
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

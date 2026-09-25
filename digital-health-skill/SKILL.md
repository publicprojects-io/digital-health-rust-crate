---
name: digital-health-metrics
description: Compute digital health KPIs — patient portal adoption, telehealth visit rate, appointment no-show rate, clinical alert override rate, digital referral turnaround time — using the `digital-health` Rust crate's pure calculation functions. Use when asked to calculate one of these metrics from raw counts, to explain what one means, or to write Rust code against this crate.
---

# Digital Health Metrics

`digital-health` (this repository) is a `std`-only, zero-dependency Rust
crate: one module per metric, each a small set of pure functions over `f64`.
Full contracts live in [`spec/`](../spec/README.md); the machine-readable
form is [`llms.json`](../llms.json). This file is the quick-reference for
picking the right function and interpreting its result.

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
  denominator; it trusts the caller's counts. Validate upstream if the data
  source might not guarantee that.
- Three of the crate's eight modules —
  `remote_patient_monitoring_adherence_rate`,
  `secure_messaging_response_time`, `e_prescribing_transmission_accuracy` —
  are not present in [Digital Health
  Metrics](https://github.com/digital-health-metrics/digital-health-metrics)
  and are sourced independently from general literature and standards
  instead; each module's rustdoc `## Sources` section says exactly what.

## Further reading

- [`spec/`](../spec/README.md) — exact formula and `None` condition per
  function
- [`AGENTS.md`](../AGENTS.md) — conventions for modifying this crate
- [`docs/tutorials/getting-started.md`](../docs/tutorials/getting-started.md)
  — a worked walkthrough

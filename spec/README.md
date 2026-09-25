# Specifications

This directory is the single source of truth for each metric's **calculation
contract**: its function signature, formula, and the conditions under which
it returns `None`. It is deliberately narrow — narrative content (why a
metric matters, its pitfalls, its data sources) belongs in the module's
rustdoc, not here, so that content is written once and doesn't drift.

## Files

Five specs mirror a topic in [Digital Health
Metrics](https://github.com/digital-health-metrics/digital-health-metrics)
directly; three more cover well-evidenced metrics not yet in that project.
Each file's `Upstream topic` line says which.

| Spec | Module | Upstream topic |
| --- | --- | --- |
| [`appointment-no-show-rate.md`](appointment-no-show-rate.md) | [`src/appointment_no_show_rate.rs`](../src/appointment_no_show_rate.rs) | yes |
| [`clinical-alert-override-rate.md`](clinical-alert-override-rate.md) | [`src/clinical_alert_override_rate.rs`](../src/clinical_alert_override_rate.rs) | yes |
| [`digital-referral-turnaround-time.md`](digital-referral-turnaround-time.md) | [`src/digital_referral_turnaround_time.rs`](../src/digital_referral_turnaround_time.rs) | yes |
| [`e-prescribing-transmission-accuracy.md`](e-prescribing-transmission-accuracy.md) | [`src/e_prescribing_transmission_accuracy.rs`](../src/e_prescribing_transmission_accuracy.rs) | no |
| [`patient-portal-adoption-rate.md`](patient-portal-adoption-rate.md) | [`src/patient_portal_adoption_rate.rs`](../src/patient_portal_adoption_rate.rs) | yes |
| [`remote-patient-monitoring-adherence-rate.md`](remote-patient-monitoring-adherence-rate.md) | [`src/remote_patient_monitoring_adherence_rate.rs`](../src/remote_patient_monitoring_adherence_rate.rs) | no |
| [`secure-messaging-response-time.md`](secure-messaging-response-time.md) | [`src/secure_messaging_response_time.rs`](../src/secure_messaging_response_time.rs) | no |
| [`telehealth-visit-rate.md`](telehealth-visit-rate.md) | [`src/telehealth_visit_rate.rs`](../src/telehealth_visit_rate.rs) | yes |

For the five upstream-backed specs, the formula and worked-example numbers
are themselves sourced from the Digital Health Metrics project's topic
docs. For the three independent specs, they're sourced from the general
literature and standards cited in that module's rustdoc `## Sources`
section — there is no upstream topic doc to check them against.

## Change process

A change to a formula, a function's signature, or the condition under which
it returns `None` must update, in the same change:

1. This directory's spec file for that metric.
2. The implementing function in `src/`.
3. That function's rustdoc (`# Arguments` / `# Returns` / `# Examples`).
4. Its unit test(s) and doctest(s) in the same module.

Narrative-only edits (why it matters, pitfalls, sources) touch only the
module's rustdoc and don't require a spec change.

## Conventions used in every spec file

- All public functions take and return `f64`; there are no other numeric
  types in this crate.
- A function returns `Option<f64>` whenever any of its arguments is a
  denominator that can legitimately be zero; it returns `None` rather than
  producing `NaN` or `inf`.
- No function panics. `digital_referral_turnaround_time::percentile` and
  `secure_messaging_response_time::percentile` both delegate to a shared,
  crate-private `internal::percentile` helper that sorts with
  `f64::total_cmp` specifically so that NaN input cannot panic.
- No function validates that a numerator is non-negative or `<=` its
  denominator — see [`AGENTS.md`](../AGENTS.md) for why that's a deliberate
  scope boundary, not an oversight.

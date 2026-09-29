# Specifications

This directory is the single source of truth for each metric's **calculation
contract**: its function signature, formula, and the conditions under which
it returns `None` (`f64`-based modules) or `Err` (`Money`-based modules). It
is deliberately narrow — narrative content (why a metric matters, its
pitfalls, its data sources) belongs in the module's rustdoc, not here, so
that content is written once and doesn't drift.

## Files

Five specs mirror a topic in [Digital Health
Metrics](https://github.com/digital-health-metrics/digital-health-metrics)
directly; fifty more cover well-evidenced metrics not yet in that
project. Each file's `Upstream topic` line says which. Thirteen of the
fifty independent specs are money-based (`Numeric type` column) — see
`## Money convention` below.

| Spec | Module | Upstream topic | Numeric type |
| --- | --- | --- | --- |
| [`active-user-rate.md`](active-user-rate.md) | [`src/active_user_rate.rs`](../src/active_user_rate.rs) | no | `f64` |
| [`appointment-no-show-rate.md`](appointment-no-show-rate.md) | [`src/appointment_no_show_rate.rs`](../src/appointment_no_show_rate.rs) | yes | `f64` |
| [`bed-day-cost-avoidance.md`](bed-day-cost-avoidance.md) | [`src/bed_day_cost_avoidance.rs`](../src/bed_day_cost_avoidance.rs) | no | `Money` |
| [`biometric-improvement.md`](biometric-improvement.md) | [`src/biometric_improvement.rs`](../src/biometric_improvement.rs) | no | `f64` |
| [`biometric-stabilization.md`](biometric-stabilization.md) | [`src/biometric_stabilization.rs`](../src/biometric_stabilization.rs) | no | `f64` |
| [`clinical-alert-firing-rate.md`](clinical-alert-firing-rate.md) | [`src/clinical_alert_firing_rate.rs`](../src/clinical_alert_firing_rate.rs) | no | `f64` |
| [`clinical-alert-override-rate.md`](clinical-alert-override-rate.md) | [`src/clinical_alert_override_rate.rs`](../src/clinical_alert_override_rate.rs) | yes | `f64` |
| [`clinician-documentation-time.md`](clinician-documentation-time.md) | [`src/clinician_documentation_time.rs`](../src/clinician_documentation_time.rs) | no | `f64` |
| [`cpoe-adoption-rate.md`](cpoe-adoption-rate.md) | [`src/cpoe_adoption_rate.rs`](../src/cpoe_adoption_rate.rs) | no | `f64` |
| [`cpoe-cost-impact.md`](cpoe-cost-impact.md) | [`src/cpoe_cost_impact.rs`](../src/cpoe_cost_impact.rs) | no | `Money` |
| [`device-health-rate.md`](device-health-rate.md) | [`src/device_health_rate.rs`](../src/device_health_rate.rs) | no | `f64` |
| [`digital-access-rate.md`](digital-access-rate.md) | [`src/digital_access_rate.rs`](../src/digital_access_rate.rs) | no | `f64` |
| [`digital-intake-cost-savings.md`](digital-intake-cost-savings.md) | [`src/digital_intake_cost_savings.rs`](../src/digital_intake_cost_savings.rs) | no | `Money` |
| [`digital-intake-form-completion-rate.md`](digital-intake-form-completion-rate.md) | [`src/digital_intake_form_completion_rate.rs`](../src/digital_intake_form_completion_rate.rs) | no | `f64` |
| [`digital-literacy-rate.md`](digital-literacy-rate.md) | [`src/digital_literacy_rate.rs`](../src/digital_literacy_rate.rs) | no | `f64` |
| [`digital-referral-acceptance-rate.md`](digital-referral-acceptance-rate.md) | [`src/digital_referral_acceptance_rate.rs`](../src/digital_referral_acceptance_rate.rs) | no | `f64` |
| [`digital-referral-turnaround-time.md`](digital-referral-turnaround-time.md) | [`src/digital_referral_turnaround_time.rs`](../src/digital_referral_turnaround_time.rs) | yes | `f64` |
| [`digital-therapeutic-retention-rate.md`](digital-therapeutic-retention-rate.md) | [`src/digital_therapeutic_retention_rate.rs`](../src/digital_therapeutic_retention_rate.rs) | no | `f64` |
| [`duplicate-patient-record-rate.md`](duplicate-patient-record-rate.md) | [`src/duplicate_patient_record_rate.rs`](../src/duplicate_patient_record_rate.rs) | no | `f64` |
| [`duplicate-record-remediation-cost.md`](duplicate-record-remediation-cost.md) | [`src/duplicate_record_remediation_cost.rs`](../src/duplicate_record_remediation_cost.rs) | no | `Money` |
| [`e-prescribing-transmission-accuracy.md`](e-prescribing-transmission-accuracy.md) | [`src/e_prescribing_transmission_accuracy.rs`](../src/e_prescribing_transmission_accuracy.rs) | no | `f64` |
| [`econsult-cost-avoidance.md`](econsult-cost-avoidance.md) | [`src/econsult_cost_avoidance.rs`](../src/econsult_cost_avoidance.rs) | no | `Money` |
| [`econsult-turnaround-time.md`](econsult-turnaround-time.md) | [`src/econsult_turnaround_time.rs`](../src/econsult_turnaround_time.rs) | no | `f64` |
| [`ed-diversion-cost-avoidance.md`](ed-diversion-cost-avoidance.md) | [`src/ed_diversion_cost_avoidance.rs`](../src/ed_diversion_cost_avoidance.rs) | no | `Money` |
| [`ed-diversion-rate.md`](ed-diversion-rate.md) | [`src/ed_diversion_rate.rs`](../src/ed_diversion_rate.rs) | no | `f64` |
| [`ehr-system-uptime-rate.md`](ehr-system-uptime-rate.md) | [`src/ehr_system_uptime_rate.rs`](../src/ehr_system_uptime_rate.rs) | no | `f64` |
| [`engagement-consistency-rate.md`](engagement-consistency-rate.md) | [`src/engagement_consistency_rate.rs`](../src/engagement_consistency_rate.rs) | no | `f64` |
| [`episode-cost-reduction.md`](episode-cost-reduction.md) | [`src/episode_cost_reduction.rs`](../src/episode_cost_reduction.rs) | no | `Money` |
| [`interoperability-document-exchange-rate.md`](interoperability-document-exchange-rate.md) | [`src/interoperability_document_exchange_rate.rs`](../src/interoperability_document_exchange_rate.rs) | no | `f64` |
| [`long-term-retention-rate.md`](long-term-retention-rate.md) | [`src/long_term_retention_rate.rs`](../src/long_term_retention_rate.rs) | no | `f64` |
| [`medication-adherence-rate.md`](medication-adherence-rate.md) | [`src/medication_adherence_rate.rs`](../src/medication_adherence_rate.rs) | no | `f64` |
| [`medication-reconciliation-rate.md`](medication-reconciliation-rate.md) | [`src/medication_reconciliation_rate.rs`](../src/medication_reconciliation_rate.rs) | no | `f64` |
| [`net-promoter-score.md`](net-promoter-score.md) | [`src/net_promoter_score.rs`](../src/net_promoter_score.rs) | no | `f64` |
| [`no-show-lost-revenue.md`](no-show-lost-revenue.md) | [`src/no_show_lost_revenue.rs`](../src/no_show_lost_revenue.rs) | no | `Money` |
| [`patient-acquisition-cost.md`](patient-acquisition-cost.md) | [`src/patient_acquisition_cost.rs`](../src/patient_acquisition_cost.rs) | no | `Money` |
| [`patient-acquisition-efficiency.md`](patient-acquisition-efficiency.md) | [`src/patient_acquisition_efficiency.rs`](../src/patient_acquisition_efficiency.rs) | no | `f64` |
| [`patient-identity-verification-rate.md`](patient-identity-verification-rate.md) | [`src/patient_identity_verification_rate.rs`](../src/patient_identity_verification_rate.rs) | no | `f64` |
| [`patient-portal-adoption-rate.md`](patient-portal-adoption-rate.md) | [`src/patient_portal_adoption_rate.rs`](../src/patient_portal_adoption_rate.rs) | yes | `f64` |
| [`patient-self-scheduling-cost-savings.md`](patient-self-scheduling-cost-savings.md) | [`src/patient_self_scheduling_cost_savings.rs`](../src/patient_self_scheduling_cost_savings.rs) | no | `Money` |
| [`patient-self-scheduling-rate.md`](patient-self-scheduling-rate.md) | [`src/patient_self_scheduling_rate.rs`](../src/patient_self_scheduling_rate.rs) | no | `f64` |
| [`platform-investment-cost.md`](platform-investment-cost.md) | [`src/platform_investment_cost.rs`](../src/platform_investment_cost.rs) | no | `Money` |
| [`prom-completion-rate.md`](prom-completion-rate.md) | [`src/prom_completion_rate.rs`](../src/prom_completion_rate.rs) | no | `f64` |
| [`re-aim-framework.md`](re-aim-framework.md) | [`src/re_aim_framework.rs`](../src/re_aim_framework.rs) | no | `f64` |
| [`readmission-rate.md`](readmission-rate.md) | [`src/readmission_rate.rs`](../src/readmission_rate.rs) | no | `f64` |
| [`remote-patient-monitoring-adherence-rate.md`](remote-patient-monitoring-adherence-rate.md) | [`src/remote_patient_monitoring_adherence_rate.rs`](../src/remote_patient_monitoring_adherence_rate.rs) | no | `f64` |
| [`remote-patient-monitoring-billing-revenue.md`](remote-patient-monitoring-billing-revenue.md) | [`src/remote_patient_monitoring_billing_revenue.rs`](../src/remote_patient_monitoring_billing_revenue.rs) | no | `Money` |
| [`return-on-investment.md`](return-on-investment.md) | [`src/return_on_investment.rs`](../src/return_on_investment.rs) | no | `f64` |
| [`secure-messaging-response-time.md`](secure-messaging-response-time.md) | [`src/secure_messaging_response_time.rs`](../src/secure_messaging_response_time.rs) | no | `f64` |
| [`system-usability-scale.md`](system-usability-scale.md) | [`src/system_usability_scale.rs`](../src/system_usability_scale.rs) | no | `f64` |
| [`telehealth-cost-avoidance.md`](telehealth-cost-avoidance.md) | [`src/telehealth_cost_avoidance.rs`](../src/telehealth_cost_avoidance.rs) | no | `Money` |
| [`telehealth-technical-failure-rate.md`](telehealth-technical-failure-rate.md) | [`src/telehealth_technical_failure_rate.rs`](../src/telehealth_technical_failure_rate.rs) | no | `f64` |
| [`telehealth-visit-rate.md`](telehealth-visit-rate.md) | [`src/telehealth_visit_rate.rs`](../src/telehealth_visit_rate.rs) | yes | `f64` |
| [`time-to-intervention.md`](time-to-intervention.md) | [`src/time_to_intervention.rs`](../src/time_to_intervention.rs) | no | `f64` |
| [`triage-accuracy-rate.md`](triage-accuracy-rate.md) | [`src/triage_accuracy_rate.rs`](../src/triage_accuracy_rate.rs) | no | `f64` |
| [`virtual-ward-bed-days.md`](virtual-ward-bed-days.md) | [`src/virtual_ward_bed_days.rs`](../src/virtual_ward_bed_days.rs) | no | `f64` |

For the five upstream-backed specs, the formula and worked-example numbers
are themselves sourced from the Digital Health Metrics project's topic
docs. For the fifty independent specs, they're sourced from the
general literature and standards cited in that module's rustdoc `##
Sources` section — there is no upstream topic doc to check them against.

## Change process

A change to a formula, a function's signature, or the condition under which
it returns `None` or `Err` must update, in the same change:

1. This directory's spec file for that metric.
2. The implementing function in `src/`.
3. That function's rustdoc (`# Arguments` / `# Returns` / `# Examples`).
4. Its unit test(s) and doctest(s) in the same module.

Narrative-only edits (why it matters, pitfalls, sources) touch only the
module's rustdoc and don't require a spec change.

## Conventions used in every `f64` spec file

- All public functions take and return `f64`.
- A function returns `Option<f64>` whenever any of its arguments is a
  denominator that can legitimately be zero; it returns `None` rather than
  producing `NaN` or `inf`.
- No function panics. `digital_referral_turnaround_time::percentile`,
  `secure_messaging_response_time::percentile`, and
  `econsult_turnaround_time::percentile` all delegate to a shared,
  crate-private `internal::percentile` helper that sorts with
  `f64::total_cmp` specifically so that NaN input cannot panic.
- No function validates that a numerator is non-negative or `<=` its
  denominator — see [`AGENTS.md`](../AGENTS.md) for why that's a deliberate
  scope boundary, not an oversight.

## Money convention

The thirteen `Money`-based modules (see the `Numeric type` column above) are
the exception to "everything is `f64`": they take counts as `u32` and a
concrete [`rusty_money::Money<'static,
rusty_money::iso::Currency>`](https://docs.rs/rusty-money) amount, and
return `Result<Money, rusty_money::MoneyError>` instead of `Option<f64>`.

- Counts are `u32`, not `f64` — a fractional appointment or patient has no
  meaning, and `u32` converts to `rusty_money`'s underlying `Decimal` type
  exactly, with none of the precision-loss risk an `f64` count would carry.
- `Err` replaces `None` as the fallible path, and it's not a
  zero-denominator condition: it's either `MoneyError::CurrencyMismatch`
  (adding or subtracting two `Money` values in different currencies) or
  `MoneyError::Overflow` (a multiplication or addition that overflows the
  underlying decimal). Each function's rustdoc `# Errors` section says
  which apply. The exceptions are `patient_acquisition_cost::
  cost_per_new_patient` and `episode_cost_reduction::cost_per_episode`,
  which divide by a count and so can also return
  `MoneyError::DivisionByZero` — `rusty_money`'s own behaviour, surfaced
  rather than converted to `None`.
- No function picks or hard-codes a reimbursement figure, a currency, or an
  exchange rate — those are always caller-supplied `Money` arguments. See
  each module's `## Data sources and caveats` for why (reimbursement rates
  change annually and vary by payer and locality).
- **`Money` is used directly, not wrapped.** Every function's body is a
  single call straight through to `rusty_money`'s own `Money::mul` or
  `Money::sub` — the function exists only to give a domain name to that one
  call, not to add logic on top of it. The `Money` type is concrete
  (`Money<'static, iso::Currency>`), not generic over
  `rusty_money::FormattableCurrency`: adding our own generic type parameter
  on top of `rusty_money`'s would itself be a form of wrapping. This means
  these functions work with rusty-money's built-in ISO-4217 currencies, not
  a caller-defined custom currency type — if that's ever needed, call
  `rusty_money`'s methods directly instead of extending this crate's
  signatures with a generic parameter.
- **USD is the default illustrative currency.** Every worked example,
  doctest, and unit test in a money-based module uses `iso::USD` unless it
  is specifically demonstrating `MoneyError::CurrencyMismatch` (where a
  second currency, e.g. `iso::EUR`, is required to trigger the error).

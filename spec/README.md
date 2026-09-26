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
directly; five more cover well-evidenced metrics not yet in that project.
Each file's `Upstream topic` line says which. Two of the five independent
specs are money-based (`Numeric type` column) — see `## Money convention`
below.

| Spec | Module | Upstream topic | Numeric type |
| --- | --- | --- | --- |
| [`appointment-no-show-rate.md`](appointment-no-show-rate.md) | [`src/appointment_no_show_rate.rs`](../src/appointment_no_show_rate.rs) | yes | `f64` |
| [`clinical-alert-override-rate.md`](clinical-alert-override-rate.md) | [`src/clinical_alert_override_rate.rs`](../src/clinical_alert_override_rate.rs) | yes | `f64` |
| [`digital-referral-turnaround-time.md`](digital-referral-turnaround-time.md) | [`src/digital_referral_turnaround_time.rs`](../src/digital_referral_turnaround_time.rs) | yes | `f64` |
| [`e-prescribing-transmission-accuracy.md`](e-prescribing-transmission-accuracy.md) | [`src/e_prescribing_transmission_accuracy.rs`](../src/e_prescribing_transmission_accuracy.rs) | no | `f64` |
| [`no-show-lost-revenue.md`](no-show-lost-revenue.md) | [`src/no_show_lost_revenue.rs`](../src/no_show_lost_revenue.rs) | no | `Money` |
| [`patient-portal-adoption-rate.md`](patient-portal-adoption-rate.md) | [`src/patient_portal_adoption_rate.rs`](../src/patient_portal_adoption_rate.rs) | yes | `f64` |
| [`remote-patient-monitoring-adherence-rate.md`](remote-patient-monitoring-adherence-rate.md) | [`src/remote_patient_monitoring_adherence_rate.rs`](../src/remote_patient_monitoring_adherence_rate.rs) | no | `f64` |
| [`remote-patient-monitoring-billing-revenue.md`](remote-patient-monitoring-billing-revenue.md) | [`src/remote_patient_monitoring_billing_revenue.rs`](../src/remote_patient_monitoring_billing_revenue.rs) | no | `Money` |
| [`secure-messaging-response-time.md`](secure-messaging-response-time.md) | [`src/secure_messaging_response_time.rs`](../src/secure_messaging_response_time.rs) | no | `f64` |
| [`telehealth-visit-rate.md`](telehealth-visit-rate.md) | [`src/telehealth_visit_rate.rs`](../src/telehealth_visit_rate.rs) | yes | `f64` |

For the five upstream-backed specs, the formula and worked-example numbers
are themselves sourced from the Digital Health Metrics project's topic
docs. For the five independent specs, they're sourced from the general
literature and standards cited in that module's rustdoc `## Sources`
section — there is no upstream topic doc to check them against.

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
- No function panics. `digital_referral_turnaround_time::percentile` and
  `secure_messaging_response_time::percentile` both delegate to a shared,
  crate-private `internal::percentile` helper that sorts with
  `f64::total_cmp` specifically so that NaN input cannot panic.
- No function validates that a numerator is non-negative or `<=` its
  denominator — see [`AGENTS.md`](../AGENTS.md) for why that's a deliberate
  scope boundary, not an oversight.

## Money convention

`no_show_lost_revenue` and `remote_patient_monitoring_billing_revenue` are
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
  which apply.
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

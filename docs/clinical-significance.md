# Clinical significance of each metric

A plain-language guide to what each metric in this crate tells a clinician or
health-system leader, how to read a high or low value, and the most common way
it is misread. It is educational context, not a specification: the formulas and
`None`/`Err` conditions live in [`spec/`](../spec/README.md), and each
module's rustdoc (`## Why it matters`, `## Pitfalls`, `## Sources`) is the
detailed reference these summaries are drawn from. Nothing here is a target or
clinical guidance; the "right" value depends on population, setting and the
measure definition you follow.

Entries use the same three themes as the `Module index` in
[`src/lib.rs`](../src/lib.rs).

- [Patient engagement and access](#patient-engagement-and-access)
- [Digital care operations and safety](#digital-care-operations-and-safety)
- [Cost and revenue](#cost-and-revenue)

## Patient engagement and access

**`patient_portal_adoption_rate`** — Whether patients actually use the portal,
as a funnel of registration, activation and active use. Higher at each later
stage is better; a big drop between stages means accounts exist but go unused.
Misreading: counting registration as adoption. Low or uneven adoption is also an
equity signal, because older, lower-literacy or non-majority-language patients
are less likely to be counted.

**`telehealth_visit_rate`** — The share of care delivered remotely. Neither high
nor low is inherently good: appropriateness varies by specialty, visit type and
patient preference. Misreading: treating it as a number to maximise, or merging
video and telephone, which are not interchangeable.

**`appointment_no_show_rate`** — Clinic capacity lost to missed appointments.
Lower means shorter waits and less wasted clinician time. Misreading: treating a
high rate as purely patient behaviour; no-shows track deprivation, transport and
multimorbidity, so blanket penalties tend to worsen inequity.

**`remote_patient_monitoring_adherence_rate`** — Whether monitored patients send
enough readings to make the monitoring clinically useful (and, for many payers,
billable). Low adherence may be a device or access problem rather than
reluctance. Misreading: counting any transmission, including unusable readings,
as qualifying.

**`telehealth_technical_failure_rate`** — How often a remote visit fails because
of technology. Higher means friction and lost visits that a healthy-looking
visit rate hides, and it tends to fall hardest on patients with older age,
limited bandwidth or unfamiliar devices. Misreading: a platform that logs only
the final successful attempt makes it invisible.

**`patient_self_scheduling_rate`** — How many bookings patients make
themselves, alongside how soon the offered slots are. High use only improves
access if appointments are actually available. Misreading: counting staff-made
bookings as self-scheduled.

**`digital_intake_form_completion_rate`** — Whether patients finish intake
forms before arriving. Pre-visit completion shortens the visit and reduces
front-desk work; a form finished in the waiting room does not. Misreading:
counting an abandoned, partly completed form as complete.

**`prom_completion_rate`** — The share of patients who complete patient-reported
outcome measures. Low or skewed completion means the outcome data may come only
from healthier, younger or more digitally engaged patients. Misreading: trusting
a high overall rate without checking who is missing.

**`digital_therapeutic_retention_rate`** — Day-7 and day-30 retention, the
standard early-attrition checkpoints. Weak day-7 points to onboarding; a steep
drop to day-30 points to engagement design. Misreading: reporting one checkpoint
or one blended "active users" figure, which hides which problem it is.

**`patient_identity_verification_rate`** — How many patients get through
identity proofing to use digital services, plus how often they need extra
steps. Low success can look like poor adoption when the fix is upstream.
Misreading: treating verification failure as portal-adoption failure; step-up
friction also falls on patients lacking a smartphone camera or familiarity.

**`net_promoter_score`** — Patient or user willingness to recommend a service.
Ranges from −100 to +100, and a negative score is meaningful. Misreading: ignoring
the response rate; a high score from few responders mostly reflects who chose to
answer.

**`digital_access_rate`** — Who can reach a digital service at all, and how
evenly (the parity ratio). A parity ratio below 1.0 means a subgroup is falling
behind. Misreading: reporting only the overall rate, or counting a shared,
data-capped phone as equivalent to home broadband.

**`patient_acquisition_efficiency`** — Whether spending to grow a patient base
pays back (LTV-to-CAC and marketing efficiency). Higher is better, and a 3:1
LTV-to-CAC is a commonly cited baseline. Misreading: computing it from an
undercounted acquisition cost, which flatters the ratio.

**`engagement_consistency_rate`** — How steadily patients keep using a tool week
to week, and how many drop out. Sustained interaction is a stronger indicator of
benefit than a login count. Misreading: counting a passive login as engagement,
or reporting survivors only.

**`digital_literacy_rate`** — Whether patients can actually use a service they
can reach, measured by unassisted completion rather than self-reported
confidence. Low values mean the service is quietly failing some patients.
Misreading: reporting only the overall rate, which hides the strata with the
largest gaps.

**`medication_adherence_rate`** — Proportion of days covered and the share of
patients reaching the 80% threshold. Non-adherence is a large, preventable
driver of poor outcomes and avoidable admissions. Misreading: treating a filled
prescription as a taken dose; it is a supply proxy.

**`active_user_rate`** — Who shows up and how often (active users, DAU/MAU
stickiness). A product with many monthly but few daily users is visited rather
than used. Misreading: counting opens or logins as activity.

**`long_term_retention_rate`** — Day-60 and day-90 retention, where most trials
find the clinical benefit is decided. A steep fall between day 30 and day 60 is
a different failure from poor onboarding. Misreading: mixing cohorts, or
reporting only cohorts old enough to reach the horizon.

**`system_usability_scale`** — A short, widely used usability score (0–100) for
patients or clinicians. About 68 is commonly cited as the cross-product average.
Misreading: reading the score as a percentage; 68 is average, not "68%
satisfied".

## Digital care operations and safety

**`clinical_alert_override_rate`** — How often clinicians override decision
support alerts, the standard signal for alert fatigue. A high rate is not
automatically unsafe, since many alerts are low value; what matters is the
trend, severity mix and whether high-severity overrides carry a documented
reason. Misreading: using the raw rate as a single safety score.

**`clinical_alert_firing_rate`** — How many alerts the system raises, and the
share that are high severity. Lower firing with a higher high-severity share is
the direction alert-fatigue work aims for. Misreading: comparing systems with
different rule sets, or ignoring severity mix, which cannot separate "well
tuned" from "categories switched off".

**`time_to_intervention`** — How fast a clinical team acts on an automated
alert. An alert nobody acts on quickly has no clinical value, and the slow tail
is where harm happens. Misreading: counting acknowledgement as intervention, or
reporting only the mean.

**`digital_referral_turnaround_time`** — How long triage takes before a patient
even joins a waiting list. Shorter and less variable is better, since this delay
is process waste before any care starts. Misreading: measuring only clean
referrals, which hides rework, or reporting the mean instead of median and
percentiles.

**`digital_referral_acceptance_rate`** — What triage actually decides: accepted,
rejected or returned for information. A fast turnaround driven by quick returns
is not an operational win. Misreading: reading turnaround and acceptance
independently.

**`econsult_turnaround_time`** — How quickly asynchronous specialist advice
returns. These services help primary care resolve questions without a full
referral, but only if the reply arrives fast enough to be useful. Misreading:
reporting the mean, which a few long-running consults pull upward.

**`secure_messaging_response_time`** — How quickly clinicians answer patient
portal messages. It is both a patient-experience measure and a proxy for in-box
workload that spills into time outside scheduled hours. Slow response to
time-sensitive messages is also a safety issue. Misreading: counting an
auto-acknowledgement as the reply, or reporting only the mean.

**`e_prescribing_transmission_accuracy`** — The pharmacy call-back rate, which
catches errors a clean network transmission cannot. Rising call-backs from
data-mapping problems are a system-quality issue; those from formulary or prior
authorisation are administrative. Misreading: treating "successfully
transmitted" as clinically correct.

**`cpoe_adoption_rate`** — How much ordering goes through electronic order entry
rather than verbal orders. A verbal order bypasses every CPOE safety check, so a
rising verbal-order rate can hide unmitigated medication risk behind a healthy
override rate. Misreading: treating high adoption as sufficient for medication
safety on its own.

**`medication_reconciliation_rate`** — Whether medication lists are reconciled at
admission and discharge, the points where lists most often drift from what the
patient actually takes. Higher completion is safer. Misreading: counting a
started-but-unsigned reconciliation as complete, or blending admission and
discharge, which have different risks.

**`duplicate_patient_record_rate`** — How many patients have fragmented records.
Each duplicate means a clinician may see only part of a patient's history, a
direct safety risk that can spike after mergers or EHR migrations. Misreading:
treating "identified" as "resolved"; only merged duplicates reduce the risk.

**`interoperability_document_exchange_rate`** — Whether exchanged documents
arrive and then become usable data. A document that fails to parse is filed but
must be read manually rather than populating problem, medication and allergy
lists. Misreading: inferring usability from transmission success alone.

**`ehr_system_uptime_rate`** — Availability of the record system that every other
digital metric assumes. Planned and unplanned downtime need different fixes.
Misreading: reporting one blended figure, or measuring the wrong scope when the
core EHR is up but a dependent service is not.

**`triage_accuracy_rate`** — Whether a digital triage tool routes patients to the
right care level. Over-triage wastes capacity; under-triage can delay care for
someone who needed it urgently, so the under-triage rate carries the safety
signal. Misreading: reporting overall accuracy alone, or reviewing a non-random
sample.

**`readmission_rate`** — Unplanned return within 30 days of discharge, and its
relative reduction against a baseline. It is the outcome payers most readily
accept as proof that a programme improved care. Misreading: comparing unadjusted
rates across hospitals; case mix drives readmission and this measure is not
risk-adjusted.

**`clinician_documentation_time`** — Time per encounter spent on EHR
documentation, which is tied to clinician burnout. A reduction shows a tool gave
time back. Misreading: counting time saved rather than time returned; a shorter
note does not reduce burnout if the freed time is filled with more patients.

**`biometric_improvement`** — Relative fall in measures such as HbA1c or BMI, and
the share reaching target. Together they show the size of the average effect and
how many patients it actually reached. Misreading: reporting an average change
alone, or treating estimated A1c as interchangeable with laboratory HbA1c.

**`biometric_stabilization`** — Blood-pressure control and glucose time in range
from connected devices. A single reading is noisy; sustained control across a
cohort is what relates to fewer complications. Misreading: silently counting or
excluding patients with no recent readings, or mixing clinic and home-device
data.

**`device_health_rate`** — Device uptime, transmission success and resource
saturation behind a monitoring programme. A faulty device produces gaps that
look like patient non-adherence. Misreading: reading a missing transmission as
non-adherence before checking the device.

**`virtual_ward_bed_days`** — Hospital bed days freed by moving recovery into a
virtual ward. Higher means more capacity released. Misreading: using an
unmatched conventional stay length, which decides the whole result, or ignoring
readmissions that make the saving illusory.

**`ed_diversion_rate`** — Emergency visits avoided through virtual triage, and
the share safely avoided. It is the headline value claim and the one most prone
to overstatement. Misreading: counting every contact not sent to the ED as
diverted, or ignoring return visits.

**`re_aim_framework`** — Reach, Adoption, Implementation and Maintenance, which
judge a programme on population impact rather than efficacy alone. A strong
effect reaching few eligible patients has small impact. Misreading: reporting
only Effectiveness, or using enrolled rather than eligible patients as the reach
denominator.

## Cost and revenue

These modules turn a clinical or operational rate into currency. They are
financial counterparts of the metrics above, not measures of clinical quality;
their shared caution is to net the programme's own cost and to avoid blending
unlike cases into one average.

**`no_show_lost_revenue`** — The money behind the no-show rate. Rates alone do
not show magnitude; 9% means something very different at $50 versus $500 per
appointment. Misreading: using a blended revenue figure across appointment
types.

**`remote_patient_monitoring_billing_revenue`** — Revenue earned versus at risk
from patients who did not clear the billing threshold. "Revenue at risk" is
what an adherence intervention is justified against. Misreading: assuming one
reimbursement figure covers every billed code.

**`telehealth_cost_avoidance`** — Costs not incurred (patient travel, clinic
overhead) when care is remote, net of the platform's own cost. Misreading: using
one figure across specialties or geographies, or reporting avoided cost without
netting platform cost.

**`patient_self_scheduling_cost_savings`** — Staff time not spent on booking
calls, net of the scheduling platform's cost. Misreading: using one figure across
visit types, or treating platform cost as purely variable.

**`econsult_cost_avoidance`** — The value of in-person referrals avoided through
e-consults, net of running the service. Misreading: counting every completed
e-consult as an avoided referral.

**`digital_intake_cost_savings`** — Manual data entry avoided by digital intake,
net of the platform. Misreading: using one avoided-cost figure for new-patient
and follow-up forms.

**`duplicate_record_remediation_cost`** — The cost of resolving duplicate records
and the backlog still to close, the more actionable figure for planning.
Misreading: using one resolution cost regardless of record age or complexity.

**`cpoe_cost_impact`** — The staffing cost case behind electronic ordering:
electronic orders avoid a transcription step, while verbal orders add a
read-back and countersignature step. Misreading: netting avoided cost against
incurred cost as if they were the same kind of figure.

**`patient_acquisition_cost`** — The fully loaded cost of acquiring a patient.
Misreading: leaving out components such as agency fees and intake labour, or
counting patients who never complete onboarding.

**`bed_day_cost_avoidance`** — The currency value of virtual-ward bed days saved,
net of the ward's cost. Misreading: using the average fully allocated bed-day
cost, which overstates the saving, or assuming a freed bed saves money when it
is simply refilled; then the benefit is capacity, not cash.

**`ed_diversion_cost_avoidance`** — The money behind safely diverted ED visits.
Misreading: counting all diverted contacts rather than only safe diversions, or
using a high-acuity average ED cost for low-acuity visits.

**`episode_cost_reduction`** — Cost per episode of care against a baseline, the
unit value-based contracts are written on. Misreading: comparing against a
baseline with different case mix, or leaving the programme's own cost out.

**`platform_investment_cost`** — The fully loaded licensing, hardware and
staffing cost a payback decision rests on. Misreading: leaving out staffing,
usually the largest ongoing cost, or charging the whole hardware purchase to one
year.

**`return_on_investment`** — ROI and benefit-cost ratio, the shorthand for "does
it pay back?". Misreading: counting gross rather than attributable benefit, or
comparing ROI over different periods.

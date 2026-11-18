# Clinical standards crosswalk

This page links each metric module to the clinical, quality, interoperability,
and usability/safety standards it relates to. It is a **reader's aid, not a
conformance claim**: the crate's functions compute simple, unadjusted
ratios and differences from caller-supplied counts. Official measure
specifications define their own populations, exclusions, risk adjustment, and
data elements, and you must follow the current published specification when
reporting to a regulator, payer, or accreditor.

**Verified on 2026-11-17** against the owning bodies' public pages (see
[Sources](#sources)). Standards change often: the Joint Commission, for
example, replaced its hospital National Patient Safety Goals with National
Performance Goals on 1 January 2026. Re-check before relying on any row.

"Related to" means the metric addresses the same concern as the named
standard, not that it reproduces the standard's definition. No measure
identifiers are listed on purpose; look them up in the owning body's current
catalogue, since they are revised yearly.

## Standards families referenced

- **Quality measures**: HEDIS (NCQA), the CMS Hospital Readmissions Reduction
  Program (HRRP), CMS Part D Star Ratings adherence measures (specified by the
  Pharmacy Quality Alliance, PQA), CMS Promoting Interoperability, and the
  Joint Commission National Performance Goals (NPGs; the hospital successor to
  the National Patient Safety Goals, which still apply to some other
  programmes). NQF endorsement is historical context only: since 2023 CMS's
  federal consensus-based entity is a different organisation (see Sources).
- **Interoperability**: HL7 FHIR, USCDI (ASTP/ONC), the ONC Health IT
  Certification Program, NCPDP SCRIPT (e-prescribing), IHE profiles, ONC SAFER
  Guides. LOINC and SNOMED CT are vocabularies referenced by USCDI and are
  mentioned only where a module relates to coded observations.
- **Usability and safety**: System Usability Scale (SUS), ISO 9241-11:2018,
  IEC 62366-1, IEC 60601-1-8, ISO/TS 82304-2:2021, and WHO digital health
  monitoring-and-evaluation guidance (the MAPS toolkit and the M&E guide).

## Adoption, access, and engagement

| Module | Related standards | How this crate's definition differs |
| --- | --- | --- |
| `active_user_rate` | ISO 9241-11 (use in context); ISO/TS 82304-2 | Counts users active in a caller-chosen window; no standard defines "active". |
| `patient_portal_adoption_rate` | ONC certification (patient access to health information, FHIR APIs); CMS Promoting Interoperability patient-access objectives | Share of patients enrolled; programme objectives define their own numerators (e.g. view/download/transmit). |
| `digital_access_rate` | WHO digital health M&E guidance (equity); ONC patient-access goals | Caller defines "access"; no risk adjustment or equity stratification. |
| `digital_literacy_rate` | WHO digital health M&E guidance (equity) | Depends entirely on the screening instrument the caller uses. |
| `engagement_consistency_rate` | None meaningful (engagement is vendor/programme-defined) | n/a |
| `long_term_retention_rate` | None meaningful (programme-defined); RE-AIM (Maintenance) | n/a |
| `digital_therapeutic_retention_rate` | ISO/TS 82304-2 (health app quality); FDA guidance is regulatory context, not a metric definition | Retention window is caller-defined. |
| `digital_intake_form_completion_rate` | HL7 FHIR Questionnaire / QuestionnaireResponse (data representation) | Counts completions only; does not check item-level validity. |
| `patient_self_scheduling_rate` | FHIR Appointment / Slot resources (data representation) | Ratio of self-scheduled to total; no official measure. |
| `net_promoter_score` | None meaningful (proprietary survey methodology) | Standard promoter/detractor arithmetic. |
| `system_usability_scale` | SUS (Brooke); ISO 9241-11; IEC 62366-1 (usability engineering for medical devices) | Implements the published 0–100 SUS scoring; the benchmark interpretation is separate. |
| `re_aim_framework` | RE-AIM evaluation framework; WHO digital health M&E guidance | Computes the dimension ratios supplied; does not assess qualitative dimensions. |

## Clinical quality and safety

| Module | Related standards | How this crate's definition differs |
| --- | --- | --- |
| `readmission_rate` | CMS HRRP (30-day condition-specific measures, with planned readmissions excluded); HEDIS Plan All-Cause Readmissions | **Unadjusted.** Official measures are risk-adjusted (HEDIS reports an observed-to-expected ratio) and have specific exclusions. |
| `medication_adherence_rate` | CMS Part D Star Ratings adherence measures (diabetes medications, RAS antagonists, statins), specified by PQA using proportion of days covered | Simple ratio of adherent to eligible; the official measures count a patient adherent at PDC of at least 80% and use pharmacy claims only. |
| `medication_reconciliation_rate` | Joint Commission National Performance Goals (medication management topic); HEDIS Transitions of Care, Medication Reconciliation Post-Discharge | Ratio of reconciled to eligible encounters; official timing windows and numerators apply. |
| `e_prescribing_transmission_accuracy` | NCPDP SCRIPT (ONC HTI-4: version 2023011 required for certification after 31 Dec 2027) | Accuracy ratio from caller counts; does not parse messages. |
| `clinical_alert_firing_rate` | Joint Commission clinical-alarm safety requirement (hospital NPGs; NPSG in other programmes); IEC 60601-1-8 (alarm systems in medical electrical equipment, device-level) | Rate per caller-chosen denominator. |
| `clinical_alert_override_rate` | Related to the ONC decision support interventions certification criterion (§170.315(b)(11)) | No official override benchmark. |
| `cpoe_adoption_rate` | ONC CPOE certification criteria (§170.315(a)(1) medications and related); CMS Promoting Interoperability programme (check the current measure list) | Share of orders entered electronically; the programme objective's exclusions are not applied. |
| `triage_accuracy_rate` | Related to triage scales such as the AHRQ-developed Emergency Severity Index (stewardship has moved; check current owner) | Agreement ratio with a caller-defined reference standard. |
| `time_to_intervention` | None meaningful (programme-defined) | n/a |
| `biometric_improvement` | Related to HEDIS outcome measures such as blood pressure and glycemic control; LOINC (USCDI vocabulary) for observation coding | Relative change; official measures use thresholds (controlled/not controlled) and specific measurement rules. |
| `biometric_stabilization` | As above | Caller defines "stable". |
| `prom_completion_rate` | Validated PRO instruments (e.g. PROMIS); HL7 FHIR Questionnaire / QuestionnaireResponse | Measures completion only, not scores. |
| `device_health_rate` | IEC 62366-1 (usability engineering, use-error risk) | Fleet-level ratio; not a regulatory vigilance report. No post-market surveillance standard is mapped (not verified). |
| `patient_identity_verification_rate` | Joint Commission patient-identification requirement (hospital NPGs; NPSG in other programmes); NIST SP 800-63-4, volume 63A (identity proofing, Identity Assurance Levels) | Ratio of verified to attempted; assurance level not assessed. The Joint Commission requirement is about correctly identifying patients at the point of care, not digital identity proofing. |
| `duplicate_patient_record_rate` | IHE PIX / PDQ (identifier cross-referencing and demographics query; PIX does not specify the matching algorithm); Joint Commission patient-identification requirement | Rate from caller's duplicate detection; no matching algorithm included. |

## Interoperability and infrastructure

| Module | Related standards | How this crate's definition differs |
| --- | --- | --- |
| `interoperability_document_exchange_rate` | HL7 FHIR; USCDI; ONC certification; IHE XDS | Ratio of successful to attempted exchanges; does not validate content conformance. |
| `ehr_system_uptime_rate` | ONC SAFER Guides, Contingency Planning (planned and unplanned EHR unavailability; guides are not a compliance standard) | Availability ratio; the crate does not define scheduled-downtime treatment. |
| `digital_referral_acceptance_rate` | HL7 FHIR ServiceRequest / Task (data representation) | Ratio only. |
| `digital_referral_turnaround_time` | As above | Percentiles via an internal helper; no official benchmark. |
| `econsult_turnaround_time` | None meaningful (programme rules differ by payer) | Percentile and mean; programme rules define timeliness thresholds. |
| `secure_messaging_response_time` | None meaningful (no official response-time threshold) | Percentile and mean; no official threshold. |
| `clinician_documentation_time` | ISO 9241-11 (efficiency) | No standard measure. |

## Telehealth and remote monitoring

| Module | Related standards | How this crate's definition differs |
| --- | --- | --- |
| `appointment_no_show_rate` | MGMA publishes practice no-show benchmarks (DataDive; specialty-specific); no formal quality measure | Ratio only. |
| `telehealth_visit_rate` | HL7 FHIR Encounter (class); CMS claims conventions (modifiers, place of service; look up current rules) | Share of visits that were telehealth; payer codes (e.g. modifiers/place of service) define it in claims. |
| `telehealth_technical_failure_rate` | ISO 9241-11 (effectiveness); ISO/TS 82304-2 | Ratio of failed to attempted; "failure" is caller-defined. |
| `remote_patient_monitoring_adherence_rate` | CMS RPM billing requirements (days of data per 30-day period; CY2026 added short-duration codes, so look up the current thresholds) | Ratio only; billing thresholds are inputs, not enforced. |
| `ed_diversion_rate` | Related to HEDIS emergency department utilization measures | Unadjusted; official measures define avoidable categories. |
| `virtual_ward_bed_days` | NHS England virtual wards operational framework (stays are short-duration acute care at home) | Counts bed days replaced; no official definition imposed. |

## Financial and economic modules

These compute costs and returns from caller-supplied amounts. There is no
clinical standard to map to. Reimbursement rules (CPT/HCPCS, payer policy) are
inputs the caller supplies; none are encoded.

The 13 `Money`-based modules (`u32` counts and `rusty_money` amounts; see
`spec/README.md`): `bed_day_cost_avoidance`, `cpoe_cost_impact`,
`digital_intake_cost_savings`, `duplicate_record_remediation_cost`,
`econsult_cost_avoidance`, `ed_diversion_cost_avoidance`,
`episode_cost_reduction`, `no_show_lost_revenue`, `patient_acquisition_cost`,
`patient_self_scheduling_cost_savings`, `platform_investment_cost`,
`remote_patient_monitoring_billing_revenue`, `telehealth_cost_avoidance`.

Two further financial modules are plain `f64` ratios, not `Money`-based:
`patient_acquisition_efficiency` and `return_on_investment`.

`remote_patient_monitoring_billing_revenue` relates to CMS RPM billing codes
(for CY2026 CMS finalised two new codes, CPT 99445 and 99470, in addition to
the existing set); look up current codes and rates directly, since they change
annually.

## Sources

Checked on 2026-11-17. Several secondary sources were used where a primary
page was paywalled; those items are flagged below.

- Joint Commission National Performance Goals (hospital, effective 1 Jan 2026,
  replacing hospital NPSGs):
  <https://www.jointcommission.org/en-us/standards/national-performance-goals>;
  NPSGs for other programmes:
  <https://www.jointcommission.org/en-us/standards/national-patient-safety-goals>
- IEC 60601-1-8 (alarm systems; 2006 edition plus Amd 1:2012 and Amd 2:2020):
  <https://webstore.iec.ch/publication/2598>
- IEC 62366-1:2015 (with Amd 1:2020): <https://webstore.iec.ch/publication/21863>
- ISO 9241-11:2018: <https://www.iso.org/standard/63500.html>
- ISO/TS 82304-2:2021: <https://www.iso.org/standard/78182.html>
- NIST SP 800-63-4 (Digital Identity Guidelines, final July 2025; 63A covers
  identity proofing): <https://pages.nist.gov/800-63-4>
- CMS HRRP:
  <https://www.cms.gov/Medicare/Medicare-Fee-for-Service-Payment/AcuteInpatientPPS/Readmissions-Reduction-Program>
- NCQA HEDIS (Plan All-Cause Readmissions; Transitions of Care): NCQA's
  required-measure list, <https://wpcdn.ncqa.org/www-prod/wp-content/uploads/2027-HPR-List-of-Required-Performance-Measures_March-2026_Final_3.27.2026.pdf>.
  Full specifications require an NCQA licence and were not read.
- CMS Part D adherence measures (PQA specification, PDC of at least 80%):
  confirmed via secondary sources only; see the CMS Star Ratings Technical
  Notes for the current year.
- NQF and the federal consensus-based entity: Federal Register notice
  <https://public-inspection.federalregister.gov/2026-13865.pdf> (Battelle as
  the CMS consensus-based entity).
- USCDI v7 (final, with Standards Bulletin 26-2): <https://healthit.gov/?p=215519>
- ONC certification criteria:
  <https://www.healthit.gov/topic/certification-ehrs/onc-certification-criteria-health-it-regulatory-update-deadline>
- ONC HTI-4 (NCPDP SCRIPT 2023011):
  <https://healthit.gov/resources/hti-4-final-rule-electronic-prescribing-real-time-prescription-benefit-and-electronic-prior-authorization/>
- ONC SAFER Guides: <https://www.healthit.gov/safer/>
- HL7 FHIR R4: <https://hl7.org/fhir/R4/questionnaire.html>,
  <https://hl7.org/fhir/R4/encounter.html>
- IHE ITI Technical Framework (PIX, PDQ, XDS): <https://profiles.ihe.net/ITI/TF/Volume1/ch-E.html>
- NHS England virtual wards operational framework:
  <https://www.england.nhs.uk/long-read/virtual-wards-operational-framework/>
- MGMA no-show data: <https://www.mgma.com/mgma-stat/no-show-fees-in-medical-practices-on-the-rise-to-balance-bumpy-attendance-rates>
- CMS RPM codes for CY2026 (final rule CMS-1832-F): vendor summaries only;
  confirm against the CMS Physician Fee Schedule.
- WHO digital health M&E guidance and MAPS toolkit:
  <https://www.who.int/observatories/global-observatory-on-health-research-and-development/analyses-and-syntheses/digital-health/who-s-work-in-digital-health-research>

### Not verified

- Specific Joint Commission NPG numbers (a secondary source cites 01.01.01
  patient identification and 01.05.01 alarm safety); none are listed above.
- The Emergency Severity Index's current steward (AHRQ versus the Emergency
  Nurses Association).
- PROMIS, RE-AIM, LOINC and SNOMED CT mappings, FHIR Appointment, Slot,
  ServiceRequest and Task resource pages, and ISO 13485 / FDA post-market
  surveillance: not checked against owner pages this pass.
- Exact CY2026 RPM payment amounts and the 16-day threshold language.
- `spec/frameworks-for-measuring-success.md` previously called a WHO framework
  the "WHO Digital Health Assessment Framework", for which no WHO source under
  that name was found; it now uses "WHO digital health monitoring and
  evaluation guidance (including the MAPS toolkit)".

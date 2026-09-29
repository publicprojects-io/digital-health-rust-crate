//! # Device Health Rate
//!
//! Device health rate covers the hardware layer beneath a digital health
//! programme — connected wearables, home hubs, and clinic devices: how much
//! of the expected time a device was up, how much of the expected data
//! actually arrived over the carrier or VPN link, and how often CPU or
//! memory sat above a saturation threshold.
//!
//! ## How it's calculated
//!
//! ```text
//! Device uptime rate = uptime hours / expected hours × 100
//!
//! Data transmission success rate = transmissions received / transmissions expected × 100
//!
//! Resource saturation rate = samples above threshold / samples × 100
//! ```
//!
//! ## Why it matters
//!
//! A monitoring programme is only as good as its devices: a device that is
//! off, out of coverage, or starved of memory silently produces gaps that
//! look like patient non-adherence. Tracking device health separately lets
//! teams tell a technical failure from a patient behaviour. It is the
//! device-level counterpart to [EHR system uptime
//! rate](crate::ehr_system_uptime_rate).
//!
//! ## Worked example
//!
//! A remote-monitoring hub fleet is expected to be up for 2,160 hours over
//! 90 days and logs 2,138.4 hours. It should send 10,000 readings and 9,600
//! arrive. Of 3,600 CPU samples, 180 exceed 90% utilisation.
//!
//! ```rust
//! use digital_health::device_health_rate::{device_uptime_rate, data_transmission_success_rate, resource_saturation_rate};
//!
//! // 2,138.4 / 2,160 × 100 = 99% uptime.
//! let device_uptime_rate_value = device_uptime_rate(2_138.4, 2_160.0).unwrap();
//! assert!((device_uptime_rate_value - 99.0).abs() < 1e-9);
//!
//! // 9,600 / 10,000 × 100 = 96% transmitted.
//! let data_transmission_success_rate_value = data_transmission_success_rate(9_600.0, 10_000.0).unwrap();
//! assert!((data_transmission_success_rate_value - 96.0).abs() < 1e-9);
//!
//! // 180 / 3,600 × 100 = 5% saturated.
//! let resource_saturation_rate_value = resource_saturation_rate(180.0, 3_600.0).unwrap();
//! assert!((resource_saturation_rate_value - 5.0).abs() < 1e-9);
//! ```
//!
//! ## Data sources and caveats
//!
//! Uptime and resource samples come from device telemetry or mobile-device-
//! management logs; expected transmissions come from the device's schedule.
//! Exclude planned maintenance windows from expected hours, consistently.
//!
//! ## Pitfalls
//!
//! - Reading a missing transmission as patient non-adherence — check device
//!   health first, as [remote patient monitoring adherence
//!   rate](crate::remote_patient_monitoring_adherence_rate) warns.
//! - Averaging away short saturation spikes — use a threshold and a sample
//!   share, not a mean.
//! - Counting a device that is up but unable to transmit as healthy.
//!
//! ## Sources
//!
//! - Device-management and telemetry guidance from remote patient
//!   monitoring platform vendors and MDM standards.
//! - FDA and IEC 62304 guidance on medical device software maintenance and
//!   monitoring.
//!
//! Independent topic: not present in the upstream [Digital Health
//! Metrics](https://github.com/digital-health-metrics/digital-health-metrics)
//! project at the time this module was written.

/// The device uptime rate: hours a device was operating as a percentage of
/// hours it was expected to operate.
///
/// # Arguments
///
/// * `uptime_hours` — hours the device was operating.
/// * `expected_hours` — hours the device was expected to operate, excluding
///   planned maintenance.
///
/// # Returns
///
/// `Some(percentage)`, or `None` if `expected_hours` is zero.
///
/// # Examples
///
/// ```rust
/// use digital_health::device_health_rate::device_uptime_rate;
///
/// // Worked example: 2,138.4 / 2,160 × 100 = 99%.
/// let value = device_uptime_rate(2_138.4, 2_160.0).unwrap();
/// assert!((value - 99.0).abs() < 1e-9);
///
/// assert!(device_uptime_rate(1.0, 0.0).is_none());
/// ```
#[must_use]
pub fn device_uptime_rate(uptime_hours: f64, expected_hours: f64) -> Option<f64> {
    if expected_hours == 0.0 {
        None
    } else {
        Some(uptime_hours / expected_hours * 100.0)
    }
}

/// The data transmission success rate: transmissions received as a
/// percentage of transmissions expected.
///
/// # Arguments
///
/// * `received` — transmissions received (count).
/// * `expected` — transmissions the device was scheduled to send (count).
///
/// # Returns
///
/// `Some(percentage)`, or `None` if `expected` is zero.
///
/// # Examples
///
/// ```rust
/// use digital_health::device_health_rate::data_transmission_success_rate;
///
/// // Worked example: 9,600 / 10,000 × 100 = 96%.
/// let value = data_transmission_success_rate(9_600.0, 10_000.0).unwrap();
/// assert!((value - 96.0).abs() < 1e-9);
///
/// assert!(data_transmission_success_rate(1.0, 0.0).is_none());
/// ```
#[must_use]
pub fn data_transmission_success_rate(received: f64, expected: f64) -> Option<f64> {
    if expected == 0.0 {
        None
    } else {
        Some(received / expected * 100.0)
    }
}

/// The resource saturation rate: samples with CPU or memory above a
/// threshold as a percentage of all samples.
///
/// # Arguments
///
/// * `samples_above_threshold` — samples with utilisation above the
///   threshold (count).
/// * `samples` — total utilisation samples taken (count).
///
/// # Returns
///
/// `Some(percentage)`, or `None` if `samples` is zero.
///
/// # Examples
///
/// ```rust
/// use digital_health::device_health_rate::resource_saturation_rate;
///
/// // Worked example: 180 / 3,600 × 100 = 5%.
/// let value = resource_saturation_rate(180.0, 3_600.0).unwrap();
/// assert!((value - 5.0).abs() < 1e-9);
///
/// assert!(resource_saturation_rate(1.0, 0.0).is_none());
/// ```
#[must_use]
pub fn resource_saturation_rate(samples_above_threshold: f64, samples: f64) -> Option<f64> {
    if samples == 0.0 {
        None
    } else {
        Some(samples_above_threshold / samples * 100.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Doc line: "2,138.4 / 2,160 × 100 = 99% uptime."
    #[test]
    fn worked_example_device_uptime_rate() {
        let value = device_uptime_rate(2_138.4, 2_160.0).unwrap();
        assert!((value - 99.0).abs() < 1e-9, "got {value}");
    }

    // Doc line: "9,600 / 10,000 × 100 = 96% transmitted."
    #[test]
    fn worked_example_data_transmission_success_rate() {
        let value = data_transmission_success_rate(9_600.0, 10_000.0).unwrap();
        assert!((value - 96.0).abs() < 1e-9, "got {value}");
    }

    // Doc line: "180 / 3,600 × 100 = 5% saturated."
    #[test]
    fn worked_example_resource_saturation_rate() {
        let value = resource_saturation_rate(180.0, 3_600.0).unwrap();
        assert!((value - 5.0).abs() < 1e-9, "got {value}");
    }

    #[test]
    fn zero_denominator_returns_none() {
        assert!(device_uptime_rate(1.0, 0.0).is_none());
        assert!(data_transmission_success_rate(1.0, 0.0).is_none());
        assert!(resource_saturation_rate(1.0, 0.0).is_none());
    }
}

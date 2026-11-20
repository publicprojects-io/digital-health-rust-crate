//! Reference validation: public functions checked against independently
//! derived values (published definitions, hand arithmetic, or the
//! NumPy/R type-7 percentile convention) rather than against the
//! functions' own output.

use digital_health::{
    appointment_no_show_rate::{
        combined_non_attendance_rate, late_cancellation_rate, no_show_rate,
    },
    net_promoter_score::{net_promoter_score, survey_response_rate},
    readmission_rate::{readmission_rate, readmission_reduction},
    return_on_investment::{benefit_cost_ratio, roi_percent},
    secure_messaging_response_time::{percentile, response_hours},
    system_usability_scale::{mean_sus_score, sus_score},
};

fn close(actual: Option<f64>, expected: f64) {
    let a = actual.expect("expected Some");
    assert!((a - expected).abs() < 1e-9, "got {a}, expected {expected}");
}

// --- System Usability Scale (Brooke 1996: odd items r-1, even items 5-r, sum x 2.5) ---

#[test]
fn sus_brooke_reference_patterns() {
    // Best case: strongly agree with every positive item, strongly disagree with every negative one.
    // Odd items 5 -> 4 each (5 x 4 = 20); even items 1 -> 4 each (20); 40 x 2.5 = 100.
    assert!((sus_score([5., 1., 5., 1., 5., 1., 5., 1., 5., 1.]) - 100.0).abs() < 1e-9);
    // Worst case: every contribution is 0.
    assert!(sus_score([1., 5., 1., 5., 1., 5., 1., 5., 1., 5.]).abs() < 1e-9);
    // All neutral (3): each item contributes 2; 10 x 2 = 20; 20 x 2.5 = 50.
    assert!((sus_score([3.; 10]) - 50.0).abs() < 1e-9);
    // All 4s: odd items 3 each (15), even items 1 each (5); 20 x 2.5 = 50.
    assert!((sus_score([4.; 10]) - 50.0).abs() < 1e-9);
    // Agree (4) on positive, disagree (2) on negative: every item contributes 3; 30 x 2.5 = 75.
    assert!((sus_score([4., 2., 4., 2., 4., 2., 4., 2., 4., 2.]) - 75.0).abs() < 1e-9);
    // Hand-scored: odd contributions 4+4+3+4+3 = 18, even 4+4+3+4+3 = 18; 36 x 2.5 = 90.
    assert!((sus_score([5., 1., 5., 1., 4., 2., 5., 1., 4., 2.]) - 90.0).abs() < 1e-9);
}

#[test]
fn sus_mean_and_zero_respondents() {
    // 4,080 / 60 = 68, the commonly cited benchmark average (Sauro).
    close(mean_sus_score(4_080.0, 60.0), 68.0);
    assert!(mean_sus_score(100.0, 0.0).is_none());
}

// --- Net Promoter Score (Reichheld 2003: %promoters - %detractors) ---

#[test]
fn nps_reference_values() {
    // 50 promoters, 30 passives, 20 detractors of 100: 50% - 20% = 30.
    close(net_promoter_score(50.0, 20.0, 100.0), 30.0);
    // Everyone a promoter: +100. Everyone a detractor: -100. Balanced: 0.
    close(net_promoter_score(100.0, 0.0, 100.0), 100.0);
    close(net_promoter_score(0.0, 100.0, 100.0), -100.0);
    close(net_promoter_score(40.0, 40.0, 100.0), 0.0);
    // Passives only dilute: 10 promoters, 0 detractors of 40 = 25.
    close(net_promoter_score(10.0, 0.0, 40.0), 25.0);
    assert!(net_promoter_score(1.0, 1.0, 0.0).is_none());
}

#[test]
fn survey_response_rate_values() {
    // 150 of 600 invited = 25%.
    close(survey_response_rate(150.0, 600.0), 25.0);
    assert!(survey_response_rate(1.0, 0.0).is_none());
}

// --- Percentile: linear interpolation, rank = p/100 x (n-1) (R type 7 / NumPy default) ---

#[test]
fn percentile_matches_type_7_references() {
    let one_to_ten: Vec<f64> = (1..=10).map(f64::from).collect();
    // Median of 1..10: rank 4.5 between 5 and 6 = 5.5.
    close(percentile(&one_to_ten, 50.0), 5.5);
    // p25: rank 2.25 -> 3 + 0.25 = 3.25. p90: rank 8.1 -> 9 + 0.1 = 9.1.
    close(percentile(&one_to_ten, 25.0), 3.25);
    close(percentile(&one_to_ten, 90.0), 9.1);
    close(percentile(&one_to_ten, 0.0), 1.0);
    close(percentile(&one_to_ten, 100.0), 10.0);
    // NumPy documentation-style example: [15, 20, 35, 40, 50], p40 -> rank 1.6 -> 20 + 0.6 x 15 = 29.
    close(percentile(&[15.0, 20.0, 35.0, 40.0, 50.0], 40.0), 29.0);
    // Input order must not matter: median of {1,2,3} = 2.
    close(percentile(&[3.0, 1.0, 2.0], 50.0), 2.0);
    // Single value is every percentile.
    close(percentile(&[7.0], 90.0), 7.0);
    assert!(percentile(&[], 50.0).is_none());
}

#[test]
fn percentile_clamps_out_of_range_p() {
    let data = [1.0, 2.0, 3.0];
    close(percentile(&data, -10.0), 1.0);
    close(percentile(&data, 250.0), 3.0);
}

#[test]
fn response_hours_is_difference() {
    assert!((response_hours(9.0, 12.5) - 3.5).abs() < 1e-9);
}

// --- Readmission (CMS HRRP-style rate = readmissions / index discharges) ---

#[test]
fn readmission_reference_values() {
    // 400 / 2,500 = 16%.
    close(readmission_rate(400.0, 2_500.0), 16.0);
    // (16 - 12) / 16 = 25% relative reduction; an increase is negative.
    close(readmission_reduction(16.0, 12.0), 25.0);
    close(readmission_reduction(10.0, 12.0), -20.0);
    assert!(readmission_rate(1.0, 0.0).is_none());
    assert!(readmission_reduction(0.0, 5.0).is_none());
}

// --- Appointment non-attendance ---

#[test]
fn no_show_reference_values() {
    close(no_show_rate(180.0, 2_000.0), 9.0);
    close(late_cancellation_rate(60.0, 2_000.0), 3.0);
    // Additivity: 9% + 3% = 12%.
    close(combined_non_attendance_rate(180.0, 60.0, 2_000.0), 12.0);
    assert!(no_show_rate(1.0, 0.0).is_none());
    assert!(late_cancellation_rate(1.0, 0.0).is_none());
    assert!(combined_non_attendance_rate(1.0, 1.0, 0.0).is_none());
}

// --- Return on investment (textbook: (benefit - cost) / cost) ---

#[test]
fn roi_reference_values() {
    // Spend 200, gain 300: ROI 50%, BCR 1.5. Break-even: ROI 0, BCR 1. Loss: ROI -50%.
    close(roi_percent(300.0, 200.0), 50.0);
    close(benefit_cost_ratio(300.0, 200.0), 1.5);
    close(roi_percent(200.0, 200.0), 0.0);
    close(benefit_cost_ratio(200.0, 200.0), 1.0);
    close(roi_percent(100.0, 200.0), -50.0);
    assert!(roi_percent(1.0, 0.0).is_none());
    assert!(benefit_cost_ratio(1.0, 0.0).is_none());
}

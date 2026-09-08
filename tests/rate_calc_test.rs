use theasus::diagnostics::rate::RateCalculator;

#[test]
fn test_rate_calculator_empty() {
    let calc = RateCalculator::new(64);
    let stats = calc.compute_stats();
    assert_eq!(stats.samples, 0);
    assert_eq!(stats.observed_hz, 0.0);
}

#[test]
fn test_rate_calculator_perfect_100hz() {
    let mut calc = RateCalculator::new(64);

    // Simulate 10ms intervals (10,000 microseconds) -> 100 Hz
    let mut t = 1_000_000;
    calc.record_event(t);

    for _ in 0..20 {
        t += 10_000;
        calc.record_event(t);
    }

    let stats = calc.compute_stats();
    assert_eq!(stats.samples, 20);
    assert!((stats.median_ms - 10.0).abs() < 0.01);
    assert!((stats.avg_ms - 10.0).abs() < 0.01);
    assert!((stats.observed_hz - 100.0).abs() < 0.5);
    assert!(stats.jitter_ms < 0.01);
}

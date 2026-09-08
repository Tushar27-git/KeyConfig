use std::collections::VecDeque;

#[derive(Debug, Clone, Default)]
pub struct RateStats {
    pub samples: usize,
    pub min_ms: f64,
    pub max_ms: f64,
    pub avg_ms: f64,
    pub median_ms: f64,
    pub jitter_ms: f64,
    pub observed_hz: f64,
}

pub struct RateCalculator {
    intervals: VecDeque<f64>,
    max_samples: usize,
    last_micros: Option<u64>,
}

impl RateCalculator {
    pub fn new(max_samples: usize) -> Self {
        Self {
            intervals: VecDeque::with_capacity(max_samples),
            max_samples,
            last_micros: None,
        }
    }

    /// Records a new physical event timestamp (in microseconds).
    pub fn record_event(&mut self, micros: u64) {
        if let Some(prev) = self.last_micros {
            if micros > prev {
                let delta_ms = (micros - prev) as f64 / 1000.0;
                // Exclude idle pauses (> 2000ms) or micro-glitches (< 0.05ms)
                if delta_ms > 0.05 && delta_ms < 2000.0 {
                    if self.intervals.len() >= self.max_samples {
                        self.intervals.pop_front();
                    }
                    self.intervals.push_back(delta_ms);
                }
            }
        }
        self.last_micros = Some(micros);
    }

    pub fn compute_stats(&self) -> RateStats {
        if self.intervals.is_empty() {
            return RateStats::default();
        }

        let n = self.intervals.len();
        let mut sorted: Vec<f64> = self.intervals.iter().copied().collect();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

        let min_ms = sorted[0];
        let max_ms = sorted[n - 1];
        let sum: f64 = sorted.iter().sum();
        let avg_ms = sum / n as f64;

        let median_ms = if n % 2 == 0 {
            (sorted[n / 2 - 1] + sorted[n / 2]) / 2.0
        } else {
            sorted[n / 2]
        };

        // Standard deviation (Jitter)
        let variance = if n > 1 {
            let var_sum: f64 = sorted.iter().map(|x| (x - avg_ms).powi(2)).sum();
            (var_sum / (n - 1) as f64).sqrt()
        } else {
            0.0
        };

        let observed_hz = if median_ms > 0.001 {
            1000.0 / median_ms
        } else {
            0.0
        };

        RateStats {
            samples: n,
            min_ms,
            max_ms,
            avg_ms,
            median_ms,
            jitter_ms: variance,
            observed_hz,
        }
    }

    pub fn reset(&mut self) {
        self.intervals.clear();
        self.last_micros = None;
    }
}

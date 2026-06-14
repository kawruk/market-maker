use ndarray::{Array1, Array2};
use ndarray_stats::QuantileExt;
use log::debug;

/// Dynamic Time Warping Pattern Matcher
pub struct PatternMatcher {
    min_pattern_length: usize,
    max_warping_window: f64,
}

impl PatternMatcher {
    pub fn new(min_length: usize, max_warping: f64) -> Self {
        Self {
            min_pattern_length: min_length,
            max_warping_window: max_warping,
        }
    }

    /// Calculate DTW distance between two sequences
    pub fn dtw_distance(&self, seq1: &[f64], seq2: &[f64]) -> f64 {
        let n = seq1.len();
        let m = seq2.len();

        if n == 0 || m == 0 {
            return f64::INFINITY;
        }

        let mut dtw = vec![vec![f64::INFINITY; m + 1]; n + 1];
        dtw[0][0] = 0.0;

        let window_size = ((n as f64) * self.max_warping_window) as usize;

        for i in 1..=n {
            let start = std::cmp::max(1, i as i32 - window_size as i32) as usize;
            let end = std::cmp::min(m + 1, i + window_size);

            for j in start..end {
                let cost = (seq1[i - 1] - seq2[j - 1]).abs();
                dtw[i][j] = cost + dtw[i - 1][j]
                    .min(dtw[i][j - 1])
                    .min(dtw[i - 1][j - 1]);
            }
        }

        dtw[n][m]
    }

    /// Normalize DTW distance to similarity score (0-1)
    pub fn dtw_similarity(&self, seq1: &[f64], seq2: &[f64]) -> f64 {
        let distance = self.dtw_distance(seq1, seq2);
        1.0 / (1.0 + distance)
    }

    /// Calculate Euclidean distance
    pub fn euclidean_distance(&self, seq1: &[f64], seq2: &[f64]) -> f64 {
        if seq1.len() != seq2.len() {
            return f64::INFINITY;
        }

        seq1.iter()
            .zip(seq2.iter())
            .map(|(a, b)| (a - b).powi(2))
            .sum::<f64>()
            .sqrt()
    }

    /// Calculate Pearson correlation coefficient
    pub fn pearson_correlation(&self, seq1: &[f64], seq2: &[f64]) -> f64 {
        if seq1.len() != seq2.len() || seq1.len() < 2 {
            return 0.0;
        }

        let mean1 = seq1.iter().sum::<f64>() / seq1.len() as f64;
        let mean2 = seq2.iter().sum::<f64>() / seq2.len() as f64;

        let mut numerator = 0.0;
        let mut sum1 = 0.0;
        let mut sum2 = 0.0;

        for (a, b) in seq1.iter().zip(seq2.iter()) {
            let dev1 = a - mean1;
            let dev2 = b - mean2;
            numerator += dev1 * dev2;
            sum1 += dev1.powi(2);
            sum2 += dev2.powi(2);
        }

        if sum1 > 0.0 && sum2 > 0.0 {
            numerator / (sum1.sqrt() * sum2.sqrt())
        } else {
            0.0
        }
    }

    /// Find similar patterns in historical data
    pub fn find_similar_patterns(
        &self,
        current_pattern: &[f64],
        historical_data: &[Vec<f64>],
        threshold: f64,
    ) -> Vec<(usize, f64)> {
        debug!("Finding similar patterns for current pattern of length {}", current_pattern.len());

        historical_data
            .iter()
            .enumerate()
            .filter_map(|(idx, hist_pattern)| {
                if hist_pattern.len() != current_pattern.len() {
                    return None;
                }

                let similarity = self.dtw_similarity(current_pattern, hist_pattern);

                if similarity > threshold {
                    debug!("Found similar pattern at index {} with similarity {}", idx, similarity);
                    Some((idx, similarity))
                } else {
                    None
                }
            })
            .collect()
    }

    /// Find rolling window patterns
    pub fn find_rolling_patterns(
        &self,
        data: &[f64],
        window_size: usize,
        threshold: f64,
    ) -> Vec<(usize, f64)> {
        let mut patterns = Vec::new();

        for i in window_size..data.len() {
            let current = &data[i - window_size..i];
            let mut best_match = (0, 0.0);

            for j in window_size..i - window_size {
                let historical = &data[j - window_size..j];
                let similarity = self.dtw_similarity(current, historical);

                if similarity > best_match.1 && similarity > threshold {
                    best_match = (j - window_size, similarity);
                }
            }

            if best_match.1 > threshold {
                patterns.push((i - window_size, best_match.1));
            }
        }

        patterns
    }

    /// Extract features from price pattern
    pub fn extract_features(&self, prices: &[f64]) -> Vec<f64> {
        let mut features = Vec::new();

        // Normalize prices
        if let Some(&min) = prices.iter().min_by(|a, b| a.partial_cmp(b).unwrap()) {
            if let Some(&max) = prices.iter().max_by(|a, b| a.partial_cmp(b).unwrap()) {
                let range = max - min;
                if range > 0.0 {
                    let normalized: Vec<f64> = prices
                        .iter()
                        .map(|p| (p - min) / range)
                        .collect();
                    features.extend(normalized);
                }
            }
        }

        // Add returns
        for i in 1..prices.len() {
            let ret = (prices[i] - prices[i - 1]) / prices[i - 1];
            features.push(ret);
        }

        features
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dtw_distance() {
        let matcher = PatternMatcher::new(5, 0.1);
        let seq1 = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let seq2 = vec![1.0, 2.0, 3.0, 4.0, 5.0];

        let dist = matcher.dtw_distance(&seq1, &seq2);
        assert_eq!(dist, 0.0);
    }

    #[test]
    fn test_pearson_correlation() {
        let matcher = PatternMatcher::new(5, 0.1);
        let seq1 = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let seq2 = vec![1.0, 2.0, 3.0, 4.0, 5.0];

        let corr = matcher.pearson_correlation(&seq1, &seq2);
        assert!((corr - 1.0).abs() < 0.001);
    }
}

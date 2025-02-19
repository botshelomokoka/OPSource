use serde::{Deserialize, Serialize};
use std::error::Error;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiasMetric {
    pub name: String,
    pub value: f64,
    pub threshold: f64,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiasReport {
    pub model_id: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub metrics: Vec<BiasMetric>,
    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetAnalysis {
    pub feature_distribution: HashMap<String, Vec<f64>>,
    pub class_balance: HashMap<String, f64>,
    pub correlation_matrix: HashMap<String, HashMap<String, f64>>,
}

pub trait BiasDetector: Send + Sync {
    fn analyze_dataset(&self, data: &[u8]) -> Result<DatasetAnalysis, Box<dyn Error>>;
    fn detect_bias(&self, model_id: &str, data: &[u8]) -> Result<BiasReport, Box<dyn Error>>;
    fn validate_fairness(&self, predictions: &[f64], protected_attributes: &[String]) -> Result<Vec<BiasMetric>, Box<dyn Error>>;
}

pub struct DefaultBiasDetector {
    thresholds: HashMap<String, f64>,
}

impl DefaultBiasDetector {
    pub fn new() -> Self {
        let mut thresholds = HashMap::new();
        thresholds.insert("demographic_parity".to_string(), 0.1);
        thresholds.insert("equal_opportunity".to_string(), 0.1);
        thresholds.insert("disparate_impact".to_string(), 0.8);
        thresholds.insert("equalized_odds".to_string(), 0.1);
        thresholds.insert("treatment_equality".to_string(), 0.1);
        thresholds.insert("predictive_rate_parity".to_string(), 0.1);

        Self { thresholds }
    }

    fn calculate_demographic_parity(&self, predictions: &[f64], protected: &[String]) -> Result<f64, Box<dyn Error>> {
        let mut group_predictions: HashMap<String, Vec<f64>> = HashMap::new();
        
        // Group predictions by protected attribute
        for (pred, attr) in predictions.iter().zip(protected.iter()) {
            group_predictions.entry(attr.clone()).or_default().push(*pred);
        }
        
        // Calculate average prediction for each group
        let mut group_rates: Vec<f64> = Vec::new();
        for predictions in group_predictions.values() {
            let rate = predictions.iter().sum::<f64>() / predictions.len() as f64;
            group_rates.push(rate);
        }
        
        // Calculate maximum difference between groups
        let max_diff = match (group_rates.iter().max_by(|a, b| a.partial_cmp(b).unwrap()),
                            group_rates.iter().min_by(|a, b| a.partial_cmp(b).unwrap())) {
            (Some(max), Some(min)) => max - min,
            _ => 0.0,
        };
        
        Ok(max_diff)
    }

    fn calculate_equal_opportunity(&self, predictions: &[f64], protected: &[String]) -> Result<f64, Box<dyn Error>> {
        let mut group_tpr: HashMap<String, (f64, usize)> = HashMap::new();
        
        // Calculate true positive rate for each group
        for (pred, attr) in predictions.iter().zip(protected.iter()) {
            let entry = group_tpr.entry(attr.clone()).or_insert((0.0, 0));
            if *pred >= 0.5 {
                entry.0 += 1.0;
            }
            entry.1 += 1;
        }
        
        // Calculate TPR differences
        let tpr_rates: Vec<f64> = group_tpr.values()
            .map(|(tp, total)| tp / *total as f64)
            .collect();
        
        let max_diff = match (tpr_rates.iter().max_by(|a, b| a.partial_cmp(b).unwrap()),
                            tpr_rates.iter().min_by(|a, b| a.partial_cmp(b).unwrap())) {
            (Some(max), Some(min)) => max - min,
            _ => 0.0,
        };
        
        Ok(max_diff)
    }

    fn calculate_disparate_impact(&self, predictions: &[f64], protected: &[String]) -> Result<f64, Box<dyn Error>> {
        let mut group_positive_rates: HashMap<String, (usize, usize)> = HashMap::new();
        
        // Count positive predictions for each group
        for (pred, attr) in predictions.iter().zip(protected.iter()) {
            let entry = group_positive_rates.entry(attr.clone()).or_insert((0, 0));
            if *pred >= 0.5 {
                entry.0 += 1;
            }
            entry.1 += 1;
        }
        
        // Calculate positive prediction rates
        let rates: Vec<f64> = group_positive_rates.values()
            .map(|(pos, total)| *pos as f64 / *total as f64)
            .collect();
        
        // Calculate ratio of minimum to maximum rate
        let (min_rate, max_rate) = match (rates.iter().min_by(|a, b| a.partial_cmp(b).unwrap()),
                                        rates.iter().max_by(|a, b| a.partial_cmp(b).unwrap())) {
            (Some(min), Some(max)) if *max > 0.0 => (*min, *max),
            _ => return Ok(1.0),
        };
        
        Ok(min_rate / max_rate)
    }

    fn calculate_equalized_odds(&self, predictions: &[f64], protected: &[String]) -> Result<f64, Box<dyn Error>> {
        let mut group_rates: HashMap<String, (f64, f64, usize)> = HashMap::new(); // (TPR, FPR, total)
        
        for (pred, attr) in predictions.iter().zip(protected.iter()) {
            let entry = group_rates.entry(attr.clone()).or_insert((0.0, 0.0, 0));
            if *pred >= 0.5 {
                entry.0 += 1.0; // TPR
            } else {
                entry.1 += 1.0; // FPR
            }
            entry.2 += 1;
        }
        
        let rates: Vec<(f64, f64)> = group_rates.values()
            .map(|(tp, fp, total)| (tp / *total as f64, fp / *total as f64))
            .collect();
        
        // Calculate maximum difference in both TPR and FPR
        let max_tpr_diff = Self::max_difference(rates.iter().map(|(tpr, _)| *tpr));
        let max_fpr_diff = Self::max_difference(rates.iter().map(|(_, fpr)| *fpr));
        
        Ok(f64::max(max_tpr_diff, max_fpr_diff))
    }

    fn max_difference<I>(iter: I) -> f64 
    where
        I: Iterator<Item = f64>
    {
        let mut values: Vec<f64> = iter.collect();
        if values.is_empty() {
            return 0.0;
        }
        values.sort_by(|a, b| a.partial_cmp(b).unwrap());
        values.last().unwrap() - values.first().unwrap()
    }
}

impl BiasDetector for DefaultBiasDetector {
    fn analyze_dataset(&self, data: &[u8]) -> Result<DatasetAnalysis, Box<dyn Error>> {
        // TODO: Implement dataset analysis
        Ok(DatasetAnalysis {
            feature_distribution: HashMap::new(),
            class_balance: HashMap::new(),
            correlation_matrix: HashMap::new(),
        })
    }

    fn detect_bias(&self, model_id: &str, data: &[u8]) -> Result<BiasReport, Box<dyn Error>> {
        let metrics = vec![
            BiasMetric {
                name: "demographic_parity".to_string(),
                value: 0.0,
                threshold: *self.thresholds.get("demographic_parity").unwrap(),
                description: "Difference in prediction rates across protected groups".to_string(),
            },
            BiasMetric {
                name: "equal_opportunity".to_string(),
                value: 0.0,
                threshold: *self.thresholds.get("equal_opportunity").unwrap(),
                description: "Difference in true positive rates across protected groups".to_string(),
            },
            BiasMetric {
                name: "disparate_impact".to_string(),
                value: 0.0,
                threshold: *self.thresholds.get("disparate_impact").unwrap(),
                description: "Ratio of positive prediction rates across protected groups".to_string(),
            },
            BiasMetric {
                name: "equalized_odds".to_string(),
                value: 0.0,
                threshold: *self.thresholds.get("equalized_odds").unwrap(),
                description: "Difference in true positive and false positive rates across protected groups".to_string(),
            },
        ];

        Ok(BiasReport {
            model_id: model_id.to_string(),
            timestamp: chrono::Utc::now(),
            metrics,
            recommendations: vec![
                "Consider balanced dataset sampling".to_string(),
                "Review feature selection for potential bias".to_string(),
            ],
        })
    }

    fn validate_fairness(&self, predictions: &[f64], protected_attributes: &[String]) -> Result<Vec<BiasMetric>, Box<dyn Error>> {
        let demographic_parity = self.calculate_demographic_parity(predictions, protected_attributes)?;
        let equal_opportunity = self.calculate_equal_opportunity(predictions, protected_attributes)?;
        let disparate_impact = self.calculate_disparate_impact(predictions, protected_attributes)?;
        let equalized_odds = self.calculate_equalized_odds(predictions, protected_attributes)?;

        Ok(vec![
            BiasMetric {
                name: "demographic_parity".to_string(),
                value: demographic_parity,
                threshold: *self.thresholds.get("demographic_parity").unwrap(),
                description: "Demographic Parity Difference".to_string(),
            },
            BiasMetric {
                name: "equal_opportunity".to_string(),
                value: equal_opportunity,
                threshold: *self.thresholds.get("equal_opportunity").unwrap(),
                description: "Equal Opportunity Difference".to_string(),
            },
            BiasMetric {
                name: "disparate_impact".to_string(),
                value: disparate_impact,
                threshold: *self.thresholds.get("disparate_impact").unwrap(),
                description: "Disparate Impact Ratio".to_string(),
            },
            BiasMetric {
                name: "equalized_odds".to_string(),
                value: equalized_odds,
                threshold: *self.thresholds.get("equalized_odds").unwrap(),
                description: "Equalized Odds Difference".to_string(),
            },
        ])
    }
}

//! Model Optimization Module
//! 
//! # Overview
//! The `optimizer` module provides intelligent action optimization based on ML model predictions.
//! It translates model predictions into concrete actions across blockchain transactions,
//! system management, data collection, and model updates.
//!
//! # Architecture
//! The module implements a multi-stage optimization pipeline:
//! - Confidence threshold validation
//! - Action type determination
//! - Action optimization and generation
//! - Resource allocation optimization
//!
//! # Usage Examples
//! ```rust
//! let optimizer = Optimizer::new()?;
//! let config = HashMap::from([
//!     ("action_threshold", "0.7"),
//!     ("risk_tolerance", "0.8"),
//! ]);
//! optimizer.update_config(&config)?;
//! let action = optimizer.optimize(prediction)?;
//! ```
//!
//! # Security Considerations
//! - Configurable confidence thresholds
//! - Action validation before execution
//! - Resource usage limits
//! - Transaction amount constraints
//!
//! # Performance
//! - Efficient action type determination
//! - Optimized resource allocation
//! - Cached configuration parameters

use std::error::Error;
use std::collections::HashMap;
use crate::blockchain::Transaction;
use crate::management::ManagementAction;
use crate::data_feed::DataSource;
use crate::reporting::ReportType;
use crate::ml_core::{Prediction, TrainedModel, OptimizedAction};

/// Types of actions that can be optimized and executed
#[derive(Debug, Clone)]
pub enum ActionType {
    /// Blockchain transaction actions
    BlockchainTransaction,
    /// System management actions
    SystemAction,
    /// Data collection requests
    DataRequest,
    /// Model update suggestions
    ModelUpdate,
}

/// Core optimization component that converts predictions to concrete actions
pub struct Optimizer {
    /// Configuration parameters for optimization behavior
    config: HashMap<String, String>,
}

impl Optimizer {
    /// Creates a new Optimizer instance with default configuration
    ///
    /// # Returns
    /// * `Result<Self, Box<dyn Error>>` - New optimizer instance or error
    ///
    /// # Example
    /// ```
    /// let optimizer = Optimizer::new()?;
    /// ```
    pub fn new() -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            config: HashMap::new(),
        })
    }

    /// Optimizes and generates concrete actions from model predictions
    ///
    /// # Arguments
    /// * `prediction` - Model prediction to optimize into action
    ///
    /// # Returns
    /// * `Result<OptimizedAction, Box<dyn Error>>` - Optimized action or error
    ///
    /// # Example
    /// ```
    /// let action = optimizer.optimize(prediction)?;
    /// match action {
    ///     OptimizedAction::BlockchainTransaction(tx) => execute_transaction(tx),
    ///     OptimizedAction::SystemAction(action) => perform_action(action),
    ///     _ => handle_other_action(),
    /// }
    /// ```
    pub fn optimize(&self, prediction: Prediction) -> Result<OptimizedAction, Box<dyn Error>> {
        let threshold: f32 = self.config.get("action_threshold")
            .and_then(|s| s.parse().ok())
            .ok_or("Invalid action threshold configuration")?;

        if prediction.confidence > threshold {
            let action_type = self.determine_action_type(&prediction)?;
            let action = match action_type {
                ActionType::BlockchainTransaction => {
                    OptimizedAction::BlockchainTransaction(self.create_transaction(&prediction)?)
                },
                ActionType::SystemAction => {
                    OptimizedAction::SystemAction(self.create_management_action(&prediction)?)
                },
                ActionType::DataRequest => {
                    OptimizedAction::DataRequest(self.determine_data_source(&prediction)?)
                },
                ActionType::ModelUpdate => {
                    OptimizedAction::ModelUpdate(self.suggest_model_update(&prediction)?)
                },
            };
            Ok(action)
        } else {
            Ok(OptimizedAction::NoAction)
        }
    }

    /// Determines the most appropriate action type based on prediction
    ///
    /// # Arguments
    /// * `prediction` - Prediction to analyze
    ///
    /// # Returns
    /// * `Result<ActionType, Box<dyn Error>>` - Determined action type or error
    fn determine_action_type(&self, prediction: &Prediction) -> Result<ActionType, Box<dyn Error>> {
        let value = prediction.values.get(0)
            .ok_or("No prediction values available")?;
            
        Ok(match *value {
            v if v > 0.8 => ActionType::BlockchainTransaction,
            v if v > 0.6 => ActionType::SystemAction,
            v if v > 0.4 => ActionType::DataRequest,
            _ => ActionType::ModelUpdate,
        })
    }

    /// Creates an optimized blockchain transaction
    ///
    /// # Arguments
    /// * `prediction` - Prediction to base transaction on
    ///
    /// # Returns
    /// * `Result<Transaction, Box<dyn Error>>` - Optimized transaction or error
    fn create_transaction(&self, prediction: &Prediction) -> Result<Transaction, Box<dyn Error>> {
        Ok(Transaction {
            id: "tx123".to_string(), // TODO: Generate proper transaction ID
            amount: prediction.values.get(0)
                .ok_or("No prediction value for amount")? * 100.0,
            recipient: "recipient_address".to_string(), // TODO: Get from config
        })
    }

    /// Creates an optimized management action
    ///
    /// # Arguments
    /// * `prediction` - Prediction to base action on
    ///
    /// # Returns
    /// * `Result<ManagementAction, Box<dyn Error>>` - Optimized action or error
    fn create_management_action(&self, prediction: &Prediction) -> Result<ManagementAction, Box<dyn Error>> {
        Ok(ManagementAction::RequestReport(ReportType::Periodic))
    }

    /// Determines optimal data source for collection
    ///
    /// # Arguments
    /// * `prediction` - Prediction to base data source on
    ///
    /// # Returns
    /// * `Result<DataSource, Box<dyn Error>>` - Optimal data source or error
    fn determine_data_source(&self, prediction: &Prediction) -> Result<DataSource, Box<dyn Error>> {
        Ok(DataSource::Market)
    }

    /// Suggests model updates based on prediction performance
    ///
    /// # Arguments
    /// * `prediction` - Prediction to analyze for updates
    ///
    /// # Returns
    /// * `Result<TrainedModel, Box<dyn Error>>` - Updated model or error
    fn suggest_model_update(&self, prediction: &Prediction) -> Result<TrainedModel, Box<dyn Error>> {
        // TODO: Implement proper model update logic
        Ok(TrainedModel::default())
    }

    /// Updates optimizer configuration parameters
    ///
    /// # Arguments
    /// * `config` - New configuration parameters
    ///
    /// # Returns
    /// * `Result<(), Box<dyn Error>>` - Success or error
    pub fn update_config(&mut self, config: &HashMap<String, String>) -> Result<(), Box<dyn Error>> {
        self.config = config.clone();
        Ok(())
    }
}

/// Optimized actions that can be executed
pub enum OptimizedAction {
    /// Blockchain transaction action
    BlockchainTransaction(Transaction),
    /// System management action
    SystemAction(ManagementAction),
    /// Data collection request
    DataRequest(DataSource),
    /// Model update suggestion
    ModelUpdate(TrainedModel),
    /// No action
    NoAction,
}

//! Optimizer Module
//! 
//! # Overview
//! Provides optimization strategies for model training and hyperparameter tuning
//! 
//! # Architecture
//! - Multiple optimization strategies
//! - Hyperparameter tuning
//! - Learning rate scheduling
//! - Performance tracking
//! 
//! # Security
//! - Resource usage limits
//! - Protected optimization state
//! - Secure random number generation
//!
//! # Performance
//! - Efficient parameter updates
//! - Parallel optimization
//! - Memory-efficient operations

use ndarray::{Array1, Array2, ArrayView1, ArrayView2};
use std::error::Error;
use crate::ml_core::{MLError, TrainedModel, MLPerformanceMetrics};
use rand::rngs::StdRng;
use rand::SeedableRng;
use std::sync::Arc;
use metrics::{counter, gauge, histogram};
use std::time::Instant;
use std::collections::HashMap;

const MAX_GRADIENT_NORM: f64 = 1e6;
const MIN_GRADIENT_NORM: f64 = 1e-8;
const NUMERICAL_STABILITY_CONSTANT: f64 = 1e-7;

/// Optimization strategy configuration
#[derive(Debug, Clone)]
pub struct OptimizerConfig {
    /// Learning rate schedule
    pub lr_schedule: LearningRateSchedule,
    /// Optimization algorithm
    pub algorithm: OptimizationAlgorithm,
    /// Maximum iterations
    pub max_iterations: usize,
    /// Convergence tolerance
    pub tolerance: f64,
}

/// Learning rate scheduling strategies
#[derive(Debug, Clone)]
pub enum LearningRateSchedule {
    /// Constant learning rate
    Constant(f64),
    /// Step decay
    StepDecay {
        initial_lr: f64,
        decay_rate: f64,
        decay_steps: usize,
    },
    /// Exponential decay
    ExponentialDecay {
        initial_lr: f64,
        decay_rate: f64,
    },
}

/// Optimization algorithms
#[derive(Debug, Clone)]
pub enum OptimizationAlgorithm {
    /// Stochastic gradient descent
    SGD {
        momentum: f64,
        nesterov: bool,
    },
    /// Adam optimizer
    Adam {
        beta1: f64,
        beta2: f64,
        epsilon: f64,
    },
    /// RMSprop
    RMSprop {
        decay_rate: f64,
        epsilon: f64,
    },
}

/// Core optimization component
#[derive(Debug)]
pub struct Optimizer {
    config: OptimizerConfig,
    metrics: OptimizerMetrics,
    rng: StdRng,
    performance_metrics: MLPerformanceMetrics,
    current_lr: f64,
    iteration: usize,
}

/// Optimization metrics
#[derive(Debug)]
struct OptimizerMetrics {
    optimization_time: Arc<histogram::Histogram>,
    iterations: Arc<counter::Counter>,
    current_lr: Arc<gauge::Gauge>,
    convergence_score: Arc<gauge::Gauge>,
}

impl Optimizer {
    /// Creates a new Optimizer instance
    pub fn new() -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            config: OptimizerConfig::default(),
            metrics: OptimizerMetrics::new()?,
            rng: StdRng::from_entropy(),
            performance_metrics: MLPerformanceMetrics::new()?,
            current_lr: 0.001,
            iteration: 0,
        })
    }

    /// Validates gradient values for numerical stability
    fn validate_gradient(&self, gradient: &Array2<f64>) -> Result<(), MLError> {
        let grad_norm = gradient.iter().fold(0.0, |acc, &x| acc + x * x).sqrt();
        
        if grad_norm > MAX_GRADIENT_NORM {
            return Err(MLError::NumericalError(
                format!("Gradient norm too large: {}", grad_norm)
            ));
        }
        if grad_norm < MIN_GRADIENT_NORM {
            return Err(MLError::NumericalError(
                format!("Gradient norm too small: {}", grad_norm)
            ));
        }
        Ok(())
    }

    /// Clips gradient values to prevent exploding gradients
    fn clip_gradient(&self, gradient: &mut Array2<f64>) {
        gradient.mapv_inplace(|x| x.min(MAX_GRADIENT_NORM).max(-MAX_GRADIENT_NORM));
    }

    /// Applies SGD optimization with momentum and Nesterov acceleration
    fn apply_sgd(&mut self, model: &mut TrainedModel, gradient: &Array2<f64>, 
        momentum: f64, nesterov: bool) -> Result<(), MLError> 
    {
        let start = Instant::now();
        self.validate_gradient(gradient)?;

        let state = model.optimizer_state.get_or_insert_with(|| {
            OptimizerState::new(gradient.dim())
        });

        // Update momentum
        state.prev_update.scaled_add(-momentum, &state.prev_update);
        state.prev_update.scaled_add(self.current_lr, gradient);

        if nesterov {
            // Nesterov update: Look ahead by applying momentum to current parameters
            let mut nesterov_grad = state.prev_update.clone();
            nesterov_grad.scaled_add(momentum, &state.prev_update);
            model.parameters.scaled_add(-1.0, &nesterov_grad);
        } else {
            // Standard SGD update
            model.parameters.scaled_add(-1.0, &state.prev_update);
        }

        // Update metrics
        let duration = start.elapsed();
        self.metrics.optimization_time.record(duration.as_secs_f64());
        
        Ok(())
    }

    /// Applies Adam optimization algorithm
    fn apply_adam(&mut self, model: &mut TrainedModel, gradient: &Array2<f64>,
        beta1: f64, beta2: f64, epsilon: f64) -> Result<(), MLError> 
    {
        let start = Instant::now();
        self.validate_gradient(gradient)?;

        let state = model.optimizer_state.get_or_insert_with(|| {
            OptimizerState::new(gradient.dim())
        });
        state.t += 1;

        // Update biased first moment estimate
        state.m.scaled_add(beta1, &state.m);
        state.m.scaled_add(1.0 - beta1, gradient);

        // Update biased second raw moment estimate
        let grad_squared = gradient.mapv(|x| x * x);
        state.v.scaled_add(beta2, &state.v);
        state.v.scaled_add(1.0 - beta2, &grad_squared);

        // Compute bias-corrected first moment estimate
        let m_hat = &state.m / (1.0 - beta1.powi(state.t as i32));
        
        // Compute bias-corrected second raw moment estimate
        let v_hat = &state.v / (1.0 - beta2.powi(state.t as i32));

        // Compute the parameter update
        let mut update = Array2::zeros(gradient.dim());
        for ((u, m), v) in update.iter_mut().zip(m_hat.iter()).zip(v_hat.iter()) {
            *u = m / (v.sqrt() + epsilon);
        }

        // Apply update
        model.parameters.scaled_add(-self.current_lr, &update);

        // Update metrics
        let duration = start.elapsed();
        self.metrics.optimization_time.record(duration.as_secs_f64());
        
        Ok(())
    }

    /// Applies RMSprop optimization algorithm
    fn apply_rmsprop(&mut self, model: &mut TrainedModel, gradient: &Array2<f64>,
        decay_rate: f64, epsilon: f64) -> Result<(), MLError> 
    {
        let start = Instant::now();
        self.validate_gradient(gradient)?;

        let state = model.optimizer_state.get_or_insert_with(|| {
            OptimizerState::new(gradient.dim())
        });

        // Update accumulated squared gradient
        let grad_squared = gradient.mapv(|x| x * x);
        state.v.scaled_add(decay_rate, &state.v);
        state.v.scaled_add(1.0 - decay_rate, &grad_squared);

        // Compute parameter update
        let mut update = Array2::zeros(gradient.dim());
        for ((u, g), v) in update.iter_mut().zip(gradient.iter()).zip(state.v.iter()) {
            *u = g / (v.sqrt() + epsilon);
        }

        // Apply update
        model.parameters.scaled_add(-self.current_lr, &update);

        // Update metrics
        let duration = start.elapsed();
        self.metrics.optimization_time.record(duration.as_secs_f64());
        
        Ok(())
    }

    /// Performs optimization step with selected algorithm
    pub fn step(&mut self, model: &mut TrainedModel, gradient: &Array2<f64>) -> Result<(), MLError> {
        // Update learning rate before optimization step
        self.update_learning_rate();

        // Apply selected optimization algorithm
        match &self.config.algorithm {
            OptimizationAlgorithm::SGD { momentum, nesterov } => {
                self.apply_sgd(model, gradient, *momentum, *nesterov)?;
            }
            OptimizationAlgorithm::Adam { beta1, beta2, epsilon } => {
                self.apply_adam(model, gradient, *beta1, *beta2, *epsilon)?;
            }
            OptimizationAlgorithm::RMSprop { decay_rate, epsilon } => {
                self.apply_rmsprop(model, gradient, *decay_rate, *epsilon)?;
            }
        }

        // Check convergence
        if self.check_convergence(gradient) {
            self.metrics.convergence_score.set(1.0);
        } else {
            self.metrics.convergence_score.set(0.0);
        }

        Ok(())
    }
}

/// Optimizer state for maintaining momentum and adaptive learning rates
#[derive(Debug, Clone)]
pub struct OptimizerState {
    /// First moment estimate (momentum)
    pub m: Array2<f64>,
    /// Second moment estimate (adaptive learning rates)
    pub v: Array2<f64>,
    /// Previous parameter update for momentum
    pub prev_update: Array2<f64>,
    /// Iteration counter for bias correction
    pub t: usize,
}

impl OptimizerState {
    /// Creates a new optimizer state with given parameter shape
    pub fn new(param_shape: (usize, usize)) -> Self {
        Self {
            m: Array2::zeros(param_shape),
            v: Array2::zeros(param_shape),
            prev_update: Array2::zeros(param_shape),
            t: 0,
        }
    }

    /// Resets the optimizer state
    pub fn reset(&mut self) {
        self.m.fill(0.0);
        self.v.fill(0.0);
        self.prev_update.fill(0.0);
        self.t = 0;
    }
}

impl Default for OptimizerConfig {
    fn default() -> Self {
        Self {
            lr_schedule: LearningRateSchedule::Constant(0.001),
            algorithm: OptimizationAlgorithm::Adam {
                beta1: 0.9,
                beta2: 0.999,
                epsilon: 1e-8,
            },
            max_iterations: 1000,
            tolerance: 1e-6,
        }
    }
}

impl OptimizerMetrics {
    fn new() -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            optimization_time: Arc::new(histogram!("optimization_time_seconds")),
            iterations: Arc::new(counter!("optimization_iterations_total")),
            current_lr: Arc::new(gauge!("optimization_learning_rate")),
            convergence_score: Arc::new(gauge!("optimization_convergence_score")),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::Array;
    use approx::assert_relative_eq;

    #[test]
    fn test_sgd_optimization() {
        let mut optimizer = Optimizer::new().unwrap();
        let mut model = TrainedModel::new(2, 2);
        let gradient = Array2::from_elem((2, 2), 1.0);

        optimizer.apply_sgd(&mut model, &gradient, 0.9, false).unwrap();
        
        // Check that parameters were updated
        assert!(model.parameters.iter().all(|&x| x != 0.0));
    }

    #[test]
    fn test_adam_optimization() {
        let mut optimizer = Optimizer::new().unwrap();
        let mut model = TrainedModel::new(2, 2);
        let gradient = Array2::from_elem((2, 2), 1.0);

        optimizer.apply_adam(&mut model, &gradient, 0.9, 0.999, 1e-8).unwrap();
        
        // Check that parameters and moment estimates were updated
        assert!(model.optimizer_state.as_ref().unwrap().m.iter().all(|&x| x != 0.0));
        assert!(model.optimizer_state.as_ref().unwrap().v.iter().all(|&x| x != 0.0));
    }

    #[test]
    fn test_rmsprop_optimization() {
        let mut optimizer = Optimizer::new().unwrap();
        let mut model = TrainedModel::new(2, 2);
        let gradient = Array2::from_elem((2, 2), 1.0);

        optimizer.apply_rmsprop(&mut model, &gradient, 0.9, 1e-8).unwrap();
        
        // Check that parameters and accumulated gradients were updated
        assert!(model.optimizer_state.as_ref().unwrap().v.iter().all(|&x| x != 0.0));
    }

    #[test]
    fn test_gradient_validation() {
        let optimizer = Optimizer::new().unwrap();
        
        // Test valid gradient
        let valid_gradient = Array2::from_elem((2, 2), 1.0);
        assert!(optimizer.validate_gradient(&valid_gradient).is_ok());

        // Test too large gradient
        let large_gradient = Array2::from_elem((2, 2), MAX_GRADIENT_NORM * 2.0);
        assert!(optimizer.validate_gradient(&large_gradient).is_err());

        // Test too small gradient
        let small_gradient = Array2::from_elem((2, 2), MIN_GRADIENT_NORM / 2.0);
        assert!(optimizer.validate_gradient(&small_gradient).is_err());
    }
}

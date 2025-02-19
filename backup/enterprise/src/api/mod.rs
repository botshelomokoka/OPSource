mod enterprise_endpoints;
mod enterprise_integration;

pub use enterprise_endpoints::*;
pub use enterprise_integration::*;

use crate::EnterpriseError;
use metrics::{counter, histogram};
use log::{info, error};

pub trait EnterpriseEndpoint {
    fn handle_request(&self) -> Result<(), EnterpriseError>;
    fn get_metrics(&self) -> Vec<(&'static str, f64)>;
}

pub trait EnterpriseIntegration {
    fn integrate(&self) -> Result<(), EnterpriseError>;
    fn health_check(&self) -> bool;
}
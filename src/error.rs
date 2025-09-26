use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Error)]
pub enum DomainError {
    #[error("Invalid employee ID: {0}")]
    InvalidEmployeeId(String),

    #[error("Invalid timestamp: {0}")]
    InvalidTimestamp(String),

    #[error("Business rule violation: {0}")]
    BusinessRuleViolation(String),
}
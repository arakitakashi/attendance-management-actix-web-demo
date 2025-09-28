use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Error)]
pub enum EmployeeIdError {
    #[error("Employee ID cannot be empty")]
    Empty,
}

#[derive(Debug, Clone, PartialEq, Error)]
pub enum TimeError {
    #[error("Future timestamp not allowed")]
    FutureTimeStamp,
}

#[derive(Debug, Clone, PartialEq, Error)]
pub enum AttendanceRecordError {
    #[error("Invalid attendance record: {0}")]
    Invalid(String),

    #[error("Clock out time is bedore clock in time")]
    InvalidTimeOrder,
}

#[derive(Debug, Clone, PartialEq, Error)]
pub enum DatabaseError {
    #[error("Connection failed: {0}")]
    ConnectionFailed(String),

    #[error("Query execution failed: {0}")]
    QueryFailed(String),

    #[error("Serialization failed: {0}")]
    SerializationFailed(String),
}

#[derive(Debug, Clone, PartialEq, Error)]
pub enum AttendanceWorkflowError {
    #[error("Employee validation failed: {0}")]
    Employee(#[from] EmployeeIdError),

    #[error("Time validation failed: {0}")]
    Time(#[from] TimeError),

    #[error("Attendance record validation failed: {0}")]
    AttendanceRecord(#[from] AttendanceRecordError),

    #[error("Database operation failed: {0}")]
    Database(#[from] DatabaseError),
}


#[derive(Debug, Clone, PartialEq, Error)]
pub enum DomainError {
    #[error("Invalid employee ID: {0}")]
    InvalidEmployeeId(String),

    #[error("Invalid timestamp: {0}")]
    InvalidTimestamp(String),

    #[error("Business rule violation: {0}")]
    BusinessRuleViolation(String),
}
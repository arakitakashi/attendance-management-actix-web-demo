use chrono::{DateTime, FixedOffset};

#[derive(Debug, Clone, PartialEq)]
pub struct EmployeeId(String);

impl EmployeeId {
    pub fn new(value: String) -> Result<Self, String> {
        if value.is_empty() {
            return Err("Employee ID cannot be empty".to_string())
        }

        Ok(EmployeeId(value))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ClockTime(DateTime<FixedOffset>);

impl ClockTime {
    pub fn new(timestamp: DateTime<FixedOffset>) -> Result<Self, String> {
        let now = chrono::Utc::now().with_timezone(&timestamp.timezone());
        if timestamp > now {
            return Err("Future timestamp not allowed".to_string());
        }

        Ok(ClockTime(timestamp))
    }

    pub fn value(&self) -> DateTime<FixedOffset> {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_employee_id_can_be_created_with_valid_value() {
        // Arrange
        let valid_id = "ij09080022".to_string();

        // Act
        let result = EmployeeId::new(valid_id);

        // Assert
        assert!(result.is_ok());
        let employee_id = result.unwrap();
        assert_eq!(employee_id.value(), "ij09080022");
    }

    #[test]
    fn test_employee_id_can_be_created_with_invalid_value() {
        // Arrange
        let empty_id = "".to_string();

        // Act
        let result = EmployeeId::new(empty_id);

        // Assert
        assert!(result.is_err());
    }

    #[test]
    fn test_clock_time_can_be_created_with_valid_timestamp() {
        // Arrange
        let timestamp = chrono::Utc::now().with_timezone(&chrono::FixedOffset::east_opt(9 * 3600).unwrap());

        // Act
        let result = ClockTime::new(timestamp);

        // Assert
        assert!(result.is_ok());
        let clock_time = result.unwrap();
        assert_eq!(clock_time.value(), timestamp);
    }

    #[test]
    fn test_clock_time_cannot_be_created_with_future_timestamp() {
        // Arrange
        let future_timestamp = chrono::Utc::now().with_timezone(&chrono::FixedOffset::east_opt(9 * 3600).unwrap()) + chrono::Duration::hours(1);

        // Act
        let result = ClockTime::new(future_timestamp);

        // Assert
        assert!(result.is_err());
    }
}


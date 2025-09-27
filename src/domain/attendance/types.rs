use chrono::{DateTime, FixedOffset, NaiveDate};

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

#[derive(Debug, Clone, PartialEq)]
pub struct WorkDate(NaiveDate);

impl WorkDate {
    pub fn from_timestamp(timestamp: DateTime<FixedOffset>) -> Self {
        WorkDate(timestamp.date_naive())
    }

    pub fn value(&self) -> NaiveDate {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AttendanceRecord {
    employee_id: EmployeeId,
    work_date: WorkDate,
    clock_in_time: Option<ClockTime>,
    clock_out_time: Option<ClockTime>,
}

impl AttendanceRecord {
    pub fn new(
        employee_id: EmployeeId,
        work_date: WorkDate,
        clock_in_time: Option<ClockTime>,
        clock_out_time: Option<ClockTime>,
    ) -> Result<Self, String> {
        Ok(AttendanceRecord {
            employee_id,
            work_date,
            clock_in_time,
            clock_out_time,
        })
    }

    pub fn employee_id(&self) -> &EmployeeId {
        &self.employee_id
    }

    pub fn work_date(&self) -> &WorkDate {
        &self.work_date
    }

    pub fn clock_in_time(&self) -> &Option<ClockTime> {
        &self.clock_in_time
    }

    pub fn clock_out_time(&self) -> &Option<ClockTime> {
        &self.clock_out_time
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

    #[test]
    fn test_work_date_can_be_created_from_timestamp() {
        // Arrange
        let timestamp = chrono::Utc::now().with_timezone(&chrono::FixedOffset::east_opt(9 * 3600).unwrap());
        let expected_date = timestamp.date_naive();

        // Act
        let work_date = WorkDate::from_timestamp(timestamp);

        // Assert
        assert_eq!(work_date.value(), expected_date)
    }

    #[test]
    fn test_attendance_record_can_be_created_with_clock_in_only() {
        // Arrange
        let employee_id = EmployeeId::new("ij09080022".to_string()).unwrap();
        let timestamp = chrono::Utc::now().with_timezone(&chrono::FixedOffset::east_opt(9 * 3600).unwrap());
        let clock_time = ClockTime::new(timestamp).unwrap();
        let work_date = WorkDate::from_timestamp(timestamp);

        // Act
        let result = AttendanceRecord::new(employee_id.clone(), work_date.clone(), Some(clock_time), None);

        // Assert
        assert!(result.is_ok());
        let record = result.unwrap();
        assert_eq!(record.employee_id(), &employee_id);
        assert_eq!(record.work_date(), &work_date);
        assert!(record.clock_in_time().is_some());
        assert!(record.clock_out_time().is_none());
    }
}


use chrono::{DateTime, FixedOffset};
use sqlx::PgPool;
use crate::domain::attendance::types::{AttendanceRecord, ClockTime, EmployeeId, WorkDate};
use crate::{AttendanceWorkflowError};
use crate::io::database::attendance::save_attendance_record;

#[derive(Debug, Clone)]
pub struct ClockInCommand {
    pub employee_id: String,
    pub timestamp: DateTime<FixedOffset>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ClockInEvent {
    ClockInSucceeded { record: AttendanceRecord },
}

pub async fn execute_clock_in(command: ClockInCommand, pool: &PgPool) -> Result<ClockInEvent, AttendanceWorkflowError> {
    let employee_id = EmployeeId::new(command.employee_id)?;
    let clock_time = ClockTime::new(command.timestamp)?;
    let work_date = WorkDate::from_timestamp(command.timestamp);
    let record = AttendanceRecord::new(employee_id, work_date, Some(clock_time), None)?;

    let saved_record = save_attendance_record(pool, record).await?;

    Ok(ClockInEvent::ClockInSucceeded { record: saved_record})
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::PgPool;

    async fn create_test_database_pool() -> PgPool {
        let database_url = "postgresql://test_user:test_password@localhost:5432/attendance_test";

        PgPool::connect(database_url)
            .await
            .expect("Failed to connect to test database")
    }

    #[tokio::test]
    async fn test_clock_in_workflow_succeeds_with_valid_command() {
        // Arrange
        let pool = create_test_database_pool().await;
        let test_id = format!("TEST_{}", uuid::Uuid::new_v4().simple());
        
        let command = ClockInCommand {
            employee_id: test_id.clone(),
            timestamp: chrono::Utc::now().with_timezone(&chrono::FixedOffset::east_opt(9 * 3600).unwrap()),
        };

        // Act
        let result = execute_clock_in(command, &pool).await;

        // Assert
        assert!(result.is_ok());
        match result.unwrap() {
            ClockInEvent::ClockInSucceeded { record } => {
                assert_eq!(record.employee_id().value(), &test_id);
                assert!(record.clock_in_time().is_some());
                assert!(record.clock_out_time().is_none());
            }
        }
    }

   #[tokio::test]
   async fn test_clock_in_workflow_fails_with_empty_employee_id() {
       // Arrange
       let pool = create_test_database_pool().await;
       let command = ClockInCommand {
           employee_id: "".to_string(),
           timestamp: chrono::Utc::now().with_timezone(&chrono::FixedOffset::east_opt(9 * 3600).unwrap()),
       };
       
       // Act
       let result = execute_clock_in(command, &pool).await;

       // Assert
       assert!(result.is_err());
       match result.unwrap_err() {
           AttendanceWorkflowError::Employee(employee_err) => {
               assert!(matches!(employee_err, crate::error::EmployeeIdError::Empty))
           }
           _ => panic!("Expected EmployeeError::Empty"),
       }
   }

    #[tokio::test]
    async fn test_clock_in_workflow_with_persistence_succeeds() {
        // Arrange
        let pool = create_test_database_pool().await;
        let test_id = format!("TEST_{}", uuid::Uuid::new_v4().simple());
        // クリーンアップ
        sqlx::query("DELETE FROM attendance_records WHERE employee_id IN (SELECT id FROM employees WHERE employee_code = $1)")
            .bind(&test_id)
            .execute(&pool)
            .await
            .unwrap();

        let command = ClockInCommand {
            employee_id: test_id.clone(),
            timestamp: chrono::Utc::now().with_timezone(&chrono::FixedOffset::east_opt(9 * 3600).unwrap()),
        };

        // Act
        let result = execute_clock_in(command, &pool).await;

        // Assert
        assert!(result.is_ok());
        match result.unwrap() {
            ClockInEvent::ClockInSucceeded { record } => {
                assert_eq!(record.employee_id().value(), &test_id);
                assert!(record.clock_in_time().is_some());
                assert!(record.clock_out_time().is_none());
            }
        }
        // クリーンアップ
        sqlx::query("DELETE FROM attendance_records WHERE employee_id IN (SELECT id FROM employees WHERE employee_code = $1)")
            .bind(&test_id)
            .execute(&pool)
            .await
            .unwrap();
    }
}
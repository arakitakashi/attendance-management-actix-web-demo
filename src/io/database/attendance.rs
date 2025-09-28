use sqlx::PgPool;
use crate::DatabaseError;
use crate::domain::attendance::types::AttendanceRecord;

pub type SaveAttendanceRecord = dyn Fn(AttendanceRecord) -> Result<AttendanceRecord, DatabaseError>;

pub fn save_attendance_record(pool: PgPool) -> Box<SaveAttendanceRecord> {
    Box::new(|record:AttendanceRecord| {
        Ok(record)
    })
}

#[cfg(test)]
mod tests {
    use sqlx::PgPool;
    use super::*;
    use crate::domain::attendance::types::{ClockTime, EmployeeId, WorkDate};

    #[tokio::test]
    async fn test_save_attendance_record_function_can_persist_record() {
        // Arrange
        let pool = create_test_database_pool().await;
        let employee_id = EmployeeId::new("ij09080022".to_string()).unwrap();
        let timestamp = chrono::Utc::now().with_timezone(&chrono::FixedOffset::east_opt(9 * 3600).unwrap());
        let clock_time = ClockTime::new(timestamp).unwrap();
        let work_date = WorkDate::from_timestamp(timestamp);
        let record = AttendanceRecord::new(employee_id, work_date, Some(clock_time), None).unwrap();

        let count_before = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM attendance_records")
            .fetch_one(&pool.clone())
            .await
            .unwrap();

        // Act
        let save_fn = save_attendance_record(pool.clone());
        let result = save_fn(record.clone());

        // Assert
        assert!(result.is_ok());
        let saved_record = result.unwrap();
        assert_eq!(saved_record.employee_id().value(), "ij09080022");
        assert!(saved_record.clock_in_time().is_some());

        let count_after = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM attendance_records")
        .fetch_one(&pool.clone())
        .await
        .unwrap();

        assert_eq!(count_after, count_before + 1, "Record should be persisted to database");
    }

    #[tokio::test]
    async fn test_postgres_database_connection() {
        // Arrange
        let pool = create_test_database_pool().await;

        // Act
        let result = sqlx::query("SELECT 1").fetch_one(&pool).await;

        // Assert
        assert!(result.is_ok());
    }

    async fn create_test_database_pool() -> PgPool {
        let database_url = "postgresql://test_user:test_password@localhost:5432/attendance_test";

        PgPool::connect(database_url)
            .await
            .expect("Failed to connect to database")
    }
}
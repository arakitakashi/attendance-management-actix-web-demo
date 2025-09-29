use crate::DatabaseError;
use crate::domain::attendance::types::AttendanceRecord;
use sqlx::PgPool;

pub async fn save_attendance_record(
    pool: &PgPool,
    record: AttendanceRecord,
) -> Result<AttendanceRecord, DatabaseError> {
    let query = r#"
                    WITH employee_lookup AS (
                        SELECT id FROM employees WHERE employee_code = $1
                    )
                    INSERT INTO attendance_records (
                        employee_id, work_date, clock_in_time, clock_out_time, status, created_at, updated_at
                    )
                    SELECT employee_lookup.id, $2, $3, $4, $5, NOW(), NOW()
                    FROM employee_lookup
                    ON CONFLICT (employee_id, work_date)
                    DO UPDATE SET
                        clock_in_time = COALESCE(EXCLUDED.clock_in_time, attendance_records.clock_in_time),
                        clock_out_time = COALESCE(EXCLUDED.clock_out_time, attendance_records.clock_out_time),
                        status = CASE
                            WHEN COALESCE(EXCLUDED.clock_in_time, attendance_records.clock_in_time) IS NOT NULL
                                 AND COALESCE(EXCLUDED.clock_out_time, attendance_records.clock_out_time) IS NOT NULL
                            THEN 'completed'
                            WHEN COALESCE(EXCLUDED.clock_in_time, attendance_records.clock_in_time) IS NOT NULL
                            THEN 'working'
                            ELSE 'incomplete'
                        END,
                        updated_at = NOW()
                "#;

    sqlx::query(query)
        .bind(record.employee_id().value())
        .bind(record.work_date().value())
        // TODO: as_ref()
        .bind(record.clock_in_time().as_ref().map(|t| t.value()))
        .bind(record.clock_out_time().as_ref().map(|t| t.value()))
        .bind("working") // 初期ステータス
        .execute(pool)
        .await
        .map_err(|e| DatabaseError::QueryFailed(e.to_string()))?;
    Ok(record)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::attendance::types::{ClockTime, EmployeeId, WorkDate};
    use sqlx::PgPool;

    #[tokio::test]
    async fn test_save_attendance_record_function_can_persist_record() {
        // Arrange
        let pool = create_test_database_pool().await;
        let test_id = format!("TEST_{}", uuid::Uuid::new_v4().simple());
        // クリーンアップ
        sqlx::query("DELETE FROM attendance_records WHERE employee_id IN (SELECT id FROM employees WHERE employee_code = $1)")
            .bind(&test_id)
            .execute(&pool)
            .await
            .unwrap();
        sqlx::query("DELETE FROM employees WHERE employee_code = $1")
            .bind(&test_id)
            .execute(&pool)
            .await
            .unwrap();

        // テスト用のemployeeレコードを作成
        sqlx::query("INSERT INTO employees (employee_code, name) VALUES ($1, $2)")
            .bind(&test_id)
            .bind("Test Employee")
            .execute(&pool)
            .await
            .unwrap();

        let employee_id = EmployeeId::new(test_id.clone()).unwrap();
        let timestamp =
            chrono::Utc::now().with_timezone(&chrono::FixedOffset::east_opt(9 * 3600).unwrap());
        let clock_time = ClockTime::new(timestamp).unwrap();
        let work_date = WorkDate::from_timestamp(timestamp);
        let record = AttendanceRecord::new(employee_id, work_date, Some(clock_time), None).unwrap();

        let count_before = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM attendance_records")
            .fetch_one(&pool.clone())
            .await
            .unwrap();

        // Act
        let result = save_attendance_record(&pool, record).await;

        // Assert
        assert!(result.is_ok());
        let saved_record = result.unwrap();
        assert_eq!(saved_record.employee_id().value(), &test_id);
        assert!(saved_record.clock_in_time().is_some());

        let count_after = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM attendance_records")
            .fetch_one(&pool.clone())
            .await
            .unwrap();

        assert_eq!(
            count_after,
            count_before + 1,
            "Record should be persisted to database"
        );

        // クリーンアップ
        sqlx::query("DELETE FROM attendance_records WHERE employee_id IN (SELECT id FROM employees WHERE employee_code = $1)")
            .bind(&test_id)
            .execute(&pool)
            .await
            .unwrap();
        sqlx::query("DELETE FROM employees WHERE employee_code = $1")
            .bind(&test_id)
            .execute(&pool)
            .await
            .unwrap();
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

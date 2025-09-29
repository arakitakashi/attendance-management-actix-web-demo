use actix_web::{web, HttpResponse, Responder};
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::PgPool;
use crate::workflows::clock_in::{execute_clock_in, ClockInCommand};
use crate::workflows::ClockInEvent;

#[derive(Debug, Serialize, Deserialize)]
pub struct ClockInRequest {
   pub employee_id: String,
   pub timestamp: DateTime<FixedOffset>,
}

#[derive(Debug, Serialize)]
pub struct ClockInResponse {
   pub success: bool,
   pub employee_id: String,
   pub clock_in_time: DateTime<FixedOffset>,
}

pub async fn clock_in_handler(
   pool: web::Data<PgPool>,
   req: web::Json<ClockInRequest>,
) -> impl Responder {
   let command = ClockInCommand {
      employee_id: req.employee_id.clone(),
      timestamp: req.timestamp,
   };

   match execute_clock_in(command, pool.get_ref()).await {
      Ok(event) => {
         match event {
            ClockInEvent::ClockInSucceeded { record } => {
               let response = ClockInResponse {
                  success: true,
                  employee_id: record.employee_id().value().to_string(),
                  clock_in_time: record.clock_in_time()
                      .as_ref()
                      .map(|t| t.value())
                      .expect("Clock in time should exist")
               };
               HttpResponse::Ok().json(response)
            }
         }
      }
      Err(e) => {
         HttpResponse::BadRequest().json(json!({
            "success": false,
            "error": format!("{}", e)
         }))
      }
   }
}

#[cfg(test)]
mod tests {
   use super::*;
   use actix_web::{test, App};

   #[actix_web::test]
   async fn test_clock_in_endpoint_returns_success () {
      // Arrange
      let database_url = "postgresql://test_user:test_password@localhost:5432/attendance_test";
      let pool = PgPool::connect(database_url).await.unwrap();

      let app = test::init_service(
         App::new()
             .app_data(web::Data::new(pool.clone()))
             .route("/attendance/clock-in", web::post().to(clock_in_handler))
      ).await;

      let request_body = ClockInRequest {
         employee_id: "ij09080022".to_string(),
         timestamp: chrono::Utc::now().with_timezone(&chrono::FixedOffset::east_opt(9 * 3600).unwrap()),
      };

      // Act
      let request = test::TestRequest::post()
      .uri("/attendance/clock-in")
          .set_json(&request_body)
          .to_request();

      let response = test::call_service(&app, request).await;

      // Assert
      assert!(response.status().is_success());
   }
}
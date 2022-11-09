use std::sync::Arc;

use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::post,
    Json, Router,
};

use crate::context::prescription::{
    application::service::prescription::ServiceTrait,
    domain::entity::command::{CreatePrescriptionCommand, UpdatePrescriptionCommand},
    infrastructure::dtos::transport::http::{RESTPrescriptionMutation, RESTPrescriptionQuery},
};

async fn create_prescription(
    service: Extension<Arc<dyn ServiceTrait<RESTPrescriptionQuery> + Sync + Send>>,
    Json(payload): Json<RESTPrescriptionMutation>,
) -> Response {
    let mut errors = vec![];
    if payload.medication_id.is_none() {
        errors.push(serde_json::json!({
                "type": "invalid_request_error",
                "code": "parameter_missing",
                "message": "We expected a value for medication_id, but none was provided",
                "param": "medication_id"
        }));
    }
    if payload.patient_id.is_none() {
        errors.push(serde_json::json!({
                "type": "invalid_request_error",
                "code": "parameter_missing",
                "message": "We expected a value for patient_id, but none was provided",
                "param": "patient_id"
        }));
    }
    if payload.address.is_none() {
        errors.push(serde_json::json!({
                "type": "invalid_request_error",
                "code": "parameter_missing",
                "message": "We expected a value for address, but none was provided",
                "param": "address"
        }));
    }
    if errors.len() > 0 {
        return (
            StatusCode::BAD_REQUEST,
            serde_json::json!({ "errors": errors }).to_string(),
        )
            .into_response();
    }
    let command = CreatePrescriptionCommand {
        medication_id: payload.medication_id.unwrap(),
        patient_id: payload.patient_id.unwrap(),
        address: payload.address.unwrap(),
    };
    let result = service.create_prescription(command, vec![]).await;
    match result {
        Ok(x) => (StatusCode::OK, serde_json::to_string(&x).unwrap()).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
    }
}

async fn update_prescription(
    service: Extension<Arc<dyn ServiceTrait<RESTPrescriptionQuery> + Sync + Send>>,
    Json(payload): Json<RESTPrescriptionMutation>,
    Path(id): Path<String>,
) -> Response {
    let mut errors = vec![];
    if payload.medication_id.is_some() {
        errors.push(serde_json::json!({
                "type": "invalid_request_error",
                "code": "parameter_unexpected",
                "message": "Found parameter medication_id, which we did not expect",
                "param": "medication_id"
        }));
    }
    if payload.patient_id.is_some() {
        errors.push(serde_json::json!({
            "type": "invalid_request_error",
            "code": "parameter_unexpected",
            "message": "Found parameter patient_id, which we did not expect",
            "param": "patient_id"
        }));
    }
    if payload.address.is_none() {
        errors.push(serde_json::json!({
                "type": "invalid_request_error",
                "code": "parameter_missing",
                "message": "We expected a value for address, but none was provided",
                "param": "address"
        }));
    }
    if errors.len() > 0 {
        return (
            StatusCode::BAD_REQUEST,
            serde_json::json!({ "errors": errors }).to_string(),
        )
            .into_response();
    }
    let command = UpdatePrescriptionCommand {
        id: id,
        address: payload.address.unwrap(),
    };
    let result = service.update_prescription(command, vec![]).await;
    match result {
        Ok(x) => (StatusCode::OK, serde_json::to_string(&x).unwrap()).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
    }
}

pub struct RESTPrescriptionAdapter {
    router: axum::Router,
}

impl RESTPrescriptionAdapter {
    pub fn new(service: Arc<dyn ServiceTrait<RESTPrescriptionQuery> + Sync + Send>) -> Self {
        RESTPrescriptionAdapter {
            router: Router::new()
                .route("/prescription", post(create_prescription))
                .route("/prescription/:id", post(update_prescription))
                .layer(Extension(service.clone())),
        }
    }

    pub async fn run(self) -> Result<(), anyhow::Error> {
        axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
            .serve(self.router.into_make_service())
            .await
            .map_err(|e| e.into())
    }
}

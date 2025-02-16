use axum::{http::StatusCode, Extension};

use crate::{
    adapters::shared::di::account_domain,
    application::{
        entities::account_entity::AccountEntity, usecases::account_usecases::ManageAccountUseCases,
    },
    core::{
        http::{failure::HttpFailure, response::HttpResponse},
        jwt::claims::Claims,
    },
};

pub async fn execute(
    Extension(claims): Extension<Claims>,
) -> Result<HttpResponse<AccountEntity>, HttpFailure> {
    let response = account_domain()
        .find_profile_with_id(&claims.id)
        .await
        .map_err(|e| HttpFailure::new(e))?;

    let http_response = HttpResponse {
        status: StatusCode::OK,
        message: "Received successfully profile !!!".to_string(),
        payload: response,
    };

    Ok(http_response)
}

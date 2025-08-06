use crate::database::models::Tag;
use crate::schema;
use crate::state::AppState;
use crate::utils::internal_error;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use diesel::prelude::*;

/// Returns Video tags
///
/// This endpoint is used to fetch video tags list
#[utoipa::path(
    get,
    path = "/tags",
    tag = "Video",
    responses(
        (status = OK, description = "List of video tags", body = Vec<Tag>),
    )
)]
pub async fn get_tags(
    State(state): State<AppState>,
) -> Result<(StatusCode, Json<Vec<Tag>>), (StatusCode, String)> {
    use schema::tags::dsl as tags_dsl;

    let mut conn = state.pool.get().map_err(internal_error)?;

    let list = tags_dsl::tags
        .load::<Tag>(&mut conn)
        .map_err(internal_error)?;

    Ok((StatusCode::OK, Json(list)))
}

/// Returns video tag by id
///
/// This endpoint is used to fetch one video tag by it's id
#[utoipa::path(
    get,
    path = "/tags/{id}",
    tag = "Video",
    responses(
        (status = OK, description = "One video tag", body = Tag),
    )
)]
pub async fn get_tag(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<(StatusCode, Json<Tag>), (StatusCode, String)> {
    use schema::tags::dsl as tags_dsl;

    let mut conn = state.pool.get().map_err(internal_error)?;

    let list = tags_dsl::tags
        .filter(tags_dsl::id.eq(id))
        .get_result::<Tag>(&mut conn)
        .map_err(internal_error)?;

    Ok((StatusCode::OK, Json(list)))
}

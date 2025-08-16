use crate::api_prelude::*;
use diesel::prelude::*;

type GetTagsResponse = PaginatedResponse<models::Tag>;

#[derive(Debug, Deserialize, utoipa::ToSchema)]
#[serde(rename_all = "snake_case")]
enum SortBy {
    Name,
}

#[derive(Deserialize, Debug, utoipa::IntoParams)]
pub struct GetTagsParams {
    /// Sort order
    sort_order: Option<SortOrder>,
    /// Sort by specified field
    sort_by: Option<SortBy>,
    /// Data list offset
    offset: Option<i64>,
    /// Data list limit
    limit: Option<i64>,
}

/// Returns Video tags
///
/// This endpoint is used to fetch video tags list
#[utoipa::path(
    get,
    path = "/tags",
    tag = "Video",
    params(
        GetTagsParams
    ),
    responses(
        (status = OK, description = "List of video tags", body = PaginatedResponse<models::Tag>),
    )
)]
pub async fn get_tags(
    State(state): State<AppState>,
    Query(params): Query<GetTagsParams>,
) -> ApiResult<(StatusCode, Json<GetTagsResponse>)> {
    use schema::tags::dsl as tags_dsl;

    let mut conn = state.pool.get().map_err(internal_error)?;

    let mut query = tags_dsl::tags.into_boxed();

    if let Some(offset) = params.offset {
        query = query.offset(offset);
    }

    if let Some(limit) = params.limit {
        query = query.limit(limit);
    }

    if let Some(sort_by) = params.sort_by {
        query = match sort_by {
            SortBy::Name => apply_sort!(query, tags_dsl::name, params.sort_order),
        };
    }

    let list = query
        .load::<models::Tag>(&mut conn)
        .map_err(internal_error)?;

    let total = tags_dsl::tags
        .count()
        .get_result::<i64>(&mut conn)
        .unwrap_or(0);

    let res = GetTagsResponse::new(list, params.offset, params.limit, total);

    Ok((StatusCode::OK, Json(res)))
}

/// Returns video tag by id
///
/// This endpoint is used to fetch one video tag by it's id
#[utoipa::path(
    get,
    path = "/tags/{id}",
    tag = "Video",
    params(
        ("id" = String, Path, description = "Tag id")
    ),
    responses(
        (status = OK, description = "One video tag", body = models::Tag),
    )
)]
pub async fn get_tag(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> ApiResult<(StatusCode, Json<models::Tag>)> {
    use schema::tags::dsl as tags_dsl;

    let mut conn = state.pool.get().map_err(internal_error)?;

    let list = tags_dsl::tags
        .filter(tags_dsl::id.eq(id))
        .get_result::<models::Tag>(&mut conn)
        .map_err(internal_error)?;

    Ok((StatusCode::OK, Json(list)))
}

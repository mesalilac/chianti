/// Prelude for the api routes
pub use crate::database::models;
pub use crate::schema;
pub use crate::state::AppState;
pub use crate::utils;
pub use axum::{
    Json,
    body::Body,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
pub use axum_extra::extract::Query;
pub use serde::{Deserialize, Serialize};
pub use ts_rs::TS;
pub use utils::internal_error;

use crate::{RUNTIME, VERSION};
use nilsmf_lib::models::meta::{Runtime, Version};
use rocket::{self, serde::json::Json};
use utoipa;

#[utoipa::path(
        get,
        path = "/api/meta/version",
        responses(
            (status = 200, description = "Version returned successfully", body = Version)
        ),
    )]
#[rocket::get("/meta/version")]
pub async fn get_version() -> Json<Version> {
    Json(*VERSION.lock().await)
}

#[utoipa::path(
        get,
        path = "/api/meta/runtime",
        responses(
            (status = 200, description = "Runtime returned successfully", body = Runtime)
        ),
    )]
#[rocket::get("/meta/runtime")]
pub async fn get_runtime() -> Json<Runtime> {
    let mut rt = (*RUNTIME.lock().await).clone();
    //rt.time = Utc::now().timestamp_millis();

    Json(rt)
}

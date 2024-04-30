use crate::VERSION;
use nilsmf_lib::models::meta::Version;
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
pub fn get_version() -> Json<Version> {
    let ver: &Version = &VERSION;
    return Json(ver.to_owned());
}

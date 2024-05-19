use nilsmf_lib::models::meta::{Runtime, Version};
use nilsmf_lib::models::Wrapper;
use rocket::get;

#[get("/version")]
pub fn get_version() -> Wrapper<Version> {
    unimplemented!()
}

#[get("/runtime")]
pub async fn get_runtime() -> Wrapper<Runtime> {
    unimplemented!()
}

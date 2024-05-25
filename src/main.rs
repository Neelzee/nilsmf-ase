use api::meta::{get_runtime, get_version};
use nilsmf_lib::models::meta::{Runtime, Version};
use once_cell::sync::Lazy;
use rocket::{self, routes};
use rocket_errors::anyhow::Result;
use tokio::sync::Mutex;
use utoipa::OpenApi;
use utoipa_rapidoc::RapiDoc;
use utoipa_swagger_ui::SwaggerUi;

mod api;
mod db;

#[derive(OpenApi)]
#[openapi(
    info(description = "nilsmf-ase: Authenticator Service"),
    paths(api::meta::get_version, api::meta::get_runtime),
    components(schemas(Version, Runtime))
)]
struct ApiDoc;

/// Current version of this service
static VERSION: Lazy<Mutex<Version>> = Lazy::new(|| Mutex::new(Version::new(0, 1, 0)));

/// Runtime of this Service
static RUNTIME: Lazy<Mutex<Runtime>> = Lazy::new(|| Mutex::new(Runtime::init()));

#[rocket::main]
async fn main() -> Result<()> {
    let _rocket = rocket::build()
        .mount(
            "/",
            SwaggerUi::new("/swagger-ui/<_..>").url("/api-docs/openapi.json", ApiDoc::openapi()),
        )
        .mount("/", RapiDoc::new("/api-docs/openapi.json").path("/rapidoc"))
        .mount("/api/", routes![get_version, get_runtime])
        .launch()
        .await?;
    Ok(())
}

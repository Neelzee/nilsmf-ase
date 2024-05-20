use api::meta::get_version;
use nilsmf_lib::models::meta::Version;
use once_cell::sync::Lazy;
use rocket::{self, routes};
use rocket_errors::anyhow::Result;
use utoipa::OpenApi;
use utoipa_rapidoc::RapiDoc;
use utoipa_swagger_ui::SwaggerUi;

mod api;

#[derive(OpenApi)]
#[openapi(
    info(description = "nilsmf-ase: Authenticator Service"),
    paths(api::meta::get_version),
    components(schemas(Version))
)]
struct ApiDoc;

static VERSION: Lazy<Version> = Lazy::new(|| Version::new(0, 1, 0));

#[rocket::main]
async fn main() -> Result<()> {
    let _rocket = rocket::build()
        .mount(
            "/",
            SwaggerUi::new("/swagger-ui/<_..>").url("/api-docs/openapi.json", ApiDoc::openapi()),
        )
        .mount("/", RapiDoc::new("/api-docs/openapi.json").path("/rapidoc"))
        .mount("/api/", routes![get_version])
        .launch()
        .await?;
    Ok(())
}

use nilsmf_lib::models::meta::Runtime;
use once_cell::sync::Lazy;
use rocket::{self, routes, Config};
use rocket_errors::anyhow::Result;
use time::Date;
use tokio::sync::Mutex;

mod api;

pub static RUNTIME: Lazy<Mutex<Runtime>> = Lazy::new(|| Mutex::new(Runtime::new(Date::MIN, 0)));

#[rocket::main]
async fn main() -> Result<()> {
    let if Ok(mutex) = RUNTIME.try_lock() {
        todo!()
    }

    let _rocket = rocket::build().mount("/", routes![]).launch().await?;

    Ok(())
}

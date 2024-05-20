use rocket::{
    delete, get,
    http::Status,
    outcome::Outcome,
    post, put,
    request::{self, FromRequest},
    response::{status::Custom, Responder},
    serde::json::Json,
    FromForm, Request, State,
};

use nilsmf_lib::models::meta::Version;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

pub mod meta;

pub fn get_version() -> Version {
    unimplemented!()
}

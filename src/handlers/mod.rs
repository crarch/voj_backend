pub mod session;
pub mod profile;
pub mod question;

use crate::env;

use actix_web::{HttpResponse,Error,get};
use anyhow::Result;




#[get("/version")]
pub async fn get_version()->Result<HttpResponse,Error>{
    Ok(HttpResponse::Ok().body(format!("{{\"version\":\"{}\"}}",env::VERSION)))
}
pub mod session;
pub mod profile;
pub mod testbench;
pub mod judge;
pub mod queue;

use crate::env;

use actix_web::{web,HttpResponse,Error,get,HttpRequest};
use actix_web::web::Data;
use anyhow::Result;

use actix_web_actors::ws;
use crate::actors::{Judgers,JudgerWs};
use actix::Addr;

#[get("/version")]
pub async fn get_version()->Result<HttpResponse,Error>{
    Ok(HttpResponse::Ok().body(format!("{{\"version\":\"{}\"}}",env::VERSION)))
}


#[get("/websocket")]
pub async fn get_websocket(
    req:HttpRequest,
    stream:web::Payload,
    judgers:Data<Addr<Judgers>>
)->Result<HttpResponse,Error>{
    // panic!("");
    let ws=JudgerWs::new(
        judgers.get_ref().clone()
    );
    
    let resp=ws::start(ws,&req,stream);
    resp
}



use actix_web::web;

use crate::handlers::{
    get_version,
};

use crate::handlers::session::*;
use crate::handlers::profile::*;
use crate::handlers::question::*;

pub fn routing(cfg:&mut web::ServiceConfig){
    cfg
        .service(get_question)
        .service(get_jwt_token)
        .service(get_pass)
        .service(refresh_jwt_token)
        .service(get_version);
        
}
            



use actix_web::{web,HttpRequest,HttpResponse,post,Error,get};

use crate::models::UserId;
use crate::MongoDB;

use crate::models::get_pass_by_id;

#[get("/profile/pass")]
pub async fn get_pass(
    mongo:MongoDB,
    user_id:UserId
)->Result<HttpResponse,Error>{
    
    let result=get_pass_by_id(mongo,user_id.user_id);
    
    Ok(HttpResponse::Ok().json(result))
}
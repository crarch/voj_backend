use serde::{Deserialize,Serialize};
use bson::document::Document;
use mongodb::bson::doc;
use bson::oid::ObjectId;
use futures_util::TryStreamExt;

use serde_json::Value;
use bson::Bson;

use crate::utils::time::get_unix_timestamp;
use crate::MongoDB;
use crate::actors::push_job;
use crate::Queue;
use crate::models::create_new_record;

pub async fn add_job(
    mongo:MongoDB,
    queue:Queue,
    question_id:u32,
    update:u32,
    user_id:u32,
    code:&str,
)->Result<ObjectId,()>{
    let collection=mongo.collection::<Document>("queue");
    
    if let Ok(object_id)=create_new_record(mongo.clone(),user_id,question_id,code).await{
        //lock queue
        
        let doc=doc!{
            "_id":object_id,
            "user_id":user_id,
            "question_id":question_id,
            "update":update,
            "submit_time":get_unix_timestamp(),
            "code":code,
        };
        
        let job:Bson=Bson::from(doc.clone()).into();
        let job:Value=job.into();
        let job=job.to_string();
        
        push_job(queue,job).await;
        
        let insert_result=collection.insert_one(doc,None).await;
        
        let result=match insert_result{
            Ok(_)=>{
                Ok(object_id)
            },
            Err(_)=>Err(()),
        };
        
        return result;
    }
    
    Err(())
}


    


pub async fn delete_job_by_id(
    mongo:MongoDB,
    object_id:ObjectId,
)->Result<(),()>{
    
    let collection=mongo.collection::<Document>("queue");
    
    if let Ok(_result)=collection.delete_one(
        doc!{"_id":object_id},
        None
    ).await{
        return Ok(());
    }
    Err(())
}

pub async fn update_judge_result(
    mongo:MongoDB,
    object_id:ObjectId,
    is_success:bool,
    test_bench:&Document,
)->Result<(),()>{
    let collection=mongo.collection::<Document>("records");
    
    if let Ok(_result)=collection.update_one(
        doc!{"_id":object_id,"success":doc!{"$exists":false}},
        doc!{
            "$set":{
                "success":is_success,
                "test_bench":test_bench
            }
        },
        None

    ).await{
        return Ok(());
    }
    Err(())
}

async fn check_dead_job(mongo:MongoDB){
    let collection=mongo.clone().collection::<Document>("queue");
    
    if let Ok(mut cursor)=collection.find(
        doc!{"lock_time":doc!{"$lt":get_unix_timestamp()-20}},
        mongodb::options::FindOptions::builder()
            .projection(Some(doc!{"_id":1}))
            .build()
    ).await{
        while let Some(doc)=cursor.try_next().await.unwrap(){
            if let Ok(object_id)=doc.get_object_id("_id"){
                let time_out=doc!{
                    "time_out":"timeout",
                };
                
                
                let _=update_judge_result(
                    mongo.clone(),
                    object_id.clone(),
                    false,
                    &time_out
                ).await.unwrap();
                
                let _=delete_job_by_id(
                    mongo.clone(),
                    object_id,
                ).await.unwrap();
                
            }
        }
    }
    
}


#[derive(Debug,Serialize,Deserialize)]
pub struct JudgeResultJson{
    pub _id:ObjectId,
    pub success:bool,
    pub test_bench:Document,
    pub question_id:u32,
    pub user_id:u32
}

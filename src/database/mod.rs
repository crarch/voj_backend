use actix_web::web;
use r2d2;
use r2d2_postgres::{postgres::NoTls, PostgresConnectionManager};

use mongodb::sync::Client;

type _PgPool = r2d2::Pool<r2d2_postgres::PostgresConnectionManager<NoTls>>;
pub type PgPool = web::Data<_PgPool>;

pub type MongoDB=web::Data<mongodb::sync::Database>;

use crate::env::get_env;

pub fn get_database_pg_pool()->_PgPool{
    
    let pg_host = get_env("POSTGRES_HOST");
    let pg_user = get_env("POSTGRES_USER");
    let pg_password = get_env("POSTGRES_PASSWORD");
    let pg_dbname = get_env("POSTGRES_DBNAME");
    let pg_port = get_env("POSTGRES_PORT");

    let database_info = format!(
        "host={} port={} user={} password={} dbname={}",
        pg_host, pg_port, pg_user, pg_password, pg_dbname
    );
    
    let manager=PostgresConnectionManager::new(
        database_info.parse().unwrap(),
        NoTls,
    );
    
    let pg_pool=r2d2::Pool::new(manager).unwrap();
    
    pg_pool
}

pub fn get_mongo_database()->mongodb::sync::Database{
    
    let mongo_url=get_env("MONGODB_URL");
    let mongo_dbname=get_env("MONGODB_DBNAME");
    
    let client=Client::with_uri_str(&mongo_url).unwrap();
    let database=client.database(&mongo_dbname);
    
    database
}
    
    
    
        

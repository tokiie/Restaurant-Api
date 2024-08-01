use std::env;
use dotenv::dotenv;


#[derive(Debug)]
pub struct Config {
    pub host: String,
    pub port: String,
    pub db_url: String,
}

impl Config {
    pub fn setup() ->  Result<Self,  &'static str>{
       dotenv().ok();
       Ok(Config{
            host: env::var("HOST").unwrap(),
            port : env::var("PORT").unwrap(),
            db_url: env::var("DB_URL").unwrap(),
        })
    }
}



use std::fs::File;
use tokio::sync::Mutex;
use crate::models::Todo;
use std::sync::Arc;

pub type DB<T> = Arc<Mutex<Vec<T>>>;
/// initialize db
pub fn init() -> DB<Todo> {
     let file = File::open("./data/data.json");
     match file {
         Ok(f) => {
             let json = serde_json::from_reader(f).expect("unable to parse json"); 
             Arc::new(Mutex::new(json))
         }
         Err(_) => {
            println!("data/data.json not found");
            Arc::new(Mutex::new(Vec::new()))
            }
     }
}

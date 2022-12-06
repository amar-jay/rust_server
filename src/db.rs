use std::fs::File;
use std::str::FromStr;
use warp::{Reply, Rejection};
use sqlx::{ConnectOptions, sqlite::SqliteConnectOptions, SqlitePool};
use crate::models::Todo;
use tokio::sync::Mutex;
use std::sync::Arc;
use anyhow::Result;

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



#[derive(Debug)]
pub struct Handlers {
    pool: SqlitePool,
}

impl Handlers {
    /// Create a new DB instance and also connect to database
    pub async fn new(uri: &str) -> Result<Self> {
        // run migarations
        {
            let mut conn = SqliteConnectOptions::from_str(&uri)?
            .create_if_missing(true)
                .connect()
                .await?;

            sqlx::migrate!().run(&mut conn).await?;
        }

        // connect to sqlite db
        let pool = SqlitePool::connect(uri).await?;
        Ok(Self {
            pool
        })
    }

    /// check whether db connected or not
    pub async fn foobar(x: Arc<Handlers>) ->  Result<impl Reply, Rejection> {
        // TODO: properly handle errors
        let res: (bool, ) = sqlx::query_as("SELECT 1").fetch_one(&x.pool).await.unwrap();
        Ok(warp::reply::json(&res.0))
    }


    /// count the number of todos present
    pub async fn todos_len(x: Arc<Handlers>) ->  Result<impl Reply, Rejection>  {
        let res: (i64, ) = sqlx::query_as("SELECT count(*) FROM todos").fetch_one(&x.pool).await.expect("cannot fetch todos length");
        Ok(warp::reply::json(&res.0))
    }


    pub async fn select_todos_from_db(x: Arc<Handlers>) -> Result<impl Reply, Rejection> {
        //let db = db.lock().await;
        let res: (String, String, bool, ) = sqlx::query_as("SELECT id, text, done FROM todos")
        .fetch_one(&x.pool).await.expect("cannot fetch todos");


        return Ok(warp::reply::json(&res))

    }
    pub async fn insert_todo_from_db(x: Arc<Handlers>, todo: Todo) -> Result<impl Reply, Rejection> {
        //let db = db.lock().await;
        let res = sqlx::query(r#"
        INSERT INTO todos ( id, text, done)
        VALUES ($1, $2, $3)
        "#)
            .bind(todo.id)
            .bind(todo.text)
            .bind(todo.done)
            .execute(&x.pool).await.expect("cannot insert todo");


        if res.rows_affected() != 1 {
            panic!("Expected store to recieve morethan 1 row instead has {}", res.rows_affected())
        }
        return Ok(warp::reply::json(&String::from("added successfully")))

    }

    pub async fn get_todos(db: DB<Todo>) -> Result<impl Reply, Rejection> {
        let db = db.lock().await;
        let todos = db.clone();

        println!("getting all todos");
        Ok(warp::reply::json(&todos))
    }

    pub async fn get_todo_by_id(id: String, db: DB<Todo>) -> Result<impl Reply, Rejection> {

        let db = db.lock().await;
        let todos = db.clone();

        println!("getting one todo");
        for i in 0..todos.len() {
            if todos[i].id == id {
                return Ok(warp::reply::json(&todos[i]));
            }

        }
        Ok(warp::reply::json(&String::from("{\"error\": \"Not found\"}")))
    }

    pub async fn create_todo(db: DB<Todo>, todo: Todo) -> Result<impl Reply, Rejection> {
        let db = db.lock().await;
        let mut todos = db.clone();

        println!("create todo");
        for i in 0..todos.len() {
            if todos[i].id == todo.id  || todos[i].text == todo.text {
                return Ok(warp::reply::json(&String::from("{\"error\": \"Already exists\"}")));
            }
        }

        todos.push(todo);

        return Ok(warp::reply::json(&String::from("added successfully")))

    }

    #[allow(unused)]
    pub fn update_todo(db: DB<Todo>) {

    }

    #[allow(unused)]
    pub fn delete_todo(db: DB<Todo>) {

    }

}

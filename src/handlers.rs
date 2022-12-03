use warp::{Reply, Rejection};

use crate::{db::DB, models::Todo};


pub struct Handlers {}

impl Handlers {
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
            if todos[i].id == todo.id {
                return Ok(warp::reply::json(&String::from("{\"error\": \"Already exists\"}")));
            }
        }

        todos.push(todo);

        return Ok(warp::reply::json(&String::from("added successfully")))

    }

    pub fn update_todo(db: DB<Todo>) {

    }

    pub fn delete_todo(db: DB<Todo>) {

    }
}

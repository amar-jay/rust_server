//!
use std::convert::Infallible;
use std::sync::Arc;

use warp::{filters::BoxedFilter, Filter};
use warp::Reply;
use crate::db::{self, DB};
use crate::db::Handlers;
use crate::models::Todo;
/// This uses json db
pub fn init(db:Handlers) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {

    let sudo_db = db::init();
    let db = Arc::new(db);
    return get_todos(sudo_db.clone())
        .or(get_todo_by_id(sudo_db.clone()))
        .or(create_todo(sudo_db.clone()))
        .or(update_todo(sudo_db.clone()))
        .or(delete_todo(sudo_db.clone()))
        .or(create_todos_from_db(db.clone()))
        .or(select_todos_from_db(db.clone()))
        .or(get_todos_from_db(db.clone()))
        .or(status(db.clone()))
        .or(todos_len(db));

}


/// fetch todos
/// GET /todos
fn get_todos(db: DB<Todo>) ->  impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {

    warp::path("get")
    .and(warp::get())
    .and(use_db(db))
    .and_then(Handlers::get_todos)
}

/// fetch one todos by id
/// GET /todos/:id
fn get_todo_by_id(db: DB<Todo>) ->  impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {

    warp::path!("get_one" / String)
    .and(warp::get())
    .and(use_db(db))
    .and_then(Handlers::get_todo_by_id)
}

/// update a todo
/// PUT /todos/:id
#[allow(unused_variables)]
fn update_todo(db: DB<Todo>) ->  BoxedFilter<(impl Reply,)> {
    warp::path("update")
    .map(|| {
        println!("getting data");
        return Ok(warp::reply::html("<h1> Working </h1>"));
        }).boxed()

}
/// create a todo
/// POST /todos
#[allow(unused_variables)]
fn create_todo(db: DB<Todo>) ->  impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("create" / String)
    .and(warp::get())
    .and(use_db(db))
    .and_then(|t, db| {
            let todo = Todo {
                text: t,
                id: uuid::Uuid::new_v4().to_string(),
                done: false
            };
            Handlers::create_todo(db, todo)
        })
}

/// create a todo by id
/// DELETE /todos/:id
#[allow(unused_variables)]
fn delete_todo(db: DB<Todo>) ->  BoxedFilter<(impl Reply,)> {
    warp::path("delete")
    .map(|| String::from("unimplemented")).boxed()

}

/// give stats about db
fn todos_len(db: Arc<Handlers>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    println!("initing stats");
    warp::path("todos_len")
    .and(warp::get())
    .and(use_db(db))
    .and_then(|db| {
            Handlers::todos_len(db)
        })
}


/// give stats about db
fn select_todos_from_db(db: Arc<Handlers>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    println!("creating todos from db");
    warp::path("select_todos")
    .and(warp::get())
    .and(use_db(db))
    .and_then(|db| {
            Handlers::select_todos_from_db(db)
        })
}

/// give stats about db
fn create_todos_from_db(db: Arc<Handlers>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    println!("creating todos from db");
    warp::path!("create_todo" / String)
    .and(warp::get())
    .and(use_db(db))
    .and_then(|t, db| {

            let todo = Todo {
                text: t,
                id: uuid::Uuid::new_v4().to_string(),
                done: false
            };
            Handlers::insert_todo_from_db(db, todo)
        })
}


/// fetches all todos about db
fn get_todos_from_db(db: Arc<Handlers>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    println!("fetching todos from db");
    warp::path("fetch_todo")
    .and(warp::get())
    .and(use_db(db))
    .and_then(|db| {
            Handlers::select_todos_from_db(db)
        })
}

/// give stats about db
fn status(db: Arc<Handlers>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("status")
    .and(warp::get())
    .and(use_db(db))
    .and_then(|db| {
            Handlers::foobar(db)
        })
}

/// wrap db instance
/// so that we can use it in handlers
fn use_db<T: std::marker::Sync + std::marker::Send>(sudo_db: Arc<T>) ->  impl Filter<Extract = (Arc<T>, ), Error = Infallible> + Clone {
    warp::any().map(move || sudo_db.clone())
}

//!
use std::convert::Infallible;

#[allow(unused_variables)]

use warp::{filters::BoxedFilter, Filter, Reply};
use crate::db::DB;
use crate::handlers::Handlers;
use crate::models::Todo;
pub fn init(db: DB<Todo>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {

    return get_todos(db.clone())
        .or(get_todo_by_id(db.clone()))
        .or(create_todo(db.clone()))
        .or(update_todo(db.clone()))
        .or(delete_todo(db.clone()));

}

/// fetch todos
fn get_todos(db: DB<Todo>) ->  impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {

    warp::path("get")
    .and(warp::get())
    .and(use_db(db))
    .and_then(Handlers::get_todos)
}

/// fetch one todos by id
fn get_todo_by_id(db: DB<Todo>) ->  impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {

    warp::path!("get_one" / String)
    .and(warp::get())
    .and(use_db(db))
    .and_then(Handlers::get_todo_by_id)
}

/// update a todo
#[allow(unused_variables)]
fn update_todo(db: DB<Todo>) ->  BoxedFilter<(impl Reply,)> {
    warp::path("update")
    .map(|| {
        println!("getting data");
        return Ok(warp::reply::html("<h1> Working </h1>"));
        }).boxed()

}
/// create a todo
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
#[allow(unused_variables)]
fn delete_todo(db: DB<Todo>) ->  BoxedFilter<(impl Reply,)> {
    warp::path("delete")
    .map(|| String::from("unimplemented")).boxed()

}

fn use_db(db: DB<Todo>) ->  impl Filter<Extract = (DB<Todo>, ), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}

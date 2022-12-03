//!
use std::convert::Infallible;
use std::sync::Arc;

use warp::{filters::BoxedFilter, Filter, Reply};
use crate::db::DB;
use crate::db::Handlers;
use crate::models::Todo;
pub fn init(sudo_db: DB<Todo>,db:Arc<Handlers>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {

    return get_todos(sudo_db.clone())
        .or(get_todo_by_id(sudo_db.clone()))
        .or(create_todo(sudo_db.clone()))
        .or(update_todo(sudo_db.clone()))
        .or(delete_todo(sudo_db.clone()))
        .or(status(db.clone()));

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

/// give stats about db
fn status(db: Arc<Handlers>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("status")
    .and(use_db(db))
    .and_then(|d:Arc<Handlers>| {
            let dbz = d.clone();
                d.foobar()
        })
}

/// wrap db instance
fn use_db<T: std::marker::Sync + std::marker::Send>(sudo_db: Arc<T>) ->  impl Filter<Extract = (Arc<T>, ), Error = Infallible> + Clone {
    warp::any().map(move || sudo_db.clone())
}

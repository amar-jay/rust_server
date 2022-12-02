//!
#[allow(unused_variables)]

use warp::{filters::BoxedFilter, Filter, Reply};
use crate::db::DB;
use crate::models::Todo;
pub fn init(db: DB<Todo>) -> BoxedFilter<(impl Reply,)> {

    return warp::path("api")
        .and(get_todos(db.clone()))
        .or(get_todo_by_id(db.clone()))
        .or(create_todo(db.clone()))
        .or(update_todo(db.clone()))
        .or(delete_todo(db.clone()))
        .boxed();
}

/// fetch todos
#[allow(unused_variables)]
fn get_todos(db: DB<Todo>) -> BoxedFilter<(impl Reply,)> {

    warp::path("get")
    .map(|| {
        println!("getting data");
        return Ok(warp::reply::html("<h1> Working </h1>"));
        }).boxed()
}

/// fetch one todos by id
#[allow(unused_variables)]
fn get_todo_by_id(db: DB<Todo>) -> BoxedFilter<(impl Reply,)> {
    warp::path("get")
    .map(|| String::from("get todos")).boxed()

}

/// update a todo
#[allow(unused_variables)]
fn update_todo(db: DB<Todo>) ->  BoxedFilter<(impl Reply,)> {
    warp::path("update")
    .map(|| String::from("update todos")).boxed()

}
/// create a todo
#[allow(unused_variables)]
fn create_todo(db: DB<Todo>) ->  BoxedFilter<(impl Reply,)> {
    warp::path("create")
    .map(|| String::from("unimplemented")).boxed()

}

/// create a todo by id
#[allow(unused_variables)]
fn delete_todo(db: DB<Todo>) ->  BoxedFilter<(impl Reply,)> {
    warp::path("delete")
    .map(|| String::from("unimplemented")).boxed()

}

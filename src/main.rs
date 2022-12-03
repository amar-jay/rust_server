use warp;
mod routes;
mod db;
mod models;
mod handlers;

#[tokio::main]
async fn main() {
    let db = db::init();
    let routes = routes::init(db);

    println!("Server running on http://localhost:3000");
    warp::serve(routes)
        .run(([127, 0, 0, 1], 3000))
        .await;
}

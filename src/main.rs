use warp;
mod routes;
mod db;
mod models;

#[tokio::main]
async fn main() {

    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "4000".to_string())
        .parse().expect("cannot get port");

    let conn = match std::env::var("SQLITE_URI") {
        Ok(uri) => db::Handlers::new(&uri).await.expect("Unable to connect to db uri"),
        Err(err) => panic!("Unable to connect to db uri: {:#?}", err)
    };
    let db = db::init();
    let routes = routes::init(db, conn);

    println!("Server running on http://localhost:3000");
    warp::serve(routes)
        .run(([127, 0, 0, 1], port))
        .await;
}

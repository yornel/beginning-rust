#[tokio::main]
async fn main() {
    let app = axum::Router::new()
    .route("/qry", axum::routing::get(handler_query));

  axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
    .serve(app.into_make_service())
    .await
    .unwrap();
}

async fn handler_query(axum::extract::Query(params): 
    axum::extract::Query<std::collections::HashMap<String, String>>) -> String {
  format!("id:{}, name:{}.", params["id"], params["name"])
}
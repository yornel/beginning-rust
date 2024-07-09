use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Mydata {
  name:String,
  mail:String,
  age:u32,
}

#[derive(Serialize, Deserialize)]
struct Myform {
  name: String,
  mail: String,
}

#[tokio::main]
async fn main() {
    let app = axum::Router::new()
    .route("/", axum::routing::get(handle_index))
    .route("/post", axum::routing::post(handle_post));

  axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
    .serve(app.into_make_service())
    .await
    .unwrap();
}

async fn handle_index()-> axum::response::Html<String> {
    let tera = tera::Tera::new("templates/*").unwrap();
  
    let mut context = tera::Context::new();
    context.insert("title", "Index page");
    context.insert("message", "これはサンプルです。");
  
    let output = tera.render("index.html", &context);
    axum::response::Html(output.unwrap())
  }

  async fn handle_post(axum::Form(myform): axum::Form<Myform>)
    -> axum::response::Html<String> {
  let msg = format!("I am {}<{}>.", myform.name, myform.mail);
  let tera = tera::Tera::new("templates/*").unwrap();

  let mut context = tera::Context::new();
  context.insert("title", "Index page");
  context.insert("message", &msg);

  let output = tera.render("index.html", &context);
  axum::response::Html(output.unwrap())
}
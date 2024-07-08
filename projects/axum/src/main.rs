use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Mydata {
  name:String,
  mail:String,
  age:u32,
}


#[tokio::main]
async fn main() {
    let app = axum::Router::new()
    .route("/json/:id", axum::routing::get(handler_json)) ;

  axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
    .serve(app.into_make_service())
    .await
    .unwrap();
}

async fn handler_json(axum::extract::Path(id):
    axum::extract::Path<usize>) -> axum::Json<serde_json::Value> {
  let data:[Mydata;3] = [
    Mydata {name:String::from("Taro"), 
      mail:String::from("taro@yamada"), age:39},
    Mydata {name:String::from("Hanako"), 
      mail:String::from("hanako@flower"), age:28},
    Mydata {name:String::from("Sachiko"), 
      mail:String::from("sachiko@happy"), age:17},
  ];
  let item = &data[id];
  let data = serde_json::json!(item);
  axum::Json(data)
}
#![allow(unused)]
fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

//タプルのベクタに対してハッシュマップを生成する
    let teams  = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

//insertを呼び出してfield_nameとfield_valueがハッシュマップにムーブされた後は、 これらの変数を使用することはできない
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);


//ハッシュマップの値にアクセス、更新する
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    scores.insert(String::from("Yellow"), 50);
    //キーに値がなかった時のみ値を挿入する
    scores.entry(String::from("Blue")).or_insert(30);
    scores.entry(String::from("Red")).or_insert(100);

    
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}

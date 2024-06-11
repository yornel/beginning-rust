fn main() {
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        //                      "3つ目の要素は{}です"
        Some(third) => println!("The third element is {}", third),
        //               "3つ目の要素はありません。"
        None => println!("There is no third element."),
    }

//ベクタに含まれない要素の添え字を使おうとしたときのプログラムの振る舞い
    let v2 = vec![1, 2, 3, 4, 5];

    //プログラムがパニックする。
    // let does_not_exist = &v2[100]
    //パニックすることなくNoneを返す
    let does_not_exist = v2.get(100);

//ベクタ内の値を順に処理する
    let v3 = vec![100, 32, 57];
    for i in &v3 {
        println!("{}", i);
    }

//全要素に変更を加えるために
    let mut v4 = vec![100, 32, 57];
    for i in &mut v4 {
        *i += 50;
        println!("{}", i);
    }

//ベクタは同じ型の値しか保持できないが、Enumを使って複数の型を保持する
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
];
}

fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);
}
//配列やタプル型は「3.2. データ型」にある
//https://doc.rust-jp.rs/book-ja/ch03-02-data-types.html
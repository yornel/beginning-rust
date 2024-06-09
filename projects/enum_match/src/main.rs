#![allow(unused)]
fn main() {
#[derive(Debug)]
enum UsState {
   Alabama,
   Alaska,
}

enum Coin {
   Penny,
   Nickel,
   Dime,
   Quarter(UsState),
}
let coin = Coin::Penny;
let mut count = 0;
//if letという記法は等号記号で区切られたパターンと式を取り、
//式がmatchに与えられ、パターンが最初のアームになったmatchと同じ動作をする。
if let Coin::Quarter(state) = coin {
    println!("State quarter from {:?}!", state);
} else {
    count += 1;
}
}

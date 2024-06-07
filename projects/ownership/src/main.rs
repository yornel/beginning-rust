fn main() {
////String型を呼び出し元の関数に戻さないと、calculate_lengthを呼び出した後に、 Stringオブジェクトが使えなくなる例
    fn1();

////借用（&s1をcalcuate_lengthに渡し、その定義では、String型ではなく、&Stringを受け取ることで
////所有権をもらうことなく値を参照することができる）
    fn2();

////可変な参照
    fn3();

}

fn fn1() {
    let s1 = String::from("hello");

    //ここでs1はcalculate_lengthにムーブされるからこれ以降s1を使用できない。
    let (s2, len) = calculate_length_1(s1);

    //'{}'の長さは、{}です
    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length_1(s: String) -> (String, usize) {
    let length = s.len(); // len()メソッドは、Stringの長さを返します

    (s, length)
}

fn fn2() {
    let ss1 = String::from("hello");

    let len = calculate_length_2(&ss1);

    // '{}'の長さは、{}です
    println!("The length of '{}' is {}.", ss1, len);
}

fn calculate_length_2(s: &String) -> usize {
    s.len()
}

fn fn3() {
    //let s = String::from("hello");
    let mut s = String::from("hello");
    
    //変数が標準で不変なのと全く同様に、参照も不変なのです。参照している何かを変更することは叶わない。
    //mutをつけずに参照している何かを変更するとエラー
    //change(&s);
    change(&mut s);

////特定のスコープで、ある特定のデータに対しては、 一つしか可変な参照を持てない。
    let mut s1 = String::from("hello");

    //エラーになる
    // let r1 = &mut s1;
    // let r2 = &mut s1;

    // 波カッコで囲むと、r1はここでスコープを抜けるので、問題なく新しい参照を作ることができる
    {
        let r1 = &mut s1;
        println!("r1 is {}.", r1);
    
    } 
    
    let r2 = &mut s1;
    println!("r2 is {}.", r2);
}

fn change(some_string: &mut String) {
    
    some_string.push_str(", world");
}
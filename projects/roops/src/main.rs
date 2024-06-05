fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);



//while文
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    // 発射！
    println!("LIFTOFF!!!");


//ループを使ってコレクションの各要素を覗き見る
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {}", element);
    }

//Range型は、標準ライブラリで提供される片方の数字から始まって、 もう片方の数字未満の数値を順番に生成する型です。
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");

}

use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    //unwrap_or_else：panic!ではない何らか独自のエラー処理を定義できるのです。
    let config = Config::new(&args).unwrap_or_else(|err| {
        // 引数解析時に問題
        println!("Problem parsing arguments: {}", err);
        //process::exit関数は、 即座にプログラムを停止させ、渡された数字を終了コードとして返します。
        process::exit(1);
    });

    // --snip--
}

struct Config {
    query: String,
    filename: String,
}

// --snip--

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

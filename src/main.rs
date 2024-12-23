use std::{env::args, io::{self, Read}, process};
use mini_grep::{search, search_from_input, Config};

fn main() {
    let args: Vec<String> = args().collect();
    let cfg = Config::new(&args);

    if atty::is(atty::Stream::Stdin) {
        // println!("标准输入来自终端");
        if let Err(e) = search(cfg) {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    } else {
        // println!("标准输入来自管道");
        let mut input = String::new();
        if let Err(e) = io::stdin().read_to_string(&mut input) {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
        if let Err(e) = search_from_input(&input, cfg) {
            eprintln!("Error: {}", e);
        }
    }
}

use std::{env, process};
use minigrep::{Args, run};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Args::build(&args)
        .unwrap_or_else(|error| {
            println!("Problem parsing arguments: {error}");
            process::exit(1);
        });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    // return 값이 unit이기 때문에 unwrap을 사용할 필요 없음
    if let Err(error) = run(config) {
        println!("Application error: {error}");
        process::exit(1);
    }
}

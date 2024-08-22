use std::{env, process};
use minigrep::{Args, run};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Args::build(&args)
        .unwrap_or_else(|error| {
            eprintln!("Problem parsing arguments: {error}");
            process::exit(1);
        });

    // return 값이 unit이기 때문에 unwrap을 사용할 필요 없음
    if let Err(error) = run(config) {
        eprintln!("Application error: {error}");
        process::exit(1);
    }
}

use std::env;
use std::process;
use minigrep::Config;

fn main() {
    
    //now we’re passing ownership of the iterator returned from env::args to Config::build directly.
    let config = Config::build(env::args() ).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    //println!("Searching for {}", config.query);
    //println!("In file {}", config.file_path);
    
    // let contents = fs::read_to_string(config.file_path).expect("Should have been able to read the file");
    //  println!("With text:\n{contents}");

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
// 13.4
//$ IGNORE_CASE=1 cargo run -- jos poema.txt
// cargo run -- jos poema.txt

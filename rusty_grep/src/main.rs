/**
* Author : @grainme
* Config : query + file_path
* !query should be the word we're looking for.
* !file_path should be the file we're querying.
*
* ----------------------------------------
*
* Implementation details:
* Config is a struct, impl build method for Config.
* std::env::args , fs::read_to_string(file_path)
*/
use {rusty_grep::run, rusty_grep::Config, std::process};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    // unwrap_or_else calls the code in closur (TBS chp 13)
    let config: Config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // if let instead of unwrap_or_else because run doesn't return an OK :)
    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

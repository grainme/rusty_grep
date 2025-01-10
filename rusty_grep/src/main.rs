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
    let config: Config = Config::build(std::env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

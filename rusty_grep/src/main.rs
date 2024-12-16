use std::fs;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 3 {
        panic!("invalid usage: you need more args than {}", args.len());
    }

    let query = &args[1];
    let file_specified = &args[2];

    let content =
        fs::read_to_string(file_specified).expect("should have been able to read the file!");

    println!("with text:\n {content}");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 3 {
        panic!("invalid usage: you need more args than {}", args.len());
    }

    let query = &args[1];
    let file_specified = &args[2];

    dbg!(&query, &file_specified);

    !todo!("read from file...")
}

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let query = &args[1];
    let filename = &args[2];

    let first_item = args.get(1);
    match first_item {
        Some(x) => println!("from Some first item {}", x),
        None => println!("none")
    }

    println!("query {}, filename {}", query, filename)
}

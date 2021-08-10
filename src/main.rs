use std::env;
use std::process;
use minigrep::Config;
use minigrep::run;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem passing arguments: {}", err);
        process::exit(1);
    });

    print_type_of(&config);

    fn print_type_of<T>(_: &T) {
        println!("type: {}", std::any::type_name::<T>())
    }

    let first_item = args.get(1);
    match first_item {
        Some(x) => println!("from Some first item {}", x),
        None => println!("none")
    }

    println!("query: {}, filename: {}", config.query, config.filename);

    if let Err(e) = run(config) {
        println!("Application error {}", e);
        process::exit(1);
    }

}

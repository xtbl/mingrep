use std::env;
use std::process;
use minigrep::Config;

fn main() {
    // let args: Vec<String> = env::args().collect();
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem passing arguments: {}", err);
        process::exit(1);
    });

    print_type_of(&config);

    fn print_type_of<T>(_: &T) {
        eprintln!("type: {}", std::any::type_name::<T>())
    }

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error {}", e);
        process::exit(1);
    }

}

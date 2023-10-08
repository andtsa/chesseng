pub mod moveordering;

pub fn run() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 2 {
        match args[2].as_str() {
            "ordering" => { moveordering::run() }
            _ => {
                println!("Unknown command");
            },
        }
    } else {
        println!("No experiment specified")
    }
}
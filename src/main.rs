use std::env;

fn main() {
    let argv: Vec<_> = env::args().collect();
    let argc = argv.len();
    match argc {
        1 => loop { println!("y"); },
        _ => loop { for i in 1..argc { print!("{} ", argv[i]); println!(""); } }
    }
}

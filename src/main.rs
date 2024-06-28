use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut result: String = String::new();

    for i in 1..args.len() {
        result.push_str(&args[i]);
        if i < args.len() - 1 {
            result.push_str(" ");
        }
    }

    println!("{}", result);
}

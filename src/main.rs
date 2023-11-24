use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let sliced_args = &args[1..];

    println!("{}", sliced_args.join(" "));
}

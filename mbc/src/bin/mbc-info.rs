use mbc::MBC;

use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);

    if args.len() != 2 {
        eprintln!("Usage: {} <rom file>", args[0]);
        std::process::exit(1);
    }

    let file = fs::read(args[1].clone()).unwrap();

    println!("{:#?}", MBC::new(file));
}

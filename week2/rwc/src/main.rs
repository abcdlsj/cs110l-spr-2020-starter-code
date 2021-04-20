use std::{env, fs::File, io::{self, BufRead}};
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Too few arguments.");
        process::exit(1);
    }
    let filename = &args[1];
    let file = File::open(filename).unwrap();
    let mut cha = 0;
    let mut wor  = 0;
    let mut lin = 0;
    for line in io::BufReader::new(file).lines() {
        let line_str = line.unwrap();
        lin += 1;
        cha += line_str.len();
        let v: Vec<&str> = line_str.split(' ').collect();
        wor += v.len();
    }
    println!("{} {} {} {}", lin, wor, cha, filename);
    // Your code here :)
}

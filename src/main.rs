#![allow(dead_code)]

use std::env;

fn main() {
    get_cli_args();
}


fn get_cli_args() {
    // check the length of args
    let args = env::args();

    let args: Vec<String> = args.map(|x| x.parse().unwrap()).collect();
    let args = args[1..].to_vec();

    if args.is_empty() {
        println!("No arguments specified, see --help for usage")
    }
    let oper = &args[0];
    println!("{oper}");
}

#[derive(Debug)]
enum OpType {
    Find,
    Create,
    Edit,
}





#![allow(dead_code)]

use std::env;
mod find;
mod create;
mod edit;


fn main() {
    get_cli_args();
}


fn get_cli_args() {
    let args = env::args();
    let args: Vec<String> = args.map(|x| x.parse().unwrap()).collect();
    let args = args[1..].to_vec();

    if args.is_empty() {
        eprintln!("no arguments specified, try --help for usage");
        std::process::exit(1);
    }

    let operation = &args[0];
    let flag_args = &args[1..];

    match operation.as_str() {
        "find" => find::find(flag_args.to_vec()),    
        "create" => create::create(flag_args.to_vec()),
        "edit" => edit::edit(flag_args.to_vec()),
        _ => { println!("invalid operation, try --help for usage"); }
    }
}



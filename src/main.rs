#![allow(dead_code)]

use std::env;

fn main() {
    get_cli_args();
}


fn get_cli_args() {
    let args = env::args();
    let args: Vec<String> = args.map(|x| x.parse().unwrap()).collect();
    let args = args[1..].to_vec();

    if args.is_empty() {
        println!("no arguments specified, try --help for usage");
        return;
    }

    match args[0].as_str() {
        "find" => find(args),    
        "create" => create(args),
        "edit" => edit(args),
        _ => { println!("invalid operation, try --help for usage"); }
    }
}

fn find(args: Vec<String>) {
    
}

fn create(args: Vec<String>) {

}

fn edit(args: Vec<String>) {

}

// for find second last value is always target-text and last value is the path
// so it would always follow the structure below
// find -----bunch of flag, value ---------- pattern path
// so if we get just find and none of these then it is error 
// if we get find and just one value, then find pattern in current directory(not recursviley)
// if we get find pattern path, then nice

#[derive(Debug)]
enum OpType {
    Find,
    Create,
    Edit,
}





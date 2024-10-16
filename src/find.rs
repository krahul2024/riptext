pub fn find(args: Vec<String>) {
    if args.is_empty() {
        exit();
    }

    parse_args(args);

}

struct FindArgs {
    flags: Vec<(String, String, String)>,
    pattern: String,
    path: String,
}

impl FindArgs {
    fn new() -> FindArgs {
        FindArgs {
            flags: Vec::new(),
            pattern: String::new(),
            path: String::new(),
        }
    }

    fn from(flags: Vec<(String, String, String)>, pattern: String, path: String) -> FindArgs {
        FindArgs {
            flags, pattern, path }
    }
}

// find [{--flag, value(optional)}, ...]  pattern   path(optional)
// Returns : (Vec<flag, value_type, value), pattern, path)
fn parse(args: Vec<String>) -> find_args {
    let mut find_args = FindArgs::new();
    let mut i = 0;

    while i < args.len() {
        let arg_value = args[i].clone();
        // means we have a flag, need to check it in flags and then look for its value 
        // on the basis of its value type
        if arg_value.starts_with('-') {
            let flag = arg_value.trim_start_matches('-');
            let flag_value_type = get_flag_type(flag);
            if flag_value_type == "" {
                // means there is no such flag
                eprintln!("invalid flag: {flag}");
                std::process::exit(1);
            }
            // let's get the value for the flag, since it is confirmed that
            // next we have a flag which should have some value, not if the value_type is boolean
            let mut flag_value = "";
            if flag_value_type == "bool" {
                flag_value = "true";
            } else {
                i += 1;
                if i >= args.len() {
                    eprintln!("invalid flag: {flag}");
                    std::process::exit(1);
                }
                flag_value = args[i].as_str();
            }

            // now is the time to check the flag value and its type
            check_flag_value(flag_value_type, flag_value);
            // now we have a nice flag, value, value type for us
            find_args.flags.push((flag.to_string(), flag_value_type.to_string(), flag_value.to_string()));
            i += 1; // go to next set of arguments
        } else {
            // now we have to get pattern and the path(if provided)
            let pattern = arg_value.to_string();
            let mut path = "".to_string();
            if i > args.len() - 1 {
                path = ".".to_string();
            } else {
                path = args[i+1].clone();
            }
            find_args.path = path;
            find_args.pattern = pattern;
        }
    } 

    find_args
}

fn get_flag_type(flag: &str) -> &str {
    let mut flag_type = "";

    for flag_item in FLAGS {
        if flag_item.short == flag || flag_item.long == flag {
            flag_type = flag_item.value_type.clone();
            break;
        }
    }

    return flag_type;
}

// check if the value is correctly parse-able to the mentioned value_type if so, then return the
// value string and value_type
fn check_flag_value(value_type: &str, value: &str) {
    match value_type {
        "bool" => {
            if value != "true" {
                eprintln!("invalid parsing {value} to {value_type}");
                std::process::exit(1);
            }
        },
        "string" => { /* nothing to do here */ },
        "int"   => { let _: i32 = value.parse().expect("error parsing to int"); },
        "float" => { let _: f32 = value.parse().expect("error parsing to float"); },
        _ => {
            eprintln!("invalid parsing {value} to {value_type}");
            std::process::exit(1);
        }
    }
}


fn parse_args(args: Vec<String>) {
    let mut i = 0;
    while i < args.len() {
        // if the current value is flag
        if args[i].starts_with('-') {
            // meaning it is a flag
            let flag = args[i].trim_start_matches('-');
            let mut flag_value = "".to_string();
            i += 1; // go to the next index
            if i >= args.len() { // if the next value is not provided, then we return error
                exit();
            } else {
                if args[i].starts_with('-'){ // if the previous flag is a boolean
                    flag_value = "true".to_string();
                    println!("flag: {flag}, value: {flag_value}");
                    continue;
                } else {
                    flag_value = args[i].clone();
                    i += 1;
                }
            }
            println!("flag: {flag}, value: {flag_value}");
        } else { // the start value is not a flag, we get pattern and path(next possibly) 
            let pattern = args[i].clone();
            i += 1;
            let path = if i < args.len() {args[i].clone()} else {".".to_string()};
            println!("pattern: {pattern}, path: {path}");
            break;
        }
    }
}

// this should be converted to a macro
fn exit() {
    eprintln!("\ninvalid arguments, use find --help\n");
    std::process::exit(1);
}
/*
now we have the list of arguments which can be [flag, value], pattern(mandatory), path(optional)
to parse this, i should iterate from the start and check if there is any flag, then I have two 
conditions, if the next item is a flag, then the previous flag is boolean and now has true value,
otherwise current item is value for previous flag, which I will match later if the provided 
value and value_type for that flag is correct or not?
after dealing with all the flags I should get the last two(at least one) values
if there is only one value left after flags then it means that I have to consider default path
which is .
*/

#[derive(Debug)]
struct Flag {
    short: &'static str,
    long: &'static str,
    value_type: &'static str,
    short_desc: &'static str,
    long_desc: &'static str,
}

static FLAGS: &[Flag] = &[
    Flag {
        short: "-h",
        long: "--help",
        value_type: "none",
        short_desc: "Show help",
        long_desc: "Displays the help information for the command-line tool.",
    },
    Flag {
        short: "-d",
        long: "--depth",
        value_type: "int",
        short_desc: "file/directory depth",
        long_desc: "depth upto which recursively search for all the files for specified path",
    },
    Flag {
        short: "-a",
        long: "--allow-hidden",
        value_type: "bool",
        short_desc: "search hidden files",
        long_desc: "search includes all the hidden files for specified path",
    },
    Flag {
        short: "-i",
        long: "--ignore-case",
        value_type: "bool",
        short_desc: "ignore case, default value is false",
        long_desc: "ignore the case for search, default value is false",
    },
    Flag { 
        short: "-e", 
        long: "--exclude", 
        value_type: "string", 
        short_desc: "Exclude files/directories", 
        long_desc: "Specifies files or directories to exclude from the search." 
    },
    Flag { 
        short: "-o", 
        long: "--output", 
        value_type: "string", 
        short_desc: "Write output to file", 
        long_desc: "Specifies the file to which the output will be written." 
    },
    Flag {
        short: "-l",
        long: "--limit",
        value_type: "int",
        short_desc: "no. of results to display",
        long_desc: "total no. of result to display, default is all the results",
    },
    Flag {
        short: "-c",
        long: "--color",
        value_type: "bool",
        short_desc: "output has colored pattern",
        long_desc: "output contains all the occurences of pattern colored",
    },
    Flag {
        short: "-r",
        long: "--recursive",
        value_type: "bool",
        short_desc: "recursively search",
        long_desc: "recursively search for all the files for specified path",
    },
];


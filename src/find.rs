pub fn find(args: Vec<String>) {
    if args.is_empty() {
        exit();
    }

    parse_args(args);

}
// -x x --fas -fsdf fasdf fasdf fasdf fasdf
// I am not taking into account of the case when
// rt find --boolean-flag pattern 
// to solve this, when I find a flag, I would check for the type
// of flag in the flags list, if it is boolean then don't consider the next value
// if the flag is not found in the flag list then return error
// if flag has a non boolean value then try parsing to the flag value type
// if that results in error, return that error, then move on to next part
// once done keep the start logic same until all the flags are checked,
// once all flags are checked then move to the pattern and path part
// if there are more values than 2, then either don't consider rest of the
// values except two, actually that's good choice
fn parse_args(args: Vec<String>) {
    let mut i = 0;
    // when using this method, I must get a flag at each iteration
    // if the value at the start is not a flag so the first value is a pattern
    // and if there is a second value then it is the path
    // so what I can do is when i find that the start value of loop is not a flag
    // then i just take this value as pattern and check if there is a next value
    // if so then it is the path, and discard rest of the values(if any left)
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

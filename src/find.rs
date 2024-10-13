pub fn find(args: Vec<String>) {
    if args.is_empty() {
        eprintln!("\ninvalid arguments, use find --help\n");
        std::process::exit(1);
    }

}

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
        }
    }
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

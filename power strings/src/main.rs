use std::io;
fn main(){
    loop {
        let input = get_input();
        if input == String::from(".") {
            return;
        } else {
            println!("{}", cps(&input));
        }
    }
    
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("lmao");
    return String::from(input.trim());
}

fn cps(string: &String) -> usize {
    let strlen = string.len();
    // we only need to check substrings with a length dividing the length of the input string
                            // add 1 here to avoid runtime error when strlen is 1 and iterator min > iterator max
    for sublen in 1..(1 + strlen/2) {
        if strlen % sublen != 0 {
            continue;
        } else {
            let string_to_check = &string[..sublen];
            if check_string(string, string_to_check) {
                // strlen is equal to sublen * the exponent
                return strlen/sublen;
            }
        }
    }

    return 1;
}

fn check_string(string: &str, substring: &str) -> bool {
    let mut start: usize = 0;
    let mut counter = 0;
    while let Some(index) = string[start..].find(substring) {
        start += index;
        start += substring.len();
        counter += 1;
    }
    // no need to check for fractions since we already promised that string % substring == 0
    counter == string.len() / substring.len()
}
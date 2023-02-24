use std::io;
use std::io::prelude::*;


// partly "inspired" by kattis io template as always
fn main() {
    // get standard input stream
    let input = io::stdin();
    let mut time_next_case = 0;
    let mut big_string = String::new();
    let mut words_to_test: Vec<String> = vec![];

    // get input lines as iterative
    let mut lines = input
        .lock()
        .lines()
        .map(|_line| _line.ok().unwrap());
    // and get one line at a time, stopping at EOF
    // storing every word fragment in a vector

    loop {
        if let Some(next_line) = lines.next() {
            // we check if the test case has ended, we are expecting an integer then
            if time_next_case == 0 {
                for word in &words_to_test {
                    print_vec(&search_through(&big_string, word))
                }
                time_next_case = 1 + next_line.parse::<i32>().unwrap();
                words_to_test = vec![];
                continue;
            } else if time_next_case == 1 {
                big_string = String::from(next_line);
            } else {
                words_to_test.push(next_line);
            }

        } else {
            for word in &words_to_test {
                print_vec(&search_through(&big_string, word))
            }
            break;
        }
        time_next_case -= 1;
    }


}




// find the occurences of a pattern in a word, and store the indices where the pattern is found in a Vec<usize>
fn search_through(string: &str, part: &str) -> Vec<usize> {
    let mut indices = Vec::new();
    let mut start = 0;

    // keep .find()-ing the pattern until there is nothing more to .find()
    while let Some(index) = string[start..].find(part) {
        let cur_index = start + index;
        indices.push(cur_index);
        start = cur_index + part.len();
    }

    return indices;
}

// print all elements of a usize vector
fn print_vec(vec: &Vec<usize>) {
    let mut string = String::from("");
    let vec_len = (*vec).len();
    if vec_len == 0 {
        println!("");
        return;
    }
    for i in 0..vec_len - 1 {
        string.push_str(&(vec[i]).to_string());
        string.push(' ');
    }
    // we print the last element separately to avoid a trailing blank space
    string.push_str(&(vec[vec_len - 1]).to_string());
    println!("{}", string);
}
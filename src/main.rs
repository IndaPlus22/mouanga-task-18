use std::io;
use std::io::prelude::*;


// partly "inspired" by kattis io template as always
fn main() {
    assert_eq!(true, part_exists("Kys fag".as_bytes(), b"Kys"));
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
                    // print_vec(&search_through(&(big_string).as_bytes(), (word).as_bytes()))
                    print_vec(&find_all_indices(&big_string, word))
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
                // print_vec(&search_through(&(big_string).as_bytes(), (word).as_bytes()))
                print_vec(&find_all_indices(&big_string, word))
            }
            break;
        }
        time_next_case -= 1;
    }


}




// find the occurences of a pattern in a word, and store the indices where the pattern is found in a Vec<usize>
fn search_through(string: &[u8], part: &[u8]) -> Vec<usize> {
    let mut indices = Vec::new();

    for index in 0..string.len() - part.len() + 1 {
        if part_exists(&string[index..], part) {
            indices.push(index);
        }
    }

    return indices;
}

// get the minimum of two
fn min(a: usize, b: usize) -> usize {
    if a < b {
        return a;
    }
    b
}

// print all elements of a usize vector
fn print_vec(vec: &Vec<usize>) {
    let mut string = String::with_capacity(500000);
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

// does "part" exist in the string at this position?
fn part_exists(string: &[u8], part: &[u8]) -> bool {

    for letter_index in 0..part.len() {
        if string[letter_index] != part[letter_index] {
            return false;
        }
    }
    

    return true;
}

fn boyer_moore_search(haystack: &[u8], needle: &[u8]) -> Vec<usize> {
    let n = haystack.len();
    let m = needle.len();
    let mut indices = vec![];
    let mut i = m - 1;
    let mut j = m - 1;

    let mut bad_char_table = [m; 256];
    for (k, &c) in needle.iter().enumerate().take(m - 1) {
        bad_char_table[c as usize] = m - k - 1;
    }

    let mut suffixes = vec![0; m];
    suffixes[m - 1] = m;
    let mut f = m - 1;
    for g in (0..m - 1).rev() {
        if g > f && suffixes[g + m - 1 - f] < g - f {
            suffixes[g] = suffixes[g + m - 1 - f];
        } else {
            if g < f {
                f = g;
            }
            while f >= 0 && needle[f] == needle[f + m - 1 - g] {
                f -= 1;
            }
            suffixes[g] = f + 1;
        }
    }

    while i < n {
        let hc = haystack[i];
        if hc == needle[j] {
            if j == 0 {
                indices.push(i);
                i += m;
                j = m - 1;
            } else {
                i -= 1;
                j -= 1;
            }
        } else {
            i += std::cmp::max(j as isize - bad_char_table[hc as usize] as isize, suffixes[j].try_into().unwrap()) as usize;
            j = m - 1;
        }
    }

    indices
}

fn find_all_indices(haystack: &str, needle: &str) -> Vec<usize> {
    boyer_moore_search(haystack.as_bytes(), needle.as_bytes())
}
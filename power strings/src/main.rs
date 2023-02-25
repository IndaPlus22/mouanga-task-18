use std::io;
const ONE: i32 = 1;
const ZERO: i32 = 0;
fn main(){
    loop {
        let input = get_input();
        if input.eq(&vec![46]) {
            return;
        } else {
            println!("{}", cps(&input));
        }
    }
}

fn get_input() -> Vec<u8> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("lmao");
    return input.trim().as_bytes().to_vec();
}

// calculate power string
fn cps(input: &[u8]) -> usize {

    let len = input.len();
    let mut letters_used: [bool; 26] = [false; 26];
    let mut current_word: Vec<u8> = vec![];

    for letter in input {
        // this letter has already been used; string could be repeating from this point on
        if letters_used[*letter as usize - 97] {
            let mut equality = true;
            println!("Testing!");
            for _letter in &current_word {
                if _letter != letter {
                    equality = false;
                    break;
                }
            if equality {
                return len / current_word.len();
            }
        }
        }
        // note: 97 is the ascii code point for 'a'
        letters_used[*letter as usize - 97] = true;
    }
    69000
}

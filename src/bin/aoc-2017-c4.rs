use std::io;
use std::collections::BTreeSet;

// use unicode to support multi-codepoint graphemes in passphrases
extern crate unicode_segmentation;
use unicode_segmentation::UnicodeSegmentation;

fn check_valid(inputs: &[&str]) -> bool
{
    let mut set = BTreeSet::new();
    for word in inputs {
        set.insert(word);
    }
    inputs.len() == set.len()
}

fn check_valid2(inputs: &[&str]) -> bool
{
    let mut set = BTreeSet::new();
    for word in inputs {
        let mut graphemes = word.graphemes(true).collect::<Vec<&str>>();
        graphemes.sort_unstable();
        set.insert(graphemes.into_iter().collect::<String>());
    }
    inputs.len() == set.len()
}

fn main() {
    println!("Enter puzzle input: ");
    let mut valid_count1 = 0usize;
    let mut valid_count2 = 0usize;
    loop {
        let mut input_str = String::new();
        match io::stdin().read_line(&mut input_str) {
            Ok(num_bytes) if num_bytes > 1 => {
                let inputs:Vec<&str> = input_str
                    .trim()
                    .split_whitespace()
                    .collect();
                if check_valid(&inputs) { valid_count1 += 1 };
                if check_valid2(&inputs) { valid_count2 += 1 };
            }
            Ok(_) => {break;}
            Err(_) => {std::process::exit(1);}
        };
    };
    println!{"Part 1: {}",valid_count1};
    println!{"Part 2: {}",valid_count2};
}

#[test]
fn test_example_1() {
    assert_eq!(check_valid(&vec!["aa", "bb", "cc", "dd", "ee"]), true);
}

#[test]
fn test_example_2() {
    assert_eq!(check_valid(&vec!["aa", "bb", "cc", "dd", "aa"]), false);
}

#[test]
fn test_example_2_1() {
    assert_eq!(check_valid2(&vec!["a","ab","abc","abd","abf","abj"]), true);
}

#[test]
fn test_example_2_2() {
    assert_eq!(check_valid2(&vec!["oiii","ioii","iioi","iiio"]), false);
}


use std::io;
use std::io::prelude::*;

extern crate unicode_segmentation;
use unicode_segmentation::UnicodeSegmentation;

fn calc_sum1(inputs: &[u32]) -> u32
{
    let mut sum = 0u32;
    for idx in 0..inputs.len()-1 {
        if inputs[idx] == inputs[idx+1] {
            sum += inputs[idx];
        }
    }
    if inputs[inputs.len()-1] == inputs[0] {
        sum += inputs[inputs.len()-1];
    }
    sum
}

fn calc_sum2(inputs: &[u32]) -> u32
{
    let mut sum = 0u32;
    let offset = inputs.len() / 2;
    let chk_idx = |x|{ (x + offset) % inputs.len() };

    for idx in 0..inputs.len() {
        if inputs[idx] == inputs[chk_idx(idx)] {
            sum += inputs[idx];
        }
    }
    sum
}

fn main() {
    println!("Enter puzzle input: ");
    let mut input_bytes = Vec::new();
    match io::stdin().read_to_end(&mut input_bytes) {
        Ok(num_bytes) if num_bytes > 1 => {
            let inputs = String::from_utf8(input_bytes)
                .unwrap()
                .trim()
                .graphemes(true)
                .map(|c|
                {
                    match c.parse::<u32>() {
                        Ok(n)  => n,
                        Err(_) => {std::process::exit(1);}
                    }
                })
                .collect::<Vec<u32>>();
            println!{"Part 1: {}",calc_sum1(&inputs)};
            println!{"Part 2: {}",calc_sum2(&inputs)};
            std::process::exit(0);
        }
        Ok(_) => {std::process::exit(0);}
        Err(_) => {std::process::exit(1);}
    };
}

#[test]
fn test_example_1() {
    assert_eq!(calc_sum1(&vec![1,1,2,2]),3);
}

#[test]
fn test_example_2() {
    assert_eq!(calc_sum1(&vec![1,1,1,1]),4);
}

#[test]
fn test_example_3() {
    assert_eq!(calc_sum2(&vec![1,2,1,2]),6);
}

#[test]
fn test_example_4() {
    assert_eq!(calc_sum2(&vec![1,2,1,3,1,4,1,5]),4);
}

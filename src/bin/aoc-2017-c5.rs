use std::io;
use std::io::prelude::*;

fn calc_jumps(inputs: &mut [isize]) -> usize
{
    let mut pc = 0usize;
    let mut jumps = 0usize;
    while pc < inputs.len() {
        let jump = inputs[pc];
        inputs[pc] += 1;
        pc = (pc as isize + jump) as usize;
        jumps += 1;
    }
    jumps
}

fn calc_jumps2(inputs: &mut [isize]) -> usize
{
    let mut pc = 0usize;
    let mut jumps = 0usize;
    while pc < inputs.len() {
        let jump = inputs[pc];
        inputs[pc] += if jump < 3 { 1 } else { -1 };
        pc = (pc as isize + jump) as usize;
        jumps += 1;
    }
    jumps
}

fn main() {
    println!("Enter puzzle input: ");
    let mut input_bytes = Vec::new();
    match io::stdin().read_to_end(&mut input_bytes) {
        Ok(num_bytes) if num_bytes > 1 => {
            let inputs = String::from_utf8(input_bytes)
                .unwrap()
                .trim()
                .split_whitespace()
                .map(|c|
                {
                    match c.parse::<isize>() {
                        Ok(n)  => n,
                        Err(_) => {
                            println!("Couldnt parse '{}'",c);
                            std::process::exit(1);
                        }
                    }
                })
                .collect::<Vec<isize>>();
            println!{"Part 1: {}",calc_jumps(&mut inputs.clone())};
            println!{"Part 2: {}",calc_jumps2(&mut inputs.clone())};
        }
        Ok(_) => {std::process::exit(0);}
        Err(_) => {std::process::exit(1);}
    };
}

#[test]
fn test_example_1() {
    assert_eq!(calc_jumps(&mut vec![0,3,0,1,-3]),5);
}

#[test]
fn test_example_2() {
    assert_eq!(calc_jumps2(&mut vec![0,3,0,1,-3]),10);
}


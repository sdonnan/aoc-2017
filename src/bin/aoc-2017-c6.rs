use std::io;
use std::io::prelude::*;
use std::collections::BTreeSet;

fn calc_cycles(inputs: &mut [usize]) -> usize
{
    let banks = inputs.len();
    let mut set: BTreeSet<Vec<usize>> = BTreeSet::new();
    set.insert(inputs.to_vec());
    let mut cycle_count = 1usize;
    loop {
        let mut idx = 0usize;
        let mut count = 0usize;
        // find bank with most blocks
        for bank in 0..banks {
            if inputs[bank] > count {
                idx = bank;
                count = inputs[bank];
            }
        }
        // zero out bank
        inputs[idx] = 0;
        // distribute blocks
        while count > 0 {
            idx = (idx + 1) % banks;
            inputs[idx] += 1;
            count -= 1;
        }
        // check for repeat
        if set.contains(inputs) { break; }
        else { set.insert(inputs.to_vec()); }
        cycle_count += 1;
    }
    cycle_count
}

fn calc_loop_size(inputs: &[usize]) -> usize
{
    let banks = inputs.len();
    let mut cycle_count = 1usize;
    let mut scratch = inputs.to_vec();
    loop {
        let mut idx = 0usize;
        let mut count = 0usize;
        // find bank with most blocks
        for bank in 0..banks {
            if scratch[bank] > count {
                idx = bank;
                count = scratch[bank];
            }
        }
        // zero out bank
        scratch[idx] = 0;
        // distribute blocks
        while count > 0 {
            idx = (idx + 1) % banks;
            scratch[idx] += 1;
            count -= 1;
        }
        // check for input pattern
        if inputs == &scratch[0..inputs.len()] { break; }
        cycle_count += 1;
    }
    cycle_count
}

fn main() {
    println!("Enter puzzle input: ");
    let mut input_bytes = Vec::new();
    match io::stdin().read_to_end(&mut input_bytes) {
        Ok(num_bytes) if num_bytes > 1 => {
            let mut inputs = String::from_utf8(input_bytes)
                .unwrap()
                .trim()
                .split_whitespace()
                .map(|c|
                {
                    match c.parse::<usize>() {
                        Ok(n)  => n,
                        Err(_) => {
                            println!("Couldnt parse '{}'",c);
                            std::process::exit(1);
                        }
                    }
                })
                .collect::<Vec<_>>();
            println!{"Part 1: {}",calc_cycles(&mut inputs)};
            println!{"Part 2: {}",calc_loop_size(&inputs)};
        }
        Ok(_) => {std::process::exit(0);}
        Err(_) => {std::process::exit(1);}
    };
}

#[test]
fn test_example_1() {
    assert_eq!(calc_cycles(&mut vec![0,2,7,4]),5);
}

#[test]
fn test_example_2() {
    assert_eq!(calc_loop_size(&vec![2,4,1,2]),4);
}


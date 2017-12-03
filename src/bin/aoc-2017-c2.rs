use std::io;

// The slice of vectors seems ghetto

fn calc_sum1(inputs: &[Vec<u32>]) -> u32
{
    let mut check = 0u32;
    for row in inputs {
        let mut max:u32 = row[0];
        let mut min:u32 = row[0];
        for item in row {
            if *item > max { max = *item };
            if *item < min { min = *item };
        }
        check += max - min;
    }
    check
}

fn calc_sum2(inputs: &[Vec<u32>]) -> u32
{
    let mut check = 0u32;
    for row in inputs {
        let end = row.len();
        let mut div = 0u32;
        let mut flag = false;
        // for each index
        for idx in 0..end {
            // check it against the other indexes
            for jdx in 0..end {
                div = row[idx] / row[jdx];
                // set the flag to break out when a match is found
                if (row[idx] as f32 / row[jdx] as f32) == div as f32 && idx != jdx {
                    flag = true;
                }
                if flag {break};
            }
            if flag {break};
        }
        check += div;
    }
    check
}

fn main() {
    println!("Enter puzzle input: ");
    let mut lines:Vec<Vec<u32>> = Vec::new();
    loop {
        let mut input_str = String::new();
        match io::stdin().read_line(&mut input_str) {
            Ok(num_bytes) if num_bytes > 1 => {
                let inputs:Vec<u32> = input_str
                    .trim()
                    .split_whitespace()
                    .map(|s|
                    {
                        match s.parse::<u32>() {
                            Ok(n)  => n,
                            Err(_) => {std::process::exit(1);}
                        }
                    })
                    .collect();
                lines.push(inputs);
            }
            Ok(_) => {break;}
            Err(_) => {std::process::exit(1);}
        };
    };
    println!{"Part 1: {}",calc_sum1(lines.as_slice())};
    println!{"Part 2: {}",calc_sum2(lines.as_slice())};
}

#[test]
fn test_example_1() {
    assert_eq!(calc_sum1(vec![vec![5,1,9,5],
                              vec![7,5,3],
                              vec![2,4,6,8]].as_slice()),18);
}

#[test]
fn test_example_2() {
    assert_eq!(calc_sum2(vec![vec![5,9,2,8],
                              vec![9,4,7,3],
                              vec![3,8,6,5]].as_slice()),9)
}

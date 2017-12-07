use std::io;

// this seems overly complicated. I probably missed the clever way to do this
fn calc_coords(location: usize) -> (isize,isize) {
    if location == 1 { return (0,0) };
    let mut side_len = 1;
    let mut last_loc = 1;
    let mut next_last_loc;
    // calculate the ring we are on
    loop {
        side_len += 2;
        let ring_len = side_len*4-4;
        next_last_loc = last_loc + ring_len;
        if next_last_loc >= location {break;}
        last_loc = next_last_loc;
    }
    // based on the index figure out coords
    let index = location - last_loc; // relative location on current level
    let is_east  = index < side_len || location == next_last_loc;
    let is_north = index < side_len * 2 - 1 && index > side_len - 2;
    let is_west  = index < side_len * 3 - 2 && index > side_len * 2 - 3;
    let is_south = index > side_len * 3 - 4;
    let x =
        if      is_east  { (side_len as isize - 1)/2 }
        else if is_west  { -(side_len as isize - 1)/2 }
        else if is_north { ((side_len as isize - 1)/2 -
                           (index as isize - side_len as isize + 1)) }
        else if is_south { (-(side_len as isize - 1)/2 +
                           (index as isize - side_len as isize * 3 + 3)) }
        else {0};
    let y =
        if      is_north { (side_len as isize - 1)/2 }
        else if is_south { -(side_len as isize - 1)/2 }
        else if is_east  { (-(side_len as isize - 1)/2 +
                           (index as isize )) }
        else if is_west  { ((side_len as isize - 1)/2 -
                           (index as isize - side_len as isize * 2 + 2)) }
        else {0};
    (x,y)
}

// lacking cleverness at this point so will bruteforce fixed size area
fn calc_pt2(threshold: usize) -> Option<usize> {
    let mut area = [[0usize; 15]; 15];
    let mut result = Option::None;
    // initialize index 1
    area[8][8] = 1;
    for idx in 2..(13*13) {
        let (x,y) = calc_coords(idx);
        let x = (x + 8) as usize;
        let y = (y + 8) as usize;
        let val = area[x+1][y+1] +
                  area[x+1][y-1] +
                  area[x-1][y-1] +
                  area[x-1][y+1] +
                  area[x+1][y] +
                  area[x-1][y] +
                  area[x][y-1] +
                  area[x][y+1];
        area[x][y] = val;
        if val > threshold {
            result = Option::Some(val);
            break
        }
    }
    result
}

fn calc_dist(x: isize, y: isize) -> usize {
    (x.abs() + y.abs()) as usize
}

fn calc_dist_loc(location: usize) -> usize {
    let (x,y) = calc_coords(location);
    calc_dist(x,y)
}

fn main() {
    println!("Enter puzzle input: ");
    let mut input_str = String::new();
    match io::stdin().read_line(&mut input_str) {
        Ok(num_bytes) if num_bytes > 1 => {
            if let Ok(x) = input_str.trim().parse::<usize>() {
                println!{"Part 1: {}", calc_dist_loc(x)};
                println!{"Part 2: {}", calc_pt2(x).unwrap()};
            } else {
                println!("Invalid input");
                std::process::exit(1);
            }
        }
        Ok(_) => {
            println!("Input position number");
            std::process::exit(0);
        }
        Err(_) => {std::process::exit(1);}
    };
}

#[test]
fn test_example_1() {
    assert_eq!(calc_dist_loc(1),0);
}

#[test]
fn test_example_2() {
    assert_eq!(calc_dist_loc(12),3);
}

#[test]
fn test_example_3() {
    assert_eq!(calc_dist_loc(16),3);
}

#[test]
fn test_example_4() {
    assert_eq!(calc_dist_loc(23),2);
}

#[test]
fn test_example_5() {
    assert_eq!(calc_dist_loc(1024),31);
}

#[test]
fn test_part_2() {
    assert_eq!(calc_pt2(58).unwrap(),59);
}

#[test]
fn test_part_2_2() {
    assert_eq!(calc_pt2(145).unwrap(),147);
}

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;
use std::vec::Vec;

fn main() {
    let expected_sum = 2020;
    two_sum(expected_sum);
    three_sum(expected_sum);
}

fn three_sum(expected_sum: i32) {
    let mut numbers_map = HashMap::new();
    let mut numbers = Vec::new();

    if let Ok(lines) = read_lines("../puzzle_1_input.txt") {
	// for some reason we only get mutable iterator from lines.
	for line in lines {
	    if let Ok(number) = line {
		let number_int_val = number.parse::<i32>().unwrap();
		numbers.push(number_int_val);
	    }
	}
    }

    for (i, num) in numbers.iter().enumerate() {
	for (j, num_inner) in numbers.iter().enumerate() {
	    if i != j {
		let sum = num + num_inner;
		if sum < expected_sum {
		    let v = vec![num, num_inner];
		    numbers_map.insert(v, expected_sum - sum);
		}
	    }
	}
    }

    for (key, value) in numbers_map.iter() {
	let found = numbers.iter().find(|&x| x == value).is_some();
	if found {
	    println!("Three numbers are: {}, {} and {}", key[0], key[1], value);
	    println!("Three sum is: {}", key[0] * key[1] * value);
	    break;
	}	
    }
}


fn two_sum(expected_sum: i32) {
    let mut numbers = HashMap::new();
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("../puzzle_1_input.txt") {
	for line in lines {
	    if let Ok(number) = line {
		let number_int_val = number.parse::<i32>().unwrap();
		let missing_value = expected_sum - number_int_val;
		numbers.insert(number_int_val, missing_value);

		// This is sort of an optimization over our original algorithm
		// we can update map and at the same time do a lookup to find the other
		// number using O(1) look up.
		if numbers.contains_key(&missing_value) {
		    println!("Numbers: {} and {}", number_int_val, missing_value);
		    println!("Product: {}", number_int_val * missing_value);
		    break;
		}
	    }
	}
    }

    /**
     * @Speed commented becaused we are able to the lookup in previous loop.
     * 
     * A good lesson here was that if we are not changing anything in the hashmap
     * We don't need to get a mutable reference. For immutable reference, Rust doesn't 
     * need to worry about ownership.
     */
    // for (key, value) in numbers.iter() {
    // 	if numbers.contains_key(&value) {
    // 	    println!("Numbers: {} and {}", key, value);
    // 	    println!("Product: {}", key * value);
    // 	    break;
    // 	}
    // }

    println!("Total numbers: {}", numbers.len());
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

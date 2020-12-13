use std::vec::Vec;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct Password<'a> {
    upper_limit: i16,
    lower_limit: i16,
    required_char: char,
    password_text: &'a str
}

impl <'a> Password<'a> {
    fn is_valid_password(&self) -> bool {
	let mut count = 0;
	for c in self.password_text.chars() {
	    if c == self.required_char {
		count += 1;
	    }
	}
	count >= self.lower_limit && count <= self.upper_limit
    }

    // This is for second excercise of advent of code.
    fn is_valid_password_using_limit_as_position(&self) -> bool {
	if  self.password_text.len() < self.upper_limit as usize {
	    return false
	}
	
	let chars: Vec<char> = self.password_text.chars().collect();
	let lower = self.lower_limit - 1;
	let upper = self.upper_limit -1;

	let lower_contains = chars[lower as usize] == self.required_char;
	let upper_contains = chars[upper as usize] == self.required_char;


	lower_contains ^ upper_contains 
    }
}

fn find_valid_password_count() -> usize {
    let mut count = 0;
    if let Ok(lines) = read_lines("input.txt") {
	for line in lines {
	    if let Ok(text) = line {
		// split line by whitespace
		let split_values: Vec<&str> = text.split_whitespace().collect();
		
		// extract the limits
		let limits: Vec<&str> = split_values[0].split('-').collect();
		let lower_limit = limits[0].parse::<i16>().unwrap();
		let upper_limit = limits[1].parse::<i16>().unwrap();

		// get the char in policy
		let required_char = split_values[1].chars().next().unwrap();

		// get the password
		let password_text = split_values[2];
		
		let password = Password {
		    upper_limit, lower_limit, required_char, password_text
		};
		
		if password.is_valid_password_using_limit_as_position() {
		    count += 1;
		}
	    }
	}
    }

    count    
}

fn main() {
    println!("Valid passwords: {}", find_valid_password_count());
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

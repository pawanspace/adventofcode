use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;

// method finds the trees at a particular slope, give x and y
fn count_trees(slope_x: usize, slope_y: usize) -> usize {
    let mut trees = 0;
    let mut current_index = 0;
    if let Ok(lines) = read_lines("input.txt") {
	for (idx, line) in lines.enumerate() {
	    if let Ok(text) = line {
		if current_index == 0 {
		    current_index += slope_x;
		    continue;
		}

		if idx % slope_y != 0 {
		    continue;
		}
		

		// here we are running out of index so we will need to treat each
		// line as a circular array. let's say we have a line with 7 chars
		// if we hit current_index 8 then we should point to index 1.
		// so we use mod to find the next index.
		let index_in_use = current_index %  text.len();

		let c = text.chars().nth(index_in_use).unwrap();
		if c == '#' {
		    trees += 1;
		}
		
		current_index += slope_x;
	    }
	}
    }

    return trees;
}

fn main() {   
    let before = Instant::now();

    let slope_1 = count_trees(1, 1);
    let slope_2 = count_trees(3, 1);
    let slope_3 = count_trees(5, 1);
    let slope_4 = count_trees(7, 1);
    let slope_5 = count_trees(1, 2);
    
    println!("Total tree: {}", (slope_1 * slope_2 * slope_3 * slope_4 * slope_5));
    println!("Elapsed time: {:.2?}", before.elapsed());
}




// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


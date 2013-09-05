use std::{os, float};

fn main() {
	let nums = os::args();
	if nums.len() == 1 {
		println("No arguments provided!");
		return
	}

	let mut count = 0.0;
	let sum : float = nums.iter().skip(1).fold(0.0, |s, n| {
		match float::from_str(*n) {
			Some(number) => { 
				count += 1.0;
				s + number
			}, 
			None => { 
				println(fmt!("Error: %s is not a valid number.", *n));
				s
			}
		}
	});
	println(fmt!("The average is: %f", (sum / count)))
}
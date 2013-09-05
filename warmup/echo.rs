use std::{os};

fn main() {
	let words: ~[~str] = os::args();
	for words.iter().skip(1).advance |w| {
		print(fmt!("%s ",*w));
	}
	print("\n")
}
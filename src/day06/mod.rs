use crate::utils::linesFromFile;
use std::collections::HashMap;

pub fn solve(file: &str) {
	let lines = linesFromFile(file);
	let line = lines.get(0).unwrap();

	dbg!(findPacket(line, 4));
	dbg!(findPacket(line, 14));
}

fn findPacket(line: &String, packetSize: usize) -> usize {
	let mut seenLetters: HashMap<char, u32> = HashMap::new();
	for j in 0..packetSize {
		let count = seenLetters.entry(charAtLine(line, j)).or_insert(0);
		*count += 1
	}
	let mut i = 1;
	'nextWindow: while (i + packetSize - 1) < line.len() {
		let countOfPrevChar = seenLetters.entry(charAtLine(line, i - 1)).or_insert(1);
		*countOfPrevChar -= 1;

		let newCharCount = seenLetters
			.entry(charAtLine(line, i + packetSize - 1))
			.or_insert(0);
		*newCharCount += 1;

		for (_, frequency) in seenLetters.iter() {
			if frequency >= &2 {
				i += 1;
				continue 'nextWindow;
			}
		}
		return i + packetSize;
	}

	i + packetSize
}

fn charAtLine(line: &String, i: usize) -> char {
	*line.as_bytes().get(i).unwrap() as char
}

use std::collections::HashSet;

use crate::utils::linesFromFile;

pub fn sumOfPriorities(input: &str) -> u32 {
	let lines = linesFromFile(format!("src/day03/{}.txt", input));

	let mut result: u32 = 0;
	for l in lines {
		let compartments = l.split_at(l.len() / 2);
		let compartments = (
			compartments.0.chars().collect::<HashSet<_>>(),
			compartments.1.chars().collect::<HashSet<_>>(),
		);

		let mut commonChar = compartments.0.intersection(&compartments.1);
		let c = commonChar.next().unwrap();
		result += match c {
			'a'..='z' => *c as u32 - 'a' as u32 + 1,
			'A'..='Z' => *c as u32 - 'A' as u32 + 27,
			_ => panic!("aaaaa"),
		}
	}

	result
}

pub fn elfGroups(input: &str) -> u32 {
	let lines = linesFromFile(format!("src/day03/{}.txt", input));

	let mut result = 0;
	let mut i = 0;
	while i + 2 < lines.len() {
		let (elf1, elf2, elf3) = (
			lines[i].chars().collect::<HashSet<_>>(),
			lines[i + 1].chars().collect::<HashSet<_>>(),
			lines[i + 2].chars().collect::<HashSet<_>>(),
		);

		let c = elf1.intersection(&elf2).copied().collect::<HashSet<_>>();
		let c = c.intersection(&elf3).next().unwrap();

		result += match c {
			'a'..='z' => *c as u32 - 'a' as u32 + 1,
			'A'..='Z' => *c as u32 - 'A' as u32 + 27,
			_ => panic!("aaaaa"),
		};

		i += 3;
	}

	result
}

use crate::utils::linesFromFile;

pub fn solve(input: &str) {
	let fileContent = linesFromFile(format!("src/day01/{}.txt", input));

	let mut currentWeight = 0;
	let mut elves = Vec::new();

	for line in fileContent {
		if line.is_empty() {
			elves.push(currentWeight);
			currentWeight = 0;
		} else {
			currentWeight += line.parse::<u32>().expect("fuck off")
		}
	}

	elves.sort();

	match heaviestElf(&elves) {
		Some(elf) => println!("{elf}"),
		None => println!("fuck off"),
	}

	println!("{:?}", heaviest3Elves(&elves))
}

fn heaviestElf(elves: &[u32]) -> Option<&u32> {
	elves.last()
}

fn heaviest3Elves(elves: &Vec<u32>) -> u32 {
	elves[elves.len() - 3..].iter().sum()
}

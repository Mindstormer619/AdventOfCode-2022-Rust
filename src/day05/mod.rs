use std::mem;
use regex::Regex;
use crate::utils::linesFromFile;

type CharStack = Vec<char>;

pub fn solve(input: &str) {
	let lines = linesFromFile(input);

	let stackPortion: Vec<&String> = lines.iter().take_while(|&l| !l.starts_with(" 1")).collect();

	let mut stacks = createStacks(&stackPortion);

	let movePortion = lines.iter().filter(|l| l.starts_with("move"));

	let moveRegex = Regex::new(r"move ([0-9]+) from ([0-9]+) to ([0-9]+)").unwrap();
	let moves: Vec<Move> = movePortion.map(|m|
		{
			let captures = moveRegex.captures(m).unwrap();
			Move {
				count: captures.get(1).unwrap().as_str().parse::<u32>().unwrap(),
				from: captures.get(2).unwrap().as_str().parse::<usize>().unwrap() - 1,
				to: captures.get(3).unwrap().as_str().parse::<usize>().unwrap() - 1,
			}
		}
	).collect();


	dbg!(singleMove(&mut stacks, &moves));

	let mut stacks = createStacks(&stackPortion);
	dbg!(multiMove(&mut stacks, &moves));
}

fn multiMove(stacks: &mut [CharStack], moves: &Vec<Move>) -> String {
	for m in moves {
		let mut fromStack = mem::take(&mut stacks[m.from]);
		let newLength = fromStack.len() - m.count as usize;
		stacks[m.to].extend_from_slice(&fromStack[newLength..]);
		fromStack.truncate(newLength);
		stacks[m.from] = fromStack;
	}

	let result = stacks.iter().map(|s| s.last().unwrap()).collect::<String>();
	result
}

fn singleMove(stacks: &mut [CharStack], moves: &Vec<Move>) -> String {
	for m in moves {
		for _ in 0..m.count {
			let c = stacks[m.from].pop().unwrap();
			stacks[m.to as usize].push(c);
		}
	}

	let result = stacks.iter().map(|s| s.last().unwrap()).collect::<String>();
	result
}

struct Move {
	count: u32,
	from: usize,
	to: usize,
}

fn createStacks(stackPortion: &Vec<&String>) -> Vec<CharStack> {
	let mut stacks: Vec<CharStack> = Vec::new();

	let (mut j, mut index) = (1, 0);
	while j < stackPortion[0].len() {
		stacks.push(Vec::new());
		for i in (0..stackPortion.len()).rev() {
			let x = stackPortion[i].as_bytes()[j] as char;
			if x != ' ' {
				stacks[index].push(x)
			}
		}
		j += 4;
		index += 1;
	}

	stacks
}

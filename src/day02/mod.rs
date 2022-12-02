use crate::utils::linesFromFile;

struct Round {
    player1: i8,
    player2: i8,
}

/*
ROCK = 0 = lose
PAPER = 1 = draw
SCISSOR = 2 = win
*/

impl Round {
    fn score1(&self, play: i8) -> i32 {
        (play
            + 1
            + match self.player2 - play {
                0 => 3,
                1 | -2 => 0,
                _ => 6,
            })
        .into()
    }

    fn score2(&self) -> i32 {
        self.score1(self.play())
    }

    fn play(&self) -> i8 {
        match self.player1 {
            1 => self.player2,
            0 => match self.player2 {
                //LOSE
                0 => 2,
                _ => self.player2 - 1,
            },
            _ => match self.player2 {
                //WIN
                2 => 0,
                _ => self.player2 + 1,
            },
        }
    }
}

pub fn strat1(input: &str) -> i32 {
    let lines = linesFromFile(format!("src/day02/{}.txt", input));
    let rounds: Vec<Round> = collectInput(lines);

    rounds.iter().map(|r| -> i32 { r.score1(r.player1) }).sum()
}

pub fn strat2(input: &str) -> i32 {
    let lines = linesFromFile(format!("src/day02/{}.txt", input));
    let rounds: Vec<Round> = collectInput(lines);

    rounds.iter().map(|r| -> i32 { r.score2() })
	.sum()
}

fn collectInput(lines: Vec<String>) -> Vec<Round> {
    lines
        .iter()
        .map(|l| -> Round {
            let player2 = match l.chars().nth(0).unwrap() {
                'A' => 0,
                'B' => 1,
                'C' => 2,
                _ => unreachable!(),
            };
            let player1 = match l.chars().nth(2).unwrap() {
                'X' => 0,
                'Y' => 1,
                'Z' => 2,
                _ => unreachable!(),
            };
            Round {
                player1: player1,
                player2: player2,
            }
        })
        .collect()
}

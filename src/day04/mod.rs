use crate::utils::linesFromFile;

pub fn solve(input: &str) {
    let lines = linesFromFile(format!("src/day04/{}.txt", input));

    dbg!(fullOverlaps(&lines));
    dbg!(anyOverlaps(&lines));
}

fn fullOverlaps(lines: &[String]) -> u32 {
    lines.iter().map(|l| /* "2-3,4-5" */ l.split(','))
        .map(|pairOfRanges| // ["2-3", "4-5"]
            {
                let mut pairOfRanges = pairOfRanges.map(|range| // "2-3"
                    {
                        let mut numsInRange = range.split('-').map(|n| // "2"
                            n.parse::<u32>().unwrap()
                        );
                        (numsInRange.next().unwrap(), numsInRange.next().unwrap())
                    }
                );
                (pairOfRanges.next().unwrap(), pairOfRanges.next().unwrap())
            }
        )
        .filter(|pairs| contains(pairs.0, pairs.1))
        .count() as u32
}

fn anyOverlaps(lines: &[String]) -> u32 {
    lines.iter().map(|l| /* "2-3,4-5" */ l.split(','))
        .map(|pairOfRanges| // ["2-3", "4-5"]
            {
                let mut pairOfRanges = pairOfRanges.map(|range| // "2-3"
                    {
                        let mut numsInRange = range.split('-').map(|n| // "2"
                            n.parse::<u32>().unwrap()
                        );
                        (numsInRange.next().unwrap(), numsInRange.next().unwrap())
                    }
                );
                (pairOfRanges.next().unwrap(), pairOfRanges.next().unwrap())
            }
        )
        .filter(|pairs| overlaps(pairs.0, pairs.1))
        .count() as u32
}

fn contains(range1: (u32, u32), range2: (u32, u32)) -> bool {
    (range1.0 <= range2.0 && range1.1 >= range2.1)
        || (range2.0 <= range1.0 && range2.1 >= range1.1)
}

fn overlaps(range1: (u32, u32), range2: (u32, u32)) -> bool {
    range1.1 >= range2.0 && range2.1 >= range1.0
}
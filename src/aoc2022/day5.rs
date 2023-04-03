use std::{fs, vec};

fn parse_stacks(data: &str) -> Vec<Vec<char>> {
    let lines: Vec<&str> = data.lines().collect();
    let len = lines.len();

    let mut stacks: Vec<Vec<char>> = vec![vec![]; if len % 2 == 0 { len - 1 } else { len }];

    for line in lines {
        let chars: Vec<char> = line.chars().collect();

        for (idx, c) in chars
            .chunks(4)
            .map(|chunk| {
                chunk
                    .iter()
                    .skip(1)
                    .step_by(2)
                    .filter(|f| **f != ' ')
                    .copied()
                    .collect::<Vec<_>>()
            })
            .enumerate()
        {
            if !c.is_empty() && c[0].is_alphabetic() {
                stacks[idx % len].push(c[0]);
            }
        }
    }

    for stack in &mut stacks {
        stack.reverse();
    }

    stacks
}

fn parse_procedure(data: &str) -> Vec<(usize, usize, usize)> {
    data.lines()
        .map(|line| {
            let instructions: Vec<usize> = line
                .split_whitespace()
                .skip(1)
                .step_by(2)
                .map(|f| f.parse().unwrap())
                .collect();

            (instructions[0], instructions[1], instructions[2])
        })
        .collect::<Vec<_>>()
}

fn parse(data: &str) -> (Vec<Vec<char>>, Vec<(usize, usize, usize)>) {
    let (stacks, procedure) = data.split_once("\n\n").unwrap();

    (parse_stacks(stacks), parse_procedure(procedure))
}

fn part1(stacks: Vec<Vec<char>>, procedures: Vec<(usize, usize, usize)>) -> String {
    let mut stacks = stacks;
    for (quantity, from, to) in procedures {
        let len = stacks[from - 1].len();
        let items = stacks[from - 1]
            .splice(len - quantity.., vec![])
            .rev()
            .collect::<Vec<_>>();
        stacks[to - 1].extend(items);
    }

    stacks
        .into_iter()
        .map(|stack| *stack.last().unwrap())
        .collect::<String>()
}

fn part2(stacks: Vec<Vec<char>>, procedures: Vec<(usize, usize, usize)>) -> String {
    let mut stacks = stacks;
    for (quantity, from, to) in procedures {
        let len = stacks[from - 1].len();
        let items = stacks[from - 1]
            .splice(len - quantity.., vec![])
            .collect::<Vec<_>>();
        stacks[to - 1].extend(items);
    }

    stacks
        .into_iter()
        .map(|stack| *stack.last().unwrap())
        .collect::<String>()
}

pub fn main() {
    let data = fs::read_to_string("data/2022/day5").unwrap();

    let (stacks, procedures) = parse(&data);
    println!("part 1: {}", part1(stacks.clone(), procedures.clone()));
    println!("part 2: {}", part2(stacks, procedures));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn case1() {
        let (stacks, procedure) = parse(TEST_DATA);
        assert_eq!(stacks, vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']]);
        assert_eq!(procedure, vec![(1, 2, 1), (3, 1, 3), (2, 2, 1), (1, 1, 2)]);
        assert_eq!(part1(stacks.clone(), procedure.clone()), "CMZ".to_owned());
        assert_eq!(part2(stacks, procedure), "MCD");
    }
}

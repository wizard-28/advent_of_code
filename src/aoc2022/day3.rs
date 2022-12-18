use std::fs;

fn parse(rucksacks: &str) -> Vec<(String, String)> {
    rucksacks
        .trim()
        .lines()
        .map(|rucksack| {
            let split = rucksack.split_at(rucksack.len() / 2);
            (split.0.to_owned(), split.1.to_owned())
        })
        .collect()
}

fn part1(rucksacks: &[(String, String)]) -> u32 {
    rucksacks
        .iter()
        .map(|(first_compartment, second_compartment)| {
            let common = first_compartment
                .chars()
                .find(|item| second_compartment.contains(*item))
                .unwrap();

            match common {
                'a'..='z' => common as u32 - 96,
                'A'..='Z' => common as u32 - 38,
                _ => unreachable!("Invalid data"),
            }
        })
        .sum()
}

fn part2(rucksacks: &[(String, String)]) -> u32 {
    rucksacks
        .chunks(3)
        .map(|chunk| {
            let rucksacks: Vec<String> = chunk.iter().map(|(x, y)| format!("{x}{y}")).collect();

            let common = rucksacks[0]
                .chars()
                .find(|item| rucksacks[1].contains(*item) && rucksacks[2].contains(*item))
                .unwrap();

            match common {
                'a'..='z' => common as u32 - 96,
                'A'..='Z' => common as u32 - 38,
                _ => unreachable!("Invalid data"),
            }
        })
        .sum()
}

pub fn main() {
    let data = fs::read_to_string("data/2022/day3").unwrap();
    let rucksacks = parse(&data);
    println!("part 1: {}", part1(&rucksacks));
    println!("part 2: {}", part2(&rucksacks));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test() {
        let rucksacks = parse(TEST_DATA);
        assert_eq!(part1(&rucksacks), 157);
        assert_eq!(part2(&rucksacks), 70);
    }
}

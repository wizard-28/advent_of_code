use std::fs;

fn parse(data: &str) -> Vec<u32> {
    data.split("\n\n")
        .map(|items| {
            items
                .trim()
                .lines()
                .map(|line| line.parse::<u32>().unwrap())
                .sum()
        })
        .collect()
}

fn part1(calories: &[u32]) -> u32 {
    *calories.last().unwrap()
}

fn part2(calories: &[u32]) -> u32 {
    calories.iter().rev().take(3).sum()
}

pub fn main() {
    let data = fs::read_to_string("data/2022/day1").unwrap();
    let mut calories = parse(&data);
    calories.sort_unstable();
    println!("part1: {}", part1(&calories));
    println!("part2: {}", part2(&calories));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn test() {
        let mut calories = parse(TEST_DATA);

        assert_eq!(calories, vec![6000, 4000, 11000, 24000, 10000]);

        calories.sort_unstable();
        assert_eq!(24000, part1(&calories));
        assert_eq!(45000, part2(&calories));
    }
}

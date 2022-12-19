use std::fs;
use std::ops::RangeInclusive;

fn parse(data: &str) -> Vec<(RangeInclusive<u32>, RangeInclusive<u32>)> {
    data.trim()
        .lines()
        .map(|line| {
            let numbers: Vec<u32> = line
                .split(',')
                .flat_map(|range| range.split('-').map(|f| f.parse().unwrap()))
                .collect();

            (numbers[0]..=numbers[1], numbers[2]..=numbers[3])
        })
        .collect()
}

fn part1(assignment_pairs: &[(RangeInclusive<u32>, RangeInclusive<u32>)]) -> usize {
    assignment_pairs
        .iter()
        .filter(|(range1, range2)| {
            // More efficient than `.all` approach
            (range1.start() >= range2.start() && range1.end() <= range2.end())
                || (range2.start() >= range1.start() && range2.end() <= range1.end())
        })
        .count()
}

fn part2(assignment_pairs: &[(RangeInclusive<u32>, RangeInclusive<u32>)]) -> usize {
    assignment_pairs
        .iter()
        .filter(|(range1, range2)| {
            range1.clone().any(|f| range2.contains(&f))
                || range2.clone().any(|f| range1.contains(&f))
        })
        .count()
}

pub fn main() {
    let data = fs::read_to_string("data/2022/day4").unwrap();
    let assignment_pairs = parse(&data);
    println!("part 1: {}", part1(&assignment_pairs));
    println!("part 2: {}", part2(&assignment_pairs));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn test() {
        let assignment_pairs = parse(TEST_DATA);
        assert_eq!(
            assignment_pairs,
            vec![
                (2..=4, 6..=8),
                (2..=3, 4..=5),
                (5..=7, 7..=9),
                (2..=8, 3..=7),
                (6..=6, 4..=6),
                (2..=6, 4..=8)
            ]
        );

        assert_eq!(part1(&assignment_pairs), 2);
        assert_eq!(part2(&assignment_pairs), 4);
    }
}

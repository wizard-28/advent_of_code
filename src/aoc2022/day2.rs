use std::fs;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Choice {
    Rock,
    Paper,
    Scissor,
}

#[derive(Debug, PartialEq, Eq)]
enum MyOption {
    X,
    Y,
    Z,
}

#[allow(clippy::match_on_vec_items)]
fn parse(strategy_guide: &str) -> Vec<(Choice, MyOption)> {
    strategy_guide
        .trim()
        .lines()
        .map(|line| {
            let round = line.split_whitespace().collect::<Vec<&str>>();

            let enemy_choice = match round[0] {
                "A" => Choice::Rock,
                "B" => Choice::Paper,
                "C" => Choice::Scissor,
                _ => unreachable!("Invalid data"),
            };

            let my_option = match round[1] {
                "X" => MyOption::X,
                "Y" => MyOption::Y,
                "Z" => MyOption::Z,
                _ => unreachable!("Invalid data"),
            };

            (enemy_choice, my_option)
        })
        .collect()
}

fn part1(strategy_guide: &[(Choice, MyOption)]) -> u32 {
    let mut score = 0;
    for round in strategy_guide {
        // Decode my option
        let my_choice = match round.1 {
            MyOption::X => Choice::Rock,
            MyOption::Y => Choice::Paper,
            MyOption::Z => Choice::Scissor,
        };

        score += match my_choice {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissor => 3,
        };

        match (round.0, my_choice) {
            // Winning conditions
            (Choice::Rock, Choice::Paper)
            | (Choice::Paper, Choice::Scissor)
            | (Choice::Scissor, Choice::Rock) => score += 6,
            // Losing conditions
            (Choice::Rock, Choice::Scissor)
            | (Choice::Paper, Choice::Rock)
            | (Choice::Scissor, Choice::Paper) => score += 0,
            // Draw conditions
            _ => score += 3,
        }
    }

    score
}

fn part2(strategy_guide: &[(Choice, MyOption)]) -> u32 {
    let mut score = 0;
    for round in strategy_guide {
        // Decode my option
        let my_choice = match round.1 {
            // I need to lose
            MyOption::X => match round.0 {
                Choice::Rock => Choice::Scissor,
                Choice::Paper => Choice::Rock,
                Choice::Scissor => Choice::Paper,
            },
            // I need to draw
            MyOption::Y => round.0,
            // I need to win
            MyOption::Z => match round.0 {
                Choice::Rock => Choice::Paper,
                Choice::Paper => Choice::Scissor,
                Choice::Scissor => Choice::Rock,
            },
        };

        score += match my_choice {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissor => 3,
        };

        score += match round.1 {
            MyOption::X => 0,
            MyOption::Y => 3,
            MyOption::Z => 6,
        };
    }

    score
}

pub fn main() {
    let data = fs::read_to_string("data/2022/day2").unwrap();
    let strategy_guide = parse(&data);
    println!("part 1: {}", part1(&strategy_guide));
    println!("part 2: {}", part2(&strategy_guide));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "
A Y
B X
C Z
";

    #[test]
    fn test() {
        let strategy_guide = parse(TEST_DATA);
        assert_eq!(
            strategy_guide,
            vec![
                (Choice::Rock, MyOption::Y),
                (Choice::Paper, MyOption::X),
                (Choice::Scissor, MyOption::Z)
            ]
        );

        assert_eq!(part1(&strategy_guide), 15);
        assert_eq!(part2(&strategy_guide), 12);
    }
}

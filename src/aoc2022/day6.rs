use std::fs;

fn parser(data: &str, distinct_chars: usize) -> usize {
    let chars: Vec<char> = data.chars().collect();
    chars
        .windows(distinct_chars)
        .position(|window| (1..window.len()).all(|i| !window[i..].contains(&window[i - 1])))
        .map(|position| position + distinct_chars)
        .unwrap()
}

pub fn main() {
    let data = fs::read_to_string("data/2022/day6").unwrap();
    println!("part 1: {}", parser(&data, 4));
    println!("part 2: {}", parser(&data, 14));
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7)]
    #[case("bvwbjplbgvbhsrlpgdmjqwftvncz", 5)]
    #[case("nppdvjthqldpwncqszvftbrmjlhg", 6)]
    #[case("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10)]
    #[case("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11)]
    fn test_start_of_packet_marker(#[case] data: &str, #[case] output: usize) {
        assert_eq!(parser(data, 4), output);
    }

    #[rstest]
    #[case("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19)]
    #[case("bvwbjplbgvbhsrlpgdmjqwftvncz", 23)]
    #[case("nppdvjthqldpwncqszvftbrmjlhg", 23)]
    #[case("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29)]
    #[case("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26)]
    fn test_start_of_message_marker(#[case] data: &str, #[case] output: usize) {
        assert_eq!(parser(data, 14), output);
    }
}

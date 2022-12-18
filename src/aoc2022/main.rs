use advent_of_code::days;

days!(1, 2, 3);

fn main() {
    std::env::args()
        .nth(1)
        .map_or_else(run_all, |day| run_day(&day));
}

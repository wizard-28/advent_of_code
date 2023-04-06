use advent_of_code::days;

days!(1, 2, 3, 4, 5, 6, 7);

fn main() {
    std::env::args()
        .nth(1)
        .map_or_else(run_all, |day| run_day(&day));
}

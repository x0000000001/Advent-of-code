use aoc::DAYS;

#[test]
fn part1_test_input0() {
    DAYS.get("2016_14")
        .unwrap()
        .test_input(1, 0, aoc::Solution::from(22728))
}

#[test]
fn part2_test_input0() {
    DAYS.get("2016_14")
        .unwrap()
        .test_input(2, 0, aoc::Solution::from(22551))
}

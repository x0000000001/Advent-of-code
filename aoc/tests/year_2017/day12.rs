use aoc::DAYS;

#[test]
fn part1_test_input0() {
    DAYS.get("2017_12")
        .unwrap()
        .test_input(1, 0, aoc::Solution::from(6))
}

#[test]
fn part2_test_input0() {
    DAYS.get("2017_12")
        .unwrap()
        .test_input(2, 0, aoc::Solution::from(2))
}

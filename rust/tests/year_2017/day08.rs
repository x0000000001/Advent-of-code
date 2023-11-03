use aoc::DAYS;

#[test]
fn part1_test_input0() {
    DAYS.get("2017_08")
        .unwrap()
        .test_input(1, 0, aoc::Solution::from(1))
}

#[test]
fn part2_test_input0() {
    DAYS.get("2017_08")
        .unwrap()
        .test_input(2, 0, aoc::Solution::from(10))
}

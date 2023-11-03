use aoc::DAYS;

#[test]
fn part1_test_input0() {
    DAYS.get("2016_12")
        .unwrap()
        .test_input(1, 0, aoc::Solution::from(42))
}

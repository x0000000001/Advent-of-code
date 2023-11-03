use aoc::DAYS;

#[test]
fn part1_test_input0() {
    DAYS.get("2016_11")
        .unwrap()
        .test_input(1, 0, aoc::Solution::from(11))
}

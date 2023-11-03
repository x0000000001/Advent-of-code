use aoc::DAYS;

#[test]
fn part1_test_input0() {
    DAYS.get("2015_13")
        .unwrap()
        .test_input(1, 0, aoc::Solution::from(330))
}

use aoc::DAYS;

#[test]
fn part2_test_input0() {
    DAYS.get("2017_07")
        .unwrap()
        .test_input(2, 0, aoc::Solution::from(60))
}

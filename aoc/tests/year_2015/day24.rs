use aoc::DAYS;

#[test]
fn part1_test_input0() {
    DAYS.get("2015_24")
        .unwrap()
        .test_input(1, 0, aoc::Solution::from(99))
}

#[test]
fn part2_test_input0() {
    DAYS.get("2015_24")
        .unwrap()
        .test_input(2, 0, aoc::Solution::from(44))
}

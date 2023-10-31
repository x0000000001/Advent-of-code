use aoc::DAYS;
#[test]
fn test_input0() {
    DAYS.get("2015_01")
        .unwrap()
        .test_input(1, 0, aoc::Solution::from(3))
}

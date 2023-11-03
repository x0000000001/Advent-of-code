use aoc::year_2015::day17;

#[test]
fn test_count_ways() {
    assert_eq!(day17::count_ways(25, &Vec::from([20, 15, 10, 5, 5])), 4);
}

#[test]
fn test_count_min_ways() {
    assert_eq!(
        day17::count_min_ways(25, &Vec::from([20, 15, 10, 5, 5])),
        (2, 3)
    );
}

use aoc::DAYS;
use test::Bencher;

#[bench]
fn part1(b: &mut Bencher) {
    b.iter(|| DAYS.get("2015_01").unwrap().solve_for_benchmark(1))
}

#[bench]
fn part2(b: &mut Bencher) {
    b.iter(|| DAYS.get("2015_01").unwrap().solve_for_benchmark(1))
}

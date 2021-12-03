use aoc2021::{self, read_data};

#[test]
fn test_day_1() {
    let data = read_data::<u32, _>("data/day01.txt").unwrap();
    let task_1 = aoc2021::day_01::day_1(&data, 1);
    assert_eq!(task_1, 1462);
    let task_2 = aoc2021::day_01::day_1(&data, 3);
    assert_eq!(task_2, 1497);
}

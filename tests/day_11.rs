use aoc2021::{self, read_data};

#[test]
fn test_day_11() {
    let data = read_data::<String, _>("data/day11.txt").unwrap();
    let task_1 = aoc2021::day_11::day_11_1(&data, 100);
    assert_eq!(task_1, 1719);
    let task_2 = aoc2021::day_11::day_11_2(&data);
    assert_eq!(task_2, 232);
}

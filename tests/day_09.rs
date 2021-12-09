use aoc2021::{self, read_data};

#[test]
fn test_day_9() {
    let data = read_data::<String, _>("data/day09.txt").unwrap();
    let task_1 = aoc2021::day_09::day_9_1(&data);
    assert_eq!(task_1, 558);
}

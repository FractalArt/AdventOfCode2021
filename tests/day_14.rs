use aoc2021::{self, read_data};

#[test]
fn test_day_14() {
    let data = read_data::<String, _>("data/day14.txt").unwrap();
    let task_1 = aoc2021::day_14::day_14(&data, 10);
    assert_eq!(task_1, 5656);
    let task_2 = aoc2021::day_14::day_14(&data, 40);
    assert_eq!(task_2, 12271437788530);
}

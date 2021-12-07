use aoc2021::{self, read_data};

#[test]
fn test_day_7() {
    let data = read_data::<String, _>("data/day07.txt").unwrap();
    let task_1 = aoc2021::day_07::day_7_1(&data);
    assert_eq!(task_1, 339321);
    let task_2 = aoc2021::day_07::day_7_2(&data);
    assert_eq!(task_2, 95476244);
}

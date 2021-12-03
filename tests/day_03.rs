use aoc2021::{self, read_data};

#[test]
fn test_day_2() {
    let data = read_data::<String, _>("data/day03.txt").unwrap();
    let task_1 = aoc2021::day_03::day_3_1(&data);
    assert_eq!(task_1, 2724524);
    let task_2 = aoc2021::day_03::day_3_2(&data);
    assert_eq!(task_2, 2775870);
}

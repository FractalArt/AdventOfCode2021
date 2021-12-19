use aoc2021::{self, read_data};

#[test]
fn test_day_16() {
    let data = read_data::<String, _>("data/day16.txt").unwrap();
    let task_1 = aoc2021::day_16::day_16_1(&data);
    assert_eq!(task_1, 891);
    let task_2 = aoc2021::day_16::day_16_2(&data);
    assert_eq!(task_2, 673042777597);
}

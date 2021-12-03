use aoc2021::{self, read_data};

#[test]
fn test_day_2() {
    let data = read_data::<String, _>("data/day02.txt").unwrap();
    let task_1 = aoc2021::day_02::day_2_1(&data);
    assert_eq!(task_1, 1451208);
    let task_2 = aoc2021::day_02::day_2_2(&data);
    assert_eq!(task_2, 1620141160);
}

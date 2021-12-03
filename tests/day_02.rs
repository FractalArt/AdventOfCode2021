use aoc2021::{self, read_data};

#[test]
fn test_day_2() {
    let data = read_data::<String, _>("data/day02.txt").unwrap();
    let task_1 = aoc2021::day_02::day_2(&data);
    assert_eq!(task_1, 1451208);
    // let task_2 = aoc2021::day_01::day_1(&data, 3);
    // assert_eq!(task_2, 1497);
}

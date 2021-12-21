#[test]
fn test_day_17() {
    let task_1 = aoc2021::day_17::day_17_1(195, 238, -93, -67);
    assert_eq!(task_1, 4278);
    let task_2 = aoc2021::day_17::day_17_2(195, 238, -93, -67);
    assert_eq!(task_2, 1994);
}

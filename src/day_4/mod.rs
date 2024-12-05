use grid::Grid;

#[cfg(test)]
const TEST_INPUT: &str = include_str!("./test_input");
const INPUT: &str = include_str!("./input");

mod grid;

pub fn part_1_answer() -> String {
    part_1(INPUT).to_string()
} 

pub fn part_2_answer() -> String {
    part_2(INPUT).to_string()
} 

fn part_1(input: &str) -> usize {
    let grid = Grid::new(input);
    grid.count_all_xmas()
}

fn part_2(input: &str) -> usize {
    let grid = Grid::new(input);
    grid.count_all_cross()
}

#[test]
fn part_1_test() {
    assert_eq!(part_1(TEST_INPUT), 18);
}

#[test]
fn part_2_test() {
    assert_eq!(part_2(TEST_INPUT), 9);
}


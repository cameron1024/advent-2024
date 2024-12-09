use grid::Grid;

#[cfg(test)]
const TEST_INPUT: &str = include_str!("./test_input");
const INPUT: &str = include_str!("./input");

mod grid;
mod point;

pub fn part_1_answer() -> String {
    part_1(INPUT).to_string()
}

pub fn part_2_answer() -> String {
    part_2(INPUT).to_string()
}

fn part_1(input: &str) -> usize {
    let mut grid = Grid::new(input);
    grid.count_nodes_part_1()
}

fn part_2(input: &str) -> usize {
    let mut grid = Grid::new(input);
    grid.count_nodes_part_2()
}

#[test]
fn part_1_test() {
    assert_eq!(part_1(TEST_INPUT), 14);
}

#[test]
fn part_2_test() {
    assert_eq!(part_2(TEST_INPUT), 34);
}

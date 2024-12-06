use grid::Grid;

mod grid;
mod guard;

#[cfg(test)]
const TEST_INPUT: &str = include_str!("./test_input");
const INPUT: &str = include_str!("./input");

pub fn part_1_answer() -> String {
    part_1(INPUT).to_string()
}

pub fn part_2_answer() -> String {
    part_2(INPUT).to_string()
}

fn part_1(input: &str) -> usize {
    let mut grid = Grid::new(input);
    grid.exit();
    grid.count_visited()
}

fn part_2(input: &str) -> usize {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();

    let mut result = 0;

    for x in 0..width {
        for y in 0..height {
            let mut grid = Grid::new(input);
            *grid.temporary_obstacle_mut() = Some((x, y));
            if !grid.exit() {
                result += 1;
            }
        }
    }

    result
}

#[test]
fn part_1_test() {
    assert_eq!(part_1(TEST_INPUT), 41);
}

#[test]
fn part_2_test() {
    assert_eq!(part_2(TEST_INPUT), 6);
}

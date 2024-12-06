use bitvec::vec::BitVec;

use super::guard::{Direction, GuardPos};

pub struct Grid {
    obstacles: BitVec,
    visited: BitVec,
    width: usize,
    height: usize,
    guard_pos: GuardPos,
    temporary_obstacle: Option<(usize, usize)>,
}

impl Grid {
    pub fn new(input: &str) -> Self {
        let width = input.lines().next().unwrap().len();
        let height = input.lines().count();
        let obstacles = std::iter::repeat(false).take(width * height).collect();
        let visited = std::iter::repeat(false).take(width * height * 4).collect();

        let mut grid = Self {
            obstacles,
            visited,
            width,
            height,
            // temporarily invalidated, fixed in the next for loop
            guard_pos: GuardPos::default(),
            temporary_obstacle: None,
        };

        for (y, line) in input.lines().enumerate() {
            for (x, char) in line.chars().enumerate() {
                match char {
                    '#' => grid.obstacles.set(x + y * height, true),
                    '^' => {
                        // no need to update direction because up is the default
                        grid.guard_pos.x = x;
                        grid.guard_pos.y = y;
                        grid.mark_visited(x, y, Direction::Up);
                    }
                    _ => {}
                }
            }
        }

        grid
    }

    fn obstacle_at(&self, x: usize, y: usize) -> bool {
        self.obstacles[x + y * self.height]
            || self
                .temporary_obstacle
                .map(|(t_x, t_y)| t_x == x && t_y == y)
                .unwrap_or(false)
    }

    fn mark_visited(&mut self, x: usize, y: usize, direction: Direction) {
        // stored visited state in 4-bit chunks - each bit represents a direction
        let index = (x + y * self.height) * 4;
        let index = index + (direction as u8) as usize;
        self.visited.set(index, true);
    }

    fn has_visited_same_direction(&self, x: usize, y: usize, direction: Direction) -> bool {
        let index = (x + y * self.height) * 4;
        let index = index + (direction as u8) as usize;
        self.visited[index]
    }

    /// Returns true exited, false if stuck in loop
    pub fn exit(&mut self) -> bool {
        while let Some(new_pos) = self.guard_pos.move_forward(self.width, self.height) {
            if self.obstacle_at(new_pos.x, new_pos.y) {
                self.guard_pos.direction = self.guard_pos.direction.rotate_right();
            } else {
                let GuardPos { x, y, direction } = new_pos;
                if self.has_visited_same_direction(x, y, direction) {
                    // we've been in this position, facing the same direction - loop guaranteed
                    return false;
                }
                self.guard_pos = new_pos;
                self.mark_visited(x, y, direction);
            }
        }

        true
    }

    pub fn count_visited(&self) -> usize {
        self.visited.count_ones()
    }

    pub fn temporary_obstacle_mut(&mut self) -> &mut Option<(usize, usize)> {
        &mut self.temporary_obstacle
    }

}

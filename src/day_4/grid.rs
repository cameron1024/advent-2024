#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Letter {
    X,
    M,
    A,
    S,
}

#[derive(Debug)]
pub struct Grid<'a> {
    data: &'a str,
    width: usize,
    height: usize,
}

impl<'a> Grid<'a> {
    pub fn new(data: &'a str) -> Self {
        let width = data.lines().next().unwrap().len();
        let height = data.lines().count();

        Self {
            data,
            width,
            height,
        }
    }

    fn get(&self, x: usize, y: usize) -> Letter {
        assert!(x < self.width);
        assert!(y < self.height);

        // need to use width + 1 to account for \n that is stripped by .lines()
        let index = ((self.width + 1) * y) + x;

        match self.data.as_bytes()[index] {
            b'X' => Letter::X,
            b'M' => Letter::M,
            b'A' => Letter::A,
            b'S' => Letter::S,
            _ => unreachable!(),
        }
    }

    pub fn count_all_xmas(&self) -> usize {
        let mut result = 0;
        for x in 0..self.width {
            for y in 0..self.height {
                result += self.count_xmas_starting_at(x, y);
            }
        }
        result
    }

    pub fn count_all_cross(&self) -> usize {
        let mut result = 0;

        for x in 1..(self.width - 1) {
            for y in 1..(self.height - 1) {
                result += self.check_cross(x, y) as usize;
            }
        }

        result
    }

    fn count_xmas_starting_at(&self, x: usize, y: usize) -> usize {
        if self.get(x, y) != Letter::X {
            return 0;
        }

        let mut result = 0;

        let can_go_left = x >= 3;
        let can_go_right = x < self.width - 3;

        let can_go_up = y >= 3;
        let can_go_down = y < self.height - 3;

        if can_go_right {
            result += self.check([(x, y), (x + 1, y), (x + 2, y), (x + 3, y)]) as usize;
        }

        if can_go_left {
            result += self.check([(x, y), (x - 1, y), (x - 2, y), (x - 3, y)]) as usize;
        }

        if can_go_down {
            result += self.check([(x, y), (x, y + 1), (x, y + 2), (x, y + 3)]) as usize;
        }

        if can_go_up {
            result += self.check([(x, y), (x, y - 1), (x, y - 2), (x, y - 3)]) as usize;
        }

        if can_go_right && can_go_down {
            result += self.check([(x, y), (x + 1, y + 1), (x + 2, y + 2), (x + 3, y + 3)]) as usize;
        }

        if can_go_right && can_go_up {
            result += self.check([(x, y), (x + 1, y - 1), (x + 2, y - 2), (x + 3, y - 3)]) as usize;
        }

        if can_go_left && can_go_down {
            result += self.check([(x, y), (x - 1, y + 1), (x - 2, y + 2), (x - 3, y + 3)]) as usize;
        }

        if can_go_left && can_go_up {
            result += self.check([(x, y), (x - 1, y - 1), (x - 2, y - 2), (x - 3, y - 3)]) as usize;
        }

        result
    }

    fn check(&self, coords: [(usize, usize); 4]) -> bool {
        self.get(coords[0].0, coords[0].1) == Letter::X
            && self.get(coords[1].0, coords[1].1) == Letter::M
            && self.get(coords[2].0, coords[2].1) == Letter::A
            && self.get(coords[3].0, coords[3].1) == Letter::S
    }

    fn check_cross(&self, x: usize, y: usize) -> bool {
        self.check_3([(x - 1, y - 1), (x, y), (x + 1, y + 1)])
            && self.check_3([(x + 1, y - 1), (x, y), (x - 1, y + 1)])
    }

    fn check_3(&self, coords: [(usize, usize); 3]) -> bool {
        (self.get(coords[0].0, coords[0].1) == Letter::M
            && self.get(coords[1].0, coords[1].1) == Letter::A
            && self.get(coords[2].0, coords[2].1) == Letter::S)
            || (self.get(coords[0].0, coords[0].1) == Letter::S
                && self.get(coords[1].0, coords[1].1) == Letter::A
                && self.get(coords[2].0, coords[2].1) == Letter::M)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grid_get_converts_letters_correctly() {
        let grid = Grid::new("XMAS\nXMAS");

        assert_eq!(grid.get(0, 0), Letter::X);
        assert_eq!(grid.get(1, 0), Letter::M);
        assert_eq!(grid.get(2, 0), Letter::A);
        assert_eq!(grid.get(3, 0), Letter::S);
        assert_eq!(grid.get(0, 1), Letter::X);
        assert_eq!(grid.get(1, 1), Letter::M);
        assert_eq!(grid.get(2, 1), Letter::A);
        assert_eq!(grid.get(3, 1), Letter::S);
    }
}

use hashbrown::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn distance_from(self, other: Point) -> Distance {
        Distance {
            dx: self.x.abs_diff(other.x),
            dy: self.y.abs_diff(other.y),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Distance {
    pub dx: usize,
    pub dy: usize,
}

impl Distance {
    pub fn is_half_of(&self, other: Distance) -> bool {
        let doubled = Self {
            dx: self.dx * 2,
            dy: self.dy * 2,
        };

        doubled == other
    }
}

impl PartialEq for Distance {
    fn eq(&self, other: &Self) -> bool {
        (self.dx == other.dx && self.dy == other.dy) || (self.dy == other.dx && self.dx == other.dy)
    }
}

pub fn line_from(
    p1: Point,
    p2: Point,
    width: usize,
    height: usize,
    multiple_only: bool,
) -> impl Iterator<Item = Point> {
    let height = height as isize;
    let width = width as isize;

    let mut dx = (p2.x as isize) - (p1.x as isize);
    let mut dy = (p2.y as isize) - (p1.y as isize);

    // reduce both dx and dy as much as possible
    if !multiple_only {
        let max = std::cmp::min(dx, dy);
        for k in (2..).take_while(|i| i * i < max) {
            while dx % k == 0 && dy % k == 0 {
                dx /= k;
                dy /= k;
            }
        }
    }

    let in_bounds = move |x: isize, y: isize| x >= 0 && x < width && y >= 0 && y < height;

    let lowers = std::iter::successors(Some((p1.x as isize, p1.y as isize)), move |(x, y)| {
        Some((x - dx, y - dy))
    })
    .take_while(move |(x, y)| in_bounds(*x, *y));

    let highers = std::iter::successors(Some((p1.x as isize, p1.y as isize)), move |(x, y)| {
        Some((x + dx, y + dy))
    })
    .take_while(move |(x, y)| in_bounds(*x, *y));

    lowers
        .chain(highers)
        .map(|(x, y)| Point {
            x: x as usize,
            y: y as usize,
        })
        .filter(move |p| *p != p1 && *p != p2)
}

#[test]
fn distance_eq_works() {
    let d1 = Distance { dx: 2, dy: 3 };
    let d2 = Distance { dx: 3, dy: 2 };
    let d3 = Distance { dx: 3, dy: 4 };

    assert_eq!(d1, d1);
    assert_eq!(d1, d2);
    assert_ne!(d1, d3);
}

#[test]
fn line_iter_works() {
    let p1 = Point { x: 4, y: 4 };
    let p2 = Point { x: 5, y: 5 };
    let points: HashSet<_> = line_from(p1, p2, 10, 10, true).collect();
    let expected: HashSet<_> = (0..10)
        .filter(|i| *i != 4 && *i != 5)
        .map(|i| Point { x: i, y: i })
        .collect();

    assert_eq!(points, expected);
}

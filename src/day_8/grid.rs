use std::iter::repeat;

use bitvec::vec::BitVec;
use hashbrown::HashMap;

use super::point::{line_from, Point};

pub struct Grid {
    width: usize,
    height: usize,
    antennas: Vec<Vec<Point>>,
    found_nodes: BitVec,
}

impl Grid {
    pub fn new(input: &str) -> Self {
        let width = input.lines().next().unwrap().len();
        let height = input.lines().count();

        let mut antennas = HashMap::<char, Vec<Point>>::new();

        for (x, line) in input.lines().enumerate() {
            for (y, char) in line.chars().enumerate() {
                if char.is_ascii_alphanumeric() {
                    antennas.entry(char).or_default().push(Point { x, y });
                }
            }
        }

        let antennas = antennas.into_iter().map(|(_char, vec)| vec).collect();

        Self {
            width,
            height,
            antennas,
            found_nodes: repeat(false).take(width * height).collect(),
        }
    }

    pub fn count_nodes_part_1(&mut self) -> usize {
        self.populate_nodes(true);
        self.found_nodes.count_ones()
    }

    pub fn count_nodes_part_2(&mut self) -> usize {
        self.populate_nodes(false);
        self.found_nodes.count_ones()
    }

    fn populate_nodes(&mut self, multiple_only: bool) {
        while let Some(vec) = self.antennas.pop() {
            for i in 0..vec.len() {
                for j in i..vec.len() {
                    let p1 = vec[i];
                    let p2 = vec[j];

                    if p1 == p2 {
                        continue;
                    }

                    for node in line_from(p1, p2, self.width, self.height, multiple_only) {
                        let d1 = node.distance_from(p1);
                        let d2 = node.distance_from(p2);

                        if !multiple_only || d1.is_half_of(d2) || d2.is_half_of(d1) {
                            self.mark_location(node.x, node.y);
                        }
                    }
                }
            }
        }
    }

    fn mark_location(&mut self, x: usize, y: usize) {
        self.found_nodes.set(x + (y * self.width), true);
    }
}

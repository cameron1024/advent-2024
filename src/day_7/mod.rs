#[cfg(test)]
const TEST_INPUT: &str = include_str!("./test_input");
const INPUT: &str = include_str!("./input");

pub fn part_1_answer() -> String {
    part_1(INPUT).to_string()
}

pub fn part_2_answer() -> String {
    part_2(INPUT).to_string()
}

fn part_1(input: &str) -> i64 {
    input
        .lines()
        .map(Equation::from_line)
        .filter(|e| e.possible(false))
        .map(|e| e.target)
        .sum()
}

fn part_2(input: &str) -> i64 {
    input
        .lines()
        .map(Equation::from_line)
        .filter(|e| e.possible(true))
        .map(|e| e.target)
        .sum()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Op {
    Add,
    Mul,
    Concat,
}

impl Op {
    fn next(&self, include_concat: bool) -> Option<Op> {
        match include_concat {
            true => self.next_with_concat(),
            false => self.next_without_concat(),
        }
    }
    fn next_with_concat(&self) -> Option<Op> {
        match self {
            Self::Add => Some(Self::Mul),
            Self::Mul => Some(Self::Concat),
            Self::Concat => None,
        }
    }
    fn next_without_concat(&self) -> Option<Op> {
        match self {
            Self::Add => Some(Self::Mul),
            Self::Mul => None,
            _ => panic!(),
        }
    }
}

struct Equation {
    target: i64,
    data: Vec<i64>,
}

impl Equation {
    fn from_line(line: &str) -> Self {
        let (target, rest) = line.split_once(":").unwrap();
        let target = target.parse().unwrap();

        let data = rest
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        Self { target, data }
    }

    fn possible(&self, include_concat: bool) -> bool {
        // if there are n numbers, there are n - 1 ops between them
        let mut ops = vec![Op::Add; self.data.len() - 1];
        loop {
            if compute(&self.data, &ops) == self.target {
                return true;
            }
            if !permute(&mut ops, include_concat) {
                break;
            }
        }

        false
    }
}

fn compute(nums: &[i64], ops: &[Op]) -> i64 {
    let mut result = nums[0];

    for (index, num) in nums[1..].iter().enumerate() {
        match ops[index] {
            Op::Add => result += num,
            Op::Mul => result *= num,
            Op::Concat => result = fast_concat(result, *num),
        }
    }

    result
}

fn fast_concat(left: i64, right: i64) -> i64 {
    // how much we need to multiply by to concat
    // e.g. if left = 12, right = 23, left_factor would be 100
    // so that left * 100 + right = 1234
    let mut left_factor = 1;
    let mut temp = right;

    while temp != 0 {
        temp /= 10;
        left_factor *= 10;
    }

    (left * left_factor) + right
}

/// Returns `true` if the value was changed, `false` if no more permutations could be found
fn permute(ops: &mut [Op], include_concat: bool) -> bool {
    let len = ops.len();
    let mut index = len - 1;

    loop {
        let op = &mut ops[index];
        if let Some(new) = op.next(include_concat) {
            *op = new;
            for op in &mut ops[(index + 1)..len] {
                *op = Op::Add;
            }
            return true;
        } else if index == 0 {
            return false;
        } else {
            index -= 1;
        }
    }
}

#[test]
fn permute_works() {
    let mut ops = [Op::Add, Op::Add];

    assert!(permute(&mut ops, false));
    assert_eq!(ops, [Op::Add, Op::Mul]);

    assert!(permute(&mut ops, false));
    assert_eq!(ops, [Op::Mul, Op::Add]);

    assert!(permute(&mut ops, false));
    assert_eq!(ops, [Op::Mul, Op::Mul]);

    assert!(!permute(&mut ops, false));
    assert_eq!(ops, [Op::Mul, Op::Mul]);
}

#[test]
fn concat_works() {
    assert_eq!(fast_concat(12, 23), 1223);
    assert_eq!(fast_concat(1234, 5678), 12345678);
}

#[test]
fn part_1_test() {
    assert_eq!(part_1(TEST_INPUT), 3749);
}

#[test]
fn part_2_test() {
    assert_eq!(part_2(TEST_INPUT), 11387);
}

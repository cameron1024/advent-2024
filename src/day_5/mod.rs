use update_data::UpdateData;

mod update_data;

#[cfg(test)]
const TEST_INPUT: &str = include_str!("./test_input");
const INPUT: &str = include_str!("./input");

pub fn part_1_answer() -> String {
    part_1(INPUT).to_string()
}

pub fn part_2_answer() -> String {
    part_2(INPUT).to_string()
}

#[derive(Debug, Clone, Copy)]
struct Constraint<'a> {
    before: &'a str,
    after: &'a str,
}

impl<'a> Constraint<'a> {
    fn from_line(line: &'a str) -> Self {
        let (before, after) = line.split_once("|").unwrap();
        Self { before, after }
    }
}

#[derive(Debug, Clone)]
struct Update<'a> {
    data: UpdateData<'a>,
}

impl<'a> Update<'a> {
    fn from_line(line: &'a str) -> Self {
        Update {
            data: UpdateData::Unmodified(line),
        }
    }

    fn satisfies_all<'b>(&self, constraints: impl IntoIterator<Item = Constraint<'b>>) -> bool {
        for constraint in constraints {
            if !self.satisfies(constraint) {
                return false;
            }
        }

        true
    }

    fn satisfies(&self, constraint: Constraint<'a>) -> bool {
        let mut has_seen_first = false;
        let mut has_seen_second = false;

        for s in self.data.iter() {
            if s == constraint.before {
                if !has_seen_first && has_seen_second {
                    return false;
                }
                has_seen_first = true;
            }

            if s == constraint.after {
                if has_seen_first {
                    return true;
                } else {
                    has_seen_second = true;
                }
            }
        }

        true
    }

    fn fix_all<'b>(&mut self, constraints: impl IntoIterator<Item = Constraint<'b>> + Clone) {
        while !self.satisfies_all(constraints.clone()) {
            for constraint in constraints.clone() {
                self.fix(constraint);
            }
        }
    }

    fn fix(&mut self, constraint: Constraint) {
        if self.satisfies(constraint) {
            return;
        }

        if let UpdateData::Unmodified(str) = self.data {
            self.data = UpdateData::Fixed(str.split(",").collect());
        }

        let UpdateData::Fixed(vec) = &mut self.data else {
            unreachable!()
        };

        let first_index = vec.iter().position(|s| *s == constraint.before);
        let second_index = vec.iter().position(|s| *s == constraint.after);

        if let (Some(first), Some(second)) = (first_index, second_index) {
            vec.swap(first, second);
        }
    }

    fn middle(&self) -> i64 {
        let len = self.data.iter().count();
        self.data.iter().nth(len / 2).unwrap().parse().unwrap()
    }
}

fn part_1(input: &str) -> i64 {
    let constraints = input
        .lines()
        .take_while(|line| !line.is_empty())
        .map(Constraint::from_line);

    let updates = input
        .lines()
        .skip_while(|line| !line.is_empty())
        .skip(1) // skip the blank line itself
        .map(Update::from_line);

    updates
        .filter(|update| update.satisfies_all(constraints.clone()))
        .map(|update| update.middle())
        .sum()
}

fn part_2(input: &str) -> i64 {
    let constraints = input
        .lines()
        .take_while(|line| !line.is_empty())
        .map(Constraint::from_line);

    let updates = input
        .lines()
        .skip_while(|line| !line.is_empty())
        .skip(1) // skip the blank line itself
        .map(Update::from_line);

    updates
        .filter(|update| !update.satisfies_all(constraints.clone()))
        .map(|mut update| {
            update.fix_all(constraints.clone());
            update
        })
        .map(|update| update.middle())
        .sum()
}

#[test]
fn constraint_test() {
    let update = Update::from_line("1,2,3,4,5");
    assert!(update.satisfies(Constraint::from_line("1|2")));
    assert!(update.satisfies(Constraint::from_line("1|5")));
    assert!(!update.satisfies(Constraint::from_line("5|1")));

    assert!(update.satisfies_all([
        Constraint::from_line("1|2"),
        Constraint::from_line("2|3"),
        Constraint::from_line("1|2"),
    ]));
}

#[test]
fn part_1_test() {
    assert_eq!(part_1(TEST_INPUT), 143);
}

#[test]
fn part_2_test() {
    assert_eq!(part_2(TEST_INPUT), 123);
}

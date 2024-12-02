#[cfg(test)]
const TEST_INPUT: &str = include_str!("./test_input");
const INPUT: &str = include_str!("./input");

pub fn part_1_answer() -> usize {
    part_1(INPUT)
}

pub fn part_2_answer() -> usize {
    part_2(INPUT)
}

fn part_1(input: &str) -> usize {
    input.lines().filter(|s| is_safe_part_1(s)).count()
}

fn is_safe_part_1(report: &str) -> bool {
    is_safe_impl(report.split(" "))
}

fn is_safe_impl<'a>(report: impl Iterator<Item = &'a str>) -> bool {
    let mut ascending = None;

    let mut prev = None::<i32>;

    for num in report {
        let num: i32 = num.parse().unwrap();

        if let Some(prev) = prev {
            if num == prev || (num - prev).abs() > 3 {
                return false;
            }

            let currently_ascending = num > prev;
            if let Some(ascending) = ascending {
                if ascending != currently_ascending {
                    return false;
                }
            } else {
                ascending = Some(currently_ascending);
            }
        }

        prev = Some(num);
    }

    true
}

fn is_safe_part_2(report: &str) -> bool {
    let num_levels = report.split(" ").count();

    for i in 0..num_levels {
        let iter = without_nth(report.split(" "), i);
        if is_safe_impl(iter) {
            return true;
        }
    }

    false
}

fn without_nth<T>(iter: impl Iterator<Item = T>, n: usize) -> impl Iterator<Item = T> {
    iter.enumerate()
        .filter_map(move |(index, t)| if index == n { None } else { Some(t) })
}

fn part_2(input: &str) -> usize {
    input.lines().filter(|s| is_safe_part_2(s)).count()
}

#[test]
fn part_1_test() {
    assert_eq!(part_1(TEST_INPUT), 2);
}

#[test]
fn part_2_test() {
    assert_eq!(part_2(TEST_INPUT), 4);
}

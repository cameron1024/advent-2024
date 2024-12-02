use std::collections::HashMap;

#[cfg(test)]
const TEST_INPUT: &str = include_str!("./test_input");
const INPUT: &str = include_str!("./input");

pub fn part_1_answer() -> i64 {
    part_1(INPUT)
}

pub fn part_2_answer() -> i64 {
    part_2(INPUT)
}

fn part_1(input: &str) -> i64 {
    let lines = input.lines().count();

    let mut lefts: Vec<i64> = Vec::with_capacity(lines);
    let mut rights: Vec<i64> = Vec::with_capacity(lines);

    for line in input.lines() {
        let (left, right) = line.split_once("   ").unwrap();

        lefts.push(left.parse().unwrap());
        rights.push(right.parse().unwrap());
    }

    lefts.sort_unstable();
    rights.sort_unstable();

    let mut total = 0;

    for (left, right) in std::iter::zip(lefts, rights) {
        total += (left - right).abs();
    }

    total
}

fn part_2(input: &str) -> i64 {
    let lines = input.lines().count();

    let mut frequencies = HashMap::with_capacity(lines);

    for line in input.lines() {
        let (_, right) = line.split_once("   ").unwrap();
        *frequencies.entry(right.trim()).or_default() += 1;
    }

    let mut similarity = 0;

    for line in input.lines() {
        let (left, _) = line.split_once("   ").unwrap();
        let left_int: i64 = left.parse().unwrap();
        let frequency = *frequencies.get(left.trim()).unwrap_or(&0);

        similarity += frequency * left_int;
    }

    similarity
}

#[test]
fn part_1_test() {
    assert_eq!(part_1(TEST_INPUT), 11);
}

#[test]
fn part_2_test() {
    assert_eq!(part_2(TEST_INPUT), 31);
}

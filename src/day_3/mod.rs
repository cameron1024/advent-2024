#[cfg(test)]
const TEST_INPUT: &str = include_str!("./test_input");
#[cfg(test)]
const TEST_INPUT_2: &str = include_str!("./test_input_2");
const INPUT: &str = include_str!("./input");

pub fn part_1_answer() -> String {
    part_1(INPUT).to_string()
}

pub fn part_2_answer() -> String {
    part_2(INPUT).to_string()
}

fn part_1(input: &str) -> u64 {
    let mut result = 0;
    for i in 0..input.len() {
        let substring = &input[i..];
        if let Some(ans) = check(substring) {
            result += ans;
        }
    }

    result
}

fn part_2(input: &str) -> u64 {
    let mut result = 0;
    let mut enabled = true;

    for i in 0..input.len() {
        let substring = &input[i..];

        if let Some(new_enabled) = check_do_dont(substring) {
            enabled = new_enabled;
        }

        if enabled {
            if let Some(ans) = check(substring) {
                result += ans;
            }
        }
    }

    result
}

fn check_do_dont(input: &str) -> Option<bool> {
    if input.starts_with("do()") {
        Some(true)
    } else if input.starts_with("don't()") {
        Some(false)
    } else {
        None
    }
}

fn check(mut input: &str) -> Option<u64> {
    if !input.starts_with("mul(") {
        return None;
    }

    input = &input[4..];

    let (first, rest) = extract_number(input)?;
    input = rest;

    if !input.starts_with(",") {
        return None;
    }

    input = &input[1..];

    let (second, rest) = extract_number(input)?;
    input = rest;

    if !input.starts_with(")") {
        return None;
    }

    Some(first * second)
}

fn extract_number(s: &str) -> Option<(u64, &str)> {
    let digits = s.chars().take_while(|c| c.is_numeric());

    let digits_count = digits.clone().count();

    if digits_count == 0 || digits_count > 3 {
        return None;
    };

    let mut result = 0;

    for digit in digits {
        result *= 10;
        result += digit.to_digit(10).unwrap();
    }

    Some((result.into(), &s[digits_count..]))
}

#[test]
fn part_1_test() {
    assert_eq!(part_1(TEST_INPUT), 161);
}

#[test]
fn part_2_test() {
    assert_eq!(part_2(TEST_INPUT_2), 48);
}

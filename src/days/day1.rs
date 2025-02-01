use std::collections::HashMap;

use regex::Regex;

pub fn solve(input: &str, second_part: bool) -> String {
    let re = Regex::new(r"(?<l>\d+)\s+(?<r>\d+)").expect("Failed to compile regex");
    let (mut left, mut right): (Vec<u32>, Vec<u32>) = input
        .lines()
        .map(|line| {
            re.captures(line)
                .map(|caps| {
                    let l = caps["l"].parse::<u32>().unwrap();
                    let r = caps["r"].parse::<u32>().unwrap();
                    (l, r)
                })
                .expect("Failed to capture")
        })
        .unzip();

    if !second_part {
        part_one(&mut left, &mut right)
    } else {
        part_two(&left, &right)
    }
}

fn part_one(left: &mut Vec<u32>, right: &mut Vec<u32>) -> String {
    left.sort_unstable();
    right.sort_unstable();

    left.iter()
        .zip(right)
        .map(|(l, r)| l.abs_diff(*r))
        .sum::<u32>()
        .to_string()
}

fn part_two(left: &Vec<u32>, right: &Vec<u32>) -> String {
    let mut freq: HashMap<u32, u32> = HashMap::new();
    for &val in right {
        *freq.entry(val).or_insert(0) += 1;
    }

    left.iter()
        .map(|e| e * freq.get(e).copied().unwrap_or(0))
        .sum::<u32>()
        .to_string()
}

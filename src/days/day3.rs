use std::ops::Range;

use regex::Regex;

pub fn solve(input: &str, second_part: bool) -> String {
    if !second_part {
        return solve_for_input(input).to_string();
    }

    let neg = Regex::new(r"don't\(\)").expect("Failed to compile regex");
    let pos = Regex::new(r"do\(\)").expect("Failed to compile regex");
    let stops: Vec<usize> = neg.find_iter(input).map(|c| c.start()).collect();
    let starts: Vec<usize> = std::iter::once(0)
        .chain(pos.find_iter(input).map(|m| m.start()))
        .collect();

    let ranges: Vec<Range<usize>> = starts
        .into_iter()
        .map(|start| {
            let end = stops
                .iter()
                .find(|&&stop| stop > start)
                .copied()
                .unwrap_or_else(|| input.len());
            start..end
        })
        .collect();

    let filtered_ranges = ranges.into_iter().fold(Vec::new(), |mut acc, range| {
        if acc
            .last()
            .map(|prev: &Range<usize>| range.end <= prev.end)
            .unwrap_or(false)
        {
            acc
        } else {
            acc.push(range);
            acc
        }
    });

    let cut: String = filtered_ranges
        .iter()
        .map(|r| &input[r.start..r.end])
        .collect();

    solve_for_input(cut.as_str()).to_string()
}

fn solve_for_input(input: &str) -> u32 {
    let re = Regex::new(r"mul\((?<x>\d{1,3}),(?<y>\d{1,3})\)").expect("Failed to compile regex");
    re.captures_iter(input)
        .map(|caps| {
            let x: u32 = caps
                .name("x")
                .expect("Missing x")
                .as_str()
                .parse()
                .expect("Failed to parse x");
            let y: u32 = caps
                .name("y")
                .expect("Missing y")
                .as_str()
                .parse()
                .expect("Failed to parse y");
            x * y
        })
        .sum()
}

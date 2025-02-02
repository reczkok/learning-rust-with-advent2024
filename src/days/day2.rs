#[derive(Debug, Copy, Clone)]
enum Direction {
    Asc,
    Desc,
}

fn check_sequence(nums: &[u32]) -> bool {
    if nums.len() < 2 {
        return false;
    }

    let mut iter = nums.iter();
    let mut prev = *iter.next().unwrap();
    let mut dir: Option<Direction> = None;

    for &n in iter {
        if dir.is_none() {
            if n == prev {
                return false;
            }
            if n > prev {
                if n - prev > 3 {
                    return false;
                }
                dir = Some(Direction::Asc);
            } else {
                if prev - n > 3 {
                    return false;
                }
                dir = Some(Direction::Desc);
            }
        } else {
            match dir.unwrap() {
                Direction::Asc => {
                    if n <= prev || n - prev > 3 {
                        return false;
                    }
                }
                Direction::Desc => {
                    if n >= prev || prev - n > 3 {
                        return false;
                    }
                }
            }
        }
        prev = n;
    }
    true
}

fn is_valid_line(nums: &[u32], allow_removal: bool) -> bool {
    if check_sequence(nums) {
        return true;
    }

    // Brute force (╯'□')╯︵ ┻━┻
    if allow_removal {
        for i in 0..nums.len() {
            let mut new_nums = Vec::with_capacity(nums.len() - 1);
            new_nums.extend_from_slice(&nums[..i]);
            new_nums.extend_from_slice(&nums[i + 1..]);
            if check_sequence(&new_nums) {
                return true;
            }
        }
    }
    false
}

pub fn solve(input: &str, second_part: bool) -> String {
    let valid_count = input
        .lines()
        .filter(|line| {
            let nums: Vec<u32> = line
                .split_whitespace()
                .map(|s| s.parse::<u32>().expect("Failed to parse number"))
                .collect();
            is_valid_line(&nums, second_part)
        })
        .count();
    valid_count.to_string()
}

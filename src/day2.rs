use std::{fmt::Display, str::FromStr};

fn main() -> color_eyre::Result<()> {
    let input = std::fs::read_to_string("inputs/day2.txt")?;
    let mut sum = 0u64;
    for line in input.trim().split(',') {
        let range = line.parse::<Range>()?;
        sum += range.half_range();
    }
    println!("Sum of invalid ids in the range: {sum}");
    Ok(())
}

#[derive(Debug)]
struct Range {
    lower: u64,
    upper: u64,
}

impl Display for Range {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.lower, self.upper)
    }
}

impl FromStr for Range {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('-').collect();
        if parts.len() != 2 {
            return Err(eyre::eyre!("Invalid range format: {}", s));
        }
        let lower = parts[0]
            .parse::<u64>()
            .map_err(|e| eyre::eyre!("Failed to parse lower bound: {}", e))?;
        let upper = parts[1]
            .parse::<u64>()
            .map_err(|e| eyre::eyre!("Failed to parse upper bound: {}", e))?;
        Ok(Range { lower, upper })
    }
}

impl Range {
    pub fn half_range(&self) -> u64 {
        println!("Processing range: {}", self);
        let lower_str = self.lower.to_string();
        let upper_str = self.upper.to_string();
        let lower_len = lower_str.len();
        let upper_len = upper_str.len();
        if !lower_len.is_multiple_of(2) && !upper_len.is_multiple_of(2) && lower_len == upper_len {
            return 0;
        }
        let lower_half_len = lower_len / 2;
        let upper_half_len = upper_len / 2;
        let lower_half_str = if lower_len.is_multiple_of(2) {
            lower_str[..lower_half_len].to_string()
        } else {
            "1".to_string() + &"0".repeat(lower_half_len)
        };
        let mut start = lower_half_str.parse::<u64>().unwrap();
        if start * 10u64.pow(lower_half_len as u32) + start < self.lower {
            start += 1;
        }
        let upper_half_str = if upper_len.is_multiple_of(2) {
            upper_str[..upper_half_len].to_string()
        } else {
            "9".repeat(upper_half_len)
        };
        let mut end = upper_half_str.parse::<u64>().unwrap();
        if end * 10u64.pow(upper_half_len as u32) + end > self.upper {
            end -= 1;
        }

        let mut sum = 0;
        for n in start..=end {
            let candidate = (n.to_string() + &n.to_string()).parse::<u64>().unwrap();
            if candidate >= self.lower && candidate <= self.upper {
                sum += candidate;
            }
        }
        sum
    }
}

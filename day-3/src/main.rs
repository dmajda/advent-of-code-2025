use std::io;

use anyhow::{Result, ensure};

fn find_max_joltage(bank: &[u8], n: usize) -> u64 {
    let mut max_joltage = 0;
    let mut start = 0;
    let mut end = bank.len() - n + 1;

    for _ in 0..n {
        let (index, joltage) = find_max_battery(bank, start, end);

        max_joltage = 10 * max_joltage + joltage;
        start = index + 1;
        end += 1;
    }

    max_joltage
}

fn find_max_battery(bank: &[u8], start: usize, end: usize) -> (usize, u64) {
    // The `rev` call is needed because `max_by_key` finds the *last* maximum
    // value and we want the *first* one.

    bank[start..end]
        .iter()
        .enumerate()
        .rev()
        .max_by_key(|&(_, b)| b)
        .map(|(i, b)| (start + i, (b - b'0') as u64))
        .unwrap()
}

fn main() -> Result<()> {
    let mut joltage_2 = 0;
    let mut joltage_12 = 0;

    for line in io::stdin().lines() {
        let line = line?;
        let bank = line.as_bytes();

        ensure!(bank.len() >= 12, "insufficient bank length: {}", bank.len());
        ensure!(
            bank.iter().all(|&b| b >= b'0' && b <= b'9'),
            "invalid bank: {line:?}"
        );

        joltage_2 += find_max_joltage(bank, 2);
        joltage_12 += find_max_joltage(bank, 12);
    }

    println!("{}", joltage_2);
    println!("{}", joltage_12);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_max_joltage_works() {
        assert_eq!(find_max_joltage(b"987654321111111", 2), 98);
        assert_eq!(find_max_joltage(b"811111111111119", 2), 89);
        assert_eq!(find_max_joltage(b"234234234234278", 2), 78);
        assert_eq!(find_max_joltage(b"818181911112111", 2), 92);

        assert_eq!(find_max_joltage(b"987654321111111", 12), 987654321111);
        assert_eq!(find_max_joltage(b"811111111111119", 12), 811111111119);
        assert_eq!(find_max_joltage(b"234234234234278", 12), 434234234278);
        assert_eq!(find_max_joltage(b"818181911112111", 12), 888911112111);
    }
}

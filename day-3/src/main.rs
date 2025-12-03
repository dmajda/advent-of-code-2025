use std::io;

use anyhow::Result;

fn find_max_joltage(bank: &str) -> u32 {
    debug_assert!(bank.len() >= 2);
    debug_assert!(bank.bytes().all(|b| b >= b'0' && b <= b'9'));

    let (index1, joltage1) = find_max_battery(bank, 0, bank.len() - 1);
    let (_, joltage2) = find_max_battery(bank, index1 + 1, bank.len());

    10 * joltage1 + joltage2
}

fn find_max_battery(bank: &str, start: usize, end: usize) -> (usize, u32) {
    // The `rev` call is needed because `max_by_key` finds the *last* maximum
    // value and we want the *first* one.

    bank[start..end]
        .bytes()
        .enumerate()
        .rev()
        .max_by_key(|&(_, b)| b)
        .map(|(i, b)| (i, (b - b'0') as u32))
        .unwrap()
}

fn main() -> Result<()> {
    let mut joltage = 0;

    for line in io::stdin().lines() {
        joltage += find_max_joltage(&line?);
    }

    println!("{}", joltage);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_max_joltage_works() {
        assert_eq!(find_max_joltage("987654321111111"), 98);
        assert_eq!(find_max_joltage("811111111111119"), 89);
        assert_eq!(find_max_joltage("234234234234278"), 78);
        assert_eq!(find_max_joltage("818181911112111"), 92);
    }
}

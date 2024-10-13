use std::collections::HashSet;

pub fn main() {
    use std::time::Instant;
    use thousands::Separable;

    println!("Day 5: Doesn't He Have Intern-Elves For This?");
    let input = include_str!("input.txt");

    let now = Instant::now();
    let day01_result = part_one(input);
    let elapsed = now.elapsed();
    println!(
        "Part 1: {} ({:.2?})",
        day01_result.separate_with_commas(),
        elapsed
    );

    let now = Instant::now();
    let day02_result = part_two(input);
    let elapsed = now.elapsed();

    println!(
        "Part 2: {} ({:.2?})\n",
        day02_result.separate_with_commas(),
        elapsed
    );
}

fn has_three_vowels(s: &str) -> bool {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    s.chars().filter(|c| vowels.contains(c)).count() >= 3
}
fn has_double_letter(s: &str) -> bool {
    s.chars()
        .collect::<Vec<char>>()
        .windows(2)
        .any(|w| w[0] == w[1])
}

fn has_bad_strings(s: &str) -> bool {
    let bad_strings = ["ab", "cd", "pq", "xy"];
    !bad_strings.into_iter().any(|bs| s.contains(bs))
}

fn is_nice(s: &str) -> bool {
    has_three_vowels(s) && has_double_letter(s) && has_bad_strings(s)
}

fn part_one(input: &str) -> u32 {
    input
        .lines()
        .filter(|s| is_nice(s))
        .count()
        .try_into()
        .unwrap()
}

fn has_double_double_letter(s: &str) -> bool {
    false
}

fn part_two(input: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_three_vowels() {
        assert!(has_three_vowels("aei"));
        assert!(has_three_vowels("xazegov"));
        assert!(has_three_vowels("aeiouaeiouaeiou"));
        assert!(!has_three_vowels("dvszwmarrgswjxmb"));
    }

    #[test]
    fn test_has_double_letter() {
        assert!(has_double_letter("xx"));
        assert!(has_double_letter("abcdde"));
        assert!(has_double_letter("aabbccdd"));
        assert!(!has_double_letter("jchzalrnumimnmhp"));
    }

    #[test]
    fn test_has_bad_strings() {
        assert!(!has_bad_strings("haegwjzuvuyypxyu"));
    }

    #[test]
    fn test_is_nice() {
        assert!(is_nice("ugknbfddgicrmopn"));
        assert!(is_nice("aaa"));
        assert!(!is_nice("jchzalrnumimnmhp"));
        assert!(!is_nice("haegwjzuvuyypxyu"));
        assert!(!is_nice("dvszwmarrgswjxmb"));
    }

    #[test]
    fn test_part_one() {
        let input = include_str!("input.txt");
        assert_eq!(part_one(input), 255);
    }

    #[test]
    fn test_has_double_double_letter() {
        assert!(has_double_double_letter("xyxy"));
        assert!(has_double_double_letter("aabcdefgaa"));
        assert!(!has_double_double_letter("aaa"));
    }

    // #[test]
    // fn test_part_two() {
    //     let input = include_str!("input.txt");
    //     assert_eq!(part_two(input), 9_962_624);
    // }
}

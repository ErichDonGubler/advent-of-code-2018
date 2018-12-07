use {
    aoc_runner_derive::{aoc},
    ascii::{
        AsciiChar,
        AsciiStr,
        AsciiString,
        AsAsciiStr,
        AsAsciiStrError,
    },
    itertools::Itertools,
    std::{collections::HashSet},
    try_from::TryFrom,
};


#[cfg(test)]
mod test {
    macro_rules! all_solutions {
        ($input: expr, $expected_output: expr) => {{
            let input = $input;
            let expected_output = $expected_output;
            eprintln!("Running day5_part1_brute version");
            assert_eq!(super::day5_part1_brute(input), expected_output);
            eprintln!("Running day5_part1_vec_split version");
            assert_eq!(super::day5_part1_vec_split(input), expected_output);
        }};
    }

    #[test]
    fn test_day5_part1_hint() {
        all_solutions!("aA", 0);
        all_solutions!("abBA", 0);
        all_solutions!("abAB", 4);
        all_solutions!("aabAAB", 6);
        all_solutions!("dabAcCaCBAcCcaDA", 10);
    }

    #[test]
    fn test_day5_part1_random() {
        all_solutions!("DTKkCFfciIF", 3);
    }

    #[test]
    fn test_day5_part1_answer() {
        all_solutions!(include_str!("../input/2018/day5.txt"), 9686);
    }
}

fn are_opposite_case(c1: AsciiChar, c2: AsciiChar) -> bool {
    match c1.is_lowercase() {
        true => c1.to_ascii_uppercase() == c2,
        false => c1.to_ascii_lowercase() == c2,
    }
}

#[derive(Clone, Debug)]
struct Polymer<'a>(&'a AsciiStr);

#[derive(Debug)]
enum PolymerParseError {
    InputIsNotAscii(AsAsciiStrError),
    InvalidAsciiCharacterFound(usize),
}

impl<'a> TryFrom<&'a str> for Polymer<'a> {
    type Err = PolymerParseError;

    fn try_from(s: &'a str) -> Result<Self, Self::Err> {
        use self::{PolymerParseError::*};

        let a = s.as_ascii_str().map_err(InputIsNotAscii)?;
        for (i, c) in a.chars().enumerate() {
            if !c.is_alphabetic() {
                return Err(InvalidAsciiCharacterFound(i));
            }
        }

        Ok(Polymer(a))
    }
}

#[aoc(day5, part1, brute)]
pub fn day5_part1_brute(input: &str) -> usize {
    let Polymer(polymer) = Polymer::try_from(input.trim()).unwrap();
    let mut polymer = polymer.to_owned();

    let mut i = 0;
    while i < polymer.len().saturating_sub(1) {
        let c1 = polymer[i];
        let c2 = polymer[i + 1];
        if are_opposite_case(c1, c2) {
            polymer.drain(i..=i + 1);
            i = i.saturating_sub(1);
        } else {
            i += 1;
        }
    }
    polymer.len()
}

#[aoc(day5, part1, vec_split)]
pub fn day5_part1_vec_split(input: &str) -> usize {
    let Polymer(mut polymer) = Polymer::try_from(input.trim()).unwrap();

    while polymer.len() >= 2 && are_opposite_case(polymer[0], polymer[1]) {
        polymer = &polymer[2..];
    }

    while polymer.len() >= 2 && are_opposite_case(polymer[polymer.len() - 1], polymer[polymer.len() - 2]) {
        polymer = &polymer[..polymer.len() - 2];
    }

    match polymer.len() {
        0..=3 => return polymer.len(),
        _ => (),
    }

    let mut polymer_groups = Vec::new();
    let mut next_split_begin = 0;
    let mut iter = polymer[..polymer.len() - 1].chars().enumerate().tuple_windows::<(_, _)>().skip(1);
    let mut next = iter.next().unwrap();
    loop {
        let ((i, c1), (_, c2)) = next;
        if are_opposite_case(*c1, *c2) {
            let _ = iter.next();
            let (left, _) = polymer.split_at(i + 2);
            let left = &left[next_split_begin..left.len() - 2];
            if !left.is_empty() {
                polymer_groups.push(left);
            }
            next_split_begin = i + 2;
        }

        match iter.next() {
            Some(n) => next = n,
            None => break,
        }
    }
    if polymer_groups.is_empty() {
        return 0;
    }
    polymer_groups.push(&polymer[next_split_begin..]);

    loop {
        let mut changed_this_time = false;
        let mut rev_idx = 0;
        while rev_idx < polymer_groups.len() - 1 {
            let right_idx = polymer_groups.len() - rev_idx - 1;
            let left_idx = polymer_groups.len() - rev_idx - 2;
            let mut right_part = polymer_groups[right_idx];
            let mut left_part = polymer_groups[left_idx];

            let c1 = *left_part.chars().last().unwrap();
            let c2 = *right_part.chars().next().unwrap();
            if are_opposite_case(c1, c2) {
                changed_this_time = true;
                right_part = &right_part[1..];
                if right_part.is_empty() {
                    polymer_groups.remove(right_idx);
                    rev_idx += 1;
                } else {
                    polymer_groups[right_idx] = right_part;
                }
                left_part = &left_part[..left_part.len() - 1];
                if left_part.is_empty() {
                    polymer_groups.remove(left_idx);
                    rev_idx += 1;
                } else {
                    polymer_groups[left_idx] = left_part;
                }

                if polymer_groups.is_empty() {
                    return 0;
                }
            } else {
                rev_idx += 1;
            }
        }
        if !changed_this_time {
            break polymer_groups.into_iter().map(|a| a.len()).sum();
        }
    }
}

#[aoc(day5, part2)]
pub fn day5_part2(input: &str) -> usize {
    let Polymer(polymer) = Polymer::try_from(input.trim()).unwrap();

    polymer.chars()
        .fold(HashSet::with_capacity(26), |mut acc, c| { acc.insert(c); acc })
        .into_iter()
        .map(|c| {
            let polymer = polymer.chars().cloned()
                .filter(|p| p.to_ascii_lowercase() != c.to_ascii_lowercase())
                .collect::<AsciiString>();
            day5_part1_brute(polymer.as_str()) // FIXME: Make an intermediate function, silly
        }).min().unwrap()
}

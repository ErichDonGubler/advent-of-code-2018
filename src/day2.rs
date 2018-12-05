use {
    aoc_runner_derive::aoc,
    arrayvec::ArrayVec,
    std::fmt::{Display, Formatter, Result as FmtResult},
};

#[aoc(day2, part1)]
pub fn day2_part1(input: &str) -> u32 {
    let mut count_had_two: u32 = 0;
    let mut count_had_three: u32 = 0;
    let mut letters = ArrayVec::<[(char, u32); 26]>::new();
    for box_id in input.split('\n') {
        'digit: for id_digit in box_id.chars() {
            for (c, count) in letters.iter_mut() {
                if *c == id_digit {
                    *count += 1;
                    continue 'digit;
                }
            }
            letters.push((id_digit, 1));
        }

        let mut had_two = false;
        let mut had_three = false;
        for (_, count) in letters.drain(..) {
            match count {
                2 => had_two = true,
                3 => had_three = true,
                _ => (),
            }
        }

        if had_two {
            count_had_two += 1;
        }
        if had_three {
            count_had_three += 1;
        }
    }
    count_had_two.checked_mul(count_had_three).unwrap()
}

pub struct Part2Answer<'s> {
    s1: &'s str,
    s2: &'s str,
}

impl<'s> Display for Part2Answer<'s> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        for c in self
            .s1
            .chars()
            .zip(self.s2.chars())
            .filter(|(c1, c2)| c1 == c2)
            .map(|(c, _)| c)
        {
            write!(f, "{}", c)?;
        }
        Ok(())
    }
}

#[aoc(day2, part2)]
pub fn day2_part2(input: &str) -> &'static str {
    let box_ids = input
        .split('\n')
        .take_while(|s| !s.is_empty())
        .collect::<ArrayVec<[&str; 250]>>();
    for box_id in box_ids.iter() {
        assert_eq!(box_id.len(), 26);
    }
    assert!(box_ids.is_full());

    let num_box_ids = box_ids.len();
    for (box_id_idx, box_id) in box_ids[..num_box_ids - 1].iter().enumerate() {
        for other_box_id in box_ids[box_id_idx + 1..].iter() {
            let num_differences = box_id
                .chars()
                .zip(other_box_id.chars())
                .filter(|(c1, c2)| c1 != c2)
                .count();
            if num_differences == 1 {
                let answer = Part2Answer {
                    s1: box_id,
                    s2: other_box_id,
                };

                // FIXME: #1: We COULD just return this theoretically, but `aoc`'s implementation
                // prevents using a return value with a lifetime for some reason
                // return answer;
                println!("answer: {}", answer);
                return "";
            }
        }
    }
    panic!("No answer found");
}

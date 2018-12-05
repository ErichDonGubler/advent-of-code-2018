use {
    aoc_runner_derive::aoc,
    std::{collections::HashMap, str::Split},
};

struct Day1EntryIterator<'i> {
    input: Split<'i, char>,
}

impl<'i> Day1EntryIterator<'i> {
    pub fn new(input: &'i str) -> Self {
        Self {
            input: input.split('\n'),
        }
    }
}

impl<'i> Iterator for Day1EntryIterator<'i> {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        let change = self.input.next()?;
        let positive = match change.chars().next()? {
            '+' => true,
            '-' => false,
            _ => unreachable!(),
        };
        let mut number: i32 = change[1..].parse().unwrap();
        if !positive {
            number = number.checked_mul(-1).unwrap();
        }
        Some(number)
    }
}

#[aoc(day1, part1)]
pub fn day1_part1(input: &str) -> i32 {
    let mut total: i32 = 0;
    for change in Day1EntryIterator::new(input) {
        total = total.checked_add(change).unwrap();
    }
    total
}

#[aoc(day1, part2)]
pub fn day1_part2(input: &str) -> i32 {
    let changes = Day1EntryIterator::new(input).collect::<Vec<_>>();
    let mut repeat_count: i32 = 0;
    let mut seen = HashMap::new();
    let mut total: i32 = 0;
    loop {
        seen.insert(total, (repeat_count, 0));
        for (change_num, change) in changes.iter().cloned().enumerate() {
            total = total.checked_add(change).unwrap();
            if let Some(_) = seen.insert(total, (repeat_count, change_num as i32 + 1)) {
                return total;
            }
        }
        repeat_count = repeat_count.checked_add(1).unwrap();
    }
}

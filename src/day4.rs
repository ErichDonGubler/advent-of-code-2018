use {
    aoc_runner_derive::aoc,
    arrayvec::ArrayVec,
    chrono::{
        NaiveDateTime,
        Timelike,
    },
    derive_more::{
        Display,
        Sub,
    },
    lazy_static::lazy_static,
    re_parse::Regex,
    std::{
        collections::BTreeMap,
        cmp::Ordering::*,
        fmt::{
            Display,
            Formatter,
            Result as FmtResult,
        },
    },
    try_from::TryFrom,
};

#[cfg(test)]
const HINT_INPUT: &'static str =
r#"[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up
"#;

#[cfg(test)]
const INPUT: &'static str = include_str!("../input/2018/day4.txt");

#[derive(Debug, PartialEq, Eq)]
pub struct Answer {
    guard_id: GuardId,
    minute: Minute,
}

impl Display for Answer {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.minute.0 as u32 * self.guard_id.0)
    }
}

#[test]
fn test_day4_part1_hint() {
    assert_eq!(day4_part1(HINT_INPUT), Answer {
        guard_id: GuardId(10),
        minute: Minute(24),
    });
}

#[derive(Clone, Copy, Debug, Display, Eq, Ord, PartialEq, PartialOrd)]
#[display(fmt = "{}", _0)]
struct GuardId(u32);

#[derive(Clone, Copy, Debug, Display, Eq, Ord, PartialEq, PartialOrd, Sub)]
#[display(fmt = "{}", _0)]
struct Minute(u8);

const MINUTES_PER_HOUR: u8 = 60;

#[derive(Debug)]
struct MinuteConversionError(pub u8);

impl TryFrom<u8> for Minute {
    type Err = MinuteConversionError;

    fn try_from(m: u8) -> Result<Self, Self::Err> {
        match m {
            m if m < MINUTES_PER_HOUR => Ok(Minute(m)),
            m => Err(MinuteConversionError(m)),
        }
    }
}

#[derive(Debug)]
enum GuardEvent {
    ShiftChange {
        new_guard_id: GuardId,
    },
    FallAsleep,
    WakeUp,
}

type MinuteCounts = [u8; MINUTES_PER_HOUR as usize - 1];

fn process_input<F: FnMut(GuardId, (Minute, Minute))>(input: &str, mut f: F) -> BTreeMap<GuardId, MinuteCounts> {
    use self::GuardEvent::*;

    let mut log = BTreeMap::new();
    for line in input.split('\n') {
        if line == "" {
            continue;
        }

        let (time, event) = {
            let (time, mut guard_event_str) = line.split_at(line.find(']').unwrap() + 1);
            (
                NaiveDateTime::parse_from_str(time, "[%Y-%m-%d %R]").unwrap(),
                {
                    lazy_static! {
                        static ref REGEX_SHIFT_CHANGE: Regex = Regex::new(r#"Guard #(?P<new_guard_id>\d{1,4}) begins shift"#).unwrap();
                    }
                    assert_eq!(guard_event_str.chars().next().unwrap(), ' ');
                    guard_event_str = &guard_event_str[1..];
                    match guard_event_str {
                        "wakes up" => WakeUp,
                        "falls asleep" => FallAsleep,
                        other => ShiftChange {
                            new_guard_id: REGEX_SHIFT_CHANGE.captures(other).unwrap()["new_guard_id"].parse().map(GuardId).unwrap(),
                        }
                    }
                }
            )
        };
        log.insert(time, event);
    }

    let mut guard_midnight_sleep_minutes = BTreeMap::new();
    let mut log_events = log.into_iter();
    let mut next_event = log_events.next();
    'shift: loop {
        let current_guard_id = match next_event {
            Some((_, ShiftChange { new_guard_id })) => new_guard_id,
            Some(other) => panic!("expected shift change as next event, got {:?}", other),
            None => break,
        };
        next_event = log_events.next();
        loop {
            let start_time = match next_event {
                Some((_, ShiftChange { .. })) => break,
                Some((time, FallAsleep)) => time,
                None => break 'shift,
                other => panic!("expected guard falling asleep as next event, got {:?}", other),
            };
            let end_time = match log_events.next() {
                Some((time, WakeUp)) => time,
                other => panic!("expected guard waking up as next event, got {:?}", other),
            };
            // Make sure the guard sleeps during midnight hour like instructions say
            assert_eq!(start_time.hour(), 0);
            assert_eq!(end_time.hour(), 0);

            let start_minute = Minute::try_from(start_time.minute() as u8).unwrap();
            let end_minute = Minute::try_from(end_time.minute() as u8).unwrap();
            assert!(start_minute < end_minute);

            let minute_counts = guard_midnight_sleep_minutes.entry(current_guard_id).or_insert_with(|| [0u8; MINUTES_PER_HOUR as usize - 1]);
            for minute_count in &mut minute_counts[start_minute.0 as usize..end_minute.0 as usize] {
                *minute_count += 1;
            }
            f(current_guard_id, (start_minute, end_minute));

            next_event = log_events.next();
        }
    }

    guard_midnight_sleep_minutes
}

fn most_common_minutes(minute_counts: &MinuteCounts) -> (ArrayVec<[Minute; MINUTES_PER_HOUR as usize - 1]>, u8) {
    let mut minutes = ArrayVec::<[Minute; MINUTES_PER_HOUR as usize - 1]>::new();
    let mut highest_minute_count_seen = 0;
    for (minute, count) in minute_counts.iter().enumerate() {
        let minute = Minute::try_from(minute as u8).unwrap();
        match count.cmp(&highest_minute_count_seen) {
            Greater => {
                highest_minute_count_seen = *count;
                minutes.clear();
                minutes.push(minute);
            },
            Equal => minutes.push(minute),
            Less => (),
        }
    }
    (minutes, highest_minute_count_seen)
}

#[aoc(day4, part1)]
pub fn day4_part1(input: &str) -> Answer {
    let mut midnight_sleep_minute_counts = BTreeMap::new();
    let midnight_sleep_minutes = process_input(input, |guard_id, (start_minute, end_minute)| *midnight_sleep_minute_counts.entry(guard_id).or_insert(0u32) += (end_minute - start_minute).0 as u32);

    let sleepiest_guard = {
        let mut iter = midnight_sleep_minute_counts.iter();
        let (mut sleepiest_guard, mut most_seen) = iter.next().unwrap();
        let mut equals_seen = 0usize;
        for (guard, minutes_asleep) in iter {
            match minutes_asleep.cmp(most_seen) {
                Greater => {
                    most_seen = minutes_asleep;
                    sleepiest_guard = guard;
                    equals_seen = 0;
                },
                Equal => equals_seen += 1,
                Less => (),
            }
        }
        assert_eq!(equals_seen, 0);
        sleepiest_guard
    };

    let most_common_minute = {
        let (minutes, _) = most_common_minutes(midnight_sleep_minutes.get(sleepiest_guard).unwrap());
        assert_eq!(minutes.len(), 1);
        minutes[0]
    };

    Answer {
        minute: most_common_minute,
        guard_id: *sleepiest_guard,
    }
}

#[test]
fn test_day4_part1_answer() {
    assert_eq!(day4_part1(INPUT), Answer {
        guard_id: GuardId(641),
        minute: Minute(41),
    });
}

#[aoc(day4, part2)]
pub fn day4_part2(input: &str) -> Answer {
    let mut log = process_input(input, |_, _| {}).into_iter();
    let mut next_guard_minutes = || log.next().map(|(g, m)| (g, most_common_minutes(&m)));
    let (mut guard_with_highest_minute_count, (mut most_common_minutes_for_guard, mut highest_minute_count_seen)) = next_guard_minutes().unwrap();
    while let Some((guard, (most_common_minutes, greatest_minute_count))) = next_guard_minutes() {
        match greatest_minute_count.cmp(&highest_minute_count_seen) {
            Greater => {
                guard_with_highest_minute_count = guard;
                most_common_minutes_for_guard = most_common_minutes;
                highest_minute_count_seen = greatest_minute_count;
            },
            Equal | Less => (),
        }
    }
    assert_eq!(most_common_minutes_for_guard.len(), 1);
    Answer {
        guard_id: guard_with_highest_minute_count,
        minute: most_common_minutes_for_guard[0],
    }
}

#[test]
fn test_day4_part2_hint() {
    assert_eq!(day4_part2(HINT_INPUT), Answer {
        guard_id: GuardId(99),
        minute: Minute(45),
    });
}

#[test]
fn test_day4_part2_answer() {
    assert_eq!(day4_part2(INPUT), Answer {
        guard_id: GuardId(1973),
        minute: Minute(37),
    });
}

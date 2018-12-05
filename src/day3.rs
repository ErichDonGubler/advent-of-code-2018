use {
    aoc_runner_derive::aoc,
    re_parse::{
        Regex,
        ReParse,
        Error as ReParseError,
    },
    serde_derive::Deserialize,
    std::{
        cmp::max,
        mem::replace,
        ops::{
            Index,
            IndexMut,
        },
        slice::Iter as SliceIter,
        str::{
            FromStr,
            Split,
        },
    },
};

struct ClaimIterator<'s> {
    input: Split<'s, char>,
}

impl<'s> ClaimIterator<'s> {
    pub fn new(input: &'s str) -> Self {
        ClaimIterator {
            input: input.split('\n'),
        }
    }
}

#[derive(Debug, Deserialize, ReParse)]
#[re_parse(
    regex = r#"#(?P<id>\d{1,4}) @ (?P<left>\d{1,3}),(?P<top>\d{1,3}): (?P<width>\d{1,2})x(?P<height>\d{1,2})"#
)]
struct RawClaim {
    id: usize,
    left: usize,
    top: usize,
    width: usize,
    height: usize,
}

#[derive(Clone, Debug)]
struct Claim {
    id: usize,
    left: usize,
    top: usize,
    right: usize,
    bottom: usize,
}

impl Claim {
    // FIXME: This is actually wrong, and I've just compensated by making `intersects` inclusive.
    // There should be no need to call this twice!
    fn contains_edge_of(&self, other: &Self) -> (bool, bool) {
        let intersects_horizontally = {
            let bottom_in_horizontal_band = self.bottom > other.top && self.bottom <= other.bottom;
            let top_in_horizontal_band = self.top >= other.top && self.top < other.bottom;
            bottom_in_horizontal_band || top_in_horizontal_band
        };
        let intersects_vertically = {
            let left_in_vertical_band = self.left >= other.left && self.left < other.right;
            let right_in_vertical_band = self.right > other.left && self.right <= other.right;
            left_in_vertical_band || right_in_vertical_band
        };
        (intersects_horizontally, intersects_vertically)
    }

    pub fn intersects(&self, other: &Self) -> bool {
        let (self_contains_horiz, self_contains_vert) = self.contains_edge_of(other);
        let (other_contains_horiz, other_contains_vert) = other.contains_edge_of(self);
        (self_contains_horiz || other_contains_horiz) && (self_contains_vert || other_contains_vert)
    }
}

#[test]
fn test_intersection() {
    const CLAIM_TO_COMPARE_TO: &'static str = "#0 @ 2,2: 3x3";
    let claim: Claim = CLAIM_TO_COMPARE_TO.parse().unwrap();

    for other in &[
        // Close but not touching
        "#0 @ 1,1: 1x1",
        "#0 @ 2,1: 1x1",
        "#0 @ 3,1: 1x1",
        "#0 @ 4,1: 1x1",
        "#0 @ 5,1: 1x1",
        "#0 @ 5,2: 1x1",
        "#0 @ 5,3: 1x1",
        "#0 @ 5,4: 1x1",
        "#0 @ 5,5: 1x1",
        "#0 @ 4,5: 1x1",
        "#0 @ 3,5: 1x1",
        "#0 @ 2,5: 1x1",
        "#0 @ 1,5: 1x1",
        "#0 @ 1,4: 1x1",
        "#0 @ 1,3: 1x1",
        "#0 @ 1,2: 1x1",

        // Way out there
    ] {
        if claim.intersects(&other.parse().unwrap()) {
            panic!("{:?} is not supposed to intersect {:?}", other, claim);
        }
    }

    for other in &[
        // Same thing
        CLAIM_TO_COMPARE_TO,

        // Other encompasses first
        "#0 @ 1,1: 5x5",

        // First encompasses other
        "#0 @ 3,3: 1x1",

        // Edges
        "#0 @ 1,1: 2x2",
        "#0 @ 2,1: 2x2",
        "#0 @ 3,1: 2x2",
        "#0 @ 3,2: 2x2",
        "#0 @ 3,3: 2x2",
        "#0 @ 2,3: 2x2",
        "#0 @ 1,3: 2x2",
        "#0 @ 1,2: 2x2",
    ] {
        if !claim.intersects(&other.parse().unwrap()) {
            panic!("{:?} is supposed to intersect {:?}", other, claim);
        }
    }

    // Other failing cases found
    fn intersects(s1: &str, s2: &str) -> bool{
        s1.parse::<Claim>().unwrap().intersects(&s2.parse().unwrap())
    }
    //"#1236 @ ".parse().unwrap()
    assert!(intersects("#1236 @ 420,613: 19x12", "#344 @ 426,611: 12x21"));
}

#[derive(Debug)]
enum ClaimParseError {
    ParseFailed(ReParseError),
    InvalidDimensions(usize, usize),
}

impl FromStr for Claim {
    type Err = ClaimParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use self::ClaimParseError::*;

        let RawClaim {
            id,
            left,
            top,
            width,
            height,
        } = RawClaim::from_str(s).map_err(ParseFailed)?;

        if width == 0 || height == 0 {
            return Err(InvalidDimensions(width, height));
        }

        Ok(Self {
            id,
            left,
            top,
            right: left.checked_add(width).unwrap(),
            bottom: top.checked_add(height).unwrap(),
        })
    }
}

impl<'s> Iterator for ClaimIterator<'s> {
    type Item = Claim;

    fn next(&mut self) -> Option<Self::Item> {
        match self.input.next()? {
            "" => None,
            other => Some(other.parse().unwrap()),
        }
    }
}

struct GrowOnlyGrid<T> {
    inner: Vec<T>,
    len_x: usize,
    len_y: usize,
}

impl<T> GrowOnlyGrid<T> {
    pub fn new_with<F: FnMut() -> T>(x: usize, y: usize, mut f: F) -> Self {
        Self {
            inner: {
                let len = x.checked_mul(y).unwrap();
                let mut inner = Vec::with_capacity(len);
                // OPT: Use the soon-to-be-stable `resize_with` instead.
                while inner.len() < len {
                    inner.push(f());
                }
                inner
            },
            len_x: x,
            len_y: y,
        }
    }

    pub fn grow_with<F: FnMut() -> T>(&mut self, x: usize, y: usize, f: F)
        where T: Default,
    {
        let old_len_x = self.len_x;
        let old_len_y = self.len_y;
        let old = replace(self, Self::new_with(max(x, old_len_x), max(y, old_len_y), f));

        let mut old_values = old.inner.into_iter();
        for y in 0..old_len_y {
            // OPT:  We could probably just copy slices here directly
            for x in 0..old_len_x {
                let idx = unsafe {
                    self.index_from_coords_unchecked(x, y)
                };
                self.inner[idx] = old_values.next().unwrap();
            }
        }
    }

    pub fn dimensions(&self) -> (usize, usize) {
        (self.len_x, self.len_y)
    }

    fn index_from_coords(&self, x: usize, y: usize) -> usize {
        if x >= self.len_x || y >= self.len_y {
            panic!("coordinates {:?} exceed current dimensions of {:?}", (x, y), self.dimensions());
        }
        unsafe {
            self.index_from_coords_unchecked(x, y)
        }
    }

    unsafe fn index_from_coords_unchecked(&self, x: usize, y: usize) -> usize {
        y * self.len_x + x
    }
}

impl<T> Index<(usize, usize)> for GrowOnlyGrid<T> {
    type Output = T;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        let idx = self.index_from_coords(x, y);
        &self.inner[idx]
    }
}

impl<T> IndexMut<(usize, usize)> for GrowOnlyGrid<T> {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        let idx = self.index_from_coords(x, y);
        &mut self.inner[idx]
    }
}

impl<T> GrowOnlyGrid<T> {
    pub fn iter_flat(&self) -> SliceIter<T> {
        self.inner[..].iter()
    }
}

#[aoc(day3, part1)]
pub fn day3_part1(input: &str) -> usize {
    let mut grid = GrowOnlyGrid::<u8>::new_with(1000, 1000, Default::default);
    for claim in ClaimIterator::new(input) {
        let Claim {
            id: _,
            left,
            top,
            right,
            bottom,
        } = claim;

        grid.grow_with(
            right.checked_add(1).unwrap(),
            bottom.checked_add(1).unwrap(),
            Default::default,
        );

        for y in top..bottom {
            for x in left..right {
                let blarg = &mut grid[(x, y)];
                *blarg = blarg.checked_add(1).unwrap();
            }
        }
    }

    grid.iter_flat().filter(|x| x > &&1).count()
}

#[cfg(test)]
const INPUT: &'static str = include_str!("../input/2018/day3.txt");
#[cfg(test)]
const HINT_INPUT: &'static str =
r#"#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2
"#;
#[cfg(test)]
const HINT_EXPECTED_PART1_OUTPUT: usize = 4;
#[cfg(test)]
const HINT_EXPECTED_PART2_OUTPUT: usize = 3;
#[cfg(test)]
const EXPECTED_PART2_OUTPUT: usize = 603;

#[test]
fn test_day3_part1_hint() {
    assert_eq!(day3_part1(HINT_INPUT), HINT_EXPECTED_PART1_OUTPUT);
}

#[aoc(day3, part2, square_iteration)]
pub fn day3_part2_square_iteration(input: &str) -> usize {
    // OPT: Use ArrayVec for even more performance? Depends on max size.
    // OR OPT: Pre-allocating might be beneficial here, not sure how `size_hint` works for char
    // splits.
    let mut claims = ClaimIterator::new(input).map(|c| (c, true)).collect::<Vec<_>>();

    for i in 0..claims.len() {
        for j in i + 1..claims.len() {
            if claims[i].0.intersects(&claims[j].0) {
                (&mut claims[i]).1 = false;
                (&mut claims[j]).1 = false;
            }
        }
    }

    let uncontested = claims
        .into_iter()
        .filter_map(|(c, uncontested)| if uncontested {
            Some(c)
        } else {
            None
        })
        .collect::<Vec<_>>();
    if uncontested.len() != 1 {
        panic!("Expected single remaining claim, got {:?}", uncontested);
    }
    uncontested[0].id
}

#[test]
fn test_day3_part2_square_iteration_hint() {
    assert_eq!(day3_part2_square_iteration(HINT_INPUT), HINT_EXPECTED_PART2_OUTPUT);
}

#[test]
fn test_day3_part2_square_iteration_answer() {
    assert_eq!(day3_part2_square_iteration(INPUT), EXPECTED_PART2_OUTPUT);
}

#[aoc(day3, part2, grid_again)]
pub fn day3_part2_grid_again(input: &str) -> usize {
    let mut grid = GrowOnlyGrid::<u8>::new_with(1000, 1000, Default::default);
    let claims = ClaimIterator::new(input).collect::<Vec<_>>();
    for Claim {id: _, left, top, right, bottom} in claims.iter() {
        grid.grow_with(
            right.checked_add(1).unwrap(),
            bottom.checked_add(1).unwrap(),
            Default::default
        );

        for y in *top..*bottom {
            for x in *left..*right {
                *(&mut grid[(x, y)]) += 1;
            }
        }
    }

    let uncontested = claims
        .into_iter()
        .filter(|Claim { left, top, bottom, right, .. }| {
            for y in *top..*bottom {
                for x in *left..*right {
                    let count = grid[(x, y)];
                    assert!(count != 0);
                    if count > 1 {
                        return false;
                    }
                }
            }
            true
        })
        .collect::<Vec<_>>();
    assert_eq!(uncontested.len(), 1);
    uncontested[0].id
}

#[test]
fn test_day3_part2_grid_again_hint() {
    assert_eq!(day3_part2_grid_again(HINT_INPUT), HINT_EXPECTED_PART2_OUTPUT);
}

#[test]
fn test_day3_part2_grid_again_answer() {
    assert_eq!(day3_part2_grid_again(INPUT), EXPECTED_PART2_OUTPUT);
}

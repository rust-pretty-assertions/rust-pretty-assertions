// Adapted from:
// https://github.com/utkarshkukreti/diff.rs/blob/d362d6fb2ddab31fbbfeeccce8f6fb68f3f48c0f/src/lib.rs
// Credits Utkarsh Kukreti (MIT OR Apache-2.0 License)

#[cfg(all(feature = "alloc", not(feature = "std")))]
use alloc::vec;
#[cfg(all(feature = "alloc", not(feature = "std")))]
use alloc::vec::Vec;
#[cfg(feature = "std")]
use std::vec;
#[cfg(feature = "std")]
use std::vec::Vec;

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Result<T> {
    Left(T),
    Both(T, T),
    Right(T),
}

pub(crate) fn lines<'a>(left: &'a str, right: &'a str) -> Vec<Result<&'a str>> {
    let left_lines: Vec<&'a str> = left.lines().collect();
    let right_lines: Vec<&'a str> = right.lines().collect();
    let mut diff = do_diff(&left_lines, &right_lines, |line| *line);
    // str::lines() does not yield an empty str at the end if the str ends with
    // '\n'. Handle this special case by inserting one last diff item depending
    // on whether the left string ends with '\n', or the right one, or both.
    match (
        left.as_bytes().last().copied(),
        right.as_bytes().last().copied(),
    ) {
        (Some(b'\n'), Some(b'\n')) => {
            diff.push(Result::Both(&left[left.len()..], &right[right.len()..]))
        }
        (Some(b'\n'), _) => diff.push(Result::Left(&left[left.len()..])),
        (_, Some(b'\n')) => diff.push(Result::Right(&right[right.len()..])),
        _ => {}
    }
    diff
}

pub(crate) fn chars(left: &str, right: &str) -> Vec<Result<char>> {
    let left_chars: Vec<char> = left.chars().collect();
    let right_chars: Vec<char> = right.chars().collect();
    do_diff(&left_chars, &right_chars, |ch| *ch)
}

fn do_diff<'a, T, F, U>(left: &'a [T], right: &'a [T], mapper: F) -> Vec<Result<U>>
where
    T: PartialEq,
    F: Fn(&'a T) -> U,
{
    let leading_equals = left
        .iter()
        .zip(right.iter())
        .take_while(|(l, r)| l == r)
        .count();
    let trailing_equals = left[leading_equals..]
        .iter()
        .rev()
        .zip(right[leading_equals..].iter().rev())
        .take_while(|(l, r)| l == r)
        .count();

    let table: Vec2<u32> = {
        let left_diff_size = left.len() - leading_equals - trailing_equals;
        let right_diff_size = right.len() - leading_equals - trailing_equals;

        let mut table = Vec2::new(0, [left_diff_size + 1, right_diff_size + 1]);

        let left_skip = &left[leading_equals..left.len() - trailing_equals];
        let right_skip = &right[leading_equals..right.len() - trailing_equals];

        for (i, l) in left_skip.iter().enumerate() {
            for (j, r) in right_skip.iter().enumerate() {
                table.set(
                    [i + 1, j + 1],
                    if l == r {
                        table.get([i, j]) + 1
                    } else {
                        *table.get([i, j + 1]).max(table.get([i + 1, j]))
                    },
                );
            }
        }

        table
    };

    let mut diff = Vec::with_capacity(left.len().max(right.len()));

    diff.extend(
        left[..leading_equals]
            .iter()
            .zip(&right[..leading_equals])
            .map(|(l, r)| Result::Both(mapper(l), mapper(r))),
    );

    {
        let start = diff.len();
        let mut i = table.len[0] - 1;
        let mut j = table.len[1] - 1;
        let left = &left[leading_equals..];
        let right = &right[leading_equals..];

        loop {
            if j > 0 && (i == 0 || table.get([i, j]) == table.get([i, j - 1])) {
                j -= 1;
                diff.push(Result::Right(mapper(&right[j])));
            } else if i > 0 && (j == 0 || table.get([i, j]) == table.get([i - 1, j])) {
                i -= 1;
                diff.push(Result::Left(mapper(&left[i])));
            } else if i > 0 && j > 0 {
                i -= 1;
                j -= 1;
                diff.push(Result::Both(mapper(&left[i]), mapper(&right[j])));
            } else {
                break;
            }
        }
        diff[start..].reverse();
    }

    diff.extend(
        left[left.len() - trailing_equals..]
            .iter()
            .zip(&right[right.len() - trailing_equals..])
            .map(|(l, r)| Result::Both(mapper(l), mapper(r))),
    );

    diff
}

struct Vec2<T> {
    len: [usize; 2],
    data: Vec<T>,
}

impl<T> Vec2<T> {
    #[inline]
    fn new(value: T, len: [usize; 2]) -> Self
    where
        T: Clone,
    {
        Vec2 {
            len,
            data: vec![value; len[0] * len[1]],
        }
    }

    #[inline]
    fn get(&self, index: [usize; 2]) -> &T {
        debug_assert!(index[0] < self.len[0]);
        debug_assert!(index[1] < self.len[1]);
        &self.data[index[0] * self.len[1] + index[1]]
    }

    #[inline]
    fn set(&mut self, index: [usize; 2], value: T) {
        debug_assert!(index[0] < self.len[0]);
        debug_assert!(index[1] < self.len[1]);
        self.data[index[0] * self.len[1] + index[1]] = value;
    }
}

#[cfg(test)]
mod test {
    use super::{chars, lines, Result};
    #[cfg(all(feature = "alloc", not(feature = "std")))]
    use alloc::vec;
    #[cfg(feature = "std")]
    use std::vec;

    #[test]
    fn diff_lines_handles_changes_and_trailing_newlines() {
        assert_eq!(
            lines("alpha\nbeta\ngamma\n", "alpha\ndelta\ngamma\n"),
            vec![
                Result::Both("alpha", "alpha"),
                Result::Left("beta"),
                Result::Right("delta"),
                Result::Both("gamma", "gamma"),
                Result::Both("", ""),
            ]
        );
        assert_eq!(
            lines("a\n", "a"),
            vec![Result::Both("a", "a"), Result::Left("")]
        );
        assert_eq!(
            lines("a", "a\n"),
            vec![Result::Both("a", "a"), Result::Right("")]
        );
    }

    #[test]
    fn diff_chars_handles_unicode() {
        assert_eq!(
            chars("héllo", "hällo"),
            vec![
                Result::Both('h', 'h'),
                Result::Left('é'),
                Result::Right('ä'),
                Result::Both('l', 'l'),
                Result::Both('l', 'l'),
                Result::Both('o', 'o'),
            ]
        );
    }
}

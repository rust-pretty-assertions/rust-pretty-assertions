extern crate difference;
extern crate quickcheck;

use difference::{Changeset, Difference};
use quickcheck::{TestResult, quickcheck, QuickCheck};
use std::fmt;

const DEBUG: bool = false;

struct Check<'a> {
    old: &'a str,
    new: &'a str,
    changeset: Changeset,
}

impl<'a> fmt::Display for Check<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Changeset:\
                 \n    - old:   {:?}\
                 \n    - new:   {:?}\
                 \n    - split: {:?}\
                 \n    - diff: [",
            self.old,
            self.new,
            self.changeset.split)?;

        let mut iter = self.changeset.diffs.iter();
        if let Some(d) = iter.next() { write!(f, "{:?}", d)?; }
        for d in iter { write!(f, "\n             {:?}", d)?; }
        write!(f, "]\n")
    }
}

fn check_changeset(old: &str, new: &str, split: &str) -> TestResult {
    Check::new(old, new, split).check()
}

impl<'a> Check<'a> {
    fn new(old: &'a str, new: &'a str, split: &'a str) -> Check<'a> {
        Check {
            old: old,
            new: new,
            changeset: Changeset::new(old, new, split),
        }
    }

    fn check(&self) -> TestResult {
        let split = &self.changeset.split;

        let mut old = self.old.trim_left_matches(split);
        let mut new = self.new.trim_left_matches(split);

        macro_rules! expect {
            ($ident:ident, $pattern:expr, $diff:expr, $split:expr) => {
                if !$ident.starts_with($pattern) {
                    return TestResult::error(format!("`{:?}` does not match `{}` at {:?} for {}",
                                                      $diff,
                                                      stringify!($ident),
                                                      $ident,
                                                      self));
                }
                $ident = &$ident[$pattern.len()..].trim_left_matches($split);
            }
        }

        for d in &self.changeset.diffs {
            if DEBUG {
                println!("assert `{:?}` (old: {:?}, new: {:?})", d, old, new);
            }

            match *d {
                Difference::Same(ref x) => {
                    expect!(old, x, d, split);
                    expect!(new, x, d, split);
                }
                Difference::Add(ref x) => {
                    expect!(new, x, d, split);
                }
                Difference::Rem(ref x) => {
                    expect!(old, x, d, split);
                }
            }
        }
        if !old.is_empty() {
            return TestResult::error(format!("expected end of string in `old` at {:?} for {}",
                                              old, self))
        }
        if !new.is_empty() {
            return TestResult::error(format!("expected end of string in `new` at {:?} for {}",
                                              new, self))
        }
        TestResult::passed()
    }
}

#[test]
fn simple() {
    quickcheck(check_changeset("a", "a a", " "));
}

#[test]
fn issue_19a() {
    // this should work but it doesn't
    // https://github.com/johannhof/difference.rs/issues/19
    quickcheck(check_changeset("a b : g",
                               "b a : b b : g g",
                               " "));
}

#[test]
fn issue_19b() {
    // this should work but it doesn't
    // https://github.com/johannhof/difference.rs/issues/19
    quickcheck(check_changeset("a > : b b : g",
                               "a : > c : b b : g g",
                               " "));
}

#[test]
fn issue_19c() {
    let old = "Tokens(\"# [ derive ( Default , Clone ) ] pub struct FooBuilder < \'a , T > { foo : u32 , } # [ allow ( dead_code ) ] impl < \'a , T : \'a + Default > FooBuilder < \'a , T > { fn bar ( ) -> { unimplemented ! ( ) } }\")";

    let new = "Tokens(\"# [ derive ( Default , Clone ) ] pub struct FooBuilder < \'a , T : \'a + Default + Clone > { foo : u32 , } # [ allow ( dead_code ) ] impl < \'a , T : \'a + Default + Clone > FooBuilder < \'a , T > { fn bar ( ) -> { unimplemented ! ( ) } }\")";

    quickcheck(check_changeset(old, new, " "));
}

#[test]
fn fuzzy() {
    fn prop(old: Vec<usize>, new: Vec<usize>, words: Vec<char>) -> TestResult {
        if words.is_empty() {
            return TestResult::discard()
        }

        fn map_to_words(input: &[usize], words: &[char]) -> String {
            input.iter()
                .enumerate()
                .fold(String::new(), |mut acc, (i, x)| {
                    if i > 0 {
                        acc.push(' ');
                    }
                    acc.push(words[x % words.len()]);
                    acc
                })
        }
        let old = map_to_words(&old, &words);
        let new = map_to_words(&new, &words);

        check_changeset(&old, &new, " ")
    }

    QuickCheck::new()
        .tests(100) // max successful tests
        .max_tests(10000) // max attempts
        .quickcheck(prop as fn(Vec<usize>, Vec<usize>, Vec<char>) -> TestResult);
}

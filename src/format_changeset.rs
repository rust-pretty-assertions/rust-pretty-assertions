use difference::{Difference, Changeset};
use std::fmt;
use ansi_term::Colour::{Red, Green, Black};
use ansi_term::Style;

macro_rules! paint {
    ($f: ident, $colour: expr, $fmt: expr, $($args:tt)*) => (
        write!($f, "{}", $colour.paint(format!($fmt, $($args)*)))
    )
}

const SIGN_RIGHT: char = '+';
const SIGN_LEFT: char = '-';

// Adapted from:
// https://github.com/johannhof/difference.rs/blob/c5749ad7d82aa3d480c15cb61af9f6baa08f116f/examples/github-style.rs
// Credits johannhof (MIT License)

pub fn format_changeset(f: &mut fmt::Formatter, changeset: &Changeset) -> fmt::Result {
    let ref diffs = changeset.diffs;

    writeln!(f, "{} ({} / {}):",
             Style::new().bold().paint("Diff"),
             Red.paint(format!("{} left", SIGN_LEFT)),
             Green.paint(format!("{} right", SIGN_RIGHT)))?;
    for i in 0..diffs.len() {
        match diffs[i] {
            Difference::Same(ref same) => {
                // Have to split line by line in order to have the extra whitespace
                // at the beginning.
                for line in same.split("\n") {
                    writeln!(f, " {}", line)?;
                }
            }
            Difference::Add(ref added) => {
                match diffs.get(i - 1) {
                    Some(&Difference::Rem(ref removed)) => {
                        // The addition is preceded by an removal.
                        //
                        // Let's highlight the character-differences in this replaced
                        // chunk. Note that this chunk can span over multiple lines.
                        let Changeset { diffs, .. } = Changeset::new(removed, added, "");

                        // LEFT side (==what's been)
                        paint!(f, Red, "{}", SIGN_LEFT)?;
                        for c in &diffs {
                            match *c {
                                Difference::Same(ref word_diff) => {
                                    let s = word_diff.replace("\n", &format!("\n{}", SIGN_LEFT));
                                    paint!(f, Red, "{}", s)?;
                                }
                                Difference::Rem(ref word_diff) => {
                                    let s = word_diff.replace("\n", &format!("\n{}", SIGN_LEFT));
                                    paint!(f, Black.on(Red), "{}", s)?;
                                }
                                _ => (),
                            }
                        }
                        writeln!(f, "")?;

                        // RIGHT side (==what's new)
                        paint!(f, Green, "{}", SIGN_RIGHT)?;
                        for c in &diffs {
                            match *c {
                                Difference::Same(ref word_diff) => {
                                    let s = word_diff.replace("\n", &format!("\n{}", SIGN_RIGHT));
                                    paint!(f, Green, "{}", s)?;
                                }
                                Difference::Add(ref word_diff) => {
                                    let s = word_diff.replace("\n", &format!("\n{}", SIGN_RIGHT));
                                    paint!(f, Black.on(Green), "{}", s)?;
                                }
                                _ => (),
                            }
                        }
                        writeln!(f, "")?;
                    }
                    _ => {
                        for line in added.split("\n") {
                            paint!(f, Green, "{}{}\n", SIGN_RIGHT, line)?;
                        }
                    }
                };
            }
            Difference::Rem(ref removed) => {
                match diffs.get(i+1) {
                    Some(&Difference::Add(_)) => {
                        // The removal is followed by an addition.
                        //
                        // ... we'll handle both in the next iteration.
                    }
                    _ => {
                        for line in removed.split("\n") {
                            paint!(f, Red, "{}{}\n", SIGN_LEFT, line)?;
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

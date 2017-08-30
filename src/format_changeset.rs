use difference::{Difference, Changeset};
use std::fmt;
use ansi_term::Colour::{Red, Green, Black};
use ansi_term::Style;

macro_rules! paint {
    ($f: ident, $colour: ident, $fmt: expr, $($args:tt)*) => (
        write!($f, "{}", $colour.paint(format!($fmt, $($args)*)))
    )
}


// Adapted from:
// https://github.com/johannhof/difference.rs/blob/c5749ad7d82aa3d480c15cb61af9f6baa08f116f/examples/github-style.rs
// Credits johannhof (MIT License)

pub fn format_changeset(f: &mut fmt::Formatter, changeset: &Changeset) -> fmt::Result {
    let ref diffs = changeset.diffs;

    writeln!(f, "{} ({} / {}):",
             Style::new().bold().paint("Diff"),
             Red.paint("- left"),
             Green.paint("+ right"))?;
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

                        // what's been
                        write!(f, "{}", Red.paint("-"))?;
                        for c in &diffs {
                            match *c {
                                Difference::Same(ref word_diff) => {
                                    paint!(f, Red, "{}", word_diff.replace("\n", "\n-"))?;
                                }
                                Difference::Rem(ref word_diff) => {
                                    let formatted_str = Style::new()
                                        .fg(Black)
                                        .on(Red)
                                        .paint(word_diff.replace("\n", "\n-").to_string());

                                    write!(f, "{}", formatted_str)?;
                                }
                                _ => (),
                            }
                        }
                        writeln!(f, "")?;

                        // what's new
                        write!(f, "{}", Green.paint("+"))?;
                        for c in &diffs {
                            match *c {
                                Difference::Same(ref word_diff) => {
                                    paint!(f, Green, "{}", word_diff.replace("\n", "\n+"))?;
                                }
                                Difference::Add(ref word_diff) => {
                                    let formatted_str = Style::new()
                                        .fg(Black)
                                        .on(Green)
                                        .paint(word_diff.replace("\n", "\n+").to_string());

                                    write!(f, "{}", formatted_str)?;
                                }
                                _ => (),
                            }
                        }
                        writeln!(f, "")?;
                    }
                    _ => {
                        for line in added.split("\n") {
                            paint!(f, Green, "+{}\n", line)?;
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
                            paint!(f, Red, "-{}\n", line)?;
                        }
                    }
                }
            }
        }
    }
    writeln!(f, "")
}

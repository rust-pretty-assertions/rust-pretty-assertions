use difference::{Difference, Changeset};
use std::fmt;
use ansi_term::Colour::{Red, Green};
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
            Difference::Same(ref diff) => {
                // Have to split line by line in order to have the extra whitespace
                // at the beginning.
                for line in diff.split("\n") {
                    writeln!(f, " {}", line)?;
                }
            }
            Difference::Add(ref diff) => {
                match diffs[i - 1] {
                    Difference::Rem(ref prev_diff) => {
                        write!(f, "{}", Green.paint("+"))?;
                        let Changeset { diffs, .. } = Changeset::new(prev_diff, diff, "");
                        for c in diffs {
                            match c {
                                Difference::Same(ref word_diff) => {
                                    paint!(f, Green, "{}", word_diff.replace("\n", "\n+"))?;
                                }
                                Difference::Add(ref word_diff) => {
                                    let formatted_str = Style::new()
                                        .fg(Green)
                                        .underline()
                                        .paint(word_diff.replace("\n", "\n+").to_string());

                                    write!(f, "{}", formatted_str)?;
                                }
                                _ => (),
                            }
                        }
                        writeln!(f, "")?;
                    }
                    _ => {
                        paint!(f, Green, "+{}", diff)?;
                        writeln!(f, "")?;
                    }
                };
            }
            Difference::Rem(ref diff) => {
                for line in diff.split("\n") {
                    paint!(f, Red, "-{}\n", line)?;
                }
            }
        }
    }
    writeln!(f, "")
}

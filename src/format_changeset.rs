use ansi_term::{
    Colour::{Fixed, Green, Red},
    Style,
};
use diffus::{
    edit::{self, collection, string},
    Diffable,
};
use std::fmt;

macro_rules! paint {
    ($f:expr, $colour:expr, $fmt:expr, $($args:tt)*) => (
        write!($f, "{}", $colour.paint(format!($fmt, $($args)*)))
    )
}

const SIGN_RIGHT: char = '>'; // + > →
const SIGN_LEFT: char = '<'; // - < ←

// Adapted from:
// https://github.com/johannhof/difference.rs/blob/c5749ad7d82aa3d480c15cb61af9f6baa08f116f/examples/github-style.rs
// Credits johannhof (MIT License)

/// Present the diff output for two mutliline strings in a pretty, colorised manner.
pub(crate) fn format_changeset(f: &mut fmt::Formatter, left: &str, right: &str) -> fmt::Result {
    let left: Vec<&str> = left.split("\n").collect();
    let right: Vec<&str> = right.split("\n").collect();
    let diff = left.diff(&right);

    writeln!(
        f,
        "{} {} / {} :",
        Style::new().bold().paint("Diff"),
        Red.paint(format!("{} left", SIGN_LEFT)),
        Green.paint(format!("right {}", SIGN_RIGHT))
    )?;

    match diff {
        edit::Edit::Copy(lines) => {
            // If the whole debug string is the same, just pad and print one of them
            for line in lines {
                writeln!(f, " {}", line)?;
            }
        }
        edit::Edit::Change(diffs) => {
            // Otherwise, format and write the changes we are given
            format_change_diffs(f, diffs)?;
        }
    };
    Ok(())
}

/// Internal details of how we format the `diffus` changes given to us.
fn format_change_diffs(
    f: &mut fmt::Formatter,
    diffs: Vec<collection::Edit<'_, &str, Vec<string::Edit>>>,
) -> fmt::Result {
    for i in 0..diffs.len() {
        match diffs[i] {
            collection::Edit::Copy(ref same) => {
                writeln!(f, " {}", same)?;
            }
            collection::Edit::Insert(ref added) => {
                let prev = i.checked_sub(1).and_then(|x| diffs.get(x));
                match prev {
                    Some(&collection::Edit::Remove(ref removed)) => {
                        // The addition is preceded by an removal.
                        //
                        // Let's highlight the character-differences in this replaced
                        // chunk. Note that this chunk can span over multiple lines.
                        format_replacement(f, added, removed)?;
                    }
                    _ => {
                        paint!(f, Green, "{}{}", SIGN_RIGHT, added)?;
                        // write trailing newline outide of styling
                        writeln!(f)?;
                    }
                };
            }
            collection::Edit::Remove(ref removed) => {
                let next = i.checked_add(1).and_then(|x| diffs.get(x));
                match next {
                    Some(&collection::Edit::Insert(_)) => {
                        // The removal is followed by an addition.
                        //
                        // ... we'll handle both in the next iteration.
                    }
                    _ => {
                        paint!(f, Red, "{}{}", SIGN_LEFT, removed)?;
                        // write trailing newline outide of styling
                        writeln!(f)?;
                    }
                }
            }
            collection::Edit::Change(..) => {
                panic!("`Change`s only occur for nested `struct`s, which Vec<&str> is not.")
            }
        }
    }
    Ok(())
}

/// Group character styling for an inline diff, to prevent wrapping each single
/// character in terminal styling codes.
///
/// Styles are applied automatically each time a new style is given in `write_with_style`.
struct InlineWriter<'a, Writer> {
    f: &'a mut Writer,
    style: Style,
}

impl<'a, Writer> InlineWriter<'a, Writer>
where
    Writer: fmt::Write,
{
    fn new(f: &'a mut Writer) -> Self {
        InlineWriter {
            f,
            style: Style::new(),
        }
    }

    /// Push a new character into the buffer, specifying the style it should be written in.
    fn write_with_style(&mut self, c: char, style: Style) -> fmt::Result {
        // If the style is the same as previously, just write character
        if style == self.style {
            write!(self.f, "{}", c)?;
        } else {
            // Close out previous style
            write!(self.f, "{}", self.style.suffix())?;

            // Store new style and start writing it
            write!(self.f, "{}{}", style.prefix(), c)?;
            self.style = style;
        }
        Ok(())
    }

    /// Push a new character into the buffer, specifying the style it should be written in.
    fn finish_line(&mut self) -> fmt::Result {
        // Close out previous style
        write!(self.f, "{}\n", self.style.suffix())?;
        self.style = Default::default();
        Ok(())
    }
}

/// Format a single line to show an inline diff of the two strings given.
///
/// The given strings should be the output of a line diff, i.e. should contain no newline `\n`
/// characters.
///
/// The output of this function will be two lines, each with a trailing newline.
fn format_replacement<TWrite: fmt::Write>(
    f: &mut TWrite,
    added: &str,
    removed: &str,
) -> fmt::Result {
    let diff = removed.diff(added);
    match diff {
        edit::Edit::Copy(_) => {
            // If for some reason we get two strings the same, just plain print them
            writeln!(f, " {}", added)?;
        }
        edit::Edit::Change(diffs) => {
            // LEFT side (==what's been)
            let light = Red.into();
            let heavy = Red.on(Fixed(52)).bold();
            let mut writer = InlineWriter::new(f);
            writer.write_with_style(SIGN_LEFT, light)?;
            for c in &diffs {
                match *c {
                    string::Edit::Copy(c) => writer.write_with_style(c, light)?,
                    string::Edit::Remove(c) => writer.write_with_style(c, heavy)?,
                    _ => (),
                }
            }
            writer.finish_line()?;

            // RIGHT side (==what's new)
            let light = Green.into();
            let heavy = Green.on(Fixed(22)).bold();
            writer.write_with_style(SIGN_RIGHT, light)?;
            for c in &diffs {
                match *c {
                    string::Edit::Copy(c) => writer.write_with_style(c, light)?,
                    string::Edit::Insert(c) => writer.write_with_style(c, heavy)?,
                    _ => (),
                }
            }
            writer.finish_line()?;
        }
    }
    Ok(())
}

#[test]
fn test_format_replacement() {
    let added = "    84,";
    let removed = "    128,";

    let mut buf = String::new();
    let _ = format_replacement(&mut buf, added, removed);

    println!(
        "## removed ##\
         \n{}\
         \n## added ##\
         \n{}\
         \n## diff ##\
         \n{}",
        removed, added, buf
    );

    let red_light = "\u{1b}[31m";
    let green_light = "\u{1b}[32m";
    let red_heavy = "\u{1b}[1;48;5;52;31m";
    let green_heavy = "\u{1b}[1;48;5;22;32m";
    let reset = "\u{1b}[0m";

    assert_eq!(
        buf,
        format!(
            "{red_light}<    {reset}{red_heavy}12{reset}{red_light}8,{reset}\n{green_light}>    8{reset}{green_heavy}4{reset}{green_light},{reset}\n",
            red_light=red_light,
            green_light=green_light,
            red_heavy=red_heavy,
            green_heavy=green_heavy,
            reset=reset,
        ),
    );
}

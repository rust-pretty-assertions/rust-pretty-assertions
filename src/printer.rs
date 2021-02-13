use ansi_term::{
    Colour::{Fixed, Green, Red},
    Style,
};
use std::fmt;

macro_rules! paint {
    ($f:expr, $colour:expr, $fmt:expr, $($args:tt)*) => (
        write!($f, "{}", $colour.paint(format!($fmt, $($args)*)))
    )
}

const SIGN_RIGHT: char = '>'; // + > →
const SIGN_LEFT: char = '<'; // - < ←

/// Present the diff output for two mutliline strings in a pretty, colorised manner.
pub(crate) fn write_header(f: &mut fmt::Formatter) -> fmt::Result {
    writeln!(
        f,
        "{} {} / {} :",
        Style::new().bold().paint("Diff"),
        Red.paint(format!("{} left", SIGN_LEFT)),
        Green.paint(format!("right {}", SIGN_RIGHT))
    )
}

/// Delay formatting this deleted chunk until later.
///
/// It can be formatted as a whole chunk by calling `flush`, or the inner value
/// obtained with `take` for further processing.
#[derive(Default)]
struct LatentDeletion<'a> {
    value: Option<&'a str>,
}

impl<'a> LatentDeletion<'a> {
    /// Set the chunk value.
    fn set(&mut self, value: &'a str) {
        self.value = Some(value);
    }

    /// Take the underlying chunk value.
    fn take(&mut self) -> Option<&'a str> {
        self.value.take()
    }

    /// If a value is set, print it as a whole chunk, using the given formatter.
    ///
    /// Resets the internal state to default.
    fn flush<TWrite: fmt::Write>(&mut self, f: &mut TWrite) -> fmt::Result {
        if let Some(value) = self.value {
            paint!(f, Red, "{}{}", SIGN_LEFT, value)?;
            writeln!(f)?;
        }
        self.value = None;
        Ok(())
    }
}

// Adapted from:
// https://github.com/johannhof/difference.rs/blob/c5749ad7d82aa3d480c15cb61af9f6baa08f116f/examples/github-style.rs
// Credits johannhof (MIT License)

/// Present the diff output for two mutliline strings in a pretty, colorised manner.
pub(crate) fn write_lines<TWrite: fmt::Write>(
    f: &mut TWrite,
    left: &str,
    right: &str,
) -> fmt::Result {
    let diff = ::diff::lines(left, right);

    // Keep track of if the previous chunk in the iteration was a deletion.
    //
    // We defer writing all deletions to the subsequent loop, to find out if
    // we need to write a character-level diff instead.
    let mut previous_deletion = LatentDeletion::default();

    for change in diff.into_iter() {
        match change {
            ::diff::Result::Both(value, _) => {
                // Handle the previous deletion, if it exists
                previous_deletion.flush(f)?;

                // Print this line with a space at the front to preserve indentation.
                writeln!(f, " {}", value)?;
            }
            ::diff::Result::Right(inserted) => {
                if let Some(deleted) = previous_deletion.take() {
                    // The insertion is preceded by an deletion.
                    //
                    // Let's highlight the character-differences in this replaced
                    // chunk. Note that this chunk can span over multiple lines.
                    write_inline_diff(f, deleted, inserted)?;
                } else {
                    paint!(f, Green, "{}{}", SIGN_RIGHT, inserted)?;
                    writeln!(f)?;
                }
            }
            ::diff::Result::Left(deleted) => {
                // Handle the previous deletion, if it exists
                previous_deletion.flush(f)?;

                // If we get a deletion, defer writing it to the next loop
                // as we need to know what the next tag is.
                previous_deletion.set(deleted);
            }
        }
    }

    // Handle the previous deletion, if it exists
    previous_deletion.flush(f)?;

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
    fn write_with_style(&mut self, c: &char, style: Style) -> fmt::Result {
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

    /// Finish any existing style and reset to default state.
    fn finish(&mut self) -> fmt::Result {
        // Close out previous style
        writeln!(self.f, "{}", self.style.suffix())?;
        self.style = Default::default();
        Ok(())
    }
}

/// Format a single line to show an inline diff of the two strings given.
///
/// The given strings should not have a trailing newline.
///
/// The output of this function will be two lines, each with a trailing newline.
fn write_inline_diff<TWrite: fmt::Write>(f: &mut TWrite, left: &str, right: &str) -> fmt::Result {
    let diff = ::diff::chars(left, right);
    let mut writer = InlineWriter::new(f);

    // Print the left string on one line, with differences highlighted
    let light = Red.into();
    let heavy = Red.on(Fixed(52)).bold();
    writer.write_with_style(&SIGN_LEFT, light)?;
    for change in diff.iter() {
        match change {
            ::diff::Result::Both(value, _) => writer.write_with_style(value, light)?,
            ::diff::Result::Left(value) => writer.write_with_style(value, heavy)?,
            _ => (),
        }
    }
    writer.finish()?;

    // Print the right string on one line, with differences highlighted
    let light = Green.into();
    let heavy = Green.on(Fixed(22)).bold();
    writer.write_with_style(&SIGN_RIGHT, light)?;
    for change in diff.iter() {
        match change {
            ::diff::Result::Both(value, _) => writer.write_with_style(value, light)?,
            ::diff::Result::Right(value) => writer.write_with_style(value, heavy)?,
            _ => (),
        }
    }
    writer.finish()
}

#[cfg(test)]
mod test {
    use super::*;

    // ANSI terminal codes used in our outputs.
    //
    // Interpolate these into test strings to make expected values easier to read.
    const RED_LIGHT: &str = "\u{1b}[31m";
    const GREEN_LIGHT: &str = "\u{1b}[32m";
    const RED_HEAVY: &str = "\u{1b}[1;48;5;52;31m";
    const GREEN_HEAVY: &str = "\u{1b}[1;48;5;22;32m";
    const RESET: &str = "\u{1b}[0m";

    /// Given that both of our diff printing functions have the same
    /// type signature, we can reuse the same test code for them.
    ///
    /// This could probably be nicer with traits!
    fn check_printer<TPrint>(printer: TPrint, left: &str, right: &str, expected: &str)
    where
        TPrint: Fn(&mut String, &str, &str) -> fmt::Result,
    {
        let mut actual = String::new();
        printer(&mut actual, left, right).expect("printer function failed");

        println!(
            "## left ##\n\
             {}\n\
             ## right ##\n\
             {}\n\
             ## actual diff ##\n\
             {}\n\
             ## expected diff ##\n\
             {}",
            left, right, actual, expected
        );
        assert_eq!(actual, expected);
    }

    #[test]
    fn write_inline_diff_empty() {
        let left = "";
        let right = "";
        let expected = format!(
            "{red_light}<{reset}\n\
             {green_light}>{reset}\n",
            red_light = RED_LIGHT,
            green_light = GREEN_LIGHT,
            reset = RESET,
        );

        check_printer(write_inline_diff, left, right, &expected);
    }

    #[test]
    fn write_inline_diff_added() {
        let left = "";
        let right = "polymerase";
        let expected = format!(
            "{red_light}<{reset}\n\
             {green_light}>{reset}{green_heavy}polymerase{reset}\n",
            red_light = RED_LIGHT,
            green_light = GREEN_LIGHT,
            green_heavy = GREEN_HEAVY,
            reset = RESET,
        );

        check_printer(write_inline_diff, left, right, &expected);
    }

    #[test]
    fn write_inline_diff_removed() {
        let left = "polyacrylamide";
        let right = "";
        let expected = format!(
            "{red_light}<{reset}{red_heavy}polyacrylamide{reset}\n\
             {green_light}>{reset}\n",
            red_light = RED_LIGHT,
            green_light = GREEN_LIGHT,
            red_heavy = RED_HEAVY,
            reset = RESET,
        );

        check_printer(write_inline_diff, left, right, &expected);
    }

    #[test]
    fn write_inline_diff_changed() {
        let left = "polymerase";
        let right = "polyacrylamide";
        let expected = format!(
            "{red_light}<poly{reset}{red_heavy}me{reset}{red_light}ra{reset}{red_heavy}s{reset}{red_light}e{reset}\n\
             {green_light}>poly{reset}{green_heavy}ac{reset}{green_light}r{reset}{green_heavy}yl{reset}{green_light}a{reset}{green_heavy}mid{reset}{green_light}e{reset}\n",
            red_light = RED_LIGHT,
            green_light = GREEN_LIGHT,
            red_heavy = RED_HEAVY,
            green_heavy = GREEN_HEAVY,
            reset = RESET,
        );

        check_printer(write_inline_diff, left, right, &expected);
    }

    /// If one of our strings is empty, it should not be shown at all in the output.
    #[test]
    fn write_lines_empty_string() {
        let left = "";
        let right = "content";
        let expected = format!(
            "{green_light}>content{reset}\n",
            green_light = GREEN_LIGHT,
            reset = RESET,
        );

        check_printer(write_lines, left, right, &expected);
    }

    /// Realistic multiline struct diffing case.
    #[test]
    fn write_lines_struct() {
        let left = r#"Some(
    Foo {
        lorem: "Hello World!",
        ipsum: 42,
        dolor: Ok(
            "hey",
        ),
    },
)"#;
        let right = r#"Some(
    Foo {
        lorem: "Hello Wrold!",
        ipsum: 42,
        dolor: Ok(
            "hey ho!",
        ),
    },
)"#;
        let expected = format!(
            r#" Some(
     Foo {{
{red_light}<        lorem: "Hello W{reset}{red_heavy}o{reset}{red_light}rld!",{reset}
{green_light}>        lorem: "Hello Wr{reset}{green_heavy}o{reset}{green_light}ld!",{reset}
         ipsum: 42,
         dolor: Ok(
{red_light}<            "hey",{reset}
{green_light}>            "hey{reset}{green_heavy} ho!{reset}{green_light}",{reset}
         ),
     }},
 )
"#,
            red_light = RED_LIGHT,
            red_heavy = RED_HEAVY,
            green_light = GREEN_LIGHT,
            green_heavy = GREEN_HEAVY,
            reset = RESET,
        );

        check_printer(write_lines, left, right, &expected);
    }

    /// Regression test for multiline highlighting issue
    #[test]
    fn write_lines_issue12() {
        let left = r#"[
    0,
    0,
    0,
    128,
    10,
    191,
    5,
    64,
]"#;
        let right = r#"[
    84,
    248,
    45,
    64,
]"#;
        let expected = format!(
            r#" [
{red_light}<    0,{reset}
{red_light}<    0,{reset}
{red_light}<    0,{reset}
{red_light}<    128,{reset}
{red_light}<    10,{reset}
{red_light}<    191,{reset}
{red_light}<    {reset}{red_heavy}5{reset}{red_light},{reset}
{green_light}>    {reset}{green_heavy}84{reset}{green_light},{reset}
{green_light}>    248,{reset}
{green_light}>    45,{reset}
     64,
 ]
"#,
            red_light = RED_LIGHT,
            red_heavy = RED_HEAVY,
            green_light = GREEN_LIGHT,
            green_heavy = GREEN_HEAVY,
            reset = RESET,
        );

        check_printer(write_lines, left, right, &expected);
    }
}

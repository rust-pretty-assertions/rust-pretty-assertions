/// Symbols used to indicate removed and added lines.
#[derive(Clone, Copy)]
pub enum LineSymbol {
    /// Use '<' and '>'
    Arrow,
    /// Use '-' and '+'
    Sign,
}

impl Default for LineSymbol {
    fn default() -> Self {
        Self::Arrow
    }
}

/// Configuration object to pass to supported macros with `assert_eq!(config = config, ...)`
#[derive(Clone, Copy, Default)]
pub struct Config {
    _private: (),
    pub(crate) line_symbol: LineSymbol,
}

impl Config {
    /// Set the symbols used to indicate removed and added lines.
    pub fn line_symbol(mut self, value: LineSymbol) -> Self {
        self.line_symbol = value;
        self
    }
}

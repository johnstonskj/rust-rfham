//! Custom formatting options for configuration data.
//!

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum OutputKind {
    #[default]
    MarkdownList,
    MarkdownTable,
    Toml,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FormatterOptions {
    kind: OutputKind,
    nesting_depth: u16,
    flag_as_default: bool,
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for FormatterOptions {
    fn default() -> Self {
        Self {
            kind: OutputKind::MarkdownList,
            nesting_depth: 1,
            flag_as_default: false,
        }
    }
}
impl FormatterOptions {
    pub const fn with_output_kind(mut self, kind: OutputKind) -> Self {
        self.kind = kind;
        self
    }

    pub const fn with_nesting_depth(mut self, depth: u16) -> Self {
        self.nesting_depth = depth;
        self
    }

    pub const fn with_additional_depth(mut self, depth: u16) -> Self {
        self.nesting_depth += depth;
        self
    }

    pub const fn with_flag_as_default(mut self, flag_as_default: bool) -> Self {
        self.flag_as_default = flag_as_default;
        self
    }

    pub const fn output_kind(&self) -> OutputKind {
        self.kind
    }

    pub const fn nesting_depth(&self) -> u16 {
        self.nesting_depth
    }

    pub const fn is_default(&self) -> bool {
        self.flag_as_default
    }
}

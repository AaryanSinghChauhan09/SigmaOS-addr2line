//! Command-line options for addr2line.

/// Operating mode
#[derive(Debug, Clone, PartialEq)]
pub enum Mode {
    /// Default mode
    Default,
    /// Demangle symbol names
    Demangle,
    /// Show inline frames
    Inlines,
    /// Show addresses
    Addresses,
    /// Show functions only
    Functions,
}

/// Command-line options
#[derive(Debug, Clone)]
pub struct Options {
    /// Operating mode
    pub mode: Mode,
    /// Binary file path
    pub binary: String,
    /// Addresses to resolve
    pub addresses: Vec<u64>,
    /// Output file
    pub output_file: String,
    /// Verbose output
    pub verbose: bool,
    /// Quiet mode
    pub quiet: bool,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            mode: Mode::Default,
            binary: String::new(),
            addresses: vec![],
            output_file: String::new(),
            verbose: false,
            quiet: false,
        }
    }
}

//! Options model.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Default,
    Demangle,
    Inlines,
    Addresses,
    Functions,
}

#[derive(Debug, Clone)]
pub struct Addr2lineOptions {
    pub mode: Mode,
    pub binary: Option<String>,
}

impl Default for Addr2lineOptions {
    fn default() -> Self { Self { mode: Mode::Default, binary: None } }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn opts() { assert_eq!(Addr2lineOptions::default().mode, Mode::Default); }
}

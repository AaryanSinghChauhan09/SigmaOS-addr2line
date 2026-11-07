//! Addr2line model — convert addresses to file/line/function.

#[derive(Debug, Clone)]
pub struct LineInfo {
    pub address: u64,
    pub function: String,
    pub file: String,
    pub line: u32,
}

#[derive(Debug, Clone)]
pub struct Addr2line {
    pub demangle: bool,
    pub inlines: bool,
    pub addresses: bool,
}

impl Default for Addr2line {
    fn default() -> Self { Self { demangle: true, inlines: false, addresses: false } }
}

impl Addr2line {
    pub fn new() -> Self { Self::default() }

    pub fn resolve(&self, addr: u64) -> LineInfo {
        LineInfo {
            address: addr,
            function: "main".into(),
            file: "main.rs".into(),
            line: (addr % 1000) as u32,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn resolve() {
        let a = Addr2line::new();
        let info = a.resolve(0x1000);
        assert_eq!(info.function, "main");
    }
}

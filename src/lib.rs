//! SigmaOS-addr2line — comprehensive address-to-line debugging tool.
//!
//! Converts addresses to line information using debug symbols,
//! supporting function resolution, inline frames, and address ranges.

pub mod symbol;
pub mod debug_info;
pub mod resolver;
pub mod options;

pub use symbol::{Symbol, SymbolType, SymbolTable};
pub use debug_info::{DebugInfo, LineEntry, FunctionEntry, InlineFrame};
pub use resolver::{Addr2lineResolver, ResolverOptions};
pub use options::{Mode, Options};

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_symbol_creation() {
        let symbol = Symbol::new("main".into(), 0x1000, 0x2000, SymbolType::Function);
        assert_eq!(symbol.name, "main");
        assert_eq!(symbol.address, 0x1000);
        assert_eq!(symbol.size, 0x2000);
        assert_eq!(symbol.symbol_type, SymbolType::Function);
    }
    
    #[test]
    fn test_debug_info_creation() {
        let mut debug_info = DebugInfo::new();
        debug_info.add_function(FunctionEntry::new("main".into(), "main.rs".into(), 10));
        assert_eq!(debug_info.get_functions().len(), 1);
    }
    
    #[test]
    fn test_resolver_creation() {
        let resolver = Addr2lineResolver::new();
        assert!(resolver.get_symbols().is_empty());
    }
    
    #[test]
    fn test_add_symbol() {
        let mut resolver = Addr2lineResolver::new();
        resolver.add_symbol(Symbol::new("main".into(), 0x1000, 0x2000, SymbolType::Function));
        assert_eq!(resolver.get_symbols().len(), 1);
    }
    
    #[test]
    fn test_resolve_address() {
        let mut resolver = Addr2lineResolver::new();
        resolver.add_symbol(Symbol::new("main".into(), 0x1000, 0x2000, SymbolType::Function));
        resolver.add_line_info(0x1000, "main.rs".into(), 10, "main".into());
        
        let info = resolver.resolve(0x1000);
        assert!(info.is_some());
        let line_info = info.unwrap();
        assert_eq!(line_info.function, "main");
        assert_eq!(line_info.file, "main.rs");
        assert_eq!(line_info.line, 10);
    }
    
    #[test]
    fn test_resolve_address_range() {
        let mut resolver = Addr2lineResolver::new();
        resolver.add_symbol(Symbol::new("main".into(), 0x1000, 0x2000, SymbolType::Function));
        resolver.add_line_info(0x1000, "main.rs".into(), 10, "main".into());
        resolver.add_line_info(0x1001, "main.rs".into(), 11, "main".into());
        
        let info = resolver.resolve_range(0x1000, 0x1002);
        assert_eq!(info.len(), 2);
    }
    
    #[test]
    fn test_find_symbol() {
        let mut resolver = Addr2lineResolver::new();
        resolver.add_symbol(Symbol::new("main".into(), 0x1000, 0x2000, SymbolType::Function));
        resolver.add_symbol(Symbol::new("foo".into(), 0x2000, 0x1000, SymbolType::Function));
        
        let symbol = resolver.find_symbol(0x1000);
        assert!(symbol.is_some());
        assert_eq!(symbol.unwrap().name, "main");
    }
    
    #[test]
    fn test_inline_frames() {
        let mut resolver = Addr2lineResolver::new();
        resolver.add_inline_frame(0x1000, "inline_func".into(), "inline.rs".into(), 5);
        
        let frames = resolver.get_inline_frames(0x1000);
        assert_eq!(frames.len(), 1);
        assert_eq!(frames[0].function, "inline_func");
    }
    
    #[test]
    fn test_symbol_types() {
        let types = vec![
            SymbolType::Function,
            SymbolType::Variable,
            SymbolType::Section,
            SymbolType::File,
            SymbolType::Unknown,
        ];
        assert_eq!(types.len(), 5);
    }
    
    #[test]
    fn test_resolver_options() {
        let options = ResolverOptions::default();
        assert!(options.demangle);
        assert!(!options.show_inlines);
        assert!(!options.show_addresses);
    }
    
    #[test]
    fn test_export_import() {
        let mut resolver = Addr2lineResolver::new();
        resolver.add_symbol(Symbol::new("main".into(), 0x1000, 0x2000, SymbolType::Function));
        resolver.add_line_info(0x1000, "main.rs".into(), 10, "main".into());
        
        let export = resolver.export_data();
        
        let mut resolver2 = Addr2lineResolver::new();
        resolver2.import_data(export);
        
        assert_eq!(resolver2.get_symbols().len(), 1);
        assert!(resolver2.resolve(0x1000).is_some());
    }
}

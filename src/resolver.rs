//! Address resolver for addr2line.

use std::collections::HashMap;
use crate::symbol::{Symbol, SymbolTable, SymbolType};
use crate::debug_info::{DebugInfo, LineEntry, InlineFrame};

/// Line information result
#[derive(Debug, Clone)]
pub struct LineInfo {
    /// Address
    pub address: u64,
    /// Function name
    pub function: String,
    /// File name
    pub file: String,
    /// Line number
    pub line: u32,
    /// Column number
    pub column: u32,
    /// Inline frames
    pub inline_frames: Vec<InlineFrame>,
}

/// Resolver options
#[derive(Debug, Clone)]
pub struct ResolverOptions {
    /// Demangle symbol names
    pub demangle: bool,
    /// Show inline frames
    pub show_inlines: bool,
    /// Show addresses
    pub show_addresses: bool,
    /// Show function offsets
    pub show_offsets: bool,
}

impl Default for ResolverOptions {
    fn default() -> Self {
        Self {
            demangle: true,
            show_inlines: false,
            show_addresses: false,
            show_offsets: false,
        }
    }
}

/// Address-to-line resolver
#[derive(Debug, Clone, Default)]
pub struct Addr2lineResolver {
    /// Symbol table
    symbol_table: SymbolTable,
    /// Debug information
    debug_info: DebugInfo,
    /// Options
    options: ResolverOptions,
    /// Address to line mapping
    address_map: HashMap<u64, LineEntry>,
}

impl Addr2lineResolver {
    /// Create new resolver
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Create resolver with options
    pub fn with_options(options: ResolverOptions) -> Self {
        let mut resolver = Self::default();
        resolver.options = options;
        resolver
    }
    
    /// Set options
    pub fn set_options(&mut self, options: ResolverOptions) {
        self.options = options;
    }
    
    /// Get options
    pub fn get_options(&self) -> &ResolverOptions {
        &self.options
    }
    
    /// Add symbol
    pub fn add_symbol(&mut self, symbol: Symbol) {
        self.symbol_table.add_symbol(symbol);
    }
    
    /// Add line info
    pub fn add_line_info(&mut self, address: u64, file: String, line: u32, function: String) {
        let entry = LineEntry {
            address,
            file,
            line,
            column: 0,
            function,
        };
        self.address_map.insert(address, entry.clone());
        self.debug_info.add_line_entry(entry);
    }
    
    /// Add inline frame
    pub fn add_inline_frame(&mut self, address: u64, function: String, file: String, line: u32) {
        let frame = InlineFrame {
            function,
            file,
            line,
            call_site_line: 0,
        };
        self.debug_info.add_inline_frame(address, frame);
    }
    
    /// Resolve address to line info
    pub fn resolve(&self, address: u64) -> Option<LineInfo> {
        // Find symbol containing address
        let symbol = self.symbol_table.find_symbol(address);
        
        // Find line entry for address
        let line_entry = self.debug_info.get_line_info(address);
        
        // Find function containing address
        let function = self.debug_info.find_function(address);
        
        // Get inline frames if enabled
        let inline_frames = if self.options.show_inlines {
            self.debug_info.get_inline_frames(address).into_iter().cloned().collect()
        } else {
            vec![]
        };
        
        // Construct line info
        let line_info = LineInfo {
            address,
            function: function.map(|f| f.name.clone())
                .or_else(|| symbol.map(|s| s.get_display_name().to_string()))
                .unwrap_or_else(|| "unknown".into()),
            file: line_entry.map(|e| e.file.clone())
                .or_else(|| function.map(|f| f.file.clone()))
                .unwrap_or_else(|| "unknown".into()),
            line: line_entry.map(|e| e.line)
                .or_else(|| function.map(|f| f.line))
                .unwrap_or(0),
            column: line_entry.map(|e| e.column).unwrap_or(0),
            inline_frames,
        };
        
        Some(line_info)
    }
    
    /// Resolve address range
    pub fn resolve_range(&self, start: u64, end: u64) -> Vec<LineInfo> {
        let mut results = vec![];
        let mut addr = start;
        
        while addr < end {
            if let Some(info) = self.resolve(addr) {
                results.push(info);
            }
            addr += 1;
        }
        
        results
    }
    
    /// Find symbol by address
    pub fn find_symbol(&self, address: u64) -> Option<&Symbol> {
        self.symbol_table.find_symbol(address)
    }
    
    /// Get symbol by name
    pub fn get_symbol(&self, name: &str) -> Option<&Symbol> {
        self.symbol_table.get_symbol(name)
    }
    
    /// Get all symbols
    pub fn get_symbols(&self) -> Vec<&Symbol> {
        self.symbol_table.get_all_symbols()
    }
    
    /// Get symbols by type
    pub fn get_symbols_by_type(&self, symbol_type: &SymbolType) -> Vec<&Symbol> {
        self.symbol_table.get_symbols_by_type(symbol_type)
    }
    
    /// Get inline frames for address
    pub fn get_inline_frames(&self, address: u64) -> Vec<&InlineFrame> {
        self.debug_info.get_inline_frames(address)
    }
    
    /// Get function by name
    pub fn get_function(&self, name: &str) -> Option<&crate::debug_info::FunctionEntry> {
        self.debug_info.get_function(name)
    }
    
    /// Get all functions
    pub fn get_functions(&self) -> &[crate::debug_info::FunctionEntry] {
        self.debug_info.get_functions()
    }
    
    /// Get line entries
    pub fn get_line_entries(&self) -> &[LineEntry] {
        self.debug_info.get_line_entries()
    }
    
    /// Get symbol count
    pub fn get_symbol_count(&self) -> usize {
        self.symbol_table.get_symbol_count()
    }
    
    /// Get function count
    pub fn get_function_count(&self) -> usize {
        self.debug_info.get_function_count()
    }
    
    /// Get line entry count
    pub fn get_line_entry_count(&self) -> usize {
        self.debug_info.get_line_entry_count()
    }
    
    /// Clear all data
    pub fn clear(&mut self) {
        self.symbol_table.clear();
        self.debug_info.clear();
        self.address_map.clear();
    }
    
    /// Export resolver data
    pub fn export_data(&self) -> ResolverExport {
        ResolverExport {
            symbols: self.symbol_table.get_all_symbols().into_iter().cloned().collect(),
            functions: self.debug_info.get_functions().to_vec(),
            line_entries: self.debug_info.get_line_entries().to_vec(),
            inline_frames: self.debug_info.inline_frames.clone(),
        }
    }
    
    /// Import resolver data
    pub fn import_data(&mut self, export: ResolverExport) {
        self.clear();
        
        for symbol in export.symbols {
            self.symbol_table.add_symbol(symbol);
        }
        
        for function in export.functions {
            self.debug_info.add_function(function);
        }
        
        for entry in export.line_entries {
            self.address_map.insert(entry.address, entry.clone());
            self.debug_info.add_line_entry(entry);
        }
        
        for (address, frames) in export.inline_frames {
            for frame in frames {
                self.debug_info.add_inline_frame(address, frame);
            }
        }
    }
}

/// Resolver export data
#[derive(Debug, Clone)]
pub struct ResolverExport {
    /// Symbols
    pub symbols: Vec<Symbol>,
    /// Functions
    pub functions: Vec<crate::debug_info::FunctionEntry>,
    /// Line entries
    pub line_entries: Vec<LineEntry>,
    /// Inline frames
    pub inline_frames: HashMap<u64, Vec<InlineFrame>>,
}

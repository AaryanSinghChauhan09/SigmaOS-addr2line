//! Symbol table for addr2line.

use std::collections::HashMap;

/// Symbol type
#[derive(Debug, Clone, PartialEq)]
pub enum SymbolType {
    /// Function symbol
    Function,
    /// Variable symbol
    Variable,
    /// Section symbol
    Section,
    /// File symbol
    File,
    /// Unknown symbol type
    Unknown,
}

/// Symbol information
#[derive(Debug, Clone)]
pub struct Symbol {
    /// Symbol name
    pub name: String,
    /// Symbol address
    pub address: u64,
    /// Symbol size
    pub size: u64,
    /// Symbol type
    pub symbol_type: SymbolType,
    /// Demangled name
    pub demangled_name: Option<String>,
}

impl Symbol {
    /// Create new symbol
    pub fn new(name: String, address: u64, size: u64, symbol_type: SymbolType) -> Self {
        Self {
            name,
            address,
            size,
            symbol_type,
            demangled_name: None,
        }
    }
    
    /// Set demangled name
    pub fn set_demangled_name(&mut self, name: String) {
        self.demangled_name = Some(name);
    }
    
    /// Get display name (demangled if available)
    pub fn get_display_name(&self) -> &str {
        self.demangled_name.as_ref().unwrap_or(&self.name)
    }
    
    /// Check if address is in symbol range
    pub fn contains_address(&self, address: u64) -> bool {
        address >= self.address && address < self.address + self.size
    }
    
    /// Get symbol end address
    pub fn get_end_address(&self) -> u64 {
        self.address + self.size
    }
}

/// Symbol table
#[derive(Debug, Clone, Default)]
pub struct SymbolTable {
    /// Symbols by name
    symbols_by_name: HashMap<String, Symbol>,
    /// Symbols by address (sorted)
    symbols_by_address: Vec<Symbol>,
}

impl SymbolTable {
    /// Add symbol
    pub fn add_symbol(&mut self, symbol: Symbol) {
        self.symbols_by_name.insert(symbol.name.clone(), symbol.clone());
        self.symbols_by_address.push(symbol);
        // Keep sorted by address
        self.symbols_by_address.sort_by_key(|s| s.address);
    }
    
    /// Remove symbol
    pub fn remove_symbol(&mut self, name: &str) -> Option<Symbol> {
        if let Some(symbol) = self.symbols_by_name.remove(name) {
            self.symbols_by_address.retain(|s| s.name != name);
            Some(symbol)
        } else {
            None
        }
    }
    
    /// Get symbol by name
    pub fn get_symbol(&self, name: &str) -> Option<&Symbol> {
        self.symbols_by_name.get(name)
    }
    
    /// Find symbol by address
    pub fn find_symbol(&self, address: u64) -> Option<&Symbol> {
        // Binary search for symbol containing address
        self.symbols_by_address.iter()
            .find(|s| s.contains_address(address))
    }
    
    /// Get all symbols
    pub fn get_all_symbols(&self) -> Vec<&Symbol> {
        self.symbols_by_address.iter().collect()
    }
    
    /// Get symbols by type
    pub fn get_symbols_by_type(&self, symbol_type: &SymbolType) -> Vec<&Symbol> {
        self.symbols_by_address.iter()
            .filter(|s| s.symbol_type == *symbol_type)
            .collect()
    }
    
    /// Get symbol count
    pub fn get_symbol_count(&self) -> usize {
        self.symbols_by_address.len()
    }
    
    /// Clear all symbols
    pub fn clear(&mut self) {
        self.symbols_by_name.clear();
        self.symbols_by_address.clear();
    }
}

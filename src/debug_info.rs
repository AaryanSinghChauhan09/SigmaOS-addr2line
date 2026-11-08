//! Debug information for addr2line.

use std::collections::HashMap;

/// Line entry
#[derive(Debug, Clone)]
pub struct LineEntry {
    /// Address
    pub address: u64,
    /// File name
    pub file: String,
    /// Line number
    pub line: u32,
    /// Column number
    pub column: u32,
    /// Function name
    pub function: String,
}

/// Function entry
#[derive(Debug, Clone)]
pub struct FunctionEntry {
    /// Function name
    pub name: String,
    /// File name
    pub file: String,
    /// Line number
    pub line: u32,
    /// Function address range
    pub address_start: u64,
    pub address_end: u64,
    /// Local variables
    pub variables: HashMap<String, VariableInfo>,
}

impl FunctionEntry {
    /// Create new function entry
    pub fn new(name: String, file: String, line: u32) -> Self {
        Self {
            name,
            file,
            line,
            address_start: 0,
            address_end: 0,
            variables: HashMap::new(),
        }
    }
    
    /// Set address range
    pub fn set_address_range(&mut self, start: u64, end: u64) {
        self.address_start = start;
        self.address_end = end;
    }
    
    /// Add variable
    pub fn add_variable(&mut self, name: String, info: VariableInfo) {
        self.variables.insert(name, info);
    }
    
    /// Get variable
    pub fn get_variable(&self, name: &str) -> Option<&VariableInfo> {
        self.variables.get(name)
    }
    
    /// Check if address is in function range
    pub fn contains_address(&self, address: u64) -> bool {
        address >= self.address_start && address < self.address_end
    }
}

/// Variable information
#[derive(Debug, Clone)]
pub struct VariableInfo {
    /// Variable name
    pub name: String,
    /// Variable type
    pub var_type: String,
    /// Variable location
    pub location: String,
    /// Variable size
    pub size: u64,
}

/// Inline frame
#[derive(Debug, Clone)]
pub struct InlineFrame {
    /// Function name
    pub function: String,
    /// File name
    pub file: String,
    /// Line number
    pub line: u32,
    /// Call site line
    pub call_site_line: u32,
}

/// Debug information
#[derive(Debug, Clone, Default)]
pub struct DebugInfo {
    /// Functions
    functions: Vec<FunctionEntry>,
    /// Line entries
    line_entries: Vec<LineEntry>,
    /// Inline frames
    pub inline_frames: HashMap<u64, Vec<InlineFrame>>,
}

impl DebugInfo {
    /// Create new debug info
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Add function
    pub fn add_function(&mut self, function: FunctionEntry) {
        self.functions.push(function);
    }
    
    /// Add line entry
    pub fn add_line_entry(&mut self, entry: LineEntry) {
        self.line_entries.push(entry);
        self.line_entries.sort_by_key(|e| e.address);
    }
    
    /// Add inline frame
    pub fn add_inline_frame(&mut self, address: u64, frame: InlineFrame) {
        self.inline_frames.entry(address).or_insert_with(Vec::new).push(frame);
    }
    
    /// Get function by name
    pub fn get_function(&self, name: &str) -> Option<&FunctionEntry> {
        self.functions.iter().find(|f| f.name == name)
    }
    
    /// Find function by address
    pub fn find_function(&self, address: u64) -> Option<&FunctionEntry> {
        self.functions.iter()
            .find(|f| f.contains_address(address))
    }
    
    /// Get line info for address
    pub fn get_line_info(&self, address: u64) -> Option<&LineEntry> {
        // Binary search for line entry
        self.line_entries.iter()
            .find(|e| e.address <= address)
            .filter(|e| {
                // Check if this is the closest entry before address
                self.line_entries.iter()
                    .filter(|le| le.address <= address)
                    .last()
                    .map(|le| le.address == e.address)
                    .unwrap_or(false)
            })
    }
    
    /// Get all line entries
    pub fn get_line_entries(&self) -> &[LineEntry] {
        &self.line_entries
    }
    
    /// Get all functions
    pub fn get_functions(&self) -> &[FunctionEntry] {
        &self.functions
    }
    
    /// Get inline frames for address
    pub fn get_inline_frames(&self, address: u64) -> Vec<&InlineFrame> {
        self.inline_frames.get(&address)
            .map(|frames| frames.iter().collect())
            .unwrap_or_default()
    }
    
    /// Get function count
    pub fn get_function_count(&self) -> usize {
        self.functions.len()
    }
    
    /// Get line entry count
    pub fn get_line_entry_count(&self) -> usize {
        self.line_entries.len()
    }
    
    /// Clear all debug info
    pub fn clear(&mut self) {
        self.functions.clear();
        self.line_entries.clear();
        self.inline_frames.clear();
    }
}

# SigmaOS-addr2line Component Wiki

## Overview
SigmaOS-addr2line provides binary utility and symbol resolution support for the SigmaOS operating system. It demonstrates the binary analysis subsystem and debugging primitives within the SigmaOS ecosystem, following the principles of clear abstraction and deterministic symbol initialization.

## Principles
- Single Responsibility
- Binary Abstraction
- Fail-Fast on Binary Initialization Errors
- Deterministic Binary Setup
- Clear Abstraction Over Binary APIs
- Composition over Inheritance for binary extensions

## Architecture
SigmaOS-addr2line follows a binary pattern:
- **Binary Core**: Provides symbol resolution, address lookup, and line number extraction
- **ELF Parsing**: Supports ELF binary format parsing and section traversal
- **Debug Information**: Supports DWARF debug info extraction and symbol table reading
- **Error Handling**: Fail-fast on invalid binary files or unsupported formats

## API
The component provides the following public APIs:
- `binary::resolve(address)` - Resolve an address to source file and line number
- `binary::symbol(name)` - Look up a symbol name and its address
- `binary::line(address, options)` - Get source line information for an address
- `binary::status()` - Get binary analysis status and supported formats
- Error types for binary failure scenarios or unsupported formats

## Test Structure (Arrange-Act-ASSERT)
All tests follow the AAA pattern:
1. **Arrange**: Set up test environment with specific binary configuration
2. **Act**: Execute the code under test (symbol resolution, line number extraction)
3. **Assert**: Verify the expected binary results

Example:
```rust
#[test]
fn test_binary_resolve_aaa() {
    // Arrange
    let test_address: u64 = 0x4005f0;
    
    // Act
    let result = binary::resolve(test_address);
    
    // Assert
    assert!(result.is_ok());
    let (filename, line) = result.unwrap();
    assert!(!filename.is_empty());
    assert!(line > 0);
}
```

## Development
- Follows Rust best practices with `#![forbid(unsafe_code)]`
- Binary initialization is best-effort; gracefully degrades on invalid files or unsupported formats
- Comprehensive error types for all binary failure scenarios
- Unit tests provided for symbol resolution and line number extraction
- Integration tests for full binary analysis with actual compiled binaries

## TODO
- Add support for additional binary formats (PE/COFF, Mach-O)
- Expand symbol resolution capabilities (dynamic symbol lookup, weak symbols)
- Add debug information support (DWARF 4, DWARF 5 features)
- Create integration tests with actual compiled binaries and debug info
- Implement symbol caching and incremental resolution features
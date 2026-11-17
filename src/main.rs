//! SigmaOS-addr2line — thin CLI entry point.
//!
//! The library lives in `src/lib.rs`; this binary only wires argv/stdout.

use sigmaos_addr2line_lib::Component;

fn main() {
    let mut component = Component::new();
    if let Err(e) = component.init().and_then(|_| component.run()) {
        eprintln!("SigmaOS-addr2line: {e}");
        std::process::exit(1);
    }
    println!("SigmaOS-addr2line: ok");
}

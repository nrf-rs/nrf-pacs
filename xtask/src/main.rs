use std::env;

fn main() {
    let mut args = env::args().skip(1);
    let subcommand = args.next();
    match subcommand.as_deref() {
        Some("generate") => {
            xtask::generate();
        }
        _ => {
            eprintln!("usage: cargo xtask <subcommand>");
            eprintln!();
            eprintln!("subcommands:");
            eprintln!("    generate - regenerate crates");
        }
    }
}

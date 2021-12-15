use std::env;

fn main() {
    let mut args = env::args().skip(1);
    let subcommand = args.next();
    match subcommand.as_deref() {
        Some("generate") => {
            xtask::generate();
        }
        Some("publish") => {
            let dry_run = match args.next() {
                Some(arg) if arg == "--dry-run" => true,
                None => false,
                _ => panic!("xtask: malformed command line args"),
            };
            xtask::publish(dry_run);
        }
        _ => {
            eprintln!("usage: cargo xtask <subcommand>");
            eprintln!();
            eprintln!("subcommands:");
            eprintln!("    generate - regenerate crates");
            eprintln!("    publish [--dry-run] - publish crates to crates.io");
        }
    }
}

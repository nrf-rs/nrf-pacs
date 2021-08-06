use std::{collections::HashMap, env};
use xshell::cmd;

fn main() {
    // We execute from the `xtask` dir, so `cd ..` so that we can find everything.
    env::set_current_dir("..").unwrap();

    xtask::install_tools();

    // Regenerate PACs and check that nothing has changed.
    let was_clean = cmd!("git status --porcelain").read().unwrap().is_empty();
    xtask::generate();
    let is_clean = cmd!("git status --porcelain").read().unwrap().is_empty();
    if was_clean && !is_clean {
        cmd!("git status").run().unwrap();
        panic!("working directory not clean, run `cargo xtask generate` and push the result");
    }

    // Test that every PAC builds for the intended target.
    // We group them by target so that we can have Cargo build some of them in parallel.
    // FIXME: With https://github.com/rust-lang/cargo/issues/8176 this could be a single invocation.
    let mut target_map: HashMap<_, Vec<_>> = HashMap::new();
    for (pac, target) in xtask::PACS {
        target_map.entry(target).or_default().push(pac);
    }

    for (target, pacs) in target_map {
        let mut cmd = cmd!("cargo build --target {target}");
        for pac in pacs {
            let package = format!("{}-pac", pac);
            cmd = cmd.args(&["-p", &package]);
        }
        cmd.run().unwrap();
    }
}

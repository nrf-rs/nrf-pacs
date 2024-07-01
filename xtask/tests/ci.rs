use std::{collections::BTreeMap, env};
use xshell::cmd;
use xtask::is_git_clean;

fn main() {
    // We execute from the `xtask` dir, so `cd ..` so that we can find everything.
    env::set_current_dir("..").unwrap();

    // Regenerate PACs and check that nothing has changed.
    let was_clean = is_git_clean();

    xtask::generate();

    let is_clean = is_git_clean();
    if !is_clean {
        cmd!("git status").run().unwrap();
        if was_clean {
            panic!("working directory not clean, run `cargo xtask generate` and push the result");
        }
    }

    // Test that every PAC builds for the intended target.
    xtask::build();
}

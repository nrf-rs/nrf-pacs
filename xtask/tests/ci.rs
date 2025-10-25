use std::env;
use xshell::{cmd, Shell};
use xtask::is_git_clean;

fn main() {
    let sh = Shell::new().unwrap();

    // We execute from the `xtask` dir, so `cd ..` so that we can find everything.
    env::set_current_dir("..").unwrap();

    // Regenerate PACs and check that nothing has changed.
    let was_clean = is_git_clean(&sh);

    xtask::generate();

    let is_clean = is_git_clean(&sh);
    if !is_clean {
        cmd!(sh, "git status").run().unwrap();
        if was_clean {
            panic!("working directory not clean, run `cargo xtask generate` and push the result");
        }
    }

    // Test that every PAC builds for the intended target.
    xtask::build();
}

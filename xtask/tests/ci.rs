use std::env;
use xshell::cmd;

fn main() {
    // We execute from the `xtask` dir, so `cd ..` so that we can find everything.
    env::set_current_dir("..").unwrap();

    xtask::install_tools();

    // TODO: regen and check that nothing was changed

    // Build-test every PAC.
    for (pac, target) in xtask::PACS {
        let toml_path = format!("pacs/{}-pac/Cargo.toml", pac);
        cmd!("cargo build --manifest-path {toml_path} --target {target}")
            .run()
            .unwrap();
    }
}

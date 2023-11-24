use substrate_build_script_utils::{generate_cargo_keys, rerun_if_git_head_changed};

fn main() {

    println!("cargo:warning=Feature flags: {:?}", std::env::vars().filter(|(key, _)| key.starts_with("CARGO_FEATURE_")));


    // early exit if we're building the parachain


    generate_cargo_keys();

    rerun_if_git_head_changed();
}

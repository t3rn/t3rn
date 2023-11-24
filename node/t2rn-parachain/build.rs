use substrate_build_script_utils::{generate_cargo_keys, rerun_if_git_head_changed};

fn main() {
    // LOG all of the feature flags
    let features = std::env::vars()
        .filter(|(key, _)| key.starts_with("CARGO_FEATURE_"))
        .map(|(key, _)| key.replace("CARGO_FEATURE_", ""))
        .collect::<Vec<_>>()
        .join(",");

    println!("DUPA DUPA DUPA cargo:rustc-env=FEATURES={}", features);
    println!("DUPA DUPA DUPA cargo:rustc-env=FEATURES={}", features);
    println!("DUPA DUPA DUPA cargo:rustc-env=FEATURES={}", features);
    println!("DUPA DUPA DUPA cargo:rustc-env=FEATURES={}", features);
    println!("DUPA DUPA DUPA cargo:rustc-env=FEATURES={}", features);
    println!("DUPA DUPA DUPA cargo:rustc-env=FEATURES={}", features);
    println!("DUPA DUPA DUPA cargo:rustc-env=FEATURES={}", features);
    println!("DUPA DUPA DUPA cargo:rustc-env=FEATURES={}", features);
    println!("DUPA DUPA DUPA cargo:rustc-env=FEATURES={}", features);
    println!("DUPA DUPA DUPA cargo:rustc-env=FEATURES={}", features);
    println!("DUPA DUPA DUPA cargo:rustc-env=FEATURES={}", features);
    println!("DUPA DUPA DUPA cargo:rustc-env=FEATURES={}", features);

    generate_cargo_keys();

    rerun_if_git_head_changed();
}

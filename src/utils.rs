use std::env;
use std::path::PathBuf;

// TODO: find a better way to do this
pub fn find_binary(name: &str) -> PathBuf {
    let profile = env::var("PROFILE").unwrap_or_else(|_| "debug".into());

    let mut exe_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    exe_path.push("target");
    exe_path.push(&profile);
    exe_path.push(format!("{}{}", name, env::consts::EXE_SUFFIX));

    exe_path
}

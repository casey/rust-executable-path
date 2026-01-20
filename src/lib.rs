use std::{env, path::PathBuf};

/// Takes the name of a binary target and returns the path to the target's executable.
pub fn executable_path(name: &str) -> PathBuf {
  if let Some(value) = env::var_os(format!("CARGO_BIN_EXE_{}", name)) {
    return value.into();
  }

  let mut path = env::current_exe().unwrap();
  path.pop();
  if path.ends_with("deps") {
    path.pop();
  }
  let exe = String::from(name) + env::consts::EXE_SUFFIX;
  path.push(exe);
  path
}

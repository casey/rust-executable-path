use std::path::PathBuf;

pub fn executable_path(name: &str) -> PathBuf {
  let mut path = std::env::current_exe().unwrap();
  path.pop();
  if path.ends_with("deps") {
    path.pop();
  }
  let exe = String::from(name) + std::env::consts::EXE_SUFFIX;
  path.push(exe);
  path
}

use std::path::PathBuf;

fn main() {
  tauri_build::build();

  let dir: PathBuf = ["..", "tree-sitter-javascript", "src"].iter().collect();

  cc::Build::new()
    .include(&dir)
    .file(dir.join("parser.c"))
    .file(dir.join("scanner.c"))
    .compile("tree-sitter-javascript");
}

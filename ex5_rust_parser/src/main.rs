use std::fs::File;
use std::io::{Read as _, Write as _};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
// コメントを取り除くプログラム

fn format_by_rustfmt(input: &str) -> Result<String, Box<dyn std::error::Error>> {
    let fmt_subprocess = Command::new("rustfmt")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;
    fmt_subprocess.stdin.unwrap().write_all(input.as_bytes())?;
    let mut s = String::new();
    let _ = fmt_subprocess.stdout.unwrap().read_to_string(&mut s)?;
    Ok(s)
}
fn path_to_source<P>(path: &P) -> Result<String, Box<dyn std::error::Error>>
where
    P: AsRef<Path>,
{
    let content = std::fs::read_to_string(path)?;
    let ast = syn::parse_file(&content)?;
    let source = quote :: quote ! (# ast).to_string();
    let out = format_by_rustfmt(&source);
    out
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    dbg!(std::env::args());
    let read_rel_dir_path = std::env::args().nth(1).expect("relative path to read (like src)");
    let write_rel_dir_path = std::env::args().nth(2).expect("relative path to write  (like dst)");
    let traverse_dir = std::env::current_dir()?.join(read_rel_dir_path);
    
    for entry in std::fs::read_dir(&traverse_dir)? {
        let entry = entry?;
        let path = &entry.path();
        let processed_str = path_to_source(path)?;
        let relative_path = path.strip_prefix(&traverse_dir)?;
        let write_file_path = PathBuf::from(&write_rel_dir_path).join(relative_path);
        dbg!(&write_file_path);
        std::fs::create_dir_all(&write_rel_dir_path)?;
        let mut file = File::create(&write_file_path)?;
        file.write_all(&processed_str.as_bytes())?;
    }
    Ok(())
}

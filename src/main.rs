use colored::Colorize;
use std::env;
use std::path::PathBuf;

static PATH: &str = "PATH";

fn main() {
    match env::var_os(PATH) {
        Some(ev_path) => write_path(ev_path.as_os_str(), &mut std::io::stdout()),
        None => panic!("{} is not set", PATH),
    }
}

fn write_path(path: &std::ffi::OsStr, mut writer: impl std::io::Write) {
    // Collect non-blank items into a vector (Why are there blank items?)
    let paths: Vec<PathBuf> = env::split_paths(&path)
        .filter(|path_buf| path_buf.capacity() > 0)
        .collect();
    for path in &paths {
        let path_string = path.display().to_string();
        if !path.exists() {
            write_error(&path_string, "Does not exist", &mut writer);
        } else if !path.is_dir() {
            write_error(&path_string, "Is not a directory", &mut writer);
        } else if muliple_occurrences(&paths, path) {
            write_error(&path_string, "Appears more than once", &mut writer)
        } else {
            writeln!(writer, "{}", path_string).unwrap();
        }
    }
}

fn write_error(path: &String, error: &str, writer: &mut impl std::io::Write) {
    writeln!(
        writer,
        "{} {}",
        path.bright_yellow(),
        format!("[{}]", error).bright_red()
    )
    .unwrap();
}

fn muliple_occurrences(paths: &[PathBuf], target_path: &PathBuf) -> bool {
    paths.iter().filter(|path| path.eq(&target_path)).count() > 1
}

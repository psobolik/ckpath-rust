use colored::Colorize;
use std::env;
use std::fs;
use std::path::PathBuf;

static PATH: &str = "PATH";

fn main() {
    match env::var_os(PATH) {
        Some(ev_path) => {
            // eprintln!("{:?}", ev_path);
            // Collect non-blank items into a vector
            let paths: Vec<PathBuf> = env::split_paths(&ev_path)
                .filter(|path_buf| !path_buf.display().to_string().is_empty())
                .collect();
            for path in &paths {
                let path_string = path
                    .to_str()
                    .expect("Can't convert path element to string. (This should never happen.)");
                match fs::metadata(path) {
                    Ok(metadata) => {
                        if metadata.is_dir() {
                            if muliple_occurrences(&paths, path) {
                                println!(
                                    "{} {}",
                                    path_string.bright_yellow(),
                                    "[Appears more than once]".bright_red()
                                );
                            } else {
                                println!("{}", path_string);
                            }
                        } else {
                            println!(
                                "{} {}",
                                path_string.bright_yellow(),
                                "[Is not a directory]".bright_red()
                            );
                        }
                    }
                    Err(e) => {
                        println!(
                            "{} {}",
                            path_string.bright_yellow(),
                            format!("[{}]", e).bright_red()
                        )
                    }
                }
            }
        }
        None => panic!("{} is not set", PATH),
    }
}

fn muliple_occurrences(paths: &[PathBuf], target_path: &PathBuf) -> bool {
    paths.iter().filter(|path| path.eq(&target_path)).count() > 1
}

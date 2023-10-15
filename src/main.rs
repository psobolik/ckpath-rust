use colored::Colorize;
use std::env;
use std::fs;

static PATH: &'static str = "PATH";

fn main() {
    match env::var_os(PATH) {
        Some(ev_path) => {
            let paths = parse_ev_path(ev_path);
            for path in &paths {
                match fs::metadata(&path) {
                    Ok(metadata) => {
                        if metadata.is_dir() {
                            if count_occurances(&paths, &path) == 1 {
                                println!("{}", path);
                            } else {
                                println!(
                                    "{} {}",
                                    path.bright_yellow(),
                                    "[Appears more than once]".bright_red()
                                );
                            }
                        } else {
                            println!(
                                "{} {}",
                                path.bright_yellow(),
                                "[Is not a directory]".bright_red()
                            );
                        }
                    }
                    Err(e) => {
                        println!(
                            "{} {}",
                            path.bright_yellow(),
                            format!("[{}]", e).bright_red()
                        )
                    }
                }
            }
        }
        None => panic!("{} is not set", PATH),
    }
}

fn parse_ev_path(ev_path: std::ffi::OsString) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let path_strings_iter = env::split_paths(&ev_path).map(|f| f.display().to_string());
    for path in path_strings_iter {
        result.push(path);
    }
    result
}

fn count_occurances(paths: &Vec<String>, target_path: &String) -> usize {
    paths.iter().filter(|path| path.eq(&target_path)).count()
}

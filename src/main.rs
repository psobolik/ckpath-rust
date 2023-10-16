use clap::Parser;
use colored::Colorize;
use std::env;
use std::path::PathBuf;

static PATH: &str = "PATH";

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Arguments {
    #[arg(short, long = "problems", help = "Show summary of problem items")]
    problem_summary: bool,
}

fn main() {
    let args = Arguments::parse();

    match env::var_os(PATH) {
        Some(ev_path) => {
            write_path(ev_path.as_os_str(), &mut std::io::stdout(), args.problem_summary);
        }
        None => panic!("{} is not set", PATH),
    }
}

fn write_path(path: &std::ffi::OsStr, mut writer: impl std::io::Write, summary: bool) {
    // Collect non-blank items into a vector (Why are there blank items?)
    let paths: Vec<PathBuf> = env::split_paths(&path)
        .filter(|path_buf| path_buf.capacity() > 0)
        .collect();

    let mut non_existant: Vec<PathBuf> = vec![];
    let mut non_folder: Vec<PathBuf> = vec![];
    let mut duplicate: Vec<PathBuf> = vec![];

    for path in &paths {
        let path_string = path.display().to_string();
        if !path.exists() {
            write_error(&path_string, "Does not exist", &mut writer);
            non_existant.push(path.to_path_buf());
        } else if !path.is_dir() {
            write_error(&path_string, "Is not a directory", &mut writer);
            non_folder.push(path.to_path_buf());
        } else if muliple_occurrences(&paths, path) {
            write_error(&path_string, "Appears more than once", &mut writer);
            if !duplicate.contains(path) {
                duplicate.push(path.to_path_buf());
            };
        } else {
            writeln!(writer, "{}", path_string).unwrap();
        }
    }
    if summary {
        writeln!(writer, "\nProblem summary:").unwrap();
        write_summary("Items that don't exist", non_existant, &mut writer);
        write_summary("Items that are not folders", non_folder, &mut writer);
        write_summary("Items that appear more than once", duplicate, &mut writer);
    }
}

fn write_summary(description: &str, items: Vec<PathBuf>, writer: &mut impl std::io::Write) {
    writeln!(writer, "\n{description}:").unwrap();
    if items.capacity() > 0 {
        for item in items {
            writeln!(writer, "\t{}", item.display()).unwrap();
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

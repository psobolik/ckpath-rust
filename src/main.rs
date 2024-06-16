use clap::Parser;
use colored::Colorize;
use std::env;
use std::path::PathBuf;

static PATH: &str = "PATH";
static PSMODULEPATH: &str = "PSModulePath";

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Arguments {
    #[arg(short, long, default_value_t = false, help = "Show PSModulePath instead of PATH")]
    ps_module_path: bool,

    #[arg(short = 's', long = "summary", help = "Only show summary of problems")]
    problem_summary_only: bool,
}

fn main() {
    let args = Arguments::parse();
    let key = if args.ps_module_path {
        PSMODULEPATH
    } else {
        PATH
    };
    let ev = env::var_os(key);
    match ev {
        Some(path) => {
            write_path(path.as_os_str(), std::io::stdout(), args.problem_summary_only);
        }
        None => panic!("Environment variable \"{}\" is not set", key),
    }
}

fn write_path(path: &std::ffi::OsStr, mut writer: impl std::io::Write, summary_only: bool) {
    // Collect non-blank items into a vector (Why are there blank items?)
    let paths: Vec<PathBuf> = env::split_paths(&path)
        .filter(|path_buf| path_buf.capacity() > 0)
        .collect();

    let mut non_existent: Vec<PathBuf> = vec![];
    let mut non_folder: Vec<PathBuf> = vec![];
    let mut duplicate: Vec<PathBuf> = vec![];

    for path in &paths {
        let path_string = path.display().to_string();
       
        if !path.exists() {
            if !summary_only {
                write_error_item(&path_string, "Does not exist", &mut writer);
            }
            non_existent.push(path.to_path_buf());
        } else if !path.is_dir() {
            if !summary_only {
                write_error_item(&path_string, "Is not a directory", &mut writer);
            }
            non_folder.push(path.to_path_buf());
        } else if muliple_occurrences(&paths, path) {
            if !summary_only {
                write_error_item(&path_string, "Appears more than once", &mut writer);
            }
            if !duplicate.contains(path) {
                duplicate.push(path.to_path_buf());
            };
        } else if !summary_only {
            writeln!(writer, "{}", path_string).unwrap();
        }
    }
    if summary_only {
        if non_existent.is_empty() && non_folder.is_empty() && duplicate.is_empty() {
            writeln!(writer, "\nNo problems found").unwrap();
        } else {
            writeln!(writer, "\nProblem summary:").unwrap();
            if !non_existent.is_empty() {
                write_summary_category("Items don't exist", non_existent, &mut writer);
            }
            if !non_folder.is_empty() {
                write_summary_category("Items are not folders", non_folder, &mut writer);
            }
            if !duplicate.is_empty() {
                write_summary_category("Items appear more than once", duplicate, &mut writer);
            }
        }
    }
}

fn write_summary_category(description: &str, items: Vec<PathBuf>, writer: &mut impl std::io::Write) {
    writeln!(writer, "\n{description}:").unwrap();
    if items.capacity() > 0 {
        for item in items {
            writeln!(writer, "\t{}", item.display()).unwrap();
        }
    }
}

fn write_error_item(path: &String, error: &str, writer: &mut impl std::io::Write) {
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

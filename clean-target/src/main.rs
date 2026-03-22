use std::{io::BufRead, process::exit, str::FromStr};

fn has_arg(args: &[String], arg_name: &str) -> Option<usize> {
    for (position, arg) in args.iter().enumerate() {
        if arg.starts_with(arg_name) {
            return Some(position);
        }
    }
    None
}

fn parse_arg(args: &[String], arg_name: &str) -> Option<String> {
    match has_arg(args, arg_name) {
        Some(position) => args.iter().skip(position + 1).next().map(|x| x.to_string()),
        None => None,
    }
}

fn resolve_path(args: &[String]) -> String {
    match parse_arg(&args, "--path") {
        Some(p) => p,
        None => {
            eprintln!("--path is expected cli argument");
            exit(1);
        }
    }
}

fn is_contains_target(path: &std::path::Path) -> bool {
    let target_path = path.join("target");
    let cargo_toml = path.join("Cargo.toml");

    if target_path.is_dir() && cargo_toml.is_file() {
        return true;
    }
    false
}

fn rec_collect_target_dirs(
    root: &std::path::Path,
    targets: &mut Vec<std::path::PathBuf>,
) -> anyhow::Result<()> {
    if !root.is_dir() {
        return Ok(());
    }

    // if current directory contain Cargo.toml and target both
    if is_contains_target(root) {
        targets.push(root.into());
    }

    for entry in root.read_dir()? {
        let path = entry?.path();
        rec_collect_target_dirs(&path, targets)?;
    }

    Ok(())
}

// we check if Cargo.toml file and target directory exists.
fn resolve_target_dirs(root: &std::path::Path) -> anyhow::Result<Vec<std::path::PathBuf>> {
    let mut targets = vec![];
    rec_collect_target_dirs(root, &mut targets)?;
    println!("{:?}", targets);
    Ok(targets)
}

fn clean_targets(targets: &[std::path::PathBuf]) -> anyhow::Result<()> {
    for dir in targets {
        let target_path = dir.join("target");
        if target_path.exists() {
            std::fs::remove_dir_all(target_path)?;
        }
    }
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let args = std::env::args().collect::<Vec<_>>();
    let path = resolve_path(&args);

    let path = std::path::PathBuf::from_str(path.as_str())?.canonicalize()?;

    let targets_to_remove = resolve_target_dirs(&path)?;

    println!("removing targets from dirs, press y/Y to remove all of them");
    for target in targets_to_remove.iter() {
        println!("--> {:?}", target.display());
    }

    let mut command = String::new();
    std::io::stdin().lock().read_line(&mut command)?;

    match command.trim() {
        "y" | "yes" | "Y" => {
            clean_targets(&targets_to_remove)?;
        }
        _ => {}
    }

    Ok(())
}

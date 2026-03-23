/////////////////////////////////////////////////////////////
/////////////////// Resolve Argument ////////////////////////
/////////////////////////////////////////////////////////////
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

pub fn resolve_arg(args: &[String], name: &str) -> String {
    match parse_arg(&args, name) {
        Some(p) => p,
        None => {
            eprintln!("argument: `{}` is expected cli argument", name);
            std::process::exit(1);
        }
    }
}

////////////////////////////////////////////////////////////////////

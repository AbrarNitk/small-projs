use std::str::FromStr;

use linux_utils::utils;
use linux_utils::wzip;

fn main() -> anyhow::Result<()> {
    let args = std::env::args().collect::<Vec<_>>();
    let file_path = std::path::PathBuf::from_str(utils::resolve_arg(&args, "--path").as_str())?
        .canonicalize()?;
    wzip::zip(file_path.as_path())?;
    Ok(())
}

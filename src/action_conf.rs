use std::path::PathBuf;
use std::process::Command;

use crate::utils;

pub fn exec(path: PathBuf, edit: bool) -> Result<(), i32> {
    if !edit {
        println!("{}", path.display());
        return Ok(());
    }
    let editor = std::env::var("EDITOR").unwrap_or_else(|_| "vi".into());

    Err(Command::new(editor)
        .arg(path)
        .status()
        .map_err(|err| {
            utils::error!("could not spawn editor process: {err}");
            19
        })?
        .code()
        .unwrap_or(0))
}

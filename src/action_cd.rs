use std::path::PathBuf;

use crate::utils;

pub fn exec(path: PathBuf, pattern: Option<String>) -> Result<(), i32> {
    let items = utils::read_config_items(path, pattern)?;

    match items.len() {
        1 => Ok(println!("{}", items[0].1)),
        0 => {
            utils::error!("No matching bookmark found");
            Err(1337)
        }
        _ => {
            utils::error!("More than one matching bookmark found");
            Err(1337)
        }
    }
}

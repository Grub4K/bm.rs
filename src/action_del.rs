use std::{fs::OpenOptions, path::PathBuf};

use crate::utils::{self, ConfigItem};

pub fn exec(path: PathBuf, bookmark: &str) -> Result<(), i32> {
    if !utils::is_valid_bookmark_name(bookmark) {
        utils::error!("invalid name for a bookmark: {bookmark:?}");
        return Err(26);
    }
    let config = utils::parse_config(path.clone())?;
    let _ = config
        .iter()
        .find_map(|item| match item {
            ConfigItem::Value(key, value) if *key == bookmark => Some(value),
            _ => None,
        })
        .ok_or_else(|| {
            utils::error!("could not find bookmark: {bookmark}");
            27
        })?;
    utils::write_config(
        path,
        config.iter().filter_map(|x| match x {
            ConfigItem::Value(key, _) if *key == bookmark => None,
            _ => Some(x.clone()),
        }),
        OpenOptions::new().truncate(true),
    )?;
    println!("deleted {bookmark:?} from bookmarks");
    Ok(())
}

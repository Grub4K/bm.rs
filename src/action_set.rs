use path_absolutize::*;
use std::{fs::OpenOptions, path::PathBuf};

use crate::utils::{self, ConfigItem};

pub fn exec(
    path: PathBuf,
    bookmark: String,
    set_path: Option<PathBuf>,
    force: bool,
) -> Result<(), i32> {
    if !utils::is_valid_bookmark_name(&bookmark) {
        utils::error!("invalid name for a bookmark: {bookmark:?}");
        Err(21)?
    }
    let set_path = if set_path.is_none() {
        std::env::current_dir().map_err(|err| {
            utils::error!("could not determine current directory: {err}");
            22
        })
    } else {
        set_path.ok_or(23)
    }?
    .absolutize()
    .map_err(|err| {
        utils::error!("could not set path: {err}");
        24
    })?
    .into_owned()
    .into_os_string()
    .into_string()
    .expect("Why does rust have infinite amounts of string types");
    let config = utils::parse_config(path.clone())?;
    if let Some(index) = config
        .iter()
        .enumerate()
        .find_map(|(index, item)| match item {
            ConfigItem::Value(key, _) if *key == bookmark => Some(index),
            _ => None,
        })
    {
        if !force {
            utils::error!("key {bookmark:?} already exists");
            Err(25)?
        }
        utils::write_config(
            path,
            config.iter().enumerate().map(|(line, item)| {
                if line == index {
                    ConfigItem::Value(bookmark.clone(), set_path.clone())
                } else {
                    item.clone()
                }
            }),
            OpenOptions::new().truncate(true),
        )?;
    } else {
        utils::write_config(
            path,
            vec![ConfigItem::Value(bookmark.clone(), set_path.clone())].into_iter(),
            OpenOptions::new().append(true),
        )?;
    }
    println!("set {bookmark:?} to {set_path}");
    Ok(())
}

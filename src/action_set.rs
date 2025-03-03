use path_absolutize::Absolutize;
use std::{fs::OpenOptions, path::PathBuf};
use owo_colors::{OwoColorize, Stream, Style};

use crate::utils::{self, ConfigItem};

pub fn exec(
    path: PathBuf,
    bookmark: &str,
    set_path: Option<PathBuf>,
    force: bool,
) -> Result<(), i32> {
    if !utils::is_valid_bookmark_name(bookmark) {
        utils::error!("invalid name for a bookmark: {bookmark:?}");
        return Err(21);
    }
    let set_path_buf = set_path
        .ok_or(0)
        .or_else(|_| {
            std::env::current_dir().map_err(|err| {
                utils::error!("could not determine current directory: {err}");
                22
            })
        })?
        .absolutize()
        .map_err(|err| {
            utils::error!("could not set path: {err}");
            24
        })?
        .into_owned();
    if !set_path_buf.exists() {
        utils::warning!("path does not exist");
    }
    let set_path = set_path_buf
        .into_os_string()
        .into_string()
        .expect("why does rust have infinite amounts of string types");
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
            return Err(25);
        }
        utils::write_config(
            path,
            config.iter().enumerate().map(|(line, item)| {
                if line == index {
                    ConfigItem::Value(bookmark.to_string(), set_path.clone())
                } else {
                    item.clone()
                }
            }),
            OpenOptions::new().truncate(true),
        )?;
    } else {
        utils::write_config(
            path,
            vec![ConfigItem::Value(bookmark.to_string(), set_path.clone())].into_iter(),
            OpenOptions::new().append(true),
        )?;
    }

    anstream::println!(
        "{} = {}",
        bookmark.if_supports_color(Stream::Stdout, |x| Style::new().blue().bold().style(x)),
        set_path
    );
    Ok(())
}

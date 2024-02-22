use std::{
    fs::OpenOptions,
    io::{BufRead, BufReader, BufWriter, Write},
    path::PathBuf,
};

#[derive(Debug, Clone)]
pub enum ConfigItem {
    Comment(String),
    Value(String, String),
}

macro_rules! error {
    ($t: expr) => {
        use owo_colors::{OwoColorize, Stream, Style};

        anstream::eprintln!(
            "{}{} {}",
            "error".if_supports_color(Stream::Stderr, |x| Style::new().red().bold().style(x)),
            ":".if_supports_color(Stream::Stderr, |x| Style::new().style(x.bold())),
            format!($t)
        );
    };
}

pub(crate) use error;

pub fn ensure_config_path(config: Option<PathBuf>) -> Result<PathBuf, u8> {
    let path = match config {
        Some(config) => config,
        None => {
            let mut path = dirs::home_dir().ok_or_else(|| {
                error!("could not resolve system config dir");
                10
            })?;
            path.extend([".config", "bm", "default.rc"]);
            path
        }
    };
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|err| {
            error!("could not create config dir: {err}");
            11
        })?;
    }
    OpenOptions::new()
        .create(true)
        .write(true)
        .open(&path)
        .map_err(|err| {
            error!("could not create config file: {err}");
            12
        })?;
    Ok(path)
}

pub fn is_valid_bookmark_name(name: &String) -> bool {
    name.as_bytes().iter().all(
        |x| matches!(x, b'-' | b'.' | b'/' | b'0'..=b'9' | b':' | b'A'..=b'Z' | b'_' | b'a'..=b'z'),
    )
}

pub fn make_pattern(pattern: Option<String>) -> Result<Option<glob::Pattern>, i32> {
    Ok(match pattern {
        Some(pattern) => Some(glob::Pattern::new(pattern.as_str()).map_err(|err| {
            error!("could not create glob: {err}");
            28
        })?),
        None => None,
    })
}

pub fn parse_config(path: PathBuf) -> Result<Vec<ConfigItem>, u8> {
    let reader = BufReader::new(OpenOptions::new().read(true).open(path).map_err(|err| {
        error!("could not open config file for reading: {err}");
        13
    })?);
    let mut lines: Vec<ConfigItem> = vec![];
    for (line, index) in reader.lines().zip(1..) {
        let line = line.map_err(|err| {
            error!("could not read line: {err}");
            14
        })?;
        if line.is_empty() || line.starts_with('#') {
            lines.push(ConfigItem::Comment(line));
        } else if let Some((key, value)) = line.split_once('=') {
            let key = key.trim();
            if !is_valid_bookmark_name(&key.into()) {
                error!("could not parse config: invalid key in line {index}");
                Err(15)?
            }
            lines.push(ConfigItem::Value(key.into(), value.trim().into()));
        } else {
            error!("could not parse config: no value in line {index}");
            Err(16)?
        }
    }
    Ok(lines)
}

pub fn read_config_items(
    config_path: PathBuf,
    pattern: Option<String>,
) -> Result<Vec<(String, String)>, i32> {
    let pattern = make_pattern(pattern)?;

    Ok(parse_config(config_path)?
        .iter()
        .filter_map(|x| match x {
            ConfigItem::Value(key, value) => {
                if let Some(re) = &pattern {
                    if !re.matches(key) {
                        return None;
                    }
                }
                Some((key.into(), value.into()))
            }
            _ => None,
        })
        .collect())
}

pub fn write_config(
    path: PathBuf,
    lines: impl Iterator<Item = ConfigItem>,
    options: &mut OpenOptions,
) -> Result<(), i32> {
    let mut writer = BufWriter::new(options.write(true).open(path).map_err(|err| {
        error!("could not open config file for writing: {err}");
        17
    })?);
    for line in lines {
        writer
            .write_all(
                match line {
                    ConfigItem::Comment(line) => format!("{line}\n"),
                    ConfigItem::Value(key, value) => format!("{key} = {value}\n"),
                }
                .as_bytes(),
            )
            .map_err(|err| {
                error!("could not write line: {err}");
                18
            })?;
    }
    Ok(())
}

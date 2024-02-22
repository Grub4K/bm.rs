use std::io::Write;
use std::path::PathBuf;

use crate::utils;

use owo_colors::{OwoColorize, Stream, Style};

pub fn exec(path: PathBuf, pattern: Option<String>, sort: bool) -> Result<(), i32> {
    let mut items = utils::read_config_items(path, pattern)?;
    if sort {
        items.sort();
    }
    let width = match items.iter().map(|x| x.0.len()).max().unwrap_or(0) {
        0..=5 => 5,
        20.. => 20,
        x => x,
    };

    let mut lock = std::io::stdout().lock();
    for (key, value) in items {
        writeln!(
            lock,
            "{0: <1$}  {2}",
            key.if_supports_color(Stream::Stdout, |text| Style::new()
                .bright_blue()
                .bold()
                .style(text)),
            width,
            value
        )
        .map_err(|err| {
            utils::error!("failed to write to stdout: {err}");
            29
        })?;
    }
    Ok(())
}

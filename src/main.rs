use clap::{Parser, Subcommand};

use std::path::PathBuf;

mod action_cd;
mod action_conf;
mod action_del;
mod action_init;
mod action_ls;
mod action_set;
mod utils;

// TODO: document that only valid bookmark name chars are [a-zA-Z] and -./:_

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    action: Actions,
    /// Use PATH as the config file
    #[arg(short, long, value_name = "PATH")]
    config: Option<PathBuf>,
}

#[derive(Subcommand, Debug)]
enum Actions {
    /// `cd` to a bookmark (optionally matching a pattern)
    ///
    /// This will print the bookmark instead if `bm init` has not been executed
    Cd {
        /// A glob pattern to filter the bookmarks with
        pattern: Option<String>,
    },
    /// List bookmarks (optionally matching a pattern)
    Ls {
        /// A glob pattern to filter the bookmarks with
        pattern: Option<String>,
        /// Keep the ordering as it is in the config file
        #[arg(short, long)]
        keep_order: bool,
    },
    /// Set a bookmark to the specified path
    Set {
        /// Force overwriting existing bookmarks
        #[arg(short, long)]
        force: bool,
        /// The bokmark name to store the path under
        bookmark: String,
        /// The path to set the bookmark to [default: .]
        path: Option<PathBuf>,
    },
    /// Delete a bookmark
    Del {
        // TODO: Allow glob pattern for deletion behind a flag
        /// The bookmark name to delete
        bookmark: String,
    },
    /// Print the path (default) or edit the configuration file
    Conf {
        /// Open the configuration file using $EDITOR (or vi as a fallback)
        #[arg(short, long)]
        edit: bool,
    },
    /// Generate initialization script for a specific shell
    Init {
        /// The alias name to use [default: bm]
        #[arg(short, long)]
        alias: Option<String>,
        /// The shell to generate the script for
        shell: action_init::Shell,
    },
}

fn main() {
    let args = Args::parse();
    let config_path =
        utils::ensure_config_path(args.config).unwrap_or_else(|err| std::process::exit(err.into()));
    std::process::exit(
        match args.action {
            Actions::Cd { pattern } => action_cd::exec(config_path, pattern),
            Actions::Ls { pattern, keep_order } => action_ls::exec(config_path, pattern, !keep_order),
            Actions::Set {
                force,
                bookmark,
                path,
            } => action_set::exec(config_path, bookmark.as_str(), path, force),
            Actions::Del { bookmark } => action_del::exec(config_path, bookmark.as_str()),
            Actions::Conf { edit } => action_conf::exec(config_path, edit),
            Actions::Init { shell, alias } => {
                action_init::exec(&shell, alias);
                Ok(())
            },
        }
        .err()
        .unwrap_or(0),
    );
}

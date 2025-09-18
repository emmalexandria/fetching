// Most of this file is dedicated to our styled help output, because clap's
// facilities for styling help kinda suck.

use std::{fmt::Display, path::PathBuf};

use clap::{
    builder::{
        styling::{AnsiColor, Color, Effects, Style},
        Styles,
    },
    command, value_parser, Arg, ArgMatches, Command, ValueEnum,
};
use ratatui::crossterm::style::Stylize;

#[derive(ValueEnum, Clone, Copy)]
pub enum LayoutOptions {
    Compact,
    Default,
    Auto,
}

pub fn build_cli() -> Command {
    let mut main = command!()
        .arg(
            Arg::new("layout")
                .short('l')
                .help("Chooses the fetch layout to use")
                .long("layout")
                .default_value("auto")
                .value_parser(value_parser!(LayoutOptions)),
        )
        .arg(Arg::new("style").short('s').long("style"))
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .value_parser(value_parser!(PathBuf)),
        )
        .about("A beautiful and fast system information fetcher.")
        .author("Emma Alexandria")
        .after_help("MIT License")
        .help_template(help_template())
        .styles(cli_styles());

    main = add_help_info(main);

    main
}

fn cli_styles() -> Styles {
    Styles::styled().header(
        Style::new()
            .bold()
            .fg_color(Some(Color::Ansi(AnsiColor::Red)))
            .underline(),
    )
}

// Clap has limited support for styling help output unfortuntaly, so we turn to ratatui with the
// clap string feature.
//
// This function is a little ugly. First, we call .to_string() on the cmd info to prevent borrowing
// cmd, and then we have to call it again on our owo-colors strings.
fn add_help_info(mut cmd: Command) -> Command {
    let name = cmd.get_name().to_string();
    let version = cmd.get_version().map(str::to_string);
    let author = cmd.get_author().map(str::to_string);
    cmd = cmd.name(name.red().bold().to_string());

    if let Some(v) = version {
        cmd = cmd.version(v.italic().red().to_string());
    }

    if let Some(a) = author {
        cmd = cmd.author(a.bold().to_string());
    }

    cmd
}

fn help_template() -> String {
    "
{name} {version} 

{about} 
{usage-heading} {usage}

{all-args}{tab}

{after-help}
Made with love by {author}
    "
    .to_string()
}

pub fn get_matches(command: Command) -> ArgMatches {
    command.get_matches_from(wild::args())
}

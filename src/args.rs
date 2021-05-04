//! All argument parsing for the binary
//! It is discouraged to depened on the specifics of this behaviour as it is subject to frequent breaking changes
//! The API is public to facilitate integration testing

use crate::date::{Advent, Day, Year};
use anyhow::{anyhow, Result};
use std::{
    convert::TryFrom,
    path::{Path, PathBuf},
};

pub const HELP: &'static str = "\
cargo-advent

A simple helper utility to download inputs for the yearly advent of code competition: https://adventofcode.com
You need to first run with --set-credentials TOKEN to save the session cookie needed to authenticate to adventofcode.com
TOKEN should be the value of the session cookie; to view the session cookie use your browser's dev tools while logged in to adventofcode.com
Once you have set your credentials you won't need to do so again (unless you want to use different credentials for some reason)

USAGE:
  cargo advent [OPTIONS]
FLAGS:
  -h, --help         Prints help information
  --get-credentials  Reads the credentials from CRED_PATH and sends them to stdout
OPTIONS:
  -y, --year YEAR               The year to download input for [defaults to the current year]
  -d, --day DAY                 The day to download input for [defaults to current day only if it is December]
  -o, --output OUTPUT_PATH      The file path to download the input to [default ./input/${YEAR}/day${DAY}.txt]
                                This option only supports interpolating the values YEAR or DAY when the default is used
  --credentials-path CRED_PATH  The location to read and write credentials from [default is inside the current user's CONFIG directory]
  --set-credentials TOKEN       Use this option to store your session token, no downloading of inputs occurs if this option is set.
";

// TODO: --all-days option to download all (available) inputs for the year
#[derive(Debug)]
pub struct Args {
    year: Option<u32>,
    day: Option<u32>,
    output: Option<std::path::PathBuf>,
    credentials_path: Option<std::path::PathBuf>,
}

impl Args {
    pub fn parse_args(pargs: &mut pico_args::Arguments) -> Result<Args> {
        let year: Option<u32> = pargs.opt_value_from_str(["-y", "--year"])?;
        let day: Option<u32> = pargs.opt_value_from_str(["-d", "--day"])?;

        // opt_value_from_os_str forces us to give a result even though we don't error
        let credentials_path = pargs.opt_value_from_str("--credentials-path")?;

        let output = pargs.opt_value_from_str(["-o", "--output"])?;

        let args = Args {
            year,
            day,
            output,
            credentials_path,
        };

        Ok(args)
    }
}

pub struct CheckedArgs {
    pub year: Year,
    pub day: Day,
    pub output: std::path::PathBuf,
    pub credentials_path: std::path::PathBuf,
}

pub fn default_credentials_path() -> Result<PathBuf> {
    let dirs = directories::ProjectDirs::from("com.github", "drmason13", "cargo-advent");
    let config_path = dirs
        .ok_or(anyhow!(
            "No valid home directory path could be retrieved from the operating system.",
        ))?
        .config_dir()
        .join("session_token");

    Ok(config_path)
}

pub fn default_output_path(year: u32, day: u32) -> PathBuf {
    let cache_path = Path::new("./input")
        .join(&year.to_string())
        .join(&format!("day{}.txt", day));

    cache_path
}

impl TryFrom<Args> for CheckedArgs {
    type Error = anyhow::Error;

    fn try_from(args: Args) -> Result<Self, Self::Error> {
        // validate the day and year
        let date = Advent::new(args.year, args.day)?;
        let year = date.year();
        let day = date.day();

        let output = match args.output {
            Some(x) => x,
            None => default_output_path(year.inner(), day.inner()),
        };

        let credentials_path = match args.credentials_path {
            Some(x) => x,
            None => default_credentials_path()?,
        };

        Ok(CheckedArgs {
            year,
            day,
            output,
            credentials_path,
        })
    }
}

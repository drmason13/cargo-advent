use crate::args::{default_credentials_path, Args, CheckedArgs, HELP};
use anyhow::{Context, Result};
use std::{
    convert::TryFrom,
    fs,
    path::{Path, PathBuf},
};

/// entrypoint from main before arguments are fully parsed
pub fn entrypoint() -> Result<()> {
    let mut pargs = pico_args::Arguments::from_env();
    if pargs.contains(["-h", "--help"]) {
        print!("{}", HELP);
        std::process::exit(0);
    }

    // if --set-credentials is used, we can skip full argument parsing
    if let Some(token) = pargs.opt_value_from_str("--set-credentials")? {
        let credentials_path = match pargs.opt_value_from_str("--credentials-path")? {
            Some(x) => x,
            None => default_credentials_path()?,
        };

        return set_credentials(token, credentials_path);
    };

    // if --get-credentials is used, we can skip full argument parsing
    if pargs.contains("--get-credentials") {
        let credentials_path = match pargs.opt_value_from_str("--credentials-path")? {
            Some(x) => x,
            None => default_credentials_path()?,
        };

        return get_credentials(credentials_path);
    };

    let args = match Args::parse_args(&mut pargs) {
        Ok(v) => {
            let remaining = pargs.finish();
            if remaining.len() > 1 {
                eprintln!("Warning: unused arguments left: {:?}.", remaining);
            }
            v
        }
        Err(e) => {
            eprintln!("Error: {}.", e);
            std::process::exit(1);
        }
    };

    let args = CheckedArgs::try_from(args)?;

    run(args)
}

/// Main entrypoint, for downloading an Advent of Code input
pub fn run(
    CheckedArgs {
        year,
        day,
        output,
        credentials_path,
    }: CheckedArgs,
) -> Result<()> {
    if !output.exists() {
        let session_cookie = read_session_cookie_from_store(credentials_path)?;
        let input = download_input(day.inner(), year.inner(), session_cookie)?;
        save_input(output, input)?;
    }

    Ok(())
}

/// Entrypoint for setting Advent of Code session token
pub fn set_credentials(mut token: String, credentials_path: PathBuf) -> Result<()> {
    if !token.starts_with("session=") {
        token = format!("session={}", token)
    }

    write_nested(&credentials_path, token).with_context(|| {
        format!(
            "Failed to store credentials in file {}",
            credentials_path.to_string_lossy()
        )
    })?;
    println!(
        "Credentials saved to {}",
        credentials_path.to_string_lossy()
    );
    Ok(())
}

/// Entrypoint for getting Advent of Code session token
pub fn get_credentials(credentials_path: PathBuf) -> Result<()> {
    let session_cookie = std::fs::read_to_string(&credentials_path).with_context(|| {
        format!(
            "Failed to read credentials from file {}",
            credentials_path.to_string_lossy()
        )
    })?;
    eprintln!(
        "Credentials retrieved from: {}",
        credentials_path.to_string_lossy()
    );
    print!("{}", session_cookie);
    Ok(())
}

fn save_input(output: PathBuf, content: String) -> Result<()> {
    write_nested(&output, content)
        .with_context(|| format!("Failed to download input to {}", output.to_string_lossy()))?;
    println!("Input saved to {}", output.to_string_lossy());
    Ok(())
}

// helper function to create parent dirs as required and then write a file to a path
fn write_nested(path: &Path, content: String) -> Result<()> {
    if let Some(parent_dir) = &path.parent() {
        std::fs::create_dir_all(parent_dir)?;
    }
    Ok(fs::write(path, content)?)
}

fn download_input(day: u32, year: u32, session_cookie: String) -> Result<String> {
    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
    println!("Fetching {}", &url);
    let content: String = ureq::get(&url)
        .set("Cookie", &session_cookie)
        .call()?
        .into_string()
        .with_context(|| "Failed to download the input".to_string())?;
    Ok(content)
}

fn read_session_cookie_from_store(credentials_path: PathBuf) -> Result<String> {
    fs::read_to_string(credentials_path).with_context(|| {
        "Failed to read stored credentials\n\
            Do you need to set credentials?"
            .to_string()
    })
}

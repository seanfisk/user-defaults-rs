use anyhow::{anyhow, Result};
use env_logger::Env;
use log::info;
use std::process::Command;

fn main() -> Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("trace")).init();

    info!("Testing i64");
    let key = "i64";
    user_defaults::set_i64(key, 42)?;
    user_defaults::get_i64(key)?;

    info!("Testing f64");
    let key = "f64";
    user_defaults::set_f64(key, 123.456)?;
    user_defaults::get_f64(key)?;

    info!("Testing string");
    let key = "string";
    user_defaults::set_string(key, "lorem ipsum")?;
    user_defaults::get_string(key)?;

    info!("Testing string array");
    let key = "string-array";
    user_defaults::set_string_array(key, &["one", "two", "three"])?;
    user_defaults::get_string_array(key)?;

    info!("Dumping the whole app's defaults");
    run_defaults("read")?;

    info!("Clearing the whole app's defaults");
    run_defaults("delete")?;

    Ok(())
}

fn run_defaults(subcommand: &str) -> Result<()> {
    let status = Command::new("/usr/bin/defaults")
        .arg(&subcommand)
        .arg("com.vmware.user-defaults")
        .status()?;
    if status.success() {
        Ok(())
    } else {
        Err(match status.code() {
            Some(code) => anyhow!("defaults command failed with status {}", code),
            None => anyhow!("Process terminated by signal"),
        })
    }
}

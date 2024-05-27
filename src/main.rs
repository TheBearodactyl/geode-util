use commands::*;
use std::{env, io};

mod commands;
mod utils;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    match args[1].as_str() {
        "--help" => help()?,
        "--bump-ver" => bump_version()?,
        "--bump-sdk" => bump_sdk()?,
        "--add-repo" => add_repo()?,
        "--set-desc" => {
            let desc = &args[2];

            update_desc(desc)?;
        }
        "--set-name" => {
            let name = &args[2];

            update_name(name)?;
        }
        "--set-dev" => {
            let dev = &args[2];

            update_dev(dev)?;
        }
        "--set-gd-mac" => {
            let mac = &args[2];
            update_gd_mac(mac)?;
        }
        "--set-gd-win" => {
            let win = &args[2];
            update_gd_win(win)?;
        }
        "--set-gd-android" => {
            let android = &args[2];
            update_gd_android(android)?;
        }
        _ => help()?,
    }

    Ok(())
}

use std::{
    fs::{self, File},
    io::{self, Write},
    process::Command,
};

use crate::utils::*;
use serde_json::Value;

pub(crate) fn help() -> io::Result<()> {
    println!("Usage: geode-util [arguments]\n");
    println!("   --bump-ver:  Bump the value of the version field in mod.json");
    println!("   --bump-sdk:  Update the version of Geode the mod uses in mod.json");
    println!("   --add-repo:  Auto add the remote repo to mod.json\n");
    println!("   --set-desc <DESC>:     Set the value of description in mod.json");
    println!("   --set-dev   <DEV>:       Set the value of developer in mod.json");
    println!("   --set-name <NAME>:     Set the value of name in mod.json\n");
    println!("   --set-gd-mac         <MAC>: Set the Mac value in gd object in mod.json");
    println!("   --set-gd-win         <WIN>: Set the Windows value in gd object in mod.json");
    println!("   --set-gd-android <ANDROID>: Set the Android value in gd object in mod.json\n");
    println!("   --help:      Display this help message");

    Ok(())
}

pub(crate) fn bump_version() -> io::Result<()> {
    let data = fs::read_to_string("mod.json")?;
    let mut json: Value = serde_json::from_str(&data)?;

    if let Some(version_str) = json["version"].as_str() {
        if let Some((prefix, last_number)) = version_str.rsplit_once('.') {
            if let Ok(num) = last_number.parse::<i32>() {
                let new_version = format!("{}.{}", prefix, num + 1);
                json["version"] = Value::String(new_version);
            }
        }
    }

    let updated_data = serde_json::to_string_pretty(&json)?;
    let mut file = File::create("mod.json")?;

    file.write_all(updated_data.as_bytes())?;

    Ok(())
}

pub(crate) fn add_repo() -> io::Result<()> {
    let data = fs::read_to_string("mod.json")?;
    let mut json: Value = serde_json::from_str(&data)?;

    let output = Command::new("git")
        .args(["remote", "get-url", "origin"])
        .output()?;

    if output.status.success() {
        let url = String::from_utf8_lossy(&output.stdout).trim().to_string();
        let https_url = git_ssh_to_https(&url);
        json["repository"] = Value::String(https_url);
    }

    let updated_data = serde_json::to_string_pretty(&json)?;
    let mut file = File::create("mod.json")?;

    file.write_all(updated_data.as_bytes())?;

    Ok(())
}

pub(crate) fn bump_sdk() -> io::Result<()> {
    let data = fs::read_to_string("mod.json")?;
    let mut json: Value = serde_json::from_str(&data)?;

    let output = Command::new("sh")
        .arg("-c")
        .arg("geode sdk version | cut -d ':' -f 2 | cut -d ' ' -f 2")
        .output()?;

    if output.status.success() {
        let ver = String::from_utf8_lossy(&output.stdout).trim().to_string();
        json["geode"] = Value::String(ver);
    }

    let updated_data = serde_json::to_string_pretty(&json)?;
    let mut file = File::create("mod.json")?;

    file.write_all(updated_data.as_bytes())?;

    Ok(())
}

pub(crate) fn update_desc(desc: &str) -> io::Result<()> {
    let data = fs::read_to_string("mod.json")?;
    let mut json: Value = serde_json::from_str(&data)?;

    json["description"] = Value::String(desc.to_string());

    let updated_data = serde_json::to_string_pretty(&json)?;
    let mut file = File::create("mod.json")?;

    file.write_all(updated_data.as_bytes())?;

    Ok(())
}

pub(crate) fn update_name(name: &str) -> io::Result<()> {
    let data = fs::read_to_string("mod.json")?;
    let mut json: Value = serde_json::from_str(&data)?;

    json["name"] = Value::String(name.to_string());

    let updated_data = serde_json::to_string_pretty(&json)?;
    let mut file = File::create("mod.json")?;

    file.write_all(updated_data.as_bytes())?;

    Ok(())
}

pub(crate) fn update_dev(dev: &str) -> io::Result<()> {
    let data = fs::read_to_string("mod.json")?;
    let mut json: Value = serde_json::from_str(&data)?;

    json["developer"] = Value::String(dev.to_string());

    let updated_data = serde_json::to_string_pretty(&json)?;
    let mut file = File::create("mod.json")?;

    file.write_all(updated_data.as_bytes())?;

    Ok(())
}

pub(crate) fn update_gd_mac(mac: &str) -> io::Result<()> {
    update_gd("mac", mac)
}

pub(crate) fn update_gd_win(win: &str) -> io::Result<()> {
    update_gd("win", win)
}

pub(crate) fn update_gd_android(android: &str) -> io::Result<()> {
    update_gd("android", android)
}

fn update_gd(platform: &str, value: &str) -> io::Result<()> {
    let data = fs::read_to_string("mod.json")?;
    let mut json: Value = serde_json::from_str(&data)?;

    json["gd"][platform] = Value::String(value.to_string());

    let updated_data = serde_json::to_string_pretty(&json)?;
    let mut file = File::create("mod.json")?;

    file.write_all(updated_data.as_bytes())?;

    Ok(())
}

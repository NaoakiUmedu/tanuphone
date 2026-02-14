use serde::{Deserialize, Serialize};
use std::fs::{File, read};
use std::io::prelude::*;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub struct Setting {
    pub user: String,
    pub password: String,
    pub domain: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
    pub settings: Vec<Setting>,
}

pub fn write_file(settings: Settings) -> std::io::Result<()> {
    let path = Path::new("setting.json");
    // Rust はcreateで上書きできる
    let mut file = File::create(path)?;

    let settings_text = serde_json::to_string(&settings).unwrap();
    file.write(settings_text.as_bytes())?;

    let read_result = read_file()?;
    println!("{:?}", read_result);

    Ok(())
}

pub fn read_file() -> std::io::Result<Settings> {
    let path = Path::new("setting.json");

    let mut file = File::open(&path)?;

    let mut text = String::new();
    file.read_to_string(&mut text)?;

    let settings: Settings = serde_json::from_str(&text)?;

    Ok(settings)
}

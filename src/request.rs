use std::error::Error;
use serde_json::{Result, Value};
#[tokio::main]
pub(crate) async fn get_uuid_from_username(name: &str) {
    let url:String = "https://api.mojang.com/users/profiles/minecraft/".to_owned() + name;
    let res = reqwest::get(url)
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    let parsed: Value = serde_json::from_str(&res).unwrap();
    println!("Username: {}, Id: {}", parsed["name"].as_str().unwrap(), parsed["id"].as_str().unwrap());
}


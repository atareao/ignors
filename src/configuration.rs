// Copyright (c) 2022 Lorenzo Carbonell <a.k.a. atareao>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
use std::io::Error;
use log::{info, debug};
use serde::Deserialize;
use home::home_dir;
use toml::from_str;
use tokio::fs;

static DEFAULT_CONFIG: &'static str = include_str!("./ignors.toml");

#[derive(Deserialize, Debug)]
pub struct Configuration {
    #[serde(default = "get_default_log_level")]
    log_level: String,
    #[serde(default = "get_default_url")]
    pub url: String,
    #[serde(default = "get_default_templates")]
    pub templates: Vec<String>,
}
impl Configuration {
    pub fn get_log_level(&self) -> &str{
        &self.log_level
    }
    pub async fn new()->Self{
        let config_content = match read_file().await{
            Ok(content) => content,
            Err(_) => {
                create_default().await.unwrap()
            }
        };
        from_str(&config_content).unwrap()
    }
}
fn get_default_log_level() -> String{
    "info".to_string()
}
fn get_default_url() -> String{
    "https://github.com/github/gitignore".to_string()
}
fn get_default_templates() -> Vec<String>{
    Vec::new()
}

async fn read_file()->Result<String, Error>{
    info!("read_file");
    let mut config_dir = home_dir().unwrap();
    config_dir.push(".config");
    config_dir.push("ignors");
    config_dir.push("ignors.toml");
    debug!("config_dir: {:?}", config_dir);
    tokio::fs::read_to_string(config_dir.to_str().unwrap()).await
}

async fn create_default() -> Result<String, Error>{
    info!("created_default");
    let mut config_dir = home_dir().unwrap();
    config_dir.push(".config");
    config_dir.push("ignors");
    fs::create_dir_all(&config_dir).await?;
    config_dir.push("ignors.toml");
    fs::write(config_dir, DEFAULT_CONFIG).await?;
    Ok(DEFAULT_CONFIG.to_string())
}

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
use std::fs;
use std::io::Error;
use serde::Deserialize;
use home::home_dir;
use toml::from_str;

#[derive(Deserialize, Debug)]
pub struct Configuration {
    pub url: String,
    pub templates: Vec<String>,
}

fn read_file()->Result<String, Error>{
    let mut config_dir = home_dir().unwrap();
    config_dir.push(".config");
    config_dir.push("ignors");
    config_dir.push("ignors.toml");
    fs::read_to_string(config_dir.to_str().unwrap())
}

impl Configuration {
    pub fn new()->Self{
        let config_content = read_file().unwrap();
        from_str(&config_content).unwrap()
    }
}

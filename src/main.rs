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

mod configuration;
mod tools;

use inquire::{error::InquireError, Select};
use crate::configuration::Configuration;
use crate::tools::{fetch_url, get_gitignore, add_content_to_file};
use simple_logger::init_with_level;
use std::env;
use log::{Level, info, error};


#[tokio::main]
async fn main() {
    let configuration = Configuration::new().await;
    let log_level = match configuration.get_log_level(){
        "info" => Level::Info,
        "error" => Level::Error,
        "warn" => Level::Warn,
        "debug" => Level::Debug,
        "trace" => Level::Trace,
        _ => Level::Debug,
    };
    init_with_level(log_level).unwrap();
    
    let mut args: Vec<String> = env::args().collect();
    info!("{:?}", args);
    if args.len() > 1{
        let path = env::current_dir().unwrap();
        let gitignore_file = get_gitignore(&path).await.unwrap();
        info!("ignore file: {}", gitignore_file.to_str().unwrap());
        args.remove(0);
        add_content_to_file(
            &gitignore_file.to_str().unwrap(),
            args.iter().map(|s| &**s).collect(),
        ).await
    }else{
        let options: Vec<&str> = configuration.templates.iter().map(|s| &**s).collect();
        let ans: Result<&str, InquireError> = Select::new("Select programming language?",
                                                          options)
            .prompt();
        match ans{
            Ok(_) => {
                let url = format!("{}/{}.gitignore", configuration.url, ans.unwrap());
                let filename = ".gitignore";
                fetch_url(&url, filename)
                    .await
                    .unwrap()
            },
            Err(_) => error!("Nothing selected"),
        }
    }
}

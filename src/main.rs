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
use crate::tools::fetch_url;


#[tokio::main]
async fn main() {
    let configuration = Configuration::new();
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
        Err(_) => println!("Nothing selected"),
    }
}

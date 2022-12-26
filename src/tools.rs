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

use spinners::{Spinner, Spinners};
use std::{io::Cursor, env, path::Path};
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
use tokio::fs::read_dir;
use async_recursion::async_recursion;

pub async fn fetch_url(url: &str, filename: &str) -> Result<()> {
    let mut spinner = Spinner::new(Spinners::Dots9,
                                   "Downloading gitignore".into());
    let response = reqwest::get(url).await?;
    spinner.stop_and_persist("✔", "Downloaded!".into());
    let mut file = std::fs::File::create(filename)?;
    let mut content =  Cursor::new(response.bytes().await?);
    std::io::copy(&mut content, &mut file)?;
    Ok(())
}

#[async_recursion]
pub async fn get_git_path(path: &Path) -> Option<&Path>{
    if path.is_symlink(){
        return None;
    }
    if path.is_file(){
        return get_git_path(path.parent().unwrap()).await;
    };
    if let Some(filename) = path.file_name(){
        if filename == ".git"{
            return path.parent();
        }else if has_git(path).await{
            return Some(path);
        }else{
            return get_git_path(path.parent().unwrap()).await;
        }
    }
    None
}

pub async fn has_git(path: &Path) -> bool{
    let mut entries = read_dir(path).await.expect("Cant read dir");
    loop {
        match entries.next_entry().await{
            Ok(Some(entry)) => {
                if entry.file_name() == ".git"{
                    match entry.file_type().await{
                        Ok(filetype) => if filetype.is_dir(){
                            return true;
                        }
                        Err(_) => {},
                    }

                }
            },
            Ok(None) => {},
            Err(e) => {
                eprintln!("error: {}", e);
            }
        }
    }
}

pub async fn file_exists(path: &str) -> bool{
    if let Ok(metadata) = tokio::fs::metadata(path).await{
        return metadata.is_dir() || metadata.is_dir() || metadata.is_symlink();
    }
    false
}

pub async fn is_git_dir(path: &str) -> bool{
    if let Ok(metadata) = tokio::fs::metadata(path).await{
        return metadata.is_dir();
    }
    false
}

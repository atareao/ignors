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

use log::{error, info, debug};
use spinners::{Spinner, Spinners};
use std::{io::Cursor, path::Path, path::PathBuf};
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
use tokio::fs::{read_dir, File};
use tokio::io::AsyncWriteExt;
use async_recursion::async_recursion;

pub async fn fetch_url(url: &str, filename: &str) -> Result<()> {
    info!("fetch_url");
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
pub async fn get_gitignore(path: &Path) -> Option<PathBuf>{
    info!("get_gitignore");
    debug!("Path: {}", &path.to_string_lossy());
    if exist_gitignore_in_path(path).await || exist_git_in_path(path).await{
        let mut pathbuf = path.to_path_buf();
        pathbuf.push(".gitignore");
        Some(pathbuf)
    }else if let Some(parent) = path.parent(){
            get_gitignore(parent).await
    }else{
        None
    }
}
pub async fn exist_git_in_path(path: &Path) -> bool{
    info!("exist_git_in_path");
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
            Ok(None) => return false,
            Err(e) => {
                error!("error: {}", e);
            }
        }
    }
}
pub async fn exist_gitignore_in_path(path: &Path) -> bool{
    info!("exist_gitignore_in_path");
    let mut entries = read_dir(path).await.expect("Cant read dir");
    loop {
        match entries.next_entry().await{
            Ok(Some(entry)) => {
                if entry.file_name() == ".gitignore"{
                    match entry.file_type().await{
                        Ok(filetype) => if filetype.is_file(){
                            return true;
                        }
                        Err(_) => {},
                    }

                }
            },
            Ok(None) => return false,
            Err(e) => {
                error!("error: {}", e);
            }
        }
    }
}

pub async fn file_exists(path: &str) -> bool{
    info!("file_exists");
    if let Ok(metadata) = tokio::fs::metadata(path).await{
        return metadata.is_dir() || metadata.is_dir() || metadata.is_symlink();
    }
    false
}

pub async fn add_content_to_file(path: &str, items: Vec<&str>){
    info!("add_content_to_file");
    let content: String = if !file_exists(path).await{
        tokio::fs::read_to_string(path).await.expect("Cant read file")
    }else{
        "".to_string()
    };
    let mut contents: Vec<&str> = content.split("\n").collect();
    contents.retain(|item| !item.is_empty());
    for item in items{
        if !item.is_empty() && !contents.contains(&item){
            contents.push(item);
        }
    }
    debug!("open path: {}", path);
    let mut file = File::create(path).await.expect("Cant read file");
    let data = contents.join("\n");
    debug!("content: {}", data);
    file.write_all(data.as_bytes()).await.expect("Cant write to file");
    file.write_all(b"\n").await.expect("Cant write to file");
    file.flush().await.expect("Cant flush file");
    info!("writed");
}

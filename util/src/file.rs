use std::path::PathBuf;
/*
use chrono::prelude::*;
use dirs::home_dir;*/

// 创建文件夹
pub fn check_create_path(path: PathBuf) -> String {
    // let path = std::env::current_dir().unwrap();
    // 获取当前文件的根目录
    //拼接文件夹的路径
    let bible = match path.to_str() {
        None => "0",
        Some(s) => s
    };
    std::fs::create_dir_all(&bible).unwrap();
    // 创建文件夹，不存在则创建，存在则不管 ， 并返回 文件夹的路径
    bible.to_string()
}



// 验证文件是否存在 fn checkFile(biblePath: &str) -> i32 { let f = File::open(biblePath); let result = match f { Ok(file) => 1, Err(err) => 0 }; result }
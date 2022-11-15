use std::collections::HashMap;
use std::ffi::OsStr;
use std::path::Path;
use std::{fs};
use regex::Regex;

use super::file_util;

pub fn format(path: &Path) {
    if crate::CONFIG.format.enable == false {
        return;
    }
    traversal_file(path);
}

pub fn traversal_file(path: &Path) {
    if path.is_dir() {
        for child in fs::read_dir(path).unwrap() {
            let child = child.unwrap().path();
            traversal_file(&child);
        }
    } else if path.extension().and_then(OsStr::to_str) == Some("md") {
        format_md(path);
    }
}

pub fn format_md(path: &Path) {
    // 隐藏代码块和行内代码
    let map = code_hide(path);
    if crate::CONFIG.format.newline_character_convert {
        
    }
    if crate::CONFIG.format.clear_html_tag {
        
    }
    if crate::CONFIG.format.resolve_img {
        resolve_img(path);
    }
    code_show(path, map);
}

pub fn code_hide(path: &Path) -> HashMap<String, String> {
    let text = file_util::read_file(path).unwrap();
    let mut copy = text.clone();
    let mut count = 1000;
    let mut map = HashMap::default();
    let re = Regex::new(r"```((.|\n)*?)```").unwrap();
    for caps in re.captures_iter(&text) {
        let code = caps.get(1).unwrap().as_str();
        // println!("code: {}", code);
        let from = format!("```{}```", code);
        let to = format!("<<<<<<<<<<<{}>>>>>>>>>>>", count);
        copy = copy.replace(&from, &to);
        map.insert(to, from);
        count += 1;
    }
    let copy2 = copy.clone();
    let re = Regex::new(r"`(.*?)`").unwrap();
    for caps in re.captures_iter(&copy2) {
        let code = caps.get(1).unwrap().as_str();
        // println!("inline-code: {}", code);
        let from = format!("`{}`", code);
        let to = format!("<<<<<<<<<<<{}>>>>>>>>>>>", count);
        copy = copy.replace(&from, &to);
        map.insert(to, from);
        count += 1;
    }
    file_util::write_file(path, &copy);
    map
}

pub fn code_show(path: &Path, map: HashMap<String, String>) {
    let mut text = file_util::read_file(path).unwrap();
    for (from, to) in map {
        text = text.replace(&from, &to);
    }
    file_util::write_file(path, &text)
}

pub fn resolve_img(path: &Path) {
    let text = file_util::read_file(path).unwrap();
    let mut copy = text.clone();
    let re = Regex::new(r"!\[(.*?)\]\((.*?)\)").unwrap();
    for caps in re.captures_iter(&text) {
        let url = caps.get(2).unwrap().as_str();
        let name = *url.split("/").collect::<Vec<&str>>().last().unwrap().split("#").collect::<Vec<&str>>().first().unwrap();
        // println!("name: {}, url: {}", name, url);
        let filepath = path.parent().unwrap().join("upload").join("yuque_img").join(name);
        // 下载图片
        let _ = file_util::download_file(url, filepath.as_path());
        // 将 url 替换成相对路径图片地址，也就是 upload/yuque_img/uuid.png
        copy = copy.replace(url, &format!("upload/yuque_img/{}", name));
    }
    file_util::write_file(path, &copy)
}

#[cfg(test)]
mod format_test {
    use super::*;

    #[test]
    fn format_test() {
        format(Path::new(r"D:\Temp\outline"));
    }
    
    #[test]
    fn code_hide_test() {
        code_hide(Path::new(r"E:\Temp\Debian11纯命令行版本安装及后续工作 (2).md"));
    }
    
}
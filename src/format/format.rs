use std::ffi::OsStr;
use std::path::Path;
use std::{fs, fs::File};
use std::io::{Read, Write, BufReader, BufRead};
use regex::Regex;

pub fn download(path: &Path) {
    download2(path, path.clone());
}

pub fn download2(path: &Path, path2: &Path) {
    if path2.is_dir() {
        for child in fs::read_dir(path2).unwrap() {
            let child = child.unwrap().path();
            download2(path, &child);
        }
    } else if path2.extension().and_then(OsStr::to_str) == Some("md") {
        // println!("file: {}", path2.display());
        let text = read_file(path2).unwrap();
        let re = Regex::new(r"!\[(.*?)\]\((.*?)\)").unwrap();
        for caps in re.captures_iter(&text) {
            let url = caps.get(2).unwrap().as_str();
            let name = *url.split("/").collect::<Vec<&str>>().last().unwrap().split("#").collect::<Vec<&str>>().first().unwrap();
            // println!("name: {}, url: {}", name, url);
            let _ = download_file(url, path.join("img").join(name).as_path());
        }
    }
}

#[tokio::main]
pub async fn download_file(url: &str, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let body = client.get(url).header("User-Agent", &crate::CONFIG.user_agent).send().await?
        .bytes().await?;
    let dir = path.parent();
    if None != dir {
        let dir = dir.unwrap();
        fs::create_dir_all(dir).unwrap_or_else(|e| {
            panic!("Could not create file directory: {}, {:?}", dir.display(), e)
        });
    }
    let mut file = File::create(path).unwrap_or_else(|e| {
        panic!("Could not create file: {:?}", e);
    });
    let content = body.bytes();
    let data: Result<Vec<_>, _> = content.collect();
    file.write_all(&data.unwrap())?;

    Ok(())
}

pub fn read_file(path: &Path) -> Result<String, Box<dyn std::error::Error>> {
    let file: File = File::open(path)?;
    let mut res = String::default();
    let lines = BufReader::new(file).lines();
    for line in lines {
        if let Ok(x) = line {
            res.push_str(&x);
        }
    }
    Ok(res)
}

#[cfg(test)]
mod format_test {
    use super::*;

    #[test]
    fn download_test() {
        download(Path::new(r"D:\Temp\outline"));
    }
    
    #[test]
    fn download_file_test() {
        download_file(r"https://cdn.nlark.com/yuque/0/2022/png/87167/1656408445119-82d71d17-13fc-46c9-a251-cdd1f5d7dbb4.png#clientId=uc8d82d67-efe8-4&crop=0&crop=0&crop=1&crop=1&from=paste&height=591&id=u1dfa441e&margin=%5Bobject%20Object%5D&name=image.png&originHeight=886&originWidth=1761&originalType=binary&ratio=1&rotation=0&showTitle=false&size=121278&status=done&style=none&taskId=u3e074774-de4e-48cd-ac55-b8f43728545&title=&width=1174", Path::new(r"D:\Temp\outline\1.png"));
    }
    
}
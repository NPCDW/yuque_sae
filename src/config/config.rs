use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ResolveImg {
    pub enable: bool,
    pub img_path: String,
    pub img_url_prefix: String,
    pub replace_url_to_local: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Format {
    pub enable: bool,
    pub newline_character_convert: bool,
    pub clear_html_tag: bool,
    pub resolve_img: ResolveImg,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Person {
    pub enable: bool,
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Space {
    pub enable: bool,
    pub domain: String,
    pub cookie: String,
    pub x_csrf_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Download {
    pub person: Person,
    pub space: Space,
    pub user_agent: String,
    pub timeout: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub download: Download,
    pub source_path: String,
    pub format: Format,
}

lazy_static! {
    #[derive(Debug)]
    pub static ref CONFIG: Config = {
        use std::fs::File;
        use std::io::prelude::*;

        let dir = std::env::current_dir().unwrap_or_else(|e| {
            panic!("获取程序目录失败：{:?}", e);
        });
        let filepath = dir.join("config.yml");
        let mut f = File::open(&filepath).unwrap_or_else(|e| {
            panic!("配置文件 {} 打开失败：{:?}", filepath.display(), e);
        });
        let mut buf = String::new();
        f.read_to_string(&mut buf).unwrap_or_else(|e| {
            panic!("配置文件 {} 读取失败：{:?}", filepath.display(), e);
        });
        serde_yaml::from_str(&buf).unwrap_or_else(|e| {
            panic!("配置文件 {} 转 yaml 格式失败：{:?}", filepath.display(), e);
        })
    };
}

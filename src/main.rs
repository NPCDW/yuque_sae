mod config;
mod format;
mod api;
mod util;
#[macro_use]
extern crate lazy_static;
pub use crate::config::config::CONFIG;

fn main() {
    let root_dir = std::env::current_dir().unwrap_or_else(|e| {
        panic!("获取程序目录失败：{:?}", e);
    });
    let source_path = root_dir.join(&CONFIG.source_path);

    api::download_yuque::download(&source_path);
    format::format::format(&source_path);
    
    println!("执行完成，资源目录：{}", source_path.display());
}

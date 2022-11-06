mod config;
mod api;
#[macro_use]
extern crate lazy_static;
pub use crate::config::config::CONFIG;
use std::{fs, fs::File};
use std::io::Write;

fn main() {
    let root_dir = std::env::current_dir().unwrap_or_else(|e| {
        panic!("获取程序目录失败：{:?}", e);
    });

    println!("获取用户信息...");
    let user_info = api::api::get_user_info().unwrap_or_else(|e| {
        panic!("获取用户信息失败：{:?}", e);
    });

    let user_dir = root_dir.join(user_info.data.login);
    fs::create_dir_all(&user_dir).unwrap_or_else(|e| {
        panic!("Could not create file directory: {}, {:?}", &user_dir.display(), e)
    });

    println!("获取用户知识库...");
    let repos_data = api::api::get_user_repos(user_info.data.id).unwrap_or_else(|e| {
        panic!("获取用户知识库失败：{:?}", e);
    });
    for repos in repos_data.data {
        if repos.type_ != "Book" {
            continue;
        }
        
        println!("导出知识库 > {}", repos.name);
        let user_repos_dir = user_dir.join(repos.name.replace("/", "-").replace("\\", "-").replace(":", "-").replace("*", "-").replace("?", "-").replace("\"", "-").replace("<", "-").replace(">", "-").replace("|", "-"));
        fs::create_dir_all(&user_repos_dir).unwrap_or_else(|e| {
            panic!("Could not create file directory: {}, {:?}", &user_repos_dir.display(), e)
        });

        let docs_data = api::api::get_repos_docs(repos.id).unwrap_or_else(|e| {
            panic!("获取知识库文档列表失败：{:?}", e);
        });
        for doc in docs_data.data {
            println!("导出文档 > {}", doc.title);
            let doc = api::api::get_doc(repos.id, doc.id).unwrap_or_else(|e| {
                panic!("获取文档详情失败：{:?}", e);
            });
            let doc = doc.data;

            let doc_path = user_repos_dir.join(doc.title.replace("/", "-").replace("\\", "-").replace(":", "-").replace("*", "-").replace("?", "-").replace("\"", "-").replace("<", "-").replace(">", "-").replace("|", "-") + ".md");
            let mut file: File = File::create(&doc_path).unwrap_or_else(|e| {
                panic!("Could not create file: {:?}", e);
            });
            file.write(doc.body.as_bytes()).unwrap_or_else(|e| {
                panic!("Write file: {:?}", e);
            });
            file.flush().unwrap_or_else(|e| {
                panic!("Flush file: {:?}", e);
            });
        }
    }
}

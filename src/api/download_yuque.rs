use std::path::PathBuf;
use super::api;
use crate::util::file_util;

pub fn download() -> PathBuf {
    let root_dir = std::env::current_dir().unwrap_or_else(|e| {
        panic!("获取程序目录失败：{:?}", e);
    });

    println!("获取用户信息...");
    let user_info = api::get_user_info().unwrap_or_else(|e| {
        panic!("获取用户信息失败：{:?}", e);
    });

    let user_dir = root_dir.join(user_info.data.login);
    file_util::create_dir(&user_dir);

    println!("获取用户知识库...");
    let repos_data = api::get_user_repos(user_info.data.id).unwrap_or_else(|e| {
        panic!("获取用户知识库失败：{:?}", e);
    });
    for repos in repos_data.data {
        if repos.type_ != "Book" {
            continue;
        }
        
        println!("导出知识库 > {}", repos.name);
        let user_repos_dir = user_dir.join(repos.name.replace("/", "-").replace("\\", "-").replace(":", "-").replace("*", "-").replace("?", "-").replace("\"", "-").replace("<", "-").replace(">", "-").replace("|", "-"));
        file_util::create_dir(&user_repos_dir);

        let docs_data = api::get_repos_docs(repos.id).unwrap_or_else(|e| {
            panic!("获取知识库文档列表失败：{:?}", e);
        });
        for doc in docs_data.data {
            println!("导出文档 > {}", doc.title);
            let doc = api::get_doc(repos.id, doc.id).unwrap_or_else(|e| {
                panic!("获取文档详情失败：{:?}", e);
            });
            let doc = doc.data;

            let doc_path = user_repos_dir.join(doc.title.replace("/", "-").replace("\\", "-").replace(":", "-").replace("*", "-").replace("?", "-").replace("\"", "-").replace("<", "-").replace(">", "-").replace("|", "-") + ".md");
            file_util::write_file(&doc_path, &doc.body);
        }
    }
    user_dir
}
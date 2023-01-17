use std::path::Path;

use super::space_api;
use crate::util::file_util;

pub fn download(source_path: &Path) {
    if crate::CONFIG.download.space.enable == false {
        println!("语雀空间版导出功能未启用");
        return;
    }

    println!("语雀空间版导出...");

    println!("获取空间知识库...");
    let repos_data = space_api::get_books().unwrap_or_else(|e| {
        panic!("获取空间知识库失败：{:?}", e);
    });
    for repos in repos_data.data {
        if repos.type_ != "Book" {
            continue;
        }
        
        println!("导出知识库 > {}", repos.name);
        let user_repos_dir = source_path.join(format!("{}/{}/{}", format_filename(repos.user.organization.name), format_filename(repos.user.name), format_filename(repos.name)));
        file_util::create_dir(&user_repos_dir);

        let docs_data = space_api::get_docs(repos.id).unwrap_or_else(|e| {
            panic!("获取知识库文档列表失败：{:?}", e);
        });
        for doc in docs_data.data {
            if doc.type_.as_str() != "Doc" {
                println!("跳过文档 > {}", doc.title);
                continue;
            }
            println!("导出文档 > {}", doc.title);
            let doc_export = space_api::export_doc(doc.id).unwrap_or_else(|e| {
                panic!("导出文档失败：{:?}", e);
            });
            let body = space_api::get_doc(doc_export.data.url).unwrap_or_else(|e| {
                panic!("获取文档数据失败：{:?}", e);
            });

            let doc_path = user_repos_dir.join(format_filename(doc.title) + ".md");
            file_util::write_file(&doc_path, &body);
        }
    }
}

fn format_filename(filename: String) -> String {
    filename.replace("/", "-").replace("\\", "-").replace(":", "-").replace("*", "-").replace("?", "-").replace("\"", "-").replace("<", "-").replace(">", "-").replace("|", "-")
}
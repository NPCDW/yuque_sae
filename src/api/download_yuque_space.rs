use std::path::Path;
use substring::Substring;

use super::space_api;
use crate::util::file_util;

pub fn download(source_path: &Path) {
    if crate::CONFIG.download.person.enable == false {
        println!("语雀空间版导出功能未启用");
        return;
    }

    println!("语雀空间版导出...");
    file_util::create_dir(crate::CONFIG.download.space.domain.as_str().substring(2,5));

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
            println!("导出文档 > {}", doc.title);
            let doc = api::get_doc(repos.id, doc.id).unwrap_or_else(|e| {
                panic!("获取文档详情失败：{:?}", e);
            });
            let doc = doc.data;

            let doc_path = user_repos_dir.join(doc.title.replace("/", "-").replace("\\", "-").replace(":", "-").replace("*", "-").replace("?", "-").replace("\"", "-").replace("<", "-").replace(">", "-").replace("|", "-") + ".md");
            file_util::write_file(&doc_path, &doc.body);
        }
    }
}

fn format_filename(filename: String) -> String {
    filename.replace("/", "-").replace("\\", "-").replace(":", "-").replace("*", "-").replace("?", "-").replace("\"", "-").replace("<", "-").replace(">", "-").replace("|", "-")
}
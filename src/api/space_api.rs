use std::time::Duration;
// use serde_json::value::Value;
use serde::{Serialize, Deserialize};

fn get_client() -> reqwest::Client {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("User-Agent", reqwest::header::HeaderValue::from_static(&crate::CONFIG.download.user_agent));
    headers.insert("cookie", reqwest::header::HeaderValue::from_static(&crate::CONFIG.download.space.cookie));
    headers.insert("x-csrf-token", reqwest::header::HeaderValue::from_static(&crate::CONFIG.download.space.x_csrf_token));
    reqwest::Client::builder().default_headers(headers).timeout(Duration::from_secs(crate::CONFIG.download.timeout)).build().unwrap()
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseBase<T> {
    pub data: T,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Organization {
    pub id: u32,
    pub name: String,
    pub login: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub organization: Organization,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Book {
    pub id: u32,
    pub name: String,
    pub slug: String,
    #[serde(alias = "type")]
    pub type_: String,
    pub user: User,
}

#[tokio::main]
pub async fn get_books() -> Result<ResponseBase<Vec<Book>>, Box<dyn std::error::Error>> {
    let url = format!("{}/api/mine/user_books?limit=30&offset=0&user_type=Group", &crate::CONFIG.download.space.domain); 
    let body = get_client().get(url).send().await?.json::<ResponseBase<Vec<Book>>>().await?;
    Ok(body)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Doc {
    pub id: u32,
    pub slug: String,
    pub title: String,
    #[serde(alias = "type")]
    pub type_: String,
}

#[tokio::main]
pub async fn get_docs(repos_id: u32) -> Result<ResponseBase<Vec<Doc>>, Box<dyn std::error::Error>> {
    let url = format!("{}/api/docs?book_id={}", &crate::CONFIG.download.space.domain, repos_id);
    let body = get_client().get(url).send().await?.json::<ResponseBase<Vec<Doc>>>().await?;
    Ok(body)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DocExport {
    pub url: String,
    pub state: String,
}

#[tokio::main]
pub async fn export_doc(doc_id: u32) -> Result<ResponseBase<DocExport>, Box<dyn std::error::Error>> {
    let url = format!("{}/api/docs/{}/export", &crate::CONFIG.download.space.domain, doc_id);
    let body = get_client()
            .post(url)
            .header("Content-Type", "application/json")
            .body("{\"type\":\"markdown\",\"force\":0,\"options\":\"{\\\"latexType\\\":2,\\\"enableBreak\\\":1}\"}")
            .send().await?.json::<ResponseBase<DocExport>>().await?;
    Ok(body)
}

#[tokio::main]
pub async fn get_doc(url: String) -> Result<String, Box<dyn std::error::Error>> {
    let body = get_client().get(url).send().await?.text().await?;
    Ok(body)
}

#[cfg(test)]
mod api_test {
    use super::*;

    #[test]
    fn get_books_test() {
        let res = get_books();
        println!("{:#?}", res);
        assert_eq!(true, res.is_ok());
    }
    
    #[test]
    fn get_docs_test() {
        let res = get_docs(20214876);
        println!("{:#?}", res);
        assert_eq!(true, res.is_ok());
    }
    
    #[test]
    fn export_doc_test() {
        let res = export_doc(47477825);
        println!("{:#?}", res);
        assert_eq!(true, res.is_ok());
    }
    
    #[test]
    fn get_doc_test() {
        let doc_export = export_doc(47477825).unwrap();
        let res = get_doc(doc_export.data.url);
        println!("{:#?}", res);
        assert_eq!(true, res.is_ok());
    }
    
}

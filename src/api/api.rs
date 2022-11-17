use std::time::Duration;
// use serde_json::value::Value;
use serde::{Serialize, Deserialize};

const YUQUE_BASE_URL: &str = "https://www.yuque.com/api/v2";

fn get_client() -> reqwest::Client {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("User-Agent", reqwest::header::HeaderValue::from_static(&crate::CONFIG.download.user_agent));
    headers.insert("X-Auth-Token", reqwest::header::HeaderValue::from_static(&crate::CONFIG.download.token));
    reqwest::Client::builder().default_headers(headers).timeout(Duration::from_secs(crate::CONFIG.download.timeout)).build().unwrap()
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseBase<T> {
    pub data: T,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInfo {
    pub id: u32,
    pub login: String,
    pub name: String,
}

#[tokio::main]
pub async fn get_user_info() -> Result<ResponseBase<UserInfo>, Box<dyn std::error::Error>> {
    let url = format!("{YUQUE_BASE_URL}/user"); 
    let body = get_client().get(url).send().await?.json::<ResponseBase<UserInfo>>().await?;
    Ok(body)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Repository {
    pub id: u32,
    pub name: String,
    #[serde(alias = "type")]
    pub type_: String,
    pub namespace: String,
}

#[tokio::main]
pub async fn get_user_repos(user_id: u32) -> Result<ResponseBase<Vec<Repository>>, Box<dyn std::error::Error>> {
    let url = format!("{YUQUE_BASE_URL}/users/{user_id}/repos");
    let body = get_client().get(url).send().await?.json::<ResponseBase<Vec<Repository>>>().await?;
    Ok(body)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Docs {
    pub id: u32,
    pub slug: String,
    pub title: String,
}

#[tokio::main]
pub async fn get_repos_docs(repos_id: u32) -> Result<ResponseBase<Vec<Docs>>, Box<dyn std::error::Error>> {
    let url = format!("{YUQUE_BASE_URL}/repos/{repos_id}/docs");
    let body = get_client().get(url).send().await?.json::<ResponseBase<Vec<Docs>>>().await?;
    Ok(body)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Doc {
    pub id: u32,
    pub slug: String,
    pub title: String,
    pub body: String,
    pub body_html: String,
}

#[tokio::main]
pub async fn get_doc(repos_id: u32, doc_id: u32) -> Result<ResponseBase<Doc>, Box<dyn std::error::Error>> {
    let url = format!("{YUQUE_BASE_URL}/repos/{repos_id}/docs/{doc_id}");
    let body = get_client().get(url).send().await?.json::<ResponseBase<Doc>>().await?;
    Ok(body)
}

#[cfg(test)]
mod api_test {
    use super::*;

    #[test]
    fn get_user_info_test() {
        let res = get_user_info();
        println!("{:#?}", res);
        assert_eq!(true, res.is_ok());
    }
    
    #[test]
    fn get_user_repos_test() {
        let res = get_user_repos(87167);
        println!("{:#?}", res);
        assert_eq!(true, res.is_ok());
    }
    
    #[test]
    fn get_repos_docs_test() {
        let res = get_repos_docs(2097236);
        println!("{:#?}", res);
        assert_eq!(true, res.is_ok());
    }
    
    #[test]
    fn get_doc_test() {
        let res = get_doc(2097236, 42387372);
        println!("{:#?}", res);
        assert_eq!(true, res.is_ok());
    }
    
}

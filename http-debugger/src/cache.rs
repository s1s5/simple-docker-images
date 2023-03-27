use bytes::Bytes;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

#[derive(Serialize, Deserialize)]
struct Meta {
    uri: String,
    req_headers: HashMap<String, String>,
    #[serde(with = "serde_bytes")]
    req_body: Vec<u8>,

    res_headers: HashMap<String, String>,
}

pub struct Cache {
    root: PathBuf,
}

impl Cache {
    pub fn new(root: &Path) -> Cache {
        Cache { root: root.into() }
    }

    fn get_cache_path(&self, uri: &str, referer: &Option<String>) -> PathBuf {
        let parsed = url::Url::parse(uri).unwrap();
        let host = parsed.host_str().unwrap();
        let path = if parsed.path().ends_with("/") {
            format!("{}index.html", parsed.path())
        } else {
            parsed.path().into()
        };
        let path = format!(
            "{}{}",
            path,
            parsed
                .query()
                .map(|x| format!("?{}", x))
                .unwrap_or("".into())
        );
        let root = if let Some(referer) = referer.clone() {
            let referer_parsed = url::Url::parse(&referer).unwrap();
            let referer_host = referer_parsed.host_str().unwrap();
            if referer_host != host {
                self.root.join(referer_host)
            } else {
                self.root.clone()
            }
        } else {
            self.root.clone()
        };
        root.join(host).join(split_left_slash(&path))
    }

    pub async fn get(
        &self,
        uri: &str,
        referer: &Option<String>,
    ) -> Option<(HashMap<String, String>, Bytes)> {
        let cache_path = self.get_cache_path(uri, referer);
        // println!("get cache {} -> {:?}", uri, &cache_path);
        let _file_metadata = tokio::fs::metadata(&cache_path).await.ok()?;
        let metadata = read_meta(&cache_path).await.ok()?;
        let file_bytes = read_all(&cache_path).await.ok()?;
        // TODO: ファイル最終更新日とかを修正
        Some((metadata.res_headers, file_bytes))
    }

    pub async fn set(
        &self,
        uri: &str,
        referer: &Option<String>,
        req_headers: &HashMap<String, String>,
        req_body: &Bytes,
        res_headers: &HashMap<String, String>,
        res_body: &Bytes,
    ) -> Result<(), std::io::Error> {
        let cache_path = self.get_cache_path(uri, referer);
        // println!("set cache {} -> {:?}", uri, &cache_path);
        let meta = Meta {
            uri: uri.into(),
            req_headers: req_headers.clone(),
            req_body: req_body.clone().to_vec(),
            res_headers: res_headers.clone(),
        };
        let mut dir_path = cache_path.clone();
        dir_path.pop();
        tokio::fs::create_dir_all(dir_path).await?;
        write_meta(&cache_path, &meta).await?;
        write_all(&cache_path, res_body.clone()).await?;
        Ok(())
    }

    pub async fn remove(&self, uri: &str, referer: &Option<String>) -> Result<(), std::io::Error> {
        let cached_path = self.get_cache_path(uri, referer);
        let meta_path = get_meta_path(&cached_path);
        tokio::fs::remove_file(meta_path).await?;
        tokio::fs::remove_file(cached_path).await?;
        Ok(())
    }
}

fn split_left_slash(path: &str) -> String {
    if path.starts_with("/") {
        path[1..].to_string()
    } else {
        path.to_string()
    }
}

fn get_meta_path(path: &Path) -> String {
    format!("{}-meta.json", path.as_os_str().to_str().unwrap())
}

async fn read_meta(path: &Path) -> Result<Meta, std::io::Error> {
    let path = get_meta_path(path);
    let data = tokio::fs::read_to_string(path).await?;
    Ok(serde_json::from_str(&data)?)
}

async fn write_meta(path: &Path, meta: &Meta) -> Result<(), std::io::Error> {
    let path = get_meta_path(path);
    // let file = tokio::fs::File::create(&path).await?; // std::io::Error の可能性
    tokio::fs::write(path, serde_json::to_string_pretty(meta).unwrap().as_bytes()).await?;
    // serde_json::to_writer_pretty(file, meta)?; // serde_json::Error の可能性
    Ok(())
}

async fn read_all(path: &Path) -> Result<Bytes, std::io::Error> {
    Ok(Bytes::from_iter(tokio::fs::read(path).await?))
    // let mut file = std::fs::File::open(path)?;
    // let mut buf: Vec<u8> = Vec::new();
    // let _ = file.read_to_end(&mut buf)?;
    // Ok(buf.into())
}

async fn write_all(path: &Path, bytes: Bytes) -> Result<(), std::io::Error> {
    // let file = tokio::fs::File::create(&path).await?;
    tokio::fs::write(path, &bytes).await?;
    // let mut file = std::fs::File::create(path)?;
    // file.write_all(&bytes)?;
    // file.flush()?;
    Ok(())
}

use super::cache::Cache;
use super::cert_manager::CertManager;
use bytes::Bytes;
use http::{HeaderName, HeaderValue};
use log::info;
use std::collections::HashMap;
use std::path::Path;
use std::sync::{Arc, Mutex};

pub struct ProxyApp {
    client: reqwest::Client,
    cert_manager: Mutex<CertManager>,
    cache: Cache,
}

impl ProxyApp {
    pub fn new(cache_root: &Path, root_key_path: &str, root_ca_path: &str) -> ProxyApp {
        ProxyApp {
            client: reqwest::Client::new(),
            cert_manager: Mutex::new(CertManager::new(root_key_path, root_ca_path)),
            cache: Cache::new(cache_root),
        }
    }
    pub fn get_tls_server_config(&self, domain: &str) -> Arc<rustls::ServerConfig> {
        self.cert_manager.lock().unwrap().get_server_config(domain)
    }

    pub async fn handle(
        &self,
        uri: String,
        method: &http::Method,
        headers: &HashMap<String, String>,
        body: Bytes,
    ) -> Result<(HashMap<String, String>, Bytes), Box<dyn std::error::Error + Send + Sync>> {
        let referer = headers.get("referer").map(|x| x.clone());
        if method == http::Method::GET {
            if let Some(res) = self.cache.get(&uri, &referer).await {
                info!(
                    "return from cache uri={} referer={:?}",
                    uri,
                    referer.unwrap_or("".to_string())
                );
                return Ok(res);
            }
        }
        let mut req_headers = reqwest::header::HeaderMap::new();
        headers
            .iter()
            .filter(|x| x.0.as_str().to_lowercase() != "host")
            .for_each(|(key, value)| {
                req_headers.insert(
                    HeaderName::from_bytes(key.as_bytes()).unwrap(),
                    HeaderValue::from_str(value).unwrap(),
                );
            });

        let response = self
            .client
            .request(method.clone(), uri.clone())
            .headers(req_headers)
            .body(body.clone())
            .send()
            .await?;
        let res_status_code = response.status().as_u16();
        let mut res_headers: HashMap<String, String> = HashMap::new();
        response.headers().iter().for_each(|(key, value)| {
            res_headers.insert(key.to_string(), value.to_str().unwrap().into());
        });
        let res_body = response.bytes().await?;
        if method == http::Method::GET && res_status_code == 200 {
            self.cache
                .set(&uri, &referer, &headers, &body, &res_headers, &res_body)
                .await?;
        }
        Ok((res_headers, res_body))
    }
}

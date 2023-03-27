use super::app::ProxyApp;
use bytes::Bytes;
use http::{HeaderName, HeaderValue};
use http_body_util::{BodyExt, Full};
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::upgrade::Upgraded;
use hyper::{Method, Request, Response};
use log::{error, info};
use std::collections::HashMap;
use std::sync::Arc;

pub async fn proxy(
    req: Request<hyper::body::Incoming>,
    base_uri: Option<String>,
    proxy_app: Arc<ProxyApp>,
) -> Result<Response<Full<Bytes>>, Box<dyn std::error::Error + Send + Sync>> {
    info!("proxy req: {:?}, base: {:?}", req, base_uri);

    let (res_headers, res_body) = {
        let uri = format!("{}{}", base_uri.unwrap_or("".into()), req.uri());

        let mut headers: HashMap<String, String> = HashMap::new();
        req.headers().iter().for_each(|(key, value)| {
            if key.as_str().to_lowercase() != "host" {
                headers.insert(key.to_string(), value.to_str().unwrap().into());
            }
        });
        let method = req.method().clone();
        let res_body = req.collect().await?.to_bytes();
        proxy_app.handle(uri, &method, &headers, res_body).await?
    };

    let mut ret = Response::new(Full::new(Bytes::from(res_body)));

    res_headers.iter().for_each(|(key, value)| {
        ret.headers_mut().append(
            HeaderName::from_bytes(key.as_bytes()).unwrap(),
            HeaderValue::from_str(value).unwrap(),
        );
    });

    Ok(ret)
}

pub async fn upgradable_proxy(
    req: Request<hyper::body::Incoming>,
    proxy_app: Arc<ProxyApp>,
) -> Result<Response<Full<Bytes>>, Box<dyn std::error::Error + Send + Sync>> {
    if Method::CONNECT == req.method() {
        // Received an HTTP request like:
        // ```
        // CONNECT www.domain.com:443 HTTP/1.1
        // Host: www.domain.com:443
        // Proxy-Connection: Keep-Alive
        // ```
        //
        // When HTTP method is CONNECT we should return an empty body
        // then we can eventually upgrade the connection and talk a new protocol.
        //
        // Note: only after client received an empty body with STATUS_OK can the
        // connection be upgraded, so we can't return a response inside
        // `on_upgrade` future.
        // if let Some(addr) = host_addr(req.uri()) {
        if let Some(host) = req.uri().host() {
            let domain = host.to_string();
            let port = req.uri().port_u16();
            tokio::task::spawn(async move {
                match hyper::upgrade::on(req).await {
                    Ok(upgraded) => {
                        if let Err(e) = tunnel(upgraded, proxy_app, domain, port).await {
                            error!("server io error: {}", e);
                        };
                    }
                    Err(e) => error!("upgrade error: {}", e),
                }
            });

            Ok(Response::new(Full::new(Bytes::from(""))))
        } else {
            error!("CONNECT host is not socket addr: {:?}", req.uri());
            Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "invalid host").into())
        }
    } else {
        proxy(req, None, proxy_app).await
    }
}

async fn tunnel(
    upgraded: Upgraded,
    proxy_app: Arc<ProxyApp>,
    domain: String,
    port: Option<u16>,
) -> std::io::Result<()> {
    let base_uri = format!(
        "https://{}{}",
        domain,
        port.filter(|x| (*x) != 443)
            .map(|x| format!(":{}", x))
            .unwrap_or("".into())
    );
    let tls_cfg = (*proxy_app).get_tls_server_config(&domain);
    let tls_stream = super::tls_stream::TlsStream::new(upgraded, tls_cfg);

    tokio::task::spawn(async move {
        let service = service_fn(move |req| {
            let base_uri = base_uri.clone();
            let proxy_app = proxy_app.clone();
            proxy(req, Some(base_uri), proxy_app)
        });
        if let Err(_err) = http1::Builder::new()
            .preserve_header_case(true)
            .title_case_headers(true)
            .serve_connection(tls_stream, service)
            .with_upgrades()
            .await
        {
            // do nothing
        }
    });

    Ok(())
}

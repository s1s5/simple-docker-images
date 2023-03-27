use super::app::ProxyApp;
use bytes::Bytes;
use http::{HeaderName, HeaderValue};
use http_body_util::{BodyExt, Full};
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::upgrade::Upgraded;
use hyper::{Method, Request, Response};
use log::info;
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

    // println!(
    //     "req: {:?}, path={}, base_uri={:?}",
    //     req,
    //     req.uri().path(),
    //     base_uri
    // );

    // println!("request = {:?}", req);
    // if req.method() == hyper::Method::CONNECT {
    //     return Ok(Response::new(Full::new(Bytes::from(
    //         "connection established",
    //     ))));
    // }

    // let client = reqwest::Client::new();

    // // let resp = client
    // //     .request(reqwest::Method::GET, "https://google.co.jp")
    // //     .send()
    // //     .await?
    // //     .json::<std::collections::HashMap<String, String>>()
    // //     .await?;
    // // println!("test: {:#?}", resp);

    // let mut headers = reqwest::header::HeaderMap::new();
    // req.headers().iter().for_each(|(key, value)| {
    //     if key.as_str().to_lowercase() != "host" {
    //         headers.insert(key, value.into());
    //     }
    // });
    // // println!("scheme: {:?}", req.uri().scheme());
    // let method = req.method().clone();
    // // let uri = req.uri().to_string();
    // // let uri = if uri.ends_with(":443") {
    // //     format!("https://{}", &uri[..uri.len() - 4])
    // // } else {
    // //     uri
    // // };
    // let uri = format!("{}{}", base_uri.unwrap_or("".into()), req.uri());
    // let bytes = req.collect().await?.to_bytes();
    // println!(
    //     "connect: method={:?}, uri={:?}, headers={:?}, body={:?}",
    //     method, uri, headers, bytes
    // );
    // let response = client
    //     .request(method, uri)
    //     .headers(headers)
    //     .body(bytes)
    //     .send()
    //     .await?;
    // println!("response = {:?}", response);

    // let headers: HeaderMap<HeaderValue> = response.headers().clone();

    // let mut ret = Response::new(Full::new(Bytes::from(response.bytes().await?)));

    // headers.iter().for_each(|(key, value)| {
    //     ret.headers_mut().append(key, value.into());
    // });

    // Ok(ret)

    // Ok(Response::new(Full::new(Bytes::from("Hello World!"))))

    // let host = req.uri().host().expect("uri has no host");
    // let port = req.uri().port_u16().unwrap_or(80);
    // let addr = format!("{}:{}", host, port);

    // let stream = TcpStream::connect(addr).await.unwrap();

    // let (mut sender, conn) = hyper::client::conn::http1::Builder::new()
    //     .preserve_header_case(true)
    //     .title_case_headers(true)
    //     .handshake(stream)
    //     .await?;
    // tokio::task::spawn(async move {
    //     if let Err(err) = conn.await {
    //         println!("Connection failed: {:?}", err);
    //     }
    // });

    // let resp = sender.send_request(req).await?;
    // Ok(resp.map(|b| b.boxed()))
}

pub async fn upgradable_proxy(
    req: Request<hyper::body::Incoming>,
    proxy_app: Arc<ProxyApp>,
) -> Result<Response<Full<Bytes>>, Box<dyn std::error::Error + Send + Sync>> {
    // println!(
    //     "req: {:?}, host={:?}, port={:?}, path={:?}",
    //     req,
    //     req.uri().host(),
    //     req.uri().port_u16(),
    //     req.uri().path()
    // );

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
                            eprintln!("server io error: {}", e);
                        };
                    }
                    Err(e) => eprintln!("upgrade error: {}", e),
                }
            });

            // Ok(Response::new(empty()))
            Ok(Response::new(Full::new(Bytes::from(""))))
        } else {
            eprintln!("CONNECT host is not socket addr: {:?}", req.uri());
            // let mut resp = Response::new(full("CONNECT must be to a socket address"));
            // *resp.status_mut() = http::StatusCode::BAD_REQUEST;
            //
            // Ok(resp)

            Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "invalid host").into())
        }
    } else {
        proxy(req, None, proxy_app).await
    }
}

// fn host_addr(uri: &http::Uri) -> Option<String> {
//     uri.authority().and_then(|auth| Some(auth.to_string()))
// }

// fn empty() -> BoxBody<Bytes, hyper::Error> {
//     Empty::<Bytes>::new()
//         .map_err(|never| match never {})
//         .boxed()
// }

// fn full<T: Into<Bytes>>(chunk: T) -> BoxBody<Bytes, hyper::Error> {
//     Full::new(chunk.into())
//         .map_err(|never| match never {})
//         .boxed()
// }

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
            // println!("Failed to serve connection: {:?}", err);
        }
    });

    Ok(())
}

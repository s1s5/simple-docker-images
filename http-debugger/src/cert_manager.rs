use std::collections::HashMap;
use std::fs;
use std::sync::Arc;

pub struct CertManager {
    root_key_pem: String,
    root_ca: rcgen::Certificate,
    cert_map: HashMap<String, rustls::Certificate>,
}

impl CertManager {
    pub fn new(root_key_path: &str, root_ca_path: &str) -> CertManager {
        let root_key_pem = fs::read_to_string(root_key_path).expect("failed to read root-ca.key");
        let root_ca_params = {
            let root_key =
                rcgen::KeyPair::from_pem(&root_key_pem).expect("failed to parse root key");
            let root_ca = fs::read_to_string(root_ca_path).expect("failed to read root-ca.crt");
            rcgen::CertificateParams::from_ca_cert_pem(&root_ca, root_key)
                .expect("failed to create certificate params")
        };
        let root_ca =
            rcgen::Certificate::from_params(root_ca_params).expect("failed to create root ca");

        CertManager {
            root_key_pem: root_key_pem,
            root_ca: root_ca,
            cert_map: HashMap::new(),
        }
    }

    pub fn get_private_key(&self) -> rustls::PrivateKey {
        rustls::PrivateKey(pem::parse(&self.root_key_pem).unwrap().contents)
    }

    pub fn get_cert(&mut self, domain: &str) -> rustls::Certificate {
        match self.cert_map.get(domain) {
            Some(cert) => cert.clone(),
            None => {
                let mut params: rcgen::CertificateParams = Default::default();
                params.key_pair = Some(rcgen::KeyPair::from_pem(&self.root_key_pem).unwrap());
                // params.alg = &PKCS_ED25519; // server_key.is_compatible
                params.alg = &rcgen::PKCS_RSA_SHA256;
                params.not_before = rcgen::date_time_ymd(1975, 01, 01);
                params.not_after = rcgen::date_time_ymd(4096, 01, 01);
                params.distinguished_name = rcgen::DistinguishedName::new();
                params.distinguished_name.push(
                    rcgen::DnType::OrganizationName,
                    format!("debug for {}", domain),
                );
                params.distinguished_name.push(
                    rcgen::DnType::CommonName,
                    format!("debug cert for {}", domain),
                );
                params.subject_alt_names = vec![rcgen::SanType::DnsName(domain.into())];

                let cert =
                    rcgen::Certificate::from_params(params).expect("from_params for server failed");
                let pem_serialized = cert
                    .serialize_pem_with_signer(&self.root_ca)
                    .expect("serialize pem with signer failed");

                let cert = rustls::Certificate(pem::parse(pem_serialized).unwrap().contents);
                self.cert_map.insert(domain.into(), cert.clone());

                cert
            }
        }
    }

    pub fn get_server_config(&mut self, domain: &str) -> Arc<rustls::ServerConfig> {
        let certs = vec![self.get_cert(domain)];
        let key = self.get_private_key();
        let mut cfg = rustls::ServerConfig::builder()
            .with_safe_defaults()
            .with_no_client_auth()
            .with_single_cert(certs, key)
            .expect("failed to build tls serverconfig");
        // Configure ALPN to accept HTTP/2, HTTP/1.1, and HTTP/1.0 in that order.
        // cfg.alpn_protocols = vec![b"h2".to_vec(), b"http/1.1".to_vec(), b"http/1.0".to_vec()];
        cfg.alpn_protocols = vec![b"http/1.1".to_vec(), b"http/1.0".to_vec()];
        Arc::new(cfg)
    }
}

use reqwest::Proxy;
use std::{collections::HashMap, time::Duration};

pub struct Config {
    pub verbosity: u64,
    pub status_code_config: StatusCodeConfig,
    pub timeout: Option<Duration>,
    pub ignore_cert: bool,
    pub redirect: usize,
    pub proxy_url: Option<String>,
    pub proxy_auth: Option<String>,
    pub retry_limit: u64,
    pub use_get: bool,
    pub use_post: bool,
    pub tty: bool,
}

pub struct BaseConnConfig {
    base_url: String,
    timeout: Option<Duration>,
    ignore_cert: bool,
    proxy: Option<Proxy>,
    retry_limit: Option<u64>,
}

pub enum StatusCodeConfig {
    General(Vec<u16>),
    ByExt(HashMap<String, u16>),
}

//stores blacklist of status codes, or black status code for each file extension.
//.check() returns true if blacklist doesn't match
impl StatusCodeConfig {
    pub fn general() -> Self {
        Self::General(Vec::new())
    }
    pub fn by_ext() -> Self {
        Self::ByExt(HashMap::new())
    }
    pub fn check(&self, code: u16, ext: &str) -> bool {
        match self {
            Self::General(codes) => !codes.contains(&code),
            Self::ByExt(ext_codes) => ext_codes.get(ext).unwrap_or(&0) != &code,
        }
    }
    pub fn insert(&mut self, code: u16, ext: &str) {
        match self {
            Self::General(codes) => {
                codes.push(code);
            }
            Self::ByExt(ext_codes) => {
                ext_codes.insert(ext.to_string(), code);
                ()
            }
        };
    }
    pub fn from_whitelist(codes: Vec<u16>) -> Self {
        Self::General({ 0..=999 }.filter(|x| !codes.contains(x)).collect())
    }
}

impl BaseConnConfig {
    pub fn new(url: &str) -> Self {
        BaseConnConfig {
            base_url: String::from(url),
            timeout: None,
            ignore_cert: false,
            //TODO set proxy url and auth from cli
            proxy: None,
            retry_limit: None,
            //TODO add method field
        }
    }
    pub fn base_url(&self) -> &str {
        &self.base_url
    }
    pub fn set_timeout(&mut self, secs: u64) {
        self.timeout = Some(Duration::from_secs(secs));
    }
    pub fn timeout_opt(&self) -> Option<Duration> {
        self.timeout
    }
    pub fn timeout(&self) -> Duration {
        self.timeout
            .expect("timeout() called on BaseConnConfig but timeout is None")
    }
    pub fn timeout_secs(&self) -> u64 {
        self.timeout
            .expect("timeout_secs() called on BaseConnconfig but timeout is None")
            .as_secs()
    }
    pub fn set_ignore_cert(&mut self, ignore: bool) {
        self.ignore_cert = ignore;
    }
    pub fn ignore_cert(&self) -> bool {
        self.ignore_cert
    }
    pub fn set_retry_limit(&mut self, limit: u64) {
        self.retry_limit = Some(limit);
    }
    pub fn retry_limit_opt(&self) -> Option<u64> {
        self.retry_limit
    }
    pub fn retry_limit(&self) -> u64 {
        self.retry_limit
            .expect("retry_limit() called on BaseConnConfig but retry_limit is None")
    }
    pub fn set_proxy(&mut self, proxy_url: &str, proxy_auth: Option<(&str, &str)>) {
        if proxy_auth.is_some() {
            let proxy_auth = proxy_auth.unwrap();
            self.proxy = Some(
                Proxy::all(proxy_url)
                    .unwrap()
                    .basic_auth(proxy_auth.0, proxy_auth.1),
            );
        } else {
            self.proxy = Some(Proxy::all(proxy_url).unwrap());
        }
    }

    pub fn proxy_opt(&self) -> Option<Proxy> {
        self.proxy.clone()
    }
    pub fn proxy_dump_string(&self) -> String {
        format!("{:?}", self.proxy)
    }
}
